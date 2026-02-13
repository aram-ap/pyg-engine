use crossbeam_channel::Sender;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use std::cell::RefCell;
use std::path::Path;
use std::sync::Arc;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::pump_events::{EventLoopExtPumpEvents, PumpStatus};

use crate::core::logging;

use crate::core::command::EngineCommand;
use crate::core::component::{ComponentTrait, MeshComponent, MeshGeometry, TransformComponent};
use crate::core::draw_manager::DrawCommand;
use crate::core::engine::Engine as RustEngine;
use crate::core::game_object::GameObject as RustGameObject;
use crate::core::input_manager::{MouseAxisBinding, MouseAxisType};
use crate::core::render_manager::CameraAspectMode;
use crate::core::time::Time as RustTime;
use crate::core::ui::{Rect, UIComponentTrait};
use crate::core::ui::button::ButtonComponent;
use crate::core::ui::panel::PanelComponent;
use crate::core::ui::label::{LabelComponent, TextAlign};
use crate::core::window_manager::{FullscreenMode, WindowConfig, load_window_icon_from_path};

// Import bindings from separate modules
use super::color_bind::PyColor;
use super::input_bind::{PyKeys, PyMouseButton, parse_key, parse_mouse_button};
use super::physics_bind::PyCollider;
use super::vector_bind::{PyVec2, PyVec3};

// ========== Engine Bindings ==========

fn parse_mouse_axis_type(axis_name: &str) -> Option<MouseAxisType> {
    match axis_name
        .trim()
        .chars()
        .flat_map(|ch| ch.to_lowercase())
        .filter(|ch| !matches!(ch, ' ' | '_' | '-'))
        .collect::<String>()
        .as_str()
    {
        "x" | "mousex" => Some(MouseAxisType::X),
        "y" | "mousey" => Some(MouseAxisType::Y),
        "wheelx" | "scrollx" | "mousescrollx" => Some(MouseAxisType::WheelX),
        "wheely" | "scroll" | "scrolly" | "mousescrolly" => Some(MouseAxisType::WheelY),
        _ => None,
    }
}

fn parse_camera_aspect_mode(mode_name: &str) -> Option<CameraAspectMode> {
    match mode_name
        .trim()
        .chars()
        .flat_map(|ch| ch.to_lowercase())
        .filter(|ch| !matches!(ch, ' ' | '_' | '-'))
        .collect::<String>()
        .as_str()
    {
        "stretch" | "scale" => Some(CameraAspectMode::Stretch),
        "matchhorizontal" | "horizontal" => Some(CameraAspectMode::MatchHorizontal),
        "matchvertical" | "vertical" => Some(CameraAspectMode::MatchVertical),
        "fitboth" | "fit" | "contain" => Some(CameraAspectMode::FitBoth),
        "fillboth" | "fill" | "cover" => Some(CameraAspectMode::FillBoth),
        _ => None,
    }
}

#[pyclass(name = "CameraAspectMode")]
pub struct PyCameraAspectMode;

#[pymethods]
impl PyCameraAspectMode {
    #[classattr]
    const STRETCH: &'static str = "stretch";
    #[classattr]
    const MATCH_HORIZONTAL: &'static str = "match_horizontal";
    #[classattr]
    const MATCH_VERTICAL: &'static str = "match_vertical";
    #[classattr]
    const FIT_BOTH: &'static str = "fit_both";
    #[classattr]
    const FILL_BOTH: &'static str = "fill_both";
}

/// Python-side draw command builder used for bulk submission.
///
/// `DrawCommand` provides a static API for creating drawing operations that can be batched
/// and submitted to the engine for rendering. All coordinates are in **screen-space pixels**
/// with origin at the **top-left corner (0, 0)**.
///
/// # Coordinate System
/// - **Origin**: Top-left corner (0, 0)
/// - **X-axis**: Increases to the right
/// - **Y-axis**: Increases downward
/// - **Units**: Pixels
///
/// # Draw Order (Z-Index)
/// All draw commands accept a `draw_order` parameter that controls rendering order:
/// - **Higher values** render **on top** (closer to viewer)
/// - **Lower values** render **behind** (further from viewer)
/// - Default: 0.0
/// - Typical ranges:
///   - Background: -10.0 to 0.0
///   - Game content: 0.0 to 5.0
///   - UI elements: 10.0 to 20.0
///
/// # Usage Patterns
///
/// ## Immediate Mode (Direct Drawing)
/// ```python
/// # Draw directly to engine each frame
/// engine.clear_draw_commands()
/// engine.draw_line(0, 0, 100, 100, Color.WHITE, thickness=2.0)
/// engine.draw_circle(50, 50, 25, Color.RED, filled=True)
/// ```
///
/// ## Bulk Submission (DrawCommand API)
/// ```python
/// # Build list of commands and submit in batch
/// commands = [
///     DrawCommand.rectangle(10, 10, 80, 80, Color.BLUE, filled=True, draw_order=1.0),
///     DrawCommand.text("Hello!", 10, 100, 24, Color.WHITE, draw_order=2.0),
///     DrawCommand.circle(50, 50, 20, Color.RED, filled=True, draw_order=3.0),
/// ]
/// engine.add_draw_commands(commands)
/// ```
///
/// # See Also
/// - `examples/python_direct_draw_demo.py` - Direct drawing usage
/// - `examples/python_bulk_draw_demo.py` - Bulk DrawCommand usage
/// - `examples/python_rendering_showcase_demo.py` - Complete rendering examples
#[pyclass(name = "DrawCommand")]
#[derive(Clone)]
pub struct PyDrawCommand {
    inner: DrawCommand,
}

#[pymethods]
impl PyDrawCommand {
    /// Draw a single pixel at the specified position.
    ///
    /// # Arguments
    /// * `x` - Horizontal position in pixels (from left edge)
    /// * `y` - Vertical position in pixels (from top edge)
    /// * `color` - Pixel color (`Color` instance)
    /// * `draw_order` - Rendering layer (default: 0.0, higher = on top)
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// # Draw single pixel
    /// pixel_cmd = pyg.DrawCommand.pixel(100, 100, pyg.Color.WHITE, draw_order=1.0)
    ///
    /// # Draw pixel grid
    /// commands = []
    /// for x in range(0, 200, 10):
    ///     for y in range(0, 200, 10):
    ///         commands.append(pyg.DrawCommand.pixel(x, y, pyg.Color.RED))
    /// engine.add_draw_commands(commands)
    /// ```
    #[staticmethod]
    #[pyo3(signature = (x, y, color, draw_order=0.0))]
    fn pixel(x: u32, y: u32, color: &PyColor, draw_order: f32) -> Self {
        Self {
            inner: DrawCommand::Pixel {
                x: x as f32,
                y: y as f32,
                color: color.inner,
                draw_order,
            },
        }
    }

    /// Draw a line segment between two points.
    ///
    /// # Arguments
    /// * `start_x` - Line start X coordinate in pixels
    /// * `start_y` - Line start Y coordinate in pixels
    /// * `end_x` - Line end X coordinate in pixels
    /// * `end_y` - Line end Y coordinate in pixels
    /// * `color` - Line color (`Color` instance)
    /// * `thickness` - Line width in pixels (default: 1.0)
    /// * `draw_order` - Rendering layer (default: 0.0, higher = on top)
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// # Draw diagonal line
    /// line_cmd = pyg.DrawCommand.line(
    ///     0, 0, 100, 100,
    ///     pyg.Color.WHITE,
    ///     thickness=2.0,
    ///     draw_order=1.0
    /// )
    ///
    /// # Draw grid
    /// commands = []
    /// for i in range(0, 800, 50):
    ///     # Vertical lines
    ///     commands.append(pyg.DrawCommand.line(i, 0, i, 600, pyg.Color.GRAY))
    ///     # Horizontal lines
    ///     commands.append(pyg.DrawCommand.line(0, i, 800, i, pyg.Color.GRAY))
    /// engine.add_draw_commands(commands)
    /// ```
    #[staticmethod]
    #[pyo3(signature = (start_x, start_y, end_x, end_y, color, thickness=1.0, draw_order=0.0))]
    fn line(
        start_x: f32,
        start_y: f32,
        end_x: f32,
        end_y: f32,
        color: &PyColor,
        thickness: f32,
        draw_order: f32,
    ) -> Self {
        Self {
            inner: DrawCommand::Line {
                start_x,
                start_y,
                end_x,
                end_y,
                thickness,
                color: color.inner,
                draw_order,
            },
        }
    }

    /// Draw a rectangle at the specified position.
    ///
    /// The position `(x, y)` represents the **top-left corner** of the rectangle.
    ///
    /// # Arguments
    /// * `x` - Left edge X coordinate in pixels
    /// * `y` - Top edge Y coordinate in pixels
    /// * `width` - Rectangle width in pixels
    /// * `height` - Rectangle height in pixels
    /// * `color` - Rectangle color (`Color` instance)
    /// * `filled` - If `True`, fills the rectangle; if `False`, draws only outline (default: `True`)
    /// * `thickness` - Outline thickness in pixels (only used when `filled=False`, default: 1.0)
    /// * `draw_order` - Rendering layer (default: 0.0, higher = on top)
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// # Filled rectangle
    /// rect_filled = pyg.DrawCommand.rectangle(
    ///     100, 100, 200, 150,
    ///     pyg.Color.BLUE,
    ///     filled=True,
    ///     draw_order=1.0
    /// )
    ///
    /// # Outline rectangle
    /// rect_outline = pyg.DrawCommand.rectangle(
    ///     100, 100, 200, 150,
    ///     pyg.Color.WHITE,
    ///     filled=False,
    ///     thickness=3.0,
    ///     draw_order=2.0
    /// )
    ///
    /// # Draw checkerboard pattern
    /// commands = []
    /// square_size = 40
    /// for row in range(8):
    ///     for col in range(8):
    ///         if (row + col) % 2 == 0:
    ///             commands.append(pyg.DrawCommand.rectangle(
    ///                 col * square_size,
    ///                 row * square_size,
    ///                 square_size,
    ///                 square_size,
    ///                 pyg.Color.BLACK,
    ///                 filled=True
    ///             ))
    /// engine.add_draw_commands(commands)
    /// ```
    #[staticmethod]
    #[pyo3(signature = (x, y, width, height, color, filled=true, thickness=1.0, draw_order=0.0))]
    fn rectangle(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color: &PyColor,
        filled: bool,
        thickness: f32,
        draw_order: f32,
    ) -> Self {
        Self {
            inner: DrawCommand::Rectangle {
                x,
                y,
                width,
                height,
                color: color.inner,
                filled,
                thickness,
                draw_order,
            },
        }
    }

    /// Draw a circle at the specified center position.
    ///
    /// # Arguments
    /// * `center_x` - Circle center X coordinate in pixels
    /// * `center_y` - Circle center Y coordinate in pixels
    /// * `radius` - Circle radius in pixels
    /// * `color` - Circle color (`Color` instance)
    /// * `filled` - If `True`, fills the circle; if `False`, draws only outline (default: `True`)
    /// * `thickness` - Outline thickness in pixels (only used when `filled=False`, default: 1.0)
    /// * `segments` - Number of segments for circle approximation (default: 32, higher = smoother)
    /// * `draw_order` - Rendering layer (default: 0.0, higher = on top)
    ///
    /// # Segments Parameter
    /// The `segments` parameter controls circle quality:
    /// - **Low (8-16)**: Fast, angular/polygonal appearance
    /// - **Medium (32-64)**: Good balance, default quality
    /// - **High (128+)**: Very smooth, more expensive to render
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// # Filled circle
    /// circle_filled = pyg.DrawCommand.circle(
    ///     200, 200, 50,
    ///     pyg.Color.RED,
    ///     filled=True,
    ///     segments=32,
    ///     draw_order=1.0
    /// )
    ///
    /// # Outline circle
    /// circle_outline = pyg.DrawCommand.circle(
    ///     200, 200, 75,
    ///     pyg.Color.WHITE,
    ///     filled=False,
    ///     thickness=3.0,
    ///     segments=64,
    ///     draw_order=2.0
    /// )
    ///
    /// # Draw target reticle
    /// commands = []
    /// center_x, center_y = 400, 300
    /// for radius in [20, 40, 60]:
    ///     commands.append(pyg.DrawCommand.circle(
    ///         center_x, center_y, radius,
    ///         pyg.Color.GREEN,
    ///         filled=False,
    ///         thickness=2.0
    ///     ))
    /// engine.add_draw_commands(commands)
    /// ```
    ///
    /// # Segment Quality Comparison
    /// ```python
    /// # Low quality (fast, angular)
    /// low = pyg.DrawCommand.circle(100, 100, 50, pyg.Color.RED, segments=8)
    ///
    /// # Medium quality (default)
    /// med = pyg.DrawCommand.circle(250, 100, 50, pyg.Color.GREEN, segments=32)
    ///
    /// # High quality (smooth, slower)
    /// high = pyg.DrawCommand.circle(400, 100, 50, pyg.Color.BLUE, segments=128)
    /// ```
    #[staticmethod]
    #[pyo3(signature = (
        center_x,
        center_y,
        radius,
        color,
        filled=true,
        thickness=1.0,
        segments=32,
        draw_order=0.0
    ))]
    fn circle(
        center_x: f32,
        center_y: f32,
        radius: f32,
        color: &PyColor,
        filled: bool,
        thickness: f32,
        segments: u32,
        draw_order: f32,
    ) -> Self {
        Self {
            inner: DrawCommand::Circle {
                center_x,
                center_y,
                radius,
                color: color.inner,
                filled,
                thickness,
                segments,
                draw_order,
            },
        }
    }

    #[staticmethod]
    #[pyo3(signature = (
        x,
        y,
        width,
        height,
        top_left,
        bottom_left,
        bottom_right,
        top_right,
        draw_order=0.0
    ))]
    /// Draw a rectangle with gradient colors at each corner.
    ///
    /// Creates a rectangle with smooth color interpolation between the four corners.
    /// Useful for backgrounds, lighting effects, and visual polish.
    ///
    /// # Arguments
    /// * `x` - Left edge X coordinate in pixels (top-left corner)
    /// * `y` - Top edge Y coordinate in pixels (top-left corner)
    /// * `width` - Rectangle width in pixels
    /// * `height` - Rectangle height in pixels
    /// * `top_left` - Color at top-left corner
    /// * `bottom_left` - Color at bottom-left corner
    /// * `bottom_right` - Color at bottom-right corner
    /// * `top_right` - Color at top-right corner
    /// * `draw_order` - Rendering layer (default: 0.0, higher = on top)
    ///
    /// # Color Interpolation
    /// Colors are smoothly blended across the rectangle using bilinear interpolation.
    /// This creates smooth transitions between corner colors.
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// # Vertical gradient (blue to red)
    /// gradient_v = pyg.DrawCommand.gradient_rect(
    ///     100, 100, 200, 200,
    ///     top_left=pyg.Color.BLUE,
    ///     bottom_left=pyg.Color.RED,
    ///     bottom_right=pyg.Color.RED,
    ///     top_right=pyg.Color.BLUE,
    ///     draw_order=0.0
    /// )
    ///
    /// # Horizontal gradient (green to yellow)
    /// gradient_h = pyg.DrawCommand.gradient_rect(
    ///     350, 100, 200, 200,
    ///     top_left=pyg.Color.GREEN,
    ///     bottom_left=pyg.Color.GREEN,
    ///     bottom_right=pyg.Color.YELLOW,
    ///     top_right=pyg.Color.YELLOW,
    ///     draw_order=0.0
    /// )
    ///
    /// # Radial-style gradient (darker corners, bright center)
    /// dark = pyg.Color.rgb(20, 20, 40)
    /// bright = pyg.Color.rgb(100, 150, 255)
    /// gradient_radial = pyg.DrawCommand.gradient_rect(
    ///     0, 0, 800, 600,
    ///     top_left=dark,
    ///     bottom_left=dark,
    ///     bottom_right=dark,
    ///     top_right=dark,
    ///     draw_order=-10.0
    /// )
    /// ```
    ///
    /// # Lighting Effect Example
    /// ```python
    /// # Simulate top-down lighting
    /// lit = pyg.Color.rgba(255, 255, 200, 255)  # Warm light
    /// shadow = pyg.Color.rgba(50, 50, 80, 255)  # Cool shadow
    ///
    /// ground = pyg.DrawCommand.gradient_rect(
    ///     0, 300, 800, 300,
    ///     top_left=lit,        # Bright at top
    ///     bottom_left=shadow,  # Dark at bottom
    ///     bottom_right=shadow,
    ///     top_right=lit,
    ///     draw_order=0.0
    /// )
    /// ```
    fn gradient_rect(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        top_left: &PyColor,
        bottom_left: &PyColor,
        bottom_right: &PyColor,
        top_right: &PyColor,
        draw_order: f32,
    ) -> Self {
        Self {
            inner: DrawCommand::GradientRect {
                x,
                y,
                width,
                height,
                top_left: top_left.inner,
                bottom_left: bottom_left.inner,
                bottom_right: bottom_right.inner,
                top_right: top_right.inner,
                draw_order,
            },
        }
    }

    /// Draw an image loaded from a file path.
    ///
    /// Loads and renders an image file at the specified position and size. Supports common
    /// image formats (PNG, JPEG, BMP, etc.). Images are automatically cached by path.
    ///
    /// # Arguments
    /// * `x` - Left edge X coordinate in pixels (top-left corner)
    /// * `y` - Top edge Y coordinate in pixels (top-left corner)
    /// * `width` - Display width in pixels (may scale image)
    /// * `height` - Display height in pixels (may scale image)
    /// * `texture_path` - File path to the image (relative or absolute)
    /// * `draw_order` - Rendering layer (default: 0.0, higher = on top)
    ///
    /// # Supported Formats
    /// - PNG (with transparency)
    /// - JPEG / JPG
    /// - BMP
    /// - Other formats supported by the `image` crate
    ///
    /// # Path Resolution
    /// Paths can be:
    /// - **Relative**: `"assets/player.png"` (relative to working directory)
    /// - **Absolute**: `"/home/user/game/sprites/enemy.png"`
    ///
    /// # Caching
    /// Images are automatically cached by file path. Subsequent draws of the same path
    /// reuse the loaded texture, improving performance.
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// # Draw image at original size
    /// sprite = pyg.DrawCommand.image(
    ///     100, 100, 64, 64,
    ///     "assets/player.png",
    ///     draw_order=5.0
    /// )
    ///
    /// # Draw scaled background
    /// background = pyg.DrawCommand.image(
    ///     0, 0, 1920, 1080,
    ///     "assets/background.jpg",
    ///     draw_order=-5.0
    /// )
    ///
    /// # Draw multiple sprites efficiently (cached)
    /// commands = []
    /// for i in range(10):
    ///     commands.append(pyg.DrawCommand.image(
    ///         i * 70, 200, 64, 64,
    ///         "assets/coin.png",  # Loaded once, reused
    ///         draw_order=3.0
    ///     ))
    /// engine.add_draw_commands(commands)
    /// ```
    ///
    /// # Error Handling
    /// If the image file cannot be loaded, an error is logged but rendering continues.
    ///
    /// # See Also
    /// - `image_from_bytes()` - Draw from raw pixel data
    /// - `examples/python_rendering_showcase_demo.py` - Image rendering examples
    #[staticmethod]
    #[pyo3(signature = (x, y, width, height, texture_path, draw_order=0.0))]
    fn image(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        texture_path: String,
        draw_order: f32,
    ) -> Self {
        Self {
            inner: DrawCommand::Image {
                x,
                y,
                width,
                height,
                texture_path,
                draw_order,
            },
        }
    }

    /// Draw an image from raw RGBA pixel data.
    ///
    /// Creates and renders a texture from a byte array of RGBA pixel data. Useful for
    /// procedurally generated textures, video frames, or custom rendering pipelines.
    ///
    /// # Arguments
    /// * `x` - Left edge X coordinate in pixels (top-left corner)
    /// * `y` - Top edge Y coordinate in pixels (top-left corner)
    /// * `width` - Display width in pixels (may scale from texture size)
    /// * `height` - Display height in pixels (may scale from texture size)
    /// * `texture_key` - Unique identifier for caching (e.g., `"procedural_1"`)
    /// * `rgba` - Byte array of RGBA pixel data (must be `texture_width * texture_height * 4` bytes)
    /// * `texture_width` - Source texture width in pixels
    /// * `texture_height` - Source texture height in pixels
    /// * `draw_order` - Rendering layer (default: 0.0, higher = on top)
    ///
    /// # RGBA Data Format
    /// The `rgba` array must contain pixel data in **RGBA format**:
    /// - **R** (red), **G** (green), **B** (blue), **A** (alpha) - each 0-255
    /// - 4 bytes per pixel, row-major order (left-to-right, top-to-bottom)
    /// - Total size: `texture_width × texture_height × 4` bytes
    ///
    /// # Texture Key Caching
    /// Textures are cached by `texture_key`. If the same key is used multiple times:
    /// - First call: Creates and caches the texture
    /// - Subsequent calls: Reuses cached texture (ignores new `rgba` data)
    ///
    /// Use unique keys for different textures: `"frame_0"`, `"frame_1"`, etc.
    ///
    /// # Errors
    /// Returns `PyRuntimeError` if:
    /// - `rgba` length doesn't match `texture_width × texture_height × 4`
    /// - Texture dimensions cause integer overflow
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// # Create a 2x2 checkerboard texture
    /// width, height = 2, 2
    /// rgba = bytes([
    ///     255, 0, 0, 255,    # Red pixel (top-left)
    ///     0, 255, 0, 255,    # Green pixel (top-right)
    ///     0, 0, 255, 255,    # Blue pixel (bottom-left)
    ///     255, 255, 0, 255,  # Yellow pixel (bottom-right)
    /// ])
    ///
    /// # Draw scaled up to 100x100 pixels
    /// texture_cmd = pyg.DrawCommand.image_from_bytes(
    ///     100, 100, 100, 100,
    ///     "checkerboard",  # Unique key for caching
    ///     rgba,
    ///     width, height,
    ///     draw_order=1.0
    /// )
    /// ```
    ///
    /// # Procedural Texture Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// # Generate gradient texture
    /// tex_w, tex_h = 64, 64
    /// rgba = bytearray(tex_w * tex_h * 4)
    ///
    /// for y in range(tex_h):
    ///     for x in range(tex_w):
    ///         idx = (y * tex_w + x) * 4
    ///         rgba[idx + 0] = x * 4       # R increases left-to-right
    ///         rgba[idx + 1] = y * 4       # G increases top-to-bottom
    ///         rgba[idx + 2] = 128         # B constant
    ///         rgba[idx + 3] = 255         # A opaque
    ///
    /// gradient = pyg.DrawCommand.image_from_bytes(
    ///     200, 200, 128, 128,
    ///     "gradient_texture",
    ///     bytes(rgba),
    ///     tex_w, tex_h
    /// )
    /// ```
    ///
    /// # Animated Texture Example
    /// ```python
    /// import pyg_engine as pyg
    /// import time
    ///
    /// frame_count = 0
    ///
    /// def update(dt, engine, data):
    ///     global frame_count
    ///     frame_count += 1
    ///
    ///     # Generate animated texture
    ///     tex_w, tex_h = 32, 32
    ///     rgba = bytearray(tex_w * tex_h * 4)
    ///
    ///     for y in range(tex_h):
    ///         for x in range(tex_w):
    ///             idx = (y * tex_w + x) * 4
    ///             rgba[idx + 0] = (x + frame_count) % 255
    ///             rgba[idx + 1] = (y + frame_count) % 255
    ///             rgba[idx + 2] = 128
    ///             rgba[idx + 3] = 255
    ///
    ///     # Use unique key per frame to avoid caching
    ///     cmd = pyg.DrawCommand.image_from_bytes(
    ///         100, 100, 128, 128,
    ///         f"animated_frame_{frame_count}",  # Unique key
    ///         bytes(rgba),
    ///         tex_w, tex_h
    ///     )
    ///     engine.add_draw_commands([cmd])
    ///     return True
    /// ```
    ///
    /// # See Also
    /// - `image()` - Draw from file path (simpler for static images)
    /// - `examples/python_rendering_showcase_demo.py` - Rendering examples
    #[staticmethod]
    #[pyo3(signature = (
        x,
        y,
        width,
        height,
        texture_key,
        rgba,
        texture_width,
        texture_height,
        draw_order=0.0
    ))]
    fn image_from_bytes(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        texture_key: String,
        rgba: Vec<u8>,
        texture_width: u32,
        texture_height: u32,
        draw_order: f32,
    ) -> PyResult<Self> {
        let expected_size = (texture_width as usize)
            .checked_mul(texture_height as usize)
            .and_then(|value| value.checked_mul(4))
            .ok_or_else(|| {
                PyRuntimeError::new_err("texture size overflow while validating RGBA buffer")
            })?;

        if rgba.len() != expected_size {
            return Err(PyRuntimeError::new_err(format!(
                "texture byte size mismatch for key '{texture_key}': expected {expected_size} bytes ({}x{} RGBA), got {} bytes",
                texture_width,
                texture_height,
                rgba.len()
            )));
        }

        Ok(Self {
            inner: DrawCommand::ImageBytes {
                x,
                y,
                width,
                height,
                texture_key,
                rgba: Arc::from(rgba),
                texture_width,
                texture_height,
                draw_order,
            },
        })
    }

    /// Draw text at the specified position.
    ///
    /// Renders text using a TrueType font. The position `(x, y)` represents the **top-left**
    /// corner of the text's bounding box.
    ///
    /// # Arguments
    /// * `text` - String to render (supports newlines `\n` for multi-line text)
    /// * `x` - Left edge X coordinate in pixels
    /// * `y` - Top edge Y coordinate in pixels (baseline, not top of capital letters)
    /// * `color` - Text color (`Color` instance)
    /// * `font_size` - Font size in pixels (default: 24.0)
    /// * `font_path` - Optional path to TTF font file (default: uses system default font)
    /// * `letter_spacing` - Extra spacing between characters in pixels (default: 0.0)
    /// * `line_spacing` - Extra spacing between lines in pixels (default: 0.0)
    /// * `draw_order` - Rendering layer (default: 0.0, higher = on top)
    ///
    /// # Font Loading
    /// - If `font_path` is `None`, uses the engine's default font
    /// - If `font_path` is provided, loads the specified TTF file
    /// - Fonts are cached by path for performance
    ///
    /// # Newlines
    /// The `text` string can contain newline characters (`\n`) for multi-line text.
    /// Use `line_spacing` to adjust vertical spacing between lines.
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// # Simple text
    /// text_cmd = pyg.DrawCommand.text(
    ///     "Hello, World!",
    ///     100, 100,
    ///     pyg.Color.WHITE,
    ///     font_size=24.0,
    ///     draw_order=10.0
    /// )
    ///
    /// # Large title text
    /// title = pyg.DrawCommand.text(
    ///     "Game Title",
    ///     400, 50,
    ///     pyg.Color.YELLOW,
    ///     font_size=48.0,
    ///     draw_order=15.0
    /// )
    ///
    /// # Multi-line text
    /// multiline = pyg.DrawCommand.text(
    ///     "Line 1\nLine 2\nLine 3",
    ///     50, 200,
    ///     pyg.Color.GREEN,
    ///     font_size=18.0,
    ///     line_spacing=5.0,
    ///     draw_order=10.0
    /// )
    ///
    /// # Custom font
    /// custom = pyg.DrawCommand.text(
    ///     "Custom Font!",
    ///     300, 300,
    ///     pyg.Color.CYAN,
    ///     font_size=32.0,
    ///     font_path="assets/fonts/pixel.ttf",
    ///     draw_order=10.0
    /// )
    /// ```
    ///
    /// # Letter Spacing Example
    /// ```python
    /// # Tight spacing
    /// tight = pyg.DrawCommand.text(
    ///     "TIGHT",
    ///     100, 100,
    ///     pyg.Color.WHITE,
    ///     font_size=24.0,
    ///     letter_spacing=-2.0  # Negative = closer together
    /// )
    ///
    /// # Wide spacing
    /// wide = pyg.DrawCommand.text(
    ///     "W I D E",
    ///     100, 150,
    ///     pyg.Color.WHITE,
    ///     font_size=24.0,
    ///     letter_spacing=10.0  # Positive = further apart
    /// )
    /// ```
    ///
    /// # Dynamic Text (Scoreboard)
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// score = 0
    ///
    /// def update(dt, engine, data):
    ///     global score
    ///     score += 1
    ///
    ///     # Render current score
    ///     score_text = pyg.DrawCommand.text(
    ///         f"Score: {score}",
    ///         10, 10,
    ///         pyg.Color.WHITE,
    ///         font_size=20.0,
    ///         draw_order=100.0  # Always on top
    ///     )
    ///     engine.add_draw_commands([score_text])
    ///     return True
    /// ```
    ///
    /// # See Also
    /// - `examples/python_rendering_showcase_demo.py` - Text rendering examples
    /// - `Label` component - UI text with alignment options
    #[staticmethod]
    #[pyo3(signature = (
        text,
        x,
        y,
        color,
        font_size=24.0,
        font_path=None,
        letter_spacing=0.0,
        line_spacing=0.0,
        draw_order=0.0
    ))]
    fn text(
        text: String,
        x: f32,
        y: f32,
        color: &PyColor,
        font_size: f32,
        font_path: Option<String>,
        letter_spacing: f32,
        line_spacing: f32,
        draw_order: f32,
    ) -> Self {
        Self {
            inner: DrawCommand::Text {
                text,
                x,
                y,
                font_size,
                color: color.inner,
                font_path,
                letter_spacing,
                line_spacing,
                draw_order,
            },
        }
    }
}

/// Python wrapper for the Rust Engine.
#[pyclass(name = "Engine", unsendable)]
pub struct PyEngine {
    inner: RustEngine,
    event_loop: Option<EventLoop<()>>,
}

impl PyEngine {
    #[allow(clippy::too_many_arguments)]
    fn build_window_config(
        title: String,
        width: u32,
        height: u32,
        resizable: bool,
        background_color: Option<PyColor>,
        vsync: bool,
        redraw_on_change_only: bool,
        show_fps_in_title: bool,
        icon_path: Option<String>,
        min_width: Option<u32>,
        min_height: Option<u32>,
    ) -> PyResult<WindowConfig> {
        let mut config = WindowConfig::new()
            .with_title(title)
            .with_size(width, height)
            .with_resizable(resizable)
            .with_fullscreen(FullscreenMode::None)
            .with_vsync(vsync)
            .with_redraw_on_change_only(redraw_on_change_only)
            .with_show_fps_in_title(show_fps_in_title);

        if let Some(color) = background_color {
            config = config.with_background_color(color.inner);
        }

        if let Some(icon_path_value) = icon_path {
            let icon = load_window_icon_from_path(Path::new(&icon_path_value))
                .map_err(PyRuntimeError::new_err)?;
            config = config.with_icon(icon);
        }

        if let (Some(min_w), Some(min_h)) = (min_width, min_height) {
            config = config.with_min_size(min_w, min_h);
        }

        Ok(config)
    }
}

#[pymethods]
impl PyEngine {
    /// Create a new Engine instance with default logging (console only, INFO level).
    #[new]
    #[pyo3(signature = (enable_file_logging=false, log_directory=None, log_level=None))]
    fn new(
        enable_file_logging: bool,
        log_directory: Option<String>,
        log_level: Option<String>,
    ) -> Self {
        let inner = if enable_file_logging || log_directory.is_some() || log_level.is_some() {
            RustEngine::with_logging(enable_file_logging, log_directory, log_level)
        } else {
            RustEngine::new()
        };

        Self {
            inner,
            event_loop: None,
        }
    }

    /// Get a thread-safe handle to the engine that can be passed to other threads.
    fn get_handle(&self) -> PyEngineHandle {
        PyEngineHandle {
            sender: self.inner.get_command_sender(),
        }
    }

    /// Initialize the engine with window configuration without starting the loop.
    #[pyo3(signature = (
        title="PyG Engine".to_string(),
        width=1280,
        height=720,
        resizable=true,
        background_color=None,
        vsync=true,
        redraw_on_change_only=true,
        show_fps_in_title=false,
        icon_path=None,
        min_width=None,
        min_height=None
    ))]
    fn initialize(
        &mut self,
        title: String,
        width: u32,
        height: u32,
        resizable: bool,
        background_color: Option<PyColor>,
        vsync: bool,
        redraw_on_change_only: bool,
        show_fps_in_title: bool,
        icon_path: Option<String>,
        min_width: Option<u32>,
        min_height: Option<u32>,
    ) -> PyResult<()> {
        let config = Self::build_window_config(
            title,
            width,
            height,
            resizable,
            background_color,
            vsync,
            redraw_on_change_only,
            show_fps_in_title,
            icon_path,
            min_width,
            min_height,
        )?;

        self.inner.set_window_config(config);
        self.inner.set_auto_step_on_redraw(false);

        // Create the event loop here.
        // On macOS, force regular activation/menu policy so OS fullscreen behavior
        // (menu bar reveal, focus transitions) matches native app expectations.
        let event_loop = {
            #[cfg(target_os = "macos")]
            {
                use winit::platform::macos::{ActivationPolicy, EventLoopBuilderExtMacOS};

                let mut builder = EventLoop::builder();
                builder.with_activation_policy(ActivationPolicy::Regular);
                builder.with_default_menu(true);
                builder
                    .build()
                    .map_err(|e| PyRuntimeError::new_err(e.to_string()))?
            }
            #[cfg(not(target_os = "macos"))]
            {
                EventLoop::new().map_err(|e| PyRuntimeError::new_err(e.to_string()))?
            }
        };
        #[cfg(target_os = "macos")]
        event_loop.set_control_flow(ControlFlow::Wait);
        #[cfg(not(target_os = "macos"))]
        event_loop.set_control_flow(ControlFlow::Poll);
        self.event_loop = Some(event_loop);

        Ok(())
    }

    /// Poll events from the window system. Returns True if the loop should continue, False if exit requested.
    fn poll_events(&mut self) -> PyResult<bool> {
        if let Some(event_loop) = &mut self.event_loop {
            let status: PumpStatus = event_loop.pump_app_events(None, &mut self.inner);
            match status {
                PumpStatus::Continue => Ok(true),
                PumpStatus::Exit(_) => Ok(false),
            }
        } else {
            Err(PyRuntimeError::new_err(
                "Engine not initialized. Call start_manual() first.",
            ))
        }
    }

    /// Run a single update step.
    fn update(&mut self) {
        self.inner.update();
    }

    /// Render a single frame.
    fn render(&mut self) {
        self.inner.render();
    }

    /// Set the window title.
    #[pyo3(signature = (title))]
    fn set_window_title(&mut self, title: String) {
        self.inner.set_window_title(title);
    }

    /// Set the window icon from an image path.
    ///
    /// In manual mode this can update an already-created window at runtime.
    /// In initialized-but-not-yet-resumed mode this updates pending window config.
    fn set_window_icon(&mut self, icon_path: String) -> PyResult<()> {
        let icon =
            load_window_icon_from_path(Path::new(&icon_path)).map_err(PyRuntimeError::new_err)?;
        self.inner.set_window_icon(icon);
        Ok(())
    }

    /// Get the current display size (window client size) in pixels.
    fn get_display_size(&self) -> (u32, u32) {
        self.inner.get_display_size()
    }

    /// Run the engine with a basic window configuration (blocking).
    #[pyo3(signature = (
        title="PyG Engine".to_string(),
        width=1280,
        height=720,
        resizable=true,
        background_color=None,
        vsync=true,
        redraw_on_change_only=true,
        show_fps_in_title=false,
        icon_path=None,
        min_width=None,
        min_height=None
    ))]
    fn run(
        &mut self,
        title: String,
        width: u32,
        height: u32,
        resizable: bool,
        background_color: Option<PyColor>,
        vsync: bool,
        redraw_on_change_only: bool,
        show_fps_in_title: bool,
        icon_path: Option<String>,
        min_width: Option<u32>,
        min_height: Option<u32>,
    ) -> PyResult<()> {
        let config = Self::build_window_config(
            title,
            width,
            height,
            resizable,
            background_color,
            vsync,
            redraw_on_change_only,
            show_fps_in_title,
            icon_path,
            min_width,
            min_height,
        )?;

        self.inner.set_auto_step_on_redraw(true);
        self.inner
            .run(config)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }

    /// Add a GameObject to the engine.
    ///
    /// The object is copied into the runtime scene using current transform + mesh state.
    fn add_game_object(&mut self, game_object: &PyGameObject) -> Option<u32> {
        let runtime_obj = game_object.to_runtime_game_object();
        let object_id = self.inner.add_game_object(runtime_obj);

        if let Some(id) = object_id {
            game_object.bind_runtime(self.inner.get_command_sender(), id);
        }
        object_id
    }

    /// Create and add a default GameObject (or named one) to the runtime scene.
    #[pyo3(signature = (name=None))]
    fn create_game_object(&mut self, name: Option<String>) -> Option<u32> {
        if let Some(name) = name {
            self.inner.create_game_object_named(name)
        } else {
            self.inner.create_game_object()
        }
    }

    /// Remove a runtime GameObject by id.
    fn remove_game_object(&mut self, object_id: u32) {
        self.inner.remove_game_object(object_id);
    }

    /// Update a runtime GameObject's position by id.
    fn set_game_object_position(&mut self, object_id: u32, position: PyVec2) -> bool {
        self.inner
            .set_game_object_position(object_id, position.inner)
    }

    /// Get the active camera object id.
    fn camera_object_id(&self) -> Option<u32> {
        self.inner.active_camera_object_id()
    }

    /// Get the active camera world position.
    ///
    /// Returns the center point of the camera in **world-space coordinates**. This determines
    /// which portion of the world is visible on screen. The camera "looks at" this position,
    /// making it the center of the viewport.
    ///
    /// # Returns
    /// `Vec2` representing the camera's center position in world units
    ///
    /// # Coordinate System
    /// **World-space coordinates:**
    /// - Origin: World center at (0, 0)
    /// - X-axis: Increases to the right
    /// - Y-axis: Increases upward (opposite of screen-space)
    /// - Units: Arbitrary world units (not pixels)
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// engine = pyg.Engine()
    /// engine.start_manual()
    ///
    /// # Get current camera position
    /// cam_pos = engine.get_camera_position()
    /// print(f"Camera at world position: {cam_pos}")  # Vec2(x, y)
    ///
    /// # Camera starts at (0, 0) by default
    /// engine.set_camera_position(pyg.Vec2(10.0, 5.0))
    /// new_pos = engine.get_camera_position()
    /// print(f"New camera position: {new_pos}")  # Vec2(10.0, 5.0)
    /// ```
    ///
    /// # See Also
    /// - `examples/python_camera_worldspace_demo.py` - Camera positioning and movement
    /// - `set_camera_position()` - Move the camera
    /// - `world_to_screen()` / `screen_to_world()` - Coordinate conversion
    /// - `set_camera_viewport_size()` - Control zoom level
    fn get_camera_position(&self) -> PyVec2 {
        PyVec2 {
            inner: self.inner.get_camera_position(),
        }
    }

    /// Set the active camera world position.
    ///
    /// Moves the camera to look at a specific point in **world-space coordinates**. The camera
    /// centers on this position, making it the focal point of the viewport. Use this for
    /// following players, panning maps, or implementing camera controls.
    ///
    /// # Arguments
    /// * `position` - `Vec2` representing the camera's new center position in world units
    ///
    /// # Returns
    /// `true` if the position was set successfully, `false` otherwise
    ///
    /// # Coordinate System
    /// **World-space coordinates:**
    /// - Origin: World center at (0, 0)
    /// - X-axis: Increases to the right
    /// - Y-axis: Increases upward (opposite of screen-space)
    /// - Units: Arbitrary world units (not pixels)
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// engine = pyg.Engine()
    /// engine.start_manual(title="Camera Movement Demo")
    ///
    /// # Set initial viewport size (controls zoom)
    /// engine.set_camera_viewport_size(24.0, 13.5)
    ///
    /// camera_x = 0.0
    /// camera_y = 0.0
    /// camera_speed = 5.0
    ///
    /// while engine.poll_events():
    ///     dt = engine.delta_time
    ///
    ///     # Move camera with arrow keys
    ///     move_x = engine.input.axis("Horizontal")
    ///     move_y = engine.input.axis("Vertical")
    ///
    ///     camera_x += move_x * camera_speed * dt
    ///     camera_y += move_y * camera_speed * dt
    ///
    ///     # Update camera position
    ///     engine.set_camera_position(pyg.Vec2(camera_x, camera_y))
    ///
    ///     engine.update()
    ///     engine.render()
    /// ```
    ///
    /// # Camera Following
    /// ```python
    /// # Follow a player GameObject
    /// player_pos = player_object.position
    /// engine.set_camera_position(player_pos)
    ///
    /// # Smooth camera following (lerp)
    /// current_cam = engine.get_camera_position()
    /// target_cam = player_pos
    /// smooth_factor = 5.0 * dt
    /// new_cam = current_cam.lerp(target_cam, smooth_factor)
    /// engine.set_camera_position(new_cam)
    /// ```
    ///
    /// # See Also
    /// - `examples/python_camera_worldspace_demo.py` - Complete camera control example
    /// - `get_camera_position()` - Query current position
    /// - `set_camera_viewport_size()` - Control zoom/visible area
    /// - `screen_to_world()` - Convert mouse position for camera panning
    fn set_camera_position(&mut self, position: PyVec2) -> bool {
        self.inner.set_camera_position(position.inner)
    }

    /// Get the active camera viewport size in world units.
    ///
    /// Returns the width and height of the visible area in **world units**, which determines
    /// the camera's zoom level. Smaller values = zoomed in (less world visible),
    /// larger values = zoomed out (more world visible).
    ///
    /// # Returns
    /// Tuple `(width, height)` in world units representing the visible area
    ///
    /// # Relationship to Zoom
    /// - **Smaller viewport** = More zoomed in (e.g., 10 x 5.625 world units)
    /// - **Larger viewport** = More zoomed out (e.g., 50 x 28.125 world units)
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// engine = pyg.Engine()
    /// engine.start_manual()
    ///
    /// # Set viewport to 24 x 13.5 world units
    /// engine.set_camera_viewport_size(24.0, 13.5)
    ///
    /// # Get current viewport size
    /// width, height = engine.get_camera_viewport_size()
    /// print(f"Visible area: {width} x {height} world units")
    ///
    /// # Calculate zoom level relative to base size
    /// base_width = 24.0
    /// zoom_level = base_width / width
    /// print(f"Zoom: {zoom_level:.2f}x")
    /// ```
    ///
    /// # See Also
    /// - `examples/python_camera_worldspace_demo.py` - Viewport and zoom control
    /// - `set_camera_viewport_size()` - Change zoom level
    /// - `set_camera_aspect_mode()` - Control aspect ratio handling
    fn get_camera_viewport_size(&self) -> (f32, f32) {
        self.inner.camera_viewport_size()
    }

    /// Set the active camera viewport size in world units.
    ///
    /// Controls how much of the world is visible on screen, effectively setting the zoom level.
    /// The viewport size defines a rectangular area in **world units** that will be mapped to
    /// the screen dimensions.
    ///
    /// # Arguments
    /// * `width` - Horizontal size of visible area in world units
    /// * `height` - Vertical size of visible area in world units
    ///
    /// # Returns
    /// `true` if the viewport size was set successfully, `false` otherwise
    ///
    /// # Zoom Relationship
    /// - **Smaller values** = Zoomed in (less world visible, objects appear larger)
    /// - **Larger values** = Zoomed out (more world visible, objects appear smaller)
    ///
    /// # Aspect Ratio Handling
    /// The aspect ratio between `width` and `height` interacts with `set_camera_aspect_mode()`:
    /// - **"stretch"** - Ignores aspect ratio, stretches to fit window
    /// - **"fit_both"** - Fits entire viewport, may show more than specified (letterboxing)
    /// - **"fill_both"** - Fills window, may show less than specified (cropping)
    /// - **"match_horizontal"** - Uses specified width, adjusts height to window aspect
    /// - **"match_vertical"** - Uses specified height, adjusts width to window aspect
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// engine = pyg.Engine()
    /// engine.start_manual(title="Viewport Zoom Demo", width=1280, height=720)
    ///
    /// # Set viewport to 24 x 13.5 world units (16:9 aspect ratio)
    /// engine.set_camera_viewport_size(24.0, 13.5)
    /// engine.set_camera_aspect_mode(pyg.CameraAspectMode.FIT_BOTH)
    ///
    /// zoom = 1.0
    /// base_width = 24.0
    /// base_height = 13.5
    ///
    /// while engine.poll_events():
    ///     # Zoom with mouse wheel
    ///     wheel_x, wheel_y = engine.input.mouse_wheel
    ///     if wheel_y != 0:
    ///         zoom += wheel_y * 0.1
    ///         zoom = max(0.5, min(3.0, zoom))  # Clamp 0.5x to 3x
    ///
    ///         # Apply zoom by adjusting viewport size
    ///         new_width = base_width / zoom
    ///         new_height = base_height / zoom
    ///         engine.set_camera_viewport_size(new_width, new_height)
    ///
    ///         print(f"Zoom: {zoom:.2f}x, Viewport: {new_width:.1f} x {new_height:.1f}")
    ///
    ///     engine.update()
    ///     engine.render()
    /// ```
    ///
    /// # Common Viewport Sizes
    /// ```python
    /// # 16:9 aspect ratio variants
    /// engine.set_camera_viewport_size(24.0, 13.5)    # Standard
    /// engine.set_camera_viewport_size(48.0, 27.0)    # Zoomed out 2x
    /// engine.set_camera_viewport_size(12.0, 6.75)    # Zoomed in 2x
    ///
    /// # 4:3 aspect ratio
    /// engine.set_camera_viewport_size(20.0, 15.0)
    ///
    /// # Square viewport
    /// engine.set_camera_viewport_size(20.0, 20.0)
    /// ```
    ///
    /// # See Also
    /// - `examples/python_camera_worldspace_demo.py` - Zoom and viewport control
    /// - `get_camera_viewport_size()` - Query current viewport
    /// - `set_camera_aspect_mode()` - Control aspect ratio behavior
    /// - `set_camera_position()` - Move camera
    fn set_camera_viewport_size(&mut self, width: f32, height: f32) -> bool {
        self.inner.set_camera_viewport_size(width, height)
    }

    /// Get the camera aspect handling mode.
    ///
    /// Returns the current mode that controls how the camera viewport adapts to different
    /// window aspect ratios. This determines whether the viewport stretches, fits, or fills
    /// the window when the window's aspect ratio doesn't match the viewport's.
    ///
    /// # Returns
    /// String representing the current aspect mode. One of:
    /// - `"stretch"` - Stretches viewport to fill window (may distort)
    /// - `"match_horizontal"` - Maintains horizontal viewport width, adjusts height
    /// - `"match_vertical"` - Maintains vertical viewport height, adjusts width
    /// - `"fit_both"` - Fits entire viewport in window (may add letterboxing/pillarboxing)
    /// - `"fill_both"` - Fills window with viewport (may crop edges)
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// engine = pyg.Engine()
    /// engine.start_manual()
    ///
    /// # Check current aspect mode
    /// mode = engine.get_camera_aspect_mode()
    /// print(f"Current aspect mode: {mode}")
    ///
    /// # Cycle through modes
    /// if mode == "fit_both":
    ///     engine.set_camera_aspect_mode(pyg.CameraAspectMode.FILL_BOTH)
    /// ```
    ///
    /// # See Also
    /// - `set_camera_aspect_mode()` - Change aspect mode
    /// - `CameraAspectMode` - Constants for all modes
    fn get_camera_aspect_mode(&self) -> String {
        self.inner.camera_aspect_mode().as_str().to_string()
    }

    /// Set the camera aspect handling mode.
    ///
    /// Controls how the camera viewport adapts when the window aspect ratio doesn't match
    /// the viewport aspect ratio. This determines whether the image stretches, fits with
    /// black bars, or crops to fill the window.
    ///
    /// # Arguments
    /// * `mode` - Aspect mode string (case-insensitive). Valid values:
    ///   - **`"stretch"`** - Stretches viewport to fill window completely
    ///     - Pros: Uses entire window, no black bars
    ///     - Cons: Objects may appear distorted if aspect ratios don't match
    ///     - Use when: Aspect ratio distortion is acceptable
    ///
    ///   - **`"match_horizontal"`** - Keeps viewport width exact, adjusts height to window aspect
    ///     - Pros: Horizontal scale always matches viewport width
    ///     - Cons: May show more/less vertical area than specified
    ///     - Use when: Horizontal measurements are critical (side-scrollers)
    ///
    ///   - **`"match_vertical"`** - Keeps viewport height exact, adjusts width to window aspect
    ///     - Pros: Vertical scale always matches viewport height
    ///     - Cons: May show more/less horizontal area than specified
    ///     - Use when: Vertical measurements are critical
    ///
    ///   - **`"fit_both"`** - Fits entire viewport in window (letterboxing/pillarboxing)
    ///     - Pros: Shows exactly the specified viewport area, no distortion
    ///     - Cons: May add black bars on sides or top/bottom
    ///     - Use when: Exact viewport area must be visible, no more, no less
    ///
    ///   - **`"fill_both"`** - Fills window with viewport (may crop edges)
    ///     - Pros: No black bars, fills entire window
    ///     - Cons: May crop some viewport area at edges
    ///     - Use when: Full window coverage is more important than showing entire viewport
    ///
    /// # Returns
    /// `true` if mode was set successfully, `false` if mode string was invalid
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// engine = pyg.Engine()
    /// engine.start_manual(title="Aspect Mode Demo", width=1280, height=720)
    ///
    /// # Set viewport with 16:9 aspect ratio
    /// engine.set_camera_viewport_size(24.0, 13.5)
    ///
    /// # Fit entire viewport (may add black bars)
    /// engine.set_camera_aspect_mode(pyg.CameraAspectMode.FIT_BOTH)
    ///
    /// # Alternative: Fill window (may crop edges)
    /// # engine.set_camera_aspect_mode(pyg.CameraAspectMode.FILL_BOTH)
    ///
    /// # Allow hotkeys to cycle aspect modes
    /// engine.input.set_action_keys("mode_fit", [pyg.Keys.F1])
    /// engine.input.set_action_keys("mode_fill", [pyg.Keys.F2])
    /// engine.input.set_action_keys("mode_stretch", [pyg.Keys.F3])
    ///
    /// while engine.poll_events():
    ///     if engine.input.action_pressed("mode_fit"):
    ///         engine.set_camera_aspect_mode(pyg.CameraAspectMode.FIT_BOTH)
    ///         print("Mode: Fit Both (letterbox)")
    ///
    ///     if engine.input.action_pressed("mode_fill"):
    ///         engine.set_camera_aspect_mode(pyg.CameraAspectMode.FILL_BOTH)
    ///         print("Mode: Fill Both (crop)")
    ///
    ///     if engine.input.action_pressed("mode_stretch"):
    ///         engine.set_camera_aspect_mode(pyg.CameraAspectMode.STRETCH)
    ///         print("Mode: Stretch (may distort)")
    ///
    ///     engine.update()
    ///     engine.render()
    /// ```
    ///
    /// # Aspect Mode Comparison
    /// Given viewport 24x13.5 (16:9) in window 1280x960 (4:3):
    /// - **stretch**: Viewport stretched to 4:3 (circles become ovals)
    /// - **match_horizontal**: Shows 24 wide, adjusts height to show 18 tall
    /// - **match_vertical**: Shows 13.5 tall, adjusts width to show 18 wide
    /// - **fit_both**: Shows exactly 24x13.5 with black bars on top/bottom
    /// - **fill_both**: Fills window, crops some horizontal area
    ///
    /// # See Also
    /// - `examples/python_camera_worldspace_demo.py` - Aspect mode switching
    /// - `CameraAspectMode` - Constants for all modes
    /// - `get_camera_aspect_mode()` - Query current mode
    /// - `set_camera_viewport_size()` - Set viewport dimensions
    fn set_camera_aspect_mode(&mut self, mode: &str) -> bool {
        let Some(parsed) = parse_camera_aspect_mode(mode) else {
            return false;
        };
        self.inner.set_camera_aspect_mode(parsed)
    }

    /// Set the active camera background clear color.
    fn set_camera_background_color(&mut self, color: &PyColor) {
        self.inner.set_camera_background_color(color.inner);
    }

    /// Get the active camera background clear color.
    fn get_camera_background_color(&self) -> PyColor {
        PyColor {
            inner: self.inner.camera_background_color(),
        }
    }

    /// Convert world-space coordinates to screen-space pixel coordinates.
    ///
    /// Transforms a position from the game world's coordinate system into window pixel
    /// coordinates. Useful for positioning UI elements at world object locations, drawing
    /// debug overlays, or implementing custom rendering.
    ///
    /// # Arguments
    /// * `world_position` - `Vec2` position in world-space coordinates
    ///
    /// # Returns
    /// Tuple `(screen_x, screen_y)` in screen-space pixels:
    /// - `screen_x`: Pixels from left edge of window
    /// - `screen_y`: Pixels from top edge of window
    ///
    /// # Coordinate Systems
    /// **World-space** (input):
    /// - Origin: World center (0, 0)
    /// - Y-axis: Increases upward
    /// - Units: World units
    ///
    /// **Screen-space** (output):
    /// - Origin: Top-left corner (0, 0)
    /// - Y-axis: Increases downward
    /// - Units: Pixels
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// engine = pyg.Engine()
    /// engine.start_manual(title="World to Screen Demo")
    ///
    /// # Set up world camera
    /// engine.set_camera_viewport_size(24.0, 13.5)
    /// engine.set_camera_position(pyg.Vec2(0.0, 0.0))
    ///
    /// # Create a world-space game object
    /// obj = pyg.GameObject("Player")
    /// obj.position = pyg.Vec2(5.0, 3.0)  # World coordinates
    /// engine.add_game_object(obj)
    ///
    /// while engine.poll_events():
    ///     # Convert world position to screen position
    ///     world_pos = obj.position
    ///     screen_x, screen_y = engine.world_to_screen(world_pos)
    ///
    ///     # Draw UI label at object's screen position
    ///     engine.draw_text(
    ///         "Player",
    ///         screen_x,
    ///         screen_y - 30,  # Offset above object
    ///         24,
    ///         pyg.Color.WHITE
    ///     )
    ///
    ///     print(f"World: {world_pos} -> Screen: ({screen_x:.0f}, {screen_y:.0f})")
    ///
    ///     engine.update()
    ///     engine.render()
    /// ```
    ///
    /// # Health Bar Example
    /// ```python
    /// # Draw health bar above enemy in world
    /// enemy_world_pos = enemy.position
    /// screen_x, screen_y = engine.world_to_screen(enemy_world_pos)
    ///
    /// # Health bar is screen-space UI
    /// bar_width = 50
    /// bar_height = 5
    /// health_percent = enemy.health / enemy.max_health
    ///
    /// engine.draw_rectangle(
    ///     screen_x - bar_width / 2,
    ///     screen_y - 40,
    ///     bar_width * health_percent,
    ///     bar_height,
    ///     pyg.Color.GREEN,
    ///     filled=True
    /// )
    /// ```
    ///
    /// # See Also
    /// - `examples/python_camera_worldspace_demo.py` - Coordinate conversion demonstration
    /// - `screen_to_world()` - Inverse transformation
    /// - `mouse_position` - Get mouse in screen-space
    fn world_to_screen(&self, world_position: PyVec2) -> (f32, f32) {
        self.inner.world_to_screen(world_position.inner)
    }

    /// Convert screen-space pixel coordinates to world-space coordinates.
    ///
    /// Transforms window pixel coordinates into the game world's coordinate system. Essential
    /// for mouse interaction with world objects, click-to-move mechanics, and placing objects
    /// at cursor position.
    ///
    /// # Arguments
    /// * `screen_x` - Horizontal pixel position from left edge of window
    /// * `screen_y` - Vertical pixel position from top edge of window
    ///
    /// # Returns
    /// `Vec2` position in world-space coordinates
    ///
    /// # Coordinate Systems
    /// **Screen-space** (input):
    /// - Origin: Top-left corner (0, 0)
    /// - Y-axis: Increases downward
    /// - Units: Pixels
    ///
    /// **World-space** (output):
    /// - Origin: World center (0, 0)
    /// - Y-axis: Increases upward
    /// - Units: World units
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// engine = pyg.Engine()
    /// engine.start_manual(title="Screen to World Demo")
    ///
    /// # Set up world camera
    /// engine.set_camera_viewport_size(24.0, 13.5)
    /// engine.set_camera_position(pyg.Vec2(0.0, 0.0))
    ///
    /// objects = []
    ///
    /// while engine.poll_events():
    ///     # Click to spawn objects at mouse position in world
    ///     if engine.input.mouse_button_pressed(pyg.MouseButton.LEFT):
    ///         # Get mouse position in screen-space
    ///         mx, my = engine.input.mouse_position
    ///
    ///         # Convert to world-space
    ///         world_pos = engine.screen_to_world(mx, my)
    ///
    ///         # Create object at world position
    ///         obj = pyg.GameObject(f"Marker_{len(objects)}")
    ///         obj.position = world_pos
    ///         engine.add_game_object(obj)
    ///         objects.append(obj)
    ///
    ///         print(f"Screen: ({mx}, {my}) -> World: {world_pos}")
    ///
    ///     engine.update()
    ///     engine.render()
    /// ```
    ///
    /// # Mouse Hover Detection
    /// ```python
    /// # Check if mouse is over a world object
    /// mx, my = engine.input.mouse_position
    /// mouse_world = engine.screen_to_world(mx, my)
    ///
    /// for obj in objects:
    ///     distance = (mouse_world - obj.position).length()
    ///     if distance < obj_radius:
    ///         print(f"Mouse over {obj.name}")
    /// ```
    ///
    /// # Camera Panning with Mouse
    /// ```python
    /// # Pan camera by dragging
    /// if engine.input.mouse_button_pressed(pyg.MouseButton.LEFT):
    ///     mx, my = engine.input.mouse_position
    ///     drag_start_world = engine.screen_to_world(mx, my)
    ///     drag_start_cam = engine.get_camera_position()
    ///
    /// if engine.input.mouse_button_down(pyg.MouseButton.LEFT):
    ///     mx, my = engine.input.mouse_position
    ///     current_world = engine.screen_to_world(mx, my)
    ///
    ///     # Calculate camera offset
    ///     drag_delta = drag_start_world - current_world
    ///     new_cam = drag_start_cam + drag_delta
    ///     engine.set_camera_position(new_cam)
    /// ```
    ///
    /// # See Also
    /// - `examples/python_camera_worldspace_demo.py` - Complete camera and coordinate example
    /// - `world_to_screen()` - Inverse transformation
    /// - `mouse_position` - Get mouse coordinates
    /// - `set_camera_position()` - For camera panning
    fn screen_to_world(&self, screen_x: f32, screen_y: f32) -> PyVec2 {
        PyVec2 {
            inner: self.inner.screen_to_world(screen_x, screen_y),
        }
    }

    /// Clear all immediate-mode draw commands.
    fn clear_draw_commands(&mut self) {
        self.inner.clear_draw_commands();
    }

    /// Submit many draw commands in one call.
    fn add_draw_commands(&mut self, py: Python<'_>, commands: Vec<Py<PyDrawCommand>>) {
        let runtime_commands: Vec<DrawCommand> = commands
            .into_iter()
            .map(|command| command.borrow(py).inner.clone())
            .collect();
        self.inner.add_draw_commands(runtime_commands);
    }

    /// Draw a pixel at window coordinates.
    #[pyo3(signature = (x, y, color, draw_order=0.0))]
    fn draw_pixel(&mut self, x: u32, y: u32, color: &PyColor, draw_order: f32) {
        self.inner
            .draw_pixel_with_order(x, y, color.inner, draw_order);
    }

    /// Draw a line at window coordinates.
    #[pyo3(signature = (start_x, start_y, end_x, end_y, color, thickness=1.0, draw_order=0.0))]
    fn draw_line(
        &mut self,
        start_x: f32,
        start_y: f32,
        end_x: f32,
        end_y: f32,
        color: &PyColor,
        thickness: f32,
        draw_order: f32,
    ) {
        self.inner.draw_line_with_options(
            start_x,
            start_y,
            end_x,
            end_y,
            thickness,
            color.inner,
            draw_order,
        );
    }

    /// Draw a rectangle at window coordinates.
    #[pyo3(signature = (x, y, width, height, color, filled=true, thickness=1.0, draw_order=0.0))]
    fn draw_rectangle(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color: &PyColor,
        filled: bool,
        thickness: f32,
        draw_order: f32,
    ) {
        self.inner.draw_rectangle_with_options(
            x,
            y,
            width,
            height,
            color.inner,
            filled,
            thickness,
            draw_order,
        );
    }

    /// Draw a circle at window coordinates.
    #[pyo3(signature = (
        center_x,
        center_y,
        radius,
        color,
        filled=true,
        thickness=1.0,
        segments=32,
        draw_order=0.0
    ))]
    fn draw_circle(
        &mut self,
        center_x: f32,
        center_y: f32,
        radius: f32,
        color: &PyColor,
        filled: bool,
        thickness: f32,
        segments: u32,
        draw_order: f32,
    ) {
        self.inner.draw_circle_with_options(
            center_x,
            center_y,
            radius,
            color.inner,
            filled,
            thickness,
            segments,
            draw_order,
        );
    }

    /// Draw a gradient rectangle with per-corner colors.
    #[pyo3(signature = (
        x,
        y,
        width,
        height,
        top_left,
        bottom_left,
        bottom_right,
        top_right,
        draw_order=0.0
    ))]
    fn draw_gradient_rect(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        top_left: &PyColor,
        bottom_left: &PyColor,
        bottom_right: &PyColor,
        top_right: &PyColor,
        draw_order: f32,
    ) {
        self.inner.draw_gradient_rect_with_options(
            x,
            y,
            width,
            height,
            top_left.inner,
            bottom_left.inner,
            bottom_right.inner,
            top_right.inner,
            draw_order,
        );
    }

    /// Draw an image from a filesystem path at window coordinates.
    #[pyo3(signature = (x, y, width, height, texture_path, draw_order=0.0))]
    fn draw_image(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        texture_path: String,
        draw_order: f32,
    ) {
        self.inner
            .draw_image_with_options(x, y, width, height, texture_path, draw_order);
    }

    /// Draw an image from raw RGBA bytes at window coordinates.
    #[pyo3(signature = (
        x,
        y,
        width,
        height,
        texture_key,
        rgba,
        texture_width,
        texture_height,
        draw_order=0.0
    ))]
    fn draw_image_from_bytes(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        texture_key: String,
        rgba: Vec<u8>,
        texture_width: u32,
        texture_height: u32,
        draw_order: f32,
    ) -> PyResult<()> {
        self.inner
            .draw_image_from_bytes_with_options(
                x,
                y,
                width,
                height,
                texture_key,
                rgba,
                texture_width,
                texture_height,
                draw_order,
            )
            .map_err(PyRuntimeError::new_err)
    }

    /// Draw text in window coordinates. Uses built-in open-source font by default,
    /// or a custom font file when `font_path` is provided.
    #[pyo3(signature = (
        text,
        x,
        y,
        color,
        font_size=24.0,
        font_path=None,
        letter_spacing=0.0,
        line_spacing=0.0,
        draw_order=0.0
    ))]
    fn draw_text(
        &mut self,
        text: String,
        x: f32,
        y: f32,
        color: &PyColor,
        font_size: f32,
        font_path: Option<String>,
        letter_spacing: f32,
        line_spacing: f32,
        draw_order: f32,
    ) {
        self.inner.draw_text_with_options(
            text,
            x,
            y,
            font_size,
            color.inner,
            font_path,
            letter_spacing,
            line_spacing,
            draw_order,
        );
    }

    /// Update a UI label's text at runtime by object ID.
    fn update_ui_label_text(&self, object_id: u32, text: String) {
        let _ = self
            .inner
            .get_command_sender()
            .send(EngineCommand::UpdateUILabelText { object_id, text });
    }

    /// Update a UI button's text at runtime by object ID.
    fn update_ui_button_text(&self, object_id: u32, text: String) {
        let _ = self
            .inner
            .get_command_sender()
            .send(EngineCommand::UpdateUIButtonText { object_id, text });
    }

    /// Log a message at INFO level (default log method).
    fn log(&self, message: &str) {
        self.inner.log(message);
    }

    /// Log a message at TRACE level (most verbose).
    fn log_trace(&self, message: &str) {
        self.inner.log_trace(message);
    }

    /// Log a message at DEBUG level.
    fn log_debug(&self, message: &str) {
        self.inner.log_debug(message);
    }

    /// Log a message at INFO level.
    fn log_info(&self, message: &str) {
        self.inner.log_info(message);
    }

    /// Log a message at WARN level.
    fn log_warn(&self, message: &str) {
        self.inner.log_warn(message);
    }

    /// Log a message at ERROR level.
    fn log_error(&self, message: &str) {
        self.inner.log_error(message);
    }

    /// Get the engine version.
    #[getter]
    fn version(&self) -> String {
        self.inner.version().to_string()
    }

    /// Get the time since the last frame in seconds.
    /// Get time since last frame in **seconds**.
    ///
    /// Returns delta time (dt) - the duration of the previous frame in **seconds**.
    /// Use for frame-rate independent movement, animations, and time-based logic.
    ///
    /// # Time Units: Seconds
    /// **IMPORTANT:** Returns time in **seconds**, not milliseconds.
    ///
    /// # Typical Values
    /// - 60 FPS: ~0.016 seconds (16ms)
    /// - 30 FPS: ~0.033 seconds (33ms)
    /// - 120 FPS: ~0.008 seconds (8ms)
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// engine = pyg.Engine()
    /// player_x = 0.0
    /// speed = 200.0  # pixels per second
    ///
    /// def update(dt, engine, data):
    ///     # dt is delta_time in seconds
    ///     data['x'] += speed * dt  # Move 200 pixels/sec regardless of FPS
    ///
    ///     # Or access directly
    ///     dt_direct = engine.delta_time
    ///     print(f"Frame time: {dt_direct:.4f}s, FPS: {1.0/dt_direct:.1f}")
    ///     return True
    ///
    /// engine.run(update=update, user_data={'x': 0.0})
    /// ```
    ///
    /// # See Also
    /// - `examples/python_input_demo.py` - Movement with delta time
    /// - `Time` class - Standalone time tracking
    #[getter]
    fn delta_time(&self) -> f32 {
        self.inner.time.delta_time()
    }

    /// Get total time since engine start in **seconds**.
    ///
    /// Returns elapsed time - the total duration since `Engine.start_manual()` or `Engine.run()`
    /// was called, measured in **seconds**.
    ///
    /// Useful for time-based effects, periodic events, and timers.
    ///
    /// # Time Units: Seconds
    /// **IMPORTANT:** Returns time in **seconds**, not milliseconds.
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    /// import math
    ///
    /// engine = pyg.Engine()
    /// engine.start_manual()
    ///
    /// while engine.poll_events():
    ///     # Get elapsed time
    ///     t = engine.elapsed_time
    ///
    ///     # Pulsing scale based on time
    ///     scale = 1.0 + 0.3 * math.sin(t * 2.0)
    ///
    ///     # Spawn enemy every 5 seconds
    ///     if int(t) % 5 == 0 and t - int(t) < engine.delta_time:
    ///         spawn_enemy()
    ///
    ///     engine.update()
    ///     engine.render()
    /// ```
    ///
    /// # See Also
    /// - `delta_time` - Time since last frame
    /// - `Time` class - Standalone time tracking
    #[getter]
    fn elapsed_time(&self) -> f32 {
        self.inner.time.elapsed_time()
    }

    // ========== Input Methods ==========

    /// Check if a keyboard key is currently held down.
    ///
    /// Returns `true` every frame while the key remains pressed, starting from the frame
    /// it was pressed until the frame it's released.
    ///
    /// # Arguments
    /// * `key_name` - Key identifier string. Can be:
    ///   - Named keys from the `Keys` class (e.g., `Keys.W`, `Keys.ESCAPE`, `Keys.SPACE`)
    ///   - String literals (e.g., `"w"`, `"escape"`, `"space"`, `"left"`, `"return"`)
    ///   - Case-insensitive, supports aliases (e.g., `"esc"` = `"escape"`)
    ///
    /// # Returns
    /// `true` if the key is currently held down, `false` otherwise
    ///
    /// # Comparison with `key_pressed()`
    /// - `key_down()`: Returns `true` **every frame** while held
    /// - `key_pressed()`: Returns `true` **only on the first frame** when pressed
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// engine = pyg.Engine()
    /// engine.start_manual(title="Key Input Demo")
    ///
    /// while engine.poll_events():
    ///     # Check if space is held down
    ///     if engine.input.key_down(pyg.Keys.SPACE):
    ///         print("Space is being held")  # Prints every frame
    ///
    ///     # Check if W is held (string form)
    ///     if engine.input.key_down("w"):
    ///         print("W key is down")
    ///
    ///     engine.update()
    ///     engine.render()
    /// ```
    ///
    /// # See Also
    /// - `examples/python_input_demo.py` - Complete input system demonstration
    /// - `key_pressed()` - Detect single key press events
    /// - `key_released()` - Detect key release events
    fn key_down(&self, key_name: &str) -> bool {
        if let Some(input) = &self.inner.input_manager {
            input.key_down(&parse_key(key_name))
        } else {
            false
        }
    }

    /// Check if a keyboard key was pressed this frame.
    ///
    /// Returns `true` **only on the first frame** when the key transitions from up to down.
    /// Use this to detect discrete key press events (e.g., jumping, shooting, menu selection).
    ///
    /// # Arguments
    /// * `key_name` - Key identifier string. Can be:
    ///   - Named keys from the `Keys` class (e.g., `Keys.W`, `Keys.ESCAPE`, `Keys.SPACE`)
    ///   - String literals (e.g., `"w"`, `"escape"`, `"space"`, `"left"`, `"return"`)
    ///   - Case-insensitive, supports aliases (e.g., `"esc"` = `"escape"`)
    ///
    /// # Returns
    /// `true` if the key was just pressed this frame, `false` otherwise
    ///
    /// # Comparison with `key_down()`
    /// - `key_pressed()`: Returns `true` **only on the first frame** when pressed
    /// - `key_down()`: Returns `true` **every frame** while held
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// engine = pyg.Engine()
    /// engine.start_manual(title="Key Pressed Demo")
    ///
    /// jump_count = 0
    ///
    /// while engine.poll_events():
    ///     # Detect single space press (won't repeat while held)
    ///     if engine.input.key_pressed(pyg.Keys.SPACE):
    ///         jump_count += 1
    ///         print(f"Jump! (count: {jump_count})")
    ///
    ///     # Detect enter key press
    ///     if engine.input.key_pressed("return"):
    ///         print("Enter pressed - submit action")
    ///
    ///     engine.update()
    ///     engine.render()
    /// ```
    ///
    /// # See Also
    /// - `examples/python_input_demo.py` - Input system demonstration
    /// - `key_down()` - Check if key is currently held
    /// - `key_released()` - Detect key release events
    fn key_pressed(&self, key_name: &str) -> bool {
        if let Some(input) = &self.inner.input_manager {
            input.key_pressed(&parse_key(key_name))
        } else {
            false
        }
    }

    /// Check if a keyboard key was released this frame.
    ///
    /// Returns `true` **only on the first frame** when the key transitions from down to up.
    /// Use this to detect when a user stops pressing a key (e.g., charge-up mechanics,
    /// detecting hold duration).
    ///
    /// # Arguments
    /// * `key_name` - Key identifier string. Can be:
    ///   - Named keys from the `Keys` class (e.g., `Keys.W`, `Keys.ESCAPE`, `Keys.SPACE`)
    ///   - String literals (e.g., `"w"`, `"escape"`, `"space"`, `"left"`, `"return"`)
    ///   - Case-insensitive, supports aliases (e.g., `"esc"` = `"escape"`)
    ///
    /// # Returns
    /// `true` if the key was just released this frame, `false` otherwise
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// engine = pyg.Engine()
    /// engine.start_manual(title="Key Released Demo")
    ///
    /// charge_time = 0.0
    /// charging = False
    ///
    /// while engine.poll_events():
    ///     dt = engine.delta_time
    ///
    ///     # Start charging when space is pressed
    ///     if engine.input.key_pressed(pyg.Keys.SPACE):
    ///         charging = True
    ///         charge_time = 0.0
    ///
    ///     # Accumulate charge time while held
    ///     if engine.input.key_down(pyg.Keys.SPACE):
    ///         charge_time += dt
    ///
    ///     # Release action when space is let go
    ///     if engine.input.key_released(pyg.Keys.SPACE):
    ///         print(f"Released charge: {charge_time:.2f}s")
    ///         charging = False
    ///         charge_time = 0.0
    ///
    ///     engine.update()
    ///     engine.render()
    /// ```
    ///
    /// # See Also
    /// - `examples/python_input_demo.py` - Input system demonstration
    /// - `key_down()` - Check if key is currently held
    /// - `key_pressed()` - Detect key press events
    fn key_released(&self, key_name: &str) -> bool {
        if let Some(input) = &self.inner.input_manager {
            input.key_released(&parse_key(key_name))
        } else {
            false
        }
    }

    /// Check if a mouse button is currently held down.
    ///
    /// Returns `true` every frame while the button remains pressed, starting from the frame
    /// it was pressed until the frame it's released.
    ///
    /// # Arguments
    /// * `button` - Mouse button identifier. Can be:
    ///   - Constants from `MouseButton` class: `MouseButton.LEFT`, `MouseButton.RIGHT`, `MouseButton.MIDDLE`
    ///   - String literals: `"left"`, `"right"`, `"middle"` (case-insensitive)
    ///
    /// # Returns
    /// `true` if the button is currently held down, `false` otherwise
    ///
    /// # Comparison with `mouse_button_pressed()`
    /// - `mouse_button_down()`: Returns `true` **every frame** while held
    /// - `mouse_button_pressed()`: Returns `true` **only on the first frame** when clicked
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// engine = pyg.Engine()
    /// engine.start_manual(title="Mouse Button Demo")
    ///
    /// while engine.poll_events():
    ///     # Check if left button is held (draws continuously)
    ///     if engine.input.mouse_button_down(pyg.MouseButton.LEFT):
    ///         mx, my = engine.input.mouse_position
    ///         engine.draw_circle(mx, my, 10, pyg.Color.RED, filled=True)
    ///         print(f"Dragging at ({mx}, {my})")
    ///
    ///     # String form also works
    ///     if engine.input.mouse_button_down("right"):
    ///         print("Right button held")
    ///
    ///     engine.update()
    ///     engine.render()
    /// ```
    ///
    /// # See Also
    /// - `examples/python_input_demo.py` - Input demonstration including mouse
    /// - `mouse_button_pressed()` - Detect single click events
    /// - `mouse_button_released()` - Detect button release
    /// - `mouse_position` - Get current mouse coordinates
    fn mouse_button_down(&self, button: &str) -> bool {
        if let Some(input) = &self.inner.input_manager {
            input.mouse_button_down(parse_mouse_button(button))
        } else {
            false
        }
    }

    /// Check if a mouse button was pressed this frame.
    ///
    /// Returns `true` **only on the first frame** when the button transitions from up to down.
    /// Use this to detect discrete click events (e.g., shooting, selecting, spawning objects).
    ///
    /// # Arguments
    /// * `button` - Mouse button identifier. Can be:
    ///   - Constants from `MouseButton` class: `MouseButton.LEFT`, `MouseButton.RIGHT`, `MouseButton.MIDDLE`
    ///   - String literals: `"left"`, `"right"`, `"middle"` (case-insensitive)
    ///
    /// # Returns
    /// `true` if the button was just pressed this frame, `false` otherwise
    ///
    /// # Comparison with `mouse_button_down()`
    /// - `mouse_button_pressed()`: Returns `true` **only on the first frame** when clicked
    /// - `mouse_button_down()`: Returns `true` **every frame** while held
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// engine = pyg.Engine()
    /// engine.start_manual(title="Mouse Click Demo")
    ///
    /// while engine.poll_events():
    ///     # Teleport to mouse position on left click
    ///     if engine.input.mouse_button_pressed(pyg.MouseButton.LEFT):
    ///         mx, my = engine.input.mouse_position
    ///         print(f"Clicked at ({mx}, {my})")
    ///
    ///     # Right click for alternative action
    ///     if engine.input.mouse_button_pressed(pyg.MouseButton.RIGHT):
    ///         print("Right click - open context menu")
    ///
    ///     engine.update()
    ///     engine.render()
    /// ```
    ///
    /// # See Also
    /// - `examples/python_input_demo.py` - Complete input demonstration
    /// - `mouse_button_down()` - Check if button is currently held
    /// - `mouse_button_released()` - Detect button release
    /// - `mouse_position` - Get mouse coordinates
    fn mouse_button_pressed(&self, button: &str) -> bool {
        if let Some(input) = &self.inner.input_manager {
            input.mouse_button_pressed(parse_mouse_button(button))
        } else {
            false
        }
    }

    /// Check if a mouse button was released this frame.
    ///
    /// Returns `true` **only on the first frame** when the button transitions from down to up.
    /// Use this to detect when a user stops clicking (e.g., completing drag operations,
    /// finishing drawing strokes).
    ///
    /// # Arguments
    /// * `button` - Mouse button identifier. Can be:
    ///   - Constants from `MouseButton` class: `MouseButton.LEFT`, `MouseButton.RIGHT`, `MouseButton.MIDDLE`
    ///   - String literals: `"left"`, `"right"`, `"middle"` (case-insensitive)
    ///
    /// # Returns
    /// `true` if the button was just released this frame, `false` otherwise
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// engine = pyg.Engine()
    /// engine.start_manual(title="Mouse Release Demo")
    ///
    /// dragging = False
    /// drag_start = (0, 0)
    ///
    /// while engine.poll_events():
    ///     # Start drag on press
    ///     if engine.input.mouse_button_pressed(pyg.MouseButton.LEFT):
    ///         dragging = True
    ///         drag_start = engine.input.mouse_position
    ///
    ///     # Finish drag on release
    ///     if engine.input.mouse_button_released(pyg.MouseButton.LEFT):
    ///         if dragging:
    ///             drag_end = engine.input.mouse_position
    ///             print(f"Dragged from {drag_start} to {drag_end}")
    ///             dragging = False
    ///
    ///     engine.update()
    ///     engine.render()
    /// ```
    ///
    /// # See Also
    /// - `examples/python_input_demo.py` - Input system demonstration
    /// - `mouse_button_down()` - Check if button is currently held
    /// - `mouse_button_pressed()` - Detect button press
    /// - `mouse_position` - Get mouse coordinates
    fn mouse_button_released(&self, button: &str) -> bool {
        if let Some(input) = &self.inner.input_manager {
            input.mouse_button_released(parse_mouse_button(button))
        } else {
            false
        }
    }

    /// Get the current mouse position in window coordinates.
    ///
    /// Returns the mouse cursor position in **screen-space pixels** with origin at the
    /// **top-left corner** of the window (0, 0). Coordinates increase right (X) and down (Y).
    ///
    /// # Returns
    /// Tuple `(x, y)` where:
    /// - `x` (float): Horizontal position from left edge in pixels
    /// - `y` (float): Vertical position from top edge in pixels
    ///
    /// Returns `(0.0, 0.0)` if the mouse position is unavailable.
    ///
    /// # Coordinate System
    /// **Screen-space coordinates:**
    /// - Origin: Top-left corner (0, 0)
    /// - X-axis: Increases to the right
    /// - Y-axis: Increases downward
    /// - Units: Pixels
    ///
    /// For **world-space coordinates** (relative to camera), use `screen_to_world()`.
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// engine = pyg.Engine()
    /// engine.start_manual(title="Mouse Position Demo")
    ///
    /// while engine.poll_events():
    ///     # Get screen-space mouse position
    ///     mx, my = engine.input.mouse_position
    ///
    ///     # Draw a circle at mouse cursor
    ///     engine.clear_draw_commands()
    ///     engine.draw_circle(mx, my, 20, pyg.Color.RED, filled=True)
    ///
    ///     # Convert to world-space for GameObject positioning
    ///     world_pos = engine.screen_to_world(pyg.Vec2(mx, my))
    ///     print(f"Mouse - Screen: ({mx}, {my}), World: {world_pos}")
    ///
    ///     engine.update()
    ///     engine.render()
    /// ```
    ///
    /// # See Also
    /// - `examples/python_input_demo.py` - Mouse input demonstration
    /// - `examples/python_camera_worldspace_demo.py` - Screen/world conversion
    /// - `screen_to_world()` - Convert screen coordinates to world-space
    /// - `mouse_delta` - Get mouse movement since last frame
    fn mouse_position(&self) -> (f64, f64) {
        if let Some(input) = &self.inner.input_manager {
            input.mouse_position()
        } else {
            (0.0, 0.0)
        }
    }

    /// Get the mouse movement delta for this frame.
    ///
    /// Returns the change in mouse position since the last frame in pixels.
    /// Useful for camera controls, mouse look, and drag operations.
    ///
    /// # Returns
    /// Tuple `(dx, dy)` where:
    /// - `dx` (float): Horizontal movement (positive = right)
    /// - `dy` (float): Vertical movement (positive = down)
    ///
    /// Returns `(0.0, 0.0)` if no movement or input unavailable.
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// engine = pyg.Engine()
    /// engine.start_manual(title="Mouse Delta Demo")
    ///
    /// camera_x = 0.0
    /// camera_y = 0.0
    /// sensitivity = 2.0
    ///
    /// while engine.poll_events():
    ///     # Get mouse movement
    ///     dx, dy = engine.input.mouse_delta
    ///
    ///     # Pan camera with mouse movement when button held
    ///     if engine.input.mouse_button_down(pyg.MouseButton.LEFT):
    ///         camera_x -= dx * sensitivity
    ///         camera_y -= dy * sensitivity
    ///         engine.set_camera_position(pyg.Vec2(camera_x, camera_y))
    ///
    ///     engine.update()
    ///     engine.render()
    /// ```
    ///
    /// # See Also
    /// - `examples/python_camera_worldspace_demo.py` - Camera panning with mouse
    /// - `mouse_position` - Get absolute mouse coordinates
    /// - `mouse_wheel` - Get scroll wheel delta
    fn mouse_delta(&self) -> (f64, f64) {
        if let Some(input) = &self.inner.input_manager {
            input.mouse_delta()
        } else {
            (0.0, 0.0)
        }
    }

    /// Get the mouse wheel delta accumulated this frame.
    ///
    /// Returns the scroll wheel movement for the current frame. Typically used for
    /// zooming, scrolling content, or adjusting values.
    ///
    /// # Returns
    /// Tuple `(wheel_x, wheel_y)` where:
    /// - `wheel_x` (float): Horizontal scroll (positive = right, rare on most mice)
    /// - `wheel_y` (float): Vertical scroll (positive = scroll up/away from user)
    ///
    /// Typical values per scroll notch: ±1.0 to ±3.0 (varies by device)
    /// Returns `(0.0, 0.0)` if no scrolling this frame.
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// engine = pyg.Engine()
    /// engine.start_manual(title="Mouse Wheel Demo")
    ///
    /// zoom = 1.0
    ///
    /// while engine.poll_events():
    ///     # Get scroll wheel delta
    ///     wheel_x, wheel_y = engine.input.mouse_wheel
    ///
    ///     # Zoom camera with scroll wheel
    ///     if wheel_y != 0:
    ///         zoom += wheel_y * 0.1
    ///         zoom = max(0.5, min(5.0, zoom))  # Clamp between 0.5x and 5x
    ///         print(f"Zoom level: {zoom:.1f}x")
    ///
    ///         # Apply zoom by adjusting viewport size
    ///         base_viewport = pyg.Vec2(24.0, 13.5)
    ///         engine.set_camera_viewport_size(
    ///             base_viewport.x / zoom,
    ///             base_viewport.y / zoom
    ///         )
    ///
    ///     engine.update()
    ///     engine.render()
    /// ```
    ///
    /// # See Also
    /// - `examples/python_camera_worldspace_demo.py` - Wheel-based camera zoom
    /// - `mouse_delta` - Get mouse cursor movement
    /// - `set_camera_viewport_size()` - Adjust camera zoom
    fn mouse_wheel(&self) -> (f64, f64) {
        if let Some(input) = &self.inner.input_manager {
            input.mouse_wheel()
        } else {
            (0.0, 0.0)
        }
    }

    /// Get the current value of a logical axis.
    ///
    /// Returns a float in the range **[-1.0, 1.0]** representing the axis state.
    /// Axes provide a high-level input abstraction for movement and analog controls,
    /// combining multiple input sources (keyboard keys, mouse axes) into a single value.
    ///
    /// # Built-in Axes
    /// The engine provides **default axes** preconfigured for common game controls:
    ///
    /// - **"Horizontal"** - Left/Right movement
    ///   - Negative (-1.0): `A`, `Left Arrow`
    ///   - Positive (+1.0): `D`, `Right Arrow`
    ///
    /// - **"Vertical"** - Up/Down movement
    ///   - Negative (-1.0): `S`, `Down Arrow`
    ///   - Positive (+1.0): `W`, `Up Arrow`
    ///
    /// # Arguments
    /// * `name` - Axis name (case-sensitive string). Can be built-in axes or custom
    ///   axes created with `set_axis_keys()` or `set_axis_mouse()`.
    ///
    /// # Returns
    /// Float value from **-1.0 to 1.0**:
    /// - `-1.0`: Full negative input (left, down, etc.)
    /// - `0.0`: No input or axis doesn't exist
    /// - `+1.0`: Full positive input (right, up, etc.)
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// engine = pyg.Engine()
    /// engine.start_manual(title="Axis Input Demo")
    ///
    /// player_x = 400.0
    /// player_y = 300.0
    /// speed = 200.0  # pixels per second
    ///
    /// while engine.poll_events():
    ///     dt = engine.delta_time
    ///
    ///     # Read built-in axes for movement
    ///     move_x = engine.input.axis("Horizontal")  # -1.0 to 1.0
    ///     move_y = engine.input.axis("Vertical")    # -1.0 to 1.0
    ///
    ///     # Apply frame-independent movement
    ///     player_x += move_x * speed * dt
    ///     player_y -= move_y * speed * dt  # Invert Y for up=positive
    ///
    ///     # Draw player
    ///     engine.clear_draw_commands()
    ///     engine.draw_circle(player_x, player_y, 20, pyg.Color.GREEN, filled=True)
    ///
    ///     # Show axis values
    ///     if move_x != 0 or move_y != 0:
    ///         print(f"Axis - H: {move_x:.2f}, V: {move_y:.2f}")
    ///
    ///     engine.update()
    ///     engine.render()
    /// ```
    ///
    /// # Custom Axes
    /// Create custom axes for specialized controls:
    /// ```python
    /// # Create a zoom axis with Q/E keys
    /// engine.input.set_axis_keys(
    ///     "CameraZoom",
    ///     positive_keys=[pyg.Keys.E],  # Zoom out
    ///     negative_keys=[pyg.Keys.Q],  # Zoom in
    ///     sensitivity=1.0
    /// )
    ///
    /// # Bind mouse X movement to an axis
    /// engine.input.set_axis_mouse(
    ///     "MouseLookX",
    ///     mouse_axis="x",
    ///     sensitivity=0.5,
    ///     invert=False
    /// )
    ///
    /// zoom_input = engine.input.axis("CameraZoom")
    /// look_input = engine.input.axis("MouseLookX")
    /// ```
    ///
    /// # See Also
    /// - `examples/python_input_demo.py` - Axis-based movement demonstration
    /// - `examples/python_camera_worldspace_demo.py` - Camera control with axes
    /// - `set_axis_keys()` - Create custom keyboard axes
    /// - `set_axis_mouse()` - Bind axes to mouse movement
    /// - `axis_names()` - List all configured axes
    fn axis(&self, name: &str) -> f32 {
        if let Some(input) = &self.inner.input_manager {
            input.axis(name)
        } else {
            0.0
        }
    }

    /// Get the previous frame's value of a logical axis.
    fn axis_previous(&self, name: &str) -> f32 {
        if let Some(input) = &self.inner.input_manager {
            input.axis_previous(name)
        } else {
            0.0
        }
    }

    /// List all configured logical axis names.
    ///
    /// Returns a list of all axis names that have been configured, including
    /// both built-in axes ("Horizontal", "Vertical") and custom axes.
    ///
    /// # Returns
    /// Vector of axis name strings
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// engine = pyg.Engine()
    ///
    /// # Create custom axes
    /// engine.input.set_axis_keys("CameraZoom", [pyg.Keys.E], [pyg.Keys.Q])
    /// engine.input.set_axis_mouse("MouseLookX", "x", 0.5)
    ///
    /// # List all axes
    /// axes = engine.input.axis_names()
    /// print(f"Available axes: {axes}")
    /// # Output: ['Horizontal', 'Vertical', 'CameraZoom', 'MouseLookX']
    /// ```
    fn axis_names(&self) -> Vec<String> {
        if let Some(input) = &self.inner.input_manager {
            input.axis_names()
        } else {
            Vec::new()
        }
    }

    /// Check whether an action is currently active (held).
    ///
    /// Actions provide high-level input abstractions for game events like jumping, shooting,
    /// or pausing. Each action can be bound to multiple keys and mouse buttons, with any of
    /// them activating the action.
    ///
    /// Returns `true` every frame while **any** bound input remains pressed.
    ///
    /// # Built-in Actions
    /// The engine provides **default actions** preconfigured for common controls:
    ///
    /// - **"jump"** - Bound to `Space` key
    /// - **"escape"** - Bound to `Escape` key
    ///
    /// # Arguments
    /// * `action_name` - Action name (case-sensitive). Can be built-in or custom actions
    ///   created with `set_action_keys()` or `set_action_mouse_buttons()`.
    ///
    /// # Returns
    /// `true` if any bound input is currently held down, `false` otherwise
    ///
    /// # Comparison with `action_pressed()`
    /// - `action_down()`: Returns `true` **every frame** while any input is held
    /// - `action_pressed()`: Returns `true` **only on the first frame** when activated
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// engine = pyg.Engine()
    /// engine.start_manual(title="Action Demo")
    ///
    /// # Configure custom action
    /// engine.input.set_action_keys("fire", [pyg.Keys.SPACE, pyg.Keys.CONTROL])
    /// engine.input.set_action_mouse_buttons("fire", [pyg.MouseButton.LEFT])
    ///
    /// while engine.poll_events():
    ///     # Check if fire action is held (any of Space, Ctrl, or Left Mouse)
    ///     if engine.input.action_down("fire"):
    ///         print("Firing continuously!")
    ///
    ///     # Check built-in escape action
    ///     if engine.input.action_down("escape"):
    ///         print("Escape key held")
    ///
    ///     engine.update()
    ///     engine.render()
    /// ```
    ///
    /// # See Also
    /// - `examples/python_input_demo.py` - Action usage demonstration
    /// - `action_pressed()` - Detect single action activation
    /// - `action_released()` - Detect when action is deactivated
    /// - `set_action_keys()` - Bind keys to an action
    /// - `set_action_mouse_buttons()` - Bind mouse buttons to an action
    fn action_down(&self, action_name: &str) -> bool {
        if let Some(input) = &self.inner.input_manager {
            input.action_down(action_name)
        } else {
            false
        }
    }

    /// Check whether an action was pressed this frame.
    ///
    /// Returns `true` **only on the first frame** when any bound input transitions from
    /// up to down. Use this for discrete game events like jumping, toggling menus, or
    /// shooting single shots.
    ///
    /// # Built-in Actions
    /// - **"jump"** - Bound to `Space` key
    /// - **"escape"** - Bound to `Escape` key
    ///
    /// # Arguments
    /// * `action_name` - Action name (case-sensitive)
    ///
    /// # Returns
    /// `true` if any bound input was just pressed this frame, `false` otherwise
    ///
    /// # Comparison with `action_down()`
    /// - `action_pressed()`: Returns `true` **only on the first frame** when activated
    /// - `action_down()`: Returns `true` **every frame** while any input is held
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// engine = pyg.Engine()
    /// engine.start_manual(title="Action Pressed Demo")
    ///
    /// # Configure actions
    /// engine.input.set_action_keys("jump", [pyg.Keys.SPACE, pyg.Keys.W])
    /// engine.input.set_action_keys("pause", [pyg.Keys.ESCAPE, pyg.Keys.P])
    ///
    /// paused = False
    ///
    /// while engine.poll_events():
    ///     # Toggle pause on press (won't repeat while held)
    ///     if engine.input.action_pressed("pause"):
    ///         paused = not paused
    ///         print(f"Game {'paused' if paused else 'resumed'}")
    ///
    ///     if not paused:
    ///         # Single jump per press
    ///         if engine.input.action_pressed("jump"):
    ///             print("Jump!")
    ///
    ///     engine.update()
    ///     engine.render()
    /// ```
    ///
    /// # See Also
    /// - `examples/python_input_demo.py` - Action system demonstration
    /// - `action_down()` - Check if action is currently held
    /// - `action_released()` - Detect action deactivation
    /// - `set_action_keys()` - Bind keys to actions
    fn action_pressed(&self, action_name: &str) -> bool {
        if let Some(input) = &self.inner.input_manager {
            input.action_pressed(action_name)
        } else {
            false
        }
    }

    /// Check whether an action was released this frame.
    ///
    /// Returns `true` **only on the first frame** when **all** bound inputs transition
    /// from down to up. Use this for charge-up mechanics or detecting when the user
    /// stops performing an action.
    ///
    /// # Arguments
    /// * `action_name` - Action name (case-sensitive)
    ///
    /// # Returns
    /// `true` if the action was just released this frame, `false` otherwise
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// engine = pyg.Engine()
    /// engine.start_manual(title="Action Released Demo")
    ///
    /// engine.input.set_action_keys("charge", [pyg.Keys.SPACE])
    ///
    /// charge_time = 0.0
    ///
    /// while engine.poll_events():
    ///     dt = engine.delta_time
    ///
    ///     # Accumulate charge while held
    ///     if engine.input.action_down("charge"):
    ///         charge_time += dt
    ///         print(f"Charging: {charge_time:.2f}s")
    ///
    ///     # Fire when released
    ///     if engine.input.action_released("charge"):
    ///         power = min(charge_time * 100, 300)  # Cap at 300
    ///         print(f"Released! Power: {power:.0f}")
    ///         charge_time = 0.0
    ///
    ///     engine.update()
    ///     engine.render()
    /// ```
    ///
    /// # See Also
    /// - `examples/python_input_demo.py` - Action system demonstration
    /// - `action_down()` - Check if action is held
    /// - `action_pressed()` - Detect action activation
    /// - `set_action_keys()` - Configure action bindings
    fn action_released(&self, action_name: &str) -> bool {
        if let Some(input) = &self.inner.input_manager {
            input.action_released(action_name)
        } else {
            false
        }
    }

    /// List all configured action names.
    ///
    /// Returns a list of all action names that have been configured, including
    /// both built-in actions ("jump", "escape") and custom actions.
    ///
    /// # Returns
    /// Vector of action name strings
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// engine = pyg.Engine()
    ///
    /// # Create custom actions
    /// engine.input.set_action_keys("fire", [pyg.Keys.SPACE])
    /// engine.input.set_action_keys("reload", [pyg.Keys.R])
    ///
    /// # List all actions
    /// actions = engine.input.action_names()
    /// print(f"Available actions: {actions}")
    /// # Output: ['jump', 'escape', 'fire', 'reload']
    /// ```
    fn action_names(&self) -> Vec<String> {
        if let Some(input) = &self.inner.input_manager {
            input.action_names()
        } else {
            Vec::new()
        }
    }

    /// Restore default axis/action bindings.
    ///
    /// Resets all input bindings back to the engine's default configuration:
    /// - "Horizontal" axis: A/D and Left/Right arrow keys
    /// - "Vertical" axis: W/S and Up/Down arrow keys
    /// - "jump" action: Space key
    /// - "escape" action: Escape key
    ///
    /// All custom axes and actions are removed.
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// engine = pyg.Engine()
    ///
    /// # Customize bindings
    /// engine.input.set_axis_keys("Horizontal", [pyg.Keys.L], [pyg.Keys.J])
    /// engine.input.set_action_keys("fire", [pyg.Keys.SPACE])
    ///
    /// # Reset to defaults
    /// engine.input.reset_input_bindings_to_defaults()
    ///
    /// # Now back to default: WASD/Arrows for axes, Space for jump
    /// ```
    fn reset_input_bindings_to_defaults(&mut self) {
        if let Some(input) = &mut self.inner.input_manager {
            input.reset_input_bindings_to_defaults();
        }
    }

    /// Configure keyboard keys for a logical axis.
    ///
    /// Creates or updates a named axis that responds to keyboard input. Multiple keys
    /// can be bound to each direction, and pressing any of them will activate the axis.
    /// If both positive and negative keys are pressed simultaneously, they cancel out to 0.
    ///
    /// # Arguments
    /// * `name` - Axis name (case-sensitive). Will create new axis or overwrite existing.
    /// * `positive_keys` - List of keys that produce positive values (+1.0).
    ///   Can use `Keys` constants or strings (e.g., `[Keys.D, Keys.RIGHT]` or `["d", "right"]`)
    /// * `negative_keys` - List of keys that produce negative values (-1.0).
    ///   Can use `Keys` constants or strings (e.g., `[Keys.A, Keys.LEFT]` or `["a", "left"]`)
    /// * `sensitivity` - Multiplier for axis value (default: 1.0). Use values < 1.0 to reduce
    ///   responsiveness or > 1.0 to amplify (though axis is typically clamped to [-1.0, 1.0])
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// engine = pyg.Engine()
    ///
    /// # Create custom horizontal axis with different keys
    /// engine.input.set_axis_keys(
    ///     "Strafe",
    ///     positive_keys=[pyg.Keys.E, pyg.Keys.RIGHT],  # Move right
    ///     negative_keys=[pyg.Keys.Q, pyg.Keys.LEFT],   # Move left
    ///     sensitivity=1.0
    /// )
    ///
    /// # Create zoom axis
    /// engine.input.set_axis_keys(
    ///     "CameraZoom",
    ///     positive_keys=[pyg.Keys.EQUAL, pyg.Keys.PAGEUP],      # Zoom in
    ///     negative_keys=[pyg.Keys.MINUS, pyg.Keys.PAGEDOWN],    # Zoom out
    ///     sensitivity=1.0
    /// )
    ///
    /// engine.start_manual(title="Custom Axes Demo")
    ///
    /// while engine.poll_events():
    ///     # Read custom axes
    ///     strafe = engine.input.axis("Strafe")
    ///     zoom = engine.input.axis("CameraZoom")
    ///
    ///     print(f"Strafe: {strafe:.2f}, Zoom: {zoom:.2f}")
    ///
    ///     engine.update()
    ///     engine.render()
    /// ```
    ///
    /// # Modifying Built-in Axes
    /// You can reconfigure the default "Horizontal" and "Vertical" axes:
    /// ```python
    /// # Change Horizontal to use J/L keys instead of A/D
    /// engine.input.set_axis_keys(
    ///     "Horizontal",
    ///     positive_keys=[pyg.Keys.L],
    ///     negative_keys=[pyg.Keys.J],
    ///     sensitivity=1.0
    /// )
    /// ```
    ///
    /// # See Also
    /// - `examples/python_camera_worldspace_demo.py` - Custom axis configuration
    /// - `axis()` - Read axis value
    /// - `set_axis_mouse()` - Bind axes to mouse movement
    /// - `add_axis_positive_key()` / `add_axis_negative_key()` - Add keys incrementally
    /// - `remove_axis()` - Delete an axis
    #[pyo3(signature = (name, positive_keys, negative_keys, sensitivity=1.0))]
    fn set_axis_keys(
        &mut self,
        name: &str,
        positive_keys: Vec<String>,
        negative_keys: Vec<String>,
        sensitivity: f32,
    ) {
        if let Some(input) = &mut self.inner.input_manager {
            let positive = positive_keys.iter().map(|k| parse_key(k)).collect();
            let negative = negative_keys.iter().map(|k| parse_key(k)).collect();
            input.set_axis_keyboard_keys(name, positive, negative, sensitivity);
        }
    }

    /// Add one positive key to an axis binding.
    ///
    /// Incrementally adds a key that produces positive axis values (+1.0)
    /// without removing existing keys.
    ///
    /// # Arguments
    /// * `axis_name` - Name of the axis to modify
    /// * `key_name` - Key to add (e.g., `"d"`, `Keys.D`)
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// engine = pyg.Engine()
    ///
    /// # Add extra positive keys to Horizontal axis
    /// engine.input.add_axis_positive_key("Horizontal", pyg.Keys.L)
    /// engine.input.add_axis_positive_key("Horizontal", pyg.Keys.NUMPAD6)
    ///
    /// # Now Horizontal axis responds to: D, Right, L, Numpad6
    /// ```
    fn add_axis_positive_key(&mut self, axis_name: &str, key_name: &str) {
        if let Some(input) = &mut self.inner.input_manager {
            input.add_axis_positive_key(axis_name, parse_key(key_name));
        }
    }

    /// Add one negative key to an axis binding.
    ///
    /// Incrementally adds a key that produces negative axis values (-1.0)
    /// without removing existing keys.
    ///
    /// # Arguments
    /// * `axis_name` - Name of the axis to modify
    /// * `key_name` - Key to add (e.g., `"a"`, `Keys.A`)
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// engine = pyg.Engine()
    ///
    /// # Add extra negative keys to Horizontal axis
    /// engine.input.add_axis_negative_key("Horizontal", pyg.Keys.J)
    /// engine.input.add_axis_negative_key("Horizontal", pyg.Keys.NUMPAD4)
    ///
    /// # Now Horizontal axis responds to: A, Left, J, Numpad4
    /// ```
    fn add_axis_negative_key(&mut self, axis_name: &str, key_name: &str) {
        if let Some(input) = &mut self.inner.input_manager {
            input.add_axis_negative_key(axis_name, parse_key(key_name));
        }
    }

    /// Remove one positive key from an axis binding.
    ///
    /// # Arguments
    /// * `axis_name` - Name of the axis to modify
    /// * `key_name` - Key to remove
    ///
    /// # Returns
    /// `true` if the key was found and removed, `false` otherwise
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// engine = pyg.Engine()
    ///
    /// # Remove arrow key from Horizontal axis (keep WASD only)
    /// if engine.input.remove_axis_positive_key("Horizontal", pyg.Keys.RIGHT):
    ///     print("Removed Right arrow from Horizontal axis")
    /// ```
    fn remove_axis_positive_key(&mut self, axis_name: &str, key_name: &str) -> bool {
        if let Some(input) = &mut self.inner.input_manager {
            input.remove_axis_positive_key(axis_name, &parse_key(key_name))
        } else {
            false
        }
    }

    /// Remove one negative key from an axis binding.
    ///
    /// # Arguments
    /// * `axis_name` - Name of the axis to modify
    /// * `key_name` - Key to remove
    ///
    /// # Returns
    /// `true` if the key was found and removed, `false` otherwise
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// engine = pyg.Engine()
    ///
    /// # Remove arrow key from Horizontal axis
    /// if engine.input.remove_axis_negative_key("Horizontal", pyg.Keys.LEFT):
    ///     print("Removed Left arrow from Horizontal axis")
    /// ```
    fn remove_axis_negative_key(&mut self, axis_name: &str, key_name: &str) -> bool {
        if let Some(input) = &mut self.inner.input_manager {
            input.remove_axis_negative_key(axis_name, &parse_key(key_name))
        } else {
            false
        }
    }

    /// Remove an entire logical axis binding.
    ///
    /// Completely removes an axis, including all key and mouse bindings.
    /// Built-in axes ("Horizontal", "Vertical") can be removed but will
    /// return if `reset_input_bindings_to_defaults()` is called.
    ///
    /// # Arguments
    /// * `axis_name` - Name of the axis to remove
    ///
    /// # Returns
    /// `true` if the axis existed and was removed, `false` otherwise
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// engine = pyg.Engine()
    ///
    /// # Create temporary axis
    /// engine.input.set_axis_keys("TempAxis", [pyg.Keys.T], [pyg.Keys.G])
    ///
    /// # Later remove it
    /// if engine.input.remove_axis("TempAxis"):
    ///     print("TempAxis removed")
    ///
    /// # Axis no longer exists
    /// value = engine.input.axis("TempAxis")  # Returns 0.0
    /// ```
    fn remove_axis(&mut self, axis_name: &str) -> bool {
        if let Some(input) = &mut self.inner.input_manager {
            input.remove_axis(axis_name)
        } else {
            false
        }
    }

    /// Configure a mouse-driven logical axis.
    ///
    /// Creates or updates a named axis that responds to mouse movement or scroll wheel.
    /// Useful for camera controls, mouse look, and zoom functionality.
    ///
    /// # Arguments
    /// * `name` - Axis name (case-sensitive). Will create new axis or overwrite existing.
    /// * `mouse_axis` - Mouse input source (case-insensitive). Valid values:
    ///   - **`"x"`** or **`"mouse_x"`** - Horizontal mouse movement (right = positive)
    ///   - **`"y"`** or **`"mouse_y"`** - Vertical mouse movement (down = positive)
    ///   - **`"wheel_x"`** or **`"scroll_x"`** - Horizontal scroll wheel
    ///   - **`"wheel_y"`**, **`"scroll"`**, or **`"scroll_y"`** - Vertical scroll wheel (up = positive)
    /// * `sensitivity` - Multiplier for mouse input (default: 1.0). Higher values = more responsive.
    ///   Typical values: 0.1 - 2.0
    /// * `invert` - Invert the axis direction (default: false). Set `true` to reverse polarity.
    ///
    /// # Returns
    /// `true` if axis was created successfully, `false` if `mouse_axis` was invalid
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// engine = pyg.Engine()
    ///
    /// # Create mouse look axes
    /// engine.input.set_axis_mouse(
    ///     "MouseLookX",
    ///     mouse_axis="x",
    ///     sensitivity=0.5,
    ///     invert=False
    /// )
    ///
    /// engine.input.set_axis_mouse(
    ///     "MouseLookY",
    ///     mouse_axis="y",
    ///     sensitivity=0.5,
    ///     invert=True  # Invert Y for flight-stick style controls
    /// )
    ///
    /// # Create scroll wheel zoom axis
    /// engine.input.set_axis_mouse(
    ///     "ScrollZoom",
    ///     mouse_axis="wheel_y",
    ///     sensitivity=0.1,
    ///     invert=False
    /// )
    ///
    /// engine.start_manual(title="Mouse Axis Demo")
    ///
    /// camera_x = 0.0
    /// camera_y = 0.0
    /// zoom = 1.0
    ///
    /// while engine.poll_events():
    ///     dt = engine.delta_time
    ///
    ///     # Read mouse axes for camera control
    ///     look_x = engine.input.axis("MouseLookX")
    ///     look_y = engine.input.axis("MouseLookY")
    ///     scroll_zoom = engine.input.axis("ScrollZoom")
    ///
    ///     # Pan camera with mouse (when button held)
    ///     if engine.input.mouse_button_down(pyg.MouseButton.LEFT):
    ///         camera_x -= look_x * 10.0 * dt
    ///         camera_y -= look_y * 10.0 * dt
    ///         engine.set_camera_position(pyg.Vec2(camera_x, camera_y))
    ///
    ///     # Zoom with scroll wheel
    ///     if scroll_zoom != 0:
    ///         zoom += scroll_zoom * 0.5
    ///         zoom = max(0.5, min(3.0, zoom))
    ///         print(f"Zoom: {zoom:.2f}x")
    ///
    ///     engine.update()
    ///     engine.render()
    /// ```
    ///
    /// # Notes
    /// - Mouse axes return **delta values** (change since last frame), not absolute positions
    /// - Mouse movement axes return 0 when the mouse isn't moving
    /// - Scroll wheel axes return 0 when not scrolling
    /// - Multiple axes can bind to the same mouse input with different sensitivity/inversion
    ///
    /// # See Also
    /// - `examples/python_camera_worldspace_demo.py` - Mouse-based camera control
    /// - `axis()` - Read axis value
    /// - `set_axis_keys()` - Bind axes to keyboard keys
    /// - `mouse_delta` - Direct access to mouse movement
    /// - `mouse_wheel` - Direct access to scroll wheel
    #[pyo3(signature = (name, mouse_axis, sensitivity=1.0, invert=false))]
    fn set_axis_mouse(
        &mut self,
        name: &str,
        mouse_axis: &str,
        sensitivity: f32,
        invert: bool,
    ) -> bool {
        let Some(axis_type) = parse_mouse_axis_type(mouse_axis) else {
            return false;
        };
        if let Some(input) = &mut self.inner.input_manager {
            input.set_axis_mouse_binding(
                name,
                MouseAxisBinding {
                    axis: axis_type,
                    sensitivity,
                    invert,
                },
            );
            true
        } else {
            false
        }
    }

    /// Configure keyboard keys for an action.
    ///
    /// Creates or updates a named action that activates when **any** of the specified keys
    /// are pressed. Use actions for discrete game events like jumping, shooting, pausing,
    /// or interacting.
    ///
    /// # Arguments
    /// * `action_name` - Action name (case-sensitive). Will create new or overwrite existing.
    /// * `key_names` - List of keys that activate this action. Pressing **any** key in the
    ///   list will trigger the action. Can use `Keys` constants or strings.
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// engine = pyg.Engine()
    ///
    /// # Create jump action with multiple keys
    /// engine.input.set_action_keys(
    ///     "jump",
    ///     [pyg.Keys.SPACE, pyg.Keys.W, pyg.Keys.UP]
    /// )
    ///
    /// # Create pause action
    /// engine.input.set_action_keys(
    ///     "pause",
    ///     [pyg.Keys.ESCAPE, pyg.Keys.P]
    /// )
    ///
    /// # Create interact action
    /// engine.input.set_action_keys(
    ///     "interact",
    ///     [pyg.Keys.E, pyg.Keys.RETURN]
    /// )
    ///
    /// engine.start_manual(title="Action Keys Demo")
    ///
    /// while engine.poll_events():
    ///     # Any bound key triggers the action
    ///     if engine.input.action_pressed("jump"):
    ///         print("Jump! (Space, W, or Up pressed)")
    ///
    ///     if engine.input.action_pressed("interact"):
    ///         print("Interacting (E or Enter pressed)")
    ///
    ///     engine.update()
    ///     engine.render()
    /// ```
    ///
    /// # Combining with Mouse Buttons
    /// Actions can bind both keys and mouse buttons:
    /// ```python
    /// # Fire action works with Space OR left mouse button
    /// engine.input.set_action_keys("fire", [pyg.Keys.SPACE])
    /// engine.input.set_action_mouse_buttons("fire", [pyg.MouseButton.LEFT])
    /// ```
    ///
    /// # See Also
    /// - `examples/python_camera_worldspace_demo.py` - Action configuration
    /// - `action_pressed()` / `action_down()` - Check action state
    /// - `set_action_mouse_buttons()` - Add mouse button bindings
    /// - `add_action_key()` - Add a key incrementally
    /// - `clear_action_bindings()` - Remove all bindings for an action
    fn set_action_keys(&mut self, action_name: &str, key_names: Vec<String>) {
        if let Some(input) = &mut self.inner.input_manager {
            let keys = key_names.iter().map(|k| parse_key(k)).collect();
            input.set_action_keys(action_name, keys);
        }
    }

    /// Add one keyboard key to an action.
    ///
    /// Incrementally adds a key to an action without removing existing bindings.
    ///
    /// # Arguments
    /// * `action_name` - Name of the action to modify
    /// * `key_name` - Key to add
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// engine = pyg.Engine()
    ///
    /// # Add multiple keys to jump action
    /// engine.input.add_action_key("jump", pyg.Keys.W)
    /// engine.input.add_action_key("jump", pyg.Keys.UP)
    ///
    /// # Now jump triggers on Space, W, or Up arrow
    /// ```
    fn add_action_key(&mut self, action_name: &str, key_name: &str) {
        if let Some(input) = &mut self.inner.input_manager {
            input.add_action_key(action_name, parse_key(key_name));
        }
    }

    /// Remove one keyboard key from an action.
    ///
    /// # Arguments
    /// * `action_name` - Name of the action to modify
    /// * `key_name` - Key to remove
    ///
    /// # Returns
    /// `true` if the key was found and removed, `false` otherwise
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// engine = pyg.Engine()
    ///
    /// # Remove Space from jump action
    /// if engine.input.remove_action_key("jump", pyg.Keys.SPACE):
    ///     print("Space key removed from jump")
    /// ```
    fn remove_action_key(&mut self, action_name: &str, key_name: &str) -> bool {
        if let Some(input) = &mut self.inner.input_manager {
            input.remove_action_key(action_name, &parse_key(key_name))
        } else {
            false
        }
    }

    /// Configure mouse buttons for an action.
    ///
    /// Creates or updates the mouse button bindings for a named action. Pressing **any**
    /// of the specified mouse buttons will activate the action. This complements keyboard
    /// bindings set with `set_action_keys()`.
    ///
    /// # Arguments
    /// * `action_name` - Action name (case-sensitive)
    /// * `buttons` - List of mouse buttons that activate this action. Can use `MouseButton`
    ///   constants or strings (e.g., `[MouseButton.LEFT, MouseButton.RIGHT]` or `["left", "right"]`)
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// engine = pyg.Engine()
    ///
    /// # Create fire action with mouse buttons
    /// engine.input.set_action_mouse_buttons(
    ///     "fire",
    ///     [pyg.MouseButton.LEFT]
    /// )
    ///
    /// # Create alternate fire with right mouse
    /// engine.input.set_action_mouse_buttons(
    ///     "alternate_fire",
    ///     [pyg.MouseButton.RIGHT, pyg.MouseButton.MIDDLE]
    /// )
    ///
    /// # Combine with keyboard for same action
    /// engine.input.set_action_keys("fire", [pyg.Keys.SPACE, pyg.Keys.CONTROL])
    ///
    /// engine.start_manual(title="Mouse Action Demo")
    ///
    /// while engine.poll_events():
    ///     # Triggers with Space, Ctrl, OR Left Mouse
    ///     if engine.input.action_pressed("fire"):
    ///         print("Fire!")
    ///
    ///     # Triggers with Right Mouse OR Middle Mouse
    ///     if engine.input.action_pressed("alternate_fire"):
    ///         print("Alternate fire!")
    ///
    ///     engine.update()
    ///     engine.render()
    /// ```
    ///
    /// # See Also
    /// - `examples/python_input_demo.py` - Mouse button usage
    /// - `set_action_keys()` - Bind keyboard keys to actions
    /// - `action_pressed()` / `action_down()` - Check action state
    /// - `add_action_mouse_button()` - Add a button incrementally
    fn set_action_mouse_buttons(&mut self, action_name: &str, buttons: Vec<String>) {
        if let Some(input) = &mut self.inner.input_manager {
            let mapped = buttons
                .iter()
                .map(|button| parse_mouse_button(button))
                .collect();
            input.set_action_mouse_buttons(action_name, mapped);
        }
    }

    /// Add one mouse button to an action.
    ///
    /// Incrementally adds a mouse button to an action without removing existing bindings.
    ///
    /// # Arguments
    /// * `action_name` - Name of the action to modify
    /// * `button` - Mouse button to add (e.g., `MouseButton.LEFT`, `"left"`)
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// engine = pyg.Engine()
    ///
    /// # Add mouse buttons to fire action
    /// engine.input.add_action_mouse_button("fire", pyg.MouseButton.LEFT)
    /// engine.input.add_action_mouse_button("fire", pyg.MouseButton.MIDDLE)
    ///
    /// # Now fire triggers on Space, Left click, or Middle click
    /// ```
    fn add_action_mouse_button(&mut self, action_name: &str, button: &str) {
        if let Some(input) = &mut self.inner.input_manager {
            input.add_action_mouse_button(action_name, parse_mouse_button(button));
        }
    }

    /// Remove one mouse button from an action.
    ///
    /// # Arguments
    /// * `action_name` - Name of the action to modify
    /// * `button` - Mouse button to remove
    ///
    /// # Returns
    /// `true` if the button was found and removed, `false` otherwise
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// engine = pyg.Engine()
    ///
    /// # Remove right click from an action
    /// if engine.input.remove_action_mouse_button("fire", pyg.MouseButton.RIGHT):
    ///     print("Right click removed from fire action")
    /// ```
    fn remove_action_mouse_button(&mut self, action_name: &str, button: &str) -> bool {
        if let Some(input) = &mut self.inner.input_manager {
            input.remove_action_mouse_button(action_name, parse_mouse_button(button))
        } else {
            false
        }
    }

    /// Clear all bindings for an action.
    ///
    /// Removes all keyboard and mouse button bindings from an action.
    /// The action still exists but has no triggers.
    ///
    /// # Arguments
    /// * `action_name` - Name of the action to clear
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// engine = pyg.Engine()
    ///
    /// # Clear all jump bindings
    /// engine.input.clear_action_bindings("jump")
    ///
    /// # Re-bind to different keys
    /// engine.input.set_action_keys("jump", [pyg.Keys.W, pyg.Keys.UP])
    /// ```
    fn clear_action_bindings(&mut self, action_name: &str) {
        if let Some(input) = &mut self.inner.input_manager {
            input.clear_action_bindings(action_name);
        }
    }
}

/// A thread-safe handle to the engine that can be passed to background threads.
///
/// Use this handle to queue commands like adding objects or drawing from other threads.
#[pyclass(name = "EngineHandle")]
#[derive(Clone)]
pub struct PyEngineHandle {
    sender: Sender<EngineCommand>,
}

#[pymethods]
impl PyEngineHandle {
    /// Add a GameObject to the engine command queue.
    ///
    /// This is thread-safe and will be processed on the next engine update.
    fn add_game_object(&self, game_object: &PyGameObject) {
        let _ = self.sender.send(EngineCommand::AddGameObject(
            game_object.to_runtime_game_object(),
        ));
    }

    /// Remove a runtime GameObject by id via command queue.
    fn remove_game_object(&self, object_id: u32) {
        let _ = self.sender.send(EngineCommand::RemoveGameObject(object_id));
    }

    /// Update a runtime GameObject position by id via command queue.
    fn set_game_object_position(&self, object_id: u32, position: PyVec2) {
        let _ = self.sender.send(EngineCommand::SetGameObjectPosition {
            object_id,
            position: position.inner,
        });
    }

    /// Update the active camera world position via command queue.
    fn set_camera_position(&self, position: PyVec2) {
        let _ = self.sender.send(EngineCommand::SetCameraPosition {
            position: position.inner,
        });
    }

    /// Update the active camera viewport size in world units via command queue.
    fn set_camera_viewport_size(&self, width: f32, height: f32) {
        let _ = self
            .sender
            .send(EngineCommand::SetCameraViewportSize { width, height });
    }

    /// Update camera aspect handling mode via command queue.
    fn set_camera_aspect_mode(&self, mode: &str) {
        if let Some(parsed) = parse_camera_aspect_mode(mode) {
            let _ = self
                .sender
                .send(EngineCommand::SetCameraAspectMode { mode: parsed });
        }
    }

    /// Update the active camera background clear color via command queue.
    fn set_camera_background_color(&self, color: &PyColor) {
        let _ = self
            .sender
            .send(EngineCommand::SetCameraBackgroundColor { color: color.inner });
    }

    /// Clear all immediate-mode draw commands via command queue.
    fn clear_draw_commands(&self) {
        let _ = self.sender.send(EngineCommand::ClearDrawCommands);
    }

    /// Submit many draw commands via command queue in one call.
    fn add_draw_commands(&self, py: Python<'_>, commands: Vec<Py<PyDrawCommand>>) {
        let runtime_commands: Vec<DrawCommand> = commands
            .into_iter()
            .map(|command| command.borrow(py).inner.clone())
            .collect();
        let _ = self
            .sender
            .send(EngineCommand::AddDrawCommands(runtime_commands));
    }

    /// Draw a pixel at window coordinates via command queue.
    #[pyo3(signature = (x, y, color, draw_order=0.0))]
    fn draw_pixel(&self, x: u32, y: u32, color: &PyColor, draw_order: f32) {
        let _ = self.sender.send(EngineCommand::DrawPixel {
            x,
            y,
            color: color.inner,
            draw_order,
        });
    }

    /// Draw a line at window coordinates via command queue.
    #[pyo3(signature = (start_x, start_y, end_x, end_y, color, thickness=1.0, draw_order=0.0))]
    fn draw_line(
        &self,
        start_x: f32,
        start_y: f32,
        end_x: f32,
        end_y: f32,
        color: &PyColor,
        thickness: f32,
        draw_order: f32,
    ) {
        let _ = self.sender.send(EngineCommand::DrawLine {
            start_x,
            start_y,
            end_x,
            end_y,
            thickness,
            color: color.inner,
            draw_order,
        });
    }

    /// Draw a rectangle at window coordinates via command queue.
    #[pyo3(signature = (x, y, width, height, color, filled=true, thickness=1.0, draw_order=0.0))]
    fn draw_rectangle(
        &self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color: &PyColor,
        filled: bool,
        thickness: f32,
        draw_order: f32,
    ) {
        let _ = self.sender.send(EngineCommand::DrawRectangle {
            x,
            y,
            width,
            height,
            color: color.inner,
            filled,
            thickness,
            draw_order,
        });
    }

    /// Draw a circle at window coordinates via command queue.
    #[pyo3(signature = (
        center_x,
        center_y,
        radius,
        color,
        filled=true,
        thickness=1.0,
        segments=32,
        draw_order=0.0
    ))]
    fn draw_circle(
        &self,
        center_x: f32,
        center_y: f32,
        radius: f32,
        color: &PyColor,
        filled: bool,
        thickness: f32,
        segments: u32,
        draw_order: f32,
    ) {
        let _ = self.sender.send(EngineCommand::DrawCircle {
            center_x,
            center_y,
            radius,
            color: color.inner,
            filled,
            thickness,
            segments,
            draw_order,
        });
    }

    /// Draw a gradient rectangle with per-corner colors via command queue.
    #[pyo3(signature = (
        x,
        y,
        width,
        height,
        top_left,
        bottom_left,
        bottom_right,
        top_right,
        draw_order=0.0
    ))]
    fn draw_gradient_rect(
        &self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        top_left: &PyColor,
        bottom_left: &PyColor,
        bottom_right: &PyColor,
        top_right: &PyColor,
        draw_order: f32,
    ) {
        let _ = self.sender.send(EngineCommand::DrawGradientRect {
            x,
            y,
            width,
            height,
            top_left: top_left.inner,
            bottom_left: bottom_left.inner,
            bottom_right: bottom_right.inner,
            top_right: top_right.inner,
            draw_order,
        });
    }

    /// Draw an image from a filesystem path via command queue.
    #[pyo3(signature = (x, y, width, height, texture_path, draw_order=0.0))]
    fn draw_image(
        &self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        texture_path: String,
        draw_order: f32,
    ) {
        let _ = self.sender.send(EngineCommand::DrawImage {
            x,
            y,
            width,
            height,
            texture_path,
            draw_order,
        });
    }

    /// Draw an image from raw RGBA bytes via command queue.
    #[pyo3(signature = (
        x,
        y,
        width,
        height,
        texture_key,
        rgba,
        texture_width,
        texture_height,
        draw_order=0.0
    ))]
    fn draw_image_from_bytes(
        &self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        texture_key: String,
        rgba: Vec<u8>,
        texture_width: u32,
        texture_height: u32,
        draw_order: f32,
    ) {
        let _ = self.sender.send(EngineCommand::DrawImageBytes {
            x,
            y,
            width,
            height,
            texture_key,
            rgba: Arc::from(rgba),
            texture_width,
            texture_height,
            draw_order,
        });
    }

    /// Draw text in window coordinates via command queue.
    #[pyo3(signature = (
        text,
        x,
        y,
        color,
        font_size=24.0,
        font_path=None,
        letter_spacing=0.0,
        line_spacing=0.0,
        draw_order=0.0
    ))]
    fn draw_text(
        &self,
        text: String,
        x: f32,
        y: f32,
        color: &PyColor,
        font_size: f32,
        font_path: Option<String>,
        letter_spacing: f32,
        line_spacing: f32,
        draw_order: f32,
    ) {
        let _ = self.sender.send(EngineCommand::DrawText {
            text,
            x,
            y,
            font_size,
            color: color.inner,
            font_path,
            letter_spacing,
            line_spacing,
            draw_order,
        });
    }

    /// Update a UI label's text at runtime by object ID via command queue.
    fn update_ui_label_text(&self, object_id: u32, text: String) {
        let _ = self
            .sender
            .send(EngineCommand::UpdateUILabelText { object_id, text });
    }

    /// Update a UI button's text at runtime by object ID via command queue.
    fn update_ui_button_text(&self, object_id: u32, text: String) {
        let _ = self
            .sender
            .send(EngineCommand::UpdateUIButtonText { object_id, text });
    }

    /// Log a message at INFO level (default log method).
    fn log(&self, message: &str) {
        let _ = self.sender.send(EngineCommand::LogInfo(message.to_string()));
    }

    /// Log a message at TRACE level (most verbose).
    fn log_trace(&self, message: &str) {
        let _ = self.sender.send(EngineCommand::LogTrace(message.to_string()));
    }

    /// Log a message at DEBUG level.
    fn log_debug(&self, message: &str) {
        let _ = self.sender.send(EngineCommand::LogDebug(message.to_string()));
    }

    /// Log a message at INFO level.
    fn log_info(&self, message: &str) {
        let _ = self.sender.send(EngineCommand::LogInfo(message.to_string()));
    }

    /// Log a message at WARN level.
    fn log_warn(&self, message: &str) {
        let _ = self.sender.send(EngineCommand::LogWarn(message.to_string()));
    }

    /// Log a message at ERROR level.
    fn log_error(&self, message: &str) {
        let _ = self.sender.send(EngineCommand::LogError(message.to_string()));
    }
}

/// Python wrapper for Time.
/// Frame timing information.
///
/// Tracks delta time and elapsed time for implementing frame-independent movement,
/// animations, and time-based game logic.
///
/// # Time Units: Seconds
/// **IMPORTANT:** All time values are in **seconds** (not milliseconds).
///
/// # Attributes
/// - `delta_time` - Time since last frame in **seconds** (float)
/// - `elapsed_time` - Total time since engine start in **seconds** (float)
///
/// # Delta Time
/// Delta time (dt) represents the duration of the last frame. Use it to make movement
/// and animations frame-rate independent:
///
/// ```python
/// # Frame-independent movement
/// speed = 100.0  # pixels per second
/// dt = engine.delta_time
/// position += speed * dt  # Moves 100 pixels/sec regardless of FPS
/// ```
///
/// **Typical delta_time values:**
/// - 60 FPS: ~0.016 seconds (16ms)
/// - 30 FPS: ~0.033 seconds (33ms)
/// - 120 FPS: ~0.008 seconds (8ms)
///
/// # Elapsed Time
/// Elapsed time tracks total time since engine startup, useful for time-based effects:
///
/// ```python
/// # Pulsing animation
/// import math
/// elapsed = engine.elapsed_time
/// scale = 1.0 + 0.2 * math.sin(elapsed * 2.0)  # Pulse every ~3 seconds
/// ```
///
/// # Example
/// ```python
/// import pyg_engine as pyg
///
/// engine = pyg.Engine()
/// engine.start_manual()
///
/// player_x = 400.0
/// player_speed = 200.0  # pixels per second
///
/// while engine.poll_events():
///     # Get delta time (in seconds)
///     dt = engine.delta_time
///
///     # Frame-independent movement
///     if engine.input.key_down(pyg.Keys.D):
///         player_x += player_speed * dt  # Move 200 pixels/sec
///
///     # Show FPS based on delta time
///     fps = 1.0 / dt if dt > 0 else 0
///     print(f"FPS: {fps:.1f}, Delta: {dt:.4f}s")
///
///     engine.update()
///     engine.render()
/// ```
///
/// # Frame-Rate Independence
/// Always multiply velocities by `delta_time` to ensure consistent behavior
/// across different frame rates:
///
/// ```python
/// # ❌ WRONG - Speed depends on FPS
/// position += 5.0  # Moves 5 pixels PER FRAME (fast at high FPS, slow at low FPS)
///
/// # ✅ CORRECT - Speed independent of FPS
/// speed = 300.0  # pixels per second
/// position += speed * engine.delta_time  # Always 300 pixels/sec
/// ```
///
/// # See Also
/// - `examples/python_input_demo.py` - Delta time usage for movement
/// - `examples/python_camera_worldspace_demo.py` - Frame-independent camera control
#[pyclass(name = "Time")]
pub struct PyTime {
    inner: RustTime,
}

#[pymethods]
impl PyTime {
    #[new]
    fn new() -> Self {
        Self {
            inner: RustTime::new(),
        }
    }

    fn tick(&mut self) {
        self.inner.tick();
    }

    /// Time since last frame in **seconds**.
    ///
    /// Use for frame-independent movement and animations.
    /// Typical values: 0.016 (60 FPS), 0.033 (30 FPS), 0.008 (120 FPS)
    ///
    /// # Example
    /// ```python
    /// dt = engine.delta_time
    /// player_x += speed * dt  # pixels per second
    /// ```
    #[getter]
    fn delta_time(&self) -> f32 {
        self.inner.delta_time()
    }

    /// Total time since engine start in **seconds**.
    ///
    /// Useful for time-based effects like animations, timers, and periodic events.
    ///
    /// # Example
    /// ```python
    /// import math
    /// t = engine.elapsed_time
    /// wave = math.sin(t * 2.0)  # Oscillates every ~3 seconds
    /// ```
    #[getter]
    fn elapsed_time(&self) -> f32 {
        self.inner.elapsed_time()
    }
}

// ========== GameObject Bindings ==========

/// Container for game entities with transform, rendering, and behavior.
///
/// `GameObject` is the fundamental building block of your game. Each GameObject has:
/// - **Transform**: Position, rotation, scale
/// - **Components**: Rendering (mesh), UI elements (button, panel, label)
/// - **State**: Active/inactive, name, unique ID
///
/// # Creation and Setup
///
/// ```python
/// from pyg_engine import GameObject, MeshComponent, Vec2, Color
///
/// # Create a named GameObject
/// player = GameObject("Player")
/// player.position = Vec2(0, 0) # Note that we're using *World Position* not pixel location
/// player.rotation = math.radians(45)  # Rotation in radians
/// player.scale = Vec2(1.0, 1.0)
///
/// # Add mesh rendering
/// mesh = MeshComponent("Rectangle")
/// mesh.set_geometry_rectangle(1.0, 1.0)
/// mesh.set_fill_color(Color.BLUE)
/// player.set_mesh_component(mesh)
///
/// # Add to engine
/// engine.add_game_object(player)
/// ```
///
/// # Transform Properties
///
/// GameObjects use a 2D transform with position, rotation, and scale:
/// - **position**: `Vec2` in world coordinates
/// - **rotation**: Angle in **radians** (counter-clockwise)
/// - **scale**: `Vec2` multiplier (1.0 = original size)
///
/// # Components
///
/// Attach components to add functionality:
/// - `MeshComponent` - 2D rendering (rectangles, circles, images)
/// - `ButtonComponent` - Clickable UI button
/// - `PanelComponent` - UI container panel
/// - `LabelComponent` - UI text label
///
/// # Active State
///
/// GameObjects can be enabled/disabled:
/// ```python
/// player.active = False  # Disable (not rendered, not updated)
/// player.active = True   # Enable
/// ```
///
/// # Object Types
///
/// GameObjects can be marked as `"UIObject"` for UI rendering:
/// ```python
/// obj.set_object_type("UIObject")  # Render as UI (screen space)
/// ```
///
/// # See Also
/// - `examples/python_game_object_transform_demo.py` - Transform operations
/// - `examples/python_mesh_demo.py` - Mesh rendering
/// - `examples/ui_demo.py` - UI components
#[pyclass(name = "GameObject", unsendable)]
pub struct PyGameObject {
    inner: RustGameObject,
    runtime_binding: RefCell<Option<RuntimeBinding>>,
}

#[derive(Clone)]
struct RuntimeBinding {
    sender: Sender<EngineCommand>,
    object_id: u32,
}

impl PyGameObject {
    fn ensure_mesh_component(&mut self) -> &mut MeshComponent {
        if self.inner.mesh_component().is_none() {
            self.inner
                .add_mesh_component(MeshComponent::new("Mesh Renderer".to_string()));
        }
        self.inner
            .mesh_component_mut()
            .expect("mesh component should exist")
    }

    fn to_runtime_game_object(&self) -> RustGameObject {
        let mut runtime = if let Some(name) = self.inner.name() {
            RustGameObject::new_named(name.to_string())
        } else {
            RustGameObject::new()
        };

        runtime.set_transform(self.inner.transform().clone());
        runtime.set_active(self.inner.is_active());
        runtime.set_object_type(self.inner.get_object_type());

        if let Some(mesh) = self.inner.mesh_component() {
            runtime.add_mesh_component(mesh.clone());
        }

        // Copy UI components (Button, Panel, Label) by downcasting and cloning
        for comp_name in &["Button", "Panel", "Label"] {
            if let Some(comp) = self.inner.get_component_by_name(comp_name) {
                if let Some(btn) = comp.as_any().downcast_ref::<ButtonComponent>() {
                    runtime.add_component(Box::new(btn.clone()));
                } else if let Some(panel) = comp.as_any().downcast_ref::<PanelComponent>() {
                    runtime.add_component(Box::new(panel.clone()));
                } else if let Some(label) = comp.as_any().downcast_ref::<LabelComponent>() {
                    runtime.add_component(Box::new(label.clone()));
                }
            }
        }

        // Copy ALL other components (including Colliders)
        use crate::core::physics::collider::ColliderComponent;
        for component in self.inner.components_iter() {
            // Check if it's a Collider (and not already copied above)
            if let Some(collider) = component.as_any().downcast_ref::<ColliderComponent>() {
                runtime.add_component(Box::new(collider.clone()));
            }
        }

        runtime
    }

    fn bind_runtime(&self, sender: Sender<EngineCommand>, object_id: u32) {
        self.runtime_binding
            .replace(Some(RuntimeBinding { sender, object_id }));
    }
}

#[pymethods]
impl PyGameObject {
    #[new]
    #[pyo3(signature = (name=None))]
    fn new(name: Option<String>) -> Self {
        let inner = if let Some(n) = name {
            RustGameObject::new_named(n)
        } else {
            RustGameObject::new()
        };
        Self {
            inner,
            runtime_binding: RefCell::new(None),
        }
    }

    /// Get the unique ID of this GameObject.
    ///
    /// Each GameObject is assigned a unique ID when created. This ID is used internally
    /// by the engine for tracking objects and can be useful for debugging or looking up
    /// objects.
    ///
    /// # Returns
    /// A unique unsigned 32-bit integer ID.
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// obj1 = pyg.GameObject("Player")
    /// obj2 = pyg.GameObject("Enemy")
    ///
    /// print(f"Player ID: {obj1.id}")  # e.g., "Player ID: 1"
    /// print(f"Enemy ID: {obj2.id}")   # e.g., "Enemy ID: 2"
    ///
    /// # IDs are unique
    /// assert obj1.id != obj2.id
    /// ```
    ///
    /// # Note
    /// IDs are assigned sequentially and persist for the lifetime of the object.
    /// Once an object is destroyed, its ID may be reused for new objects.
    #[getter]
    fn id(&self) -> u32 {
        self.inner.get_id()
    }

    /// Get the name of this GameObject.
    ///
    /// Returns the human-readable name assigned to this object, or `None` if no name was set.
    /// Names are useful for debugging, logging, and identifying objects in your game.
    ///
    /// # Returns
    /// - `Some(String)` - The object's name if one was set
    /// - `None` - If no name was assigned
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// # Create named object
    /// player = pyg.GameObject("Player")
    /// print(player.name)  # "Player"
    ///
    /// # Create unnamed object
    /// obj = pyg.GameObject()
    /// print(obj.name)  # None
    ///
    /// # Change name later
    /// obj.set_name("Bullet")
    /// print(obj.name)  # "Bullet"
    /// ```
    ///
    /// # See Also
    /// - `set_name()` - Change the object's name
    #[getter]
    fn name(&self) -> Option<String> {
        self.inner.name().map(|name| name.to_string())
    }

    /// Set or change the name of this GameObject.
    ///
    /// Assigns a human-readable name to the object. Names are useful for debugging,
    /// logging, and organizing objects in your game.
    ///
    /// # Arguments
    /// * `name` - New name for the object (any string)
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// obj = pyg.GameObject()
    /// obj.set_name("Player")
    /// print(obj.name)  # "Player"
    ///
    /// # Rename object
    /// obj.set_name("MainCharacter")
    /// print(obj.name)  # "MainCharacter"
    /// ```
    ///
    /// # See Also
    /// - `name` (property) - Get the current name
    fn set_name(&mut self, name: String) {
        self.inner.set_name(name);
    }

    /// Check if this GameObject is active.
    ///
    /// Active objects are updated and rendered each frame. Inactive objects are ignored
    /// by the engine until they are activated again.
    ///
    /// # Returns
    /// - `True` - Object is active (updated and rendered)
    /// - `False` - Object is inactive (dormant)
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// player = pyg.GameObject("Player")
    /// print(player.active)  # True (active by default)
    ///
    /// # Deactivate object
    /// player.active = False
    /// print(player.active)  # False
    ///
    /// # Reactivate object
    /// player.active = True
    /// print(player.active)  # True
    /// ```
    ///
    /// # Use Cases
    /// - **Object pooling**: Deactivate/reactivate objects instead of creating/destroying
    /// - **Pause mechanics**: Deactivate enemies when game is paused
    /// - **Conditional rendering**: Hide objects without removing them from the scene
    ///
    /// # See Also
    /// - `active` (setter) - Set active state
    #[getter]
    fn active(&self) -> bool {
        self.inner.is_active()
    }

    /// Set whether this GameObject is active.
    ///
    /// Controls whether the object is updated and rendered. Inactive objects remain in
    /// the scene but are skipped during update and render passes.
    ///
    /// # Arguments
    /// * `active` - `True` to activate, `False` to deactivate
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// enemy = pyg.GameObject("Enemy")
    /// engine.add_game_object(enemy)
    ///
    /// # Temporarily disable enemy
    /// enemy.active = False  # Enemy no longer updates or renders
    ///
    /// # Re-enable later
    /// enemy.active = True  # Enemy resumes normal behavior
    /// ```
    ///
    /// # Object Pooling Pattern
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// class BulletPool:
    ///     def __init__(self, engine, size=10):
    ///         self.bullets = []
    ///         for i in range(size):
    ///             bullet = pyg.GameObject(f"Bullet_{i}")
    ///             bullet.active = False  # Start deactivated
    ///             engine.add_game_object(bullet)
    ///             self.bullets.append(bullet)
    ///
    ///     def spawn(self, position):
    ///         # Find inactive bullet
    ///         for bullet in self.bullets:
    ///             if not bullet.active:
    ///                 bullet.position = position
    ///                 bullet.active = True  # Activate
    ///                 return bullet
    ///         return None  # Pool exhausted
    ///
    ///     def despawn(self, bullet):
    ///         bullet.active = False  # Return to pool
    /// ```
    ///
    /// # See Also
    /// - `active` (getter) - Check active state
    #[setter]
    fn set_active(&mut self, active: bool) {
        self.inner.set_active(active);
    }

    /// Get the position of this GameObject.
    ///
    /// Returns the object's position in **world-space coordinates** as a `Vec2`.
    /// The position represents the center point of the object.
    ///
    /// # Returns
    /// `Vec2` containing the object's position:
    /// - `x` - Horizontal position (right is positive)
    /// - `y` - Vertical position (up is typically positive)
    ///
    /// # Coordinate System
    /// - **World space**: Absolute positions in the game world
    /// - **Origin (0, 0)**: Typically center of screen (configurable via camera)
    /// - **Units**: Pixels by default
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// player = pyg.GameObject("Player")
    /// player.position = pyg.Vec2(100.0, 200.0)
    ///
    /// # Get current position
    /// pos = player.position
    /// print(f"Player at ({pos.x}, {pos.y})")  # "Player at (100.0, 200.0)"
    ///
    /// # Check if player is at origin
    /// if player.position == pyg.Vec2.ZERO:
    ///     print("Player at origin")
    /// ```
    ///
    /// # Movement Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// def update(dt, engine, data):
    ///     player = data['player']
    ///
    ///     # Move right at 100 pixels/second
    ///     new_pos = player.position + pyg.Vec2(100.0 * dt, 0.0)
    ///     player.position = new_pos
    ///
    ///     return True
    /// ```
    ///
    /// # See Also
    /// - `position` (setter) - Set object position
    /// - `Vec2` - 2D vector class
    /// - `examples/python_game_object_transform_demo.py` - Transform examples
    #[getter]
    fn position(&self) -> PyVec2 {
        PyVec2 {
            inner: *self.inner.transform().position(),
        }
    }

    /// Set the position of this GameObject.
    ///
    /// Moves the object to the specified **world-space coordinates**. The position
    /// represents the center point of the object.
    ///
    /// # Arguments
    /// * `position` - Target position as `Vec2` (world-space coordinates)
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// player = pyg.GameObject("Player")
    ///
    /// # Set position directly
    /// player.position = pyg.Vec2(100.0, 200.0)
    ///
    /// # Move to origin
    /// player.position = pyg.Vec2.ZERO
    ///
    /// # Move relative to current position
    /// player.position = player.position + pyg.Vec2(50.0, 0.0)  # Move right 50px
    /// ```
    ///
    /// # Smooth Movement
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// def update(dt, engine, data):
    ///     player = data['player']
    ///     speed = 150.0  # pixels per second
    ///
    ///     # Get input
    ///     move_x = engine.input.axis("Horizontal")
    ///     move_y = engine.input.axis("Vertical")
    ///
    ///     # Calculate movement
    ///     velocity = pyg.Vec2(move_x, move_y).normalize() * speed * dt
    ///
    ///     # Apply movement
    ///     player.position = player.position + velocity
    ///
    ///     return True
    /// ```
    ///
    /// # Lerp to Target
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// def update(dt, engine, data):
    ///     player = data['player']
    ///     target = data['target_pos']
    ///
    ///     # Smoothly interpolate to target (10% per frame)
    ///     player.position = player.position.lerp(target, 0.1)
    ///
    ///     return True
    /// ```
    ///
    /// # See Also
    /// - `position` (getter) - Get current position
    /// - `Vec2` - 2D vector operations
    /// - `Vec2.lerp()` - Linear interpolation for smooth movement
    /// - `examples/python_game_object_transform_demo.py` - Transform examples
    #[setter]
    fn set_position(&mut self, position: PyVec2) {
        self.inner.transform_mut().set_position(position.inner);
        if let Some(binding) = self.runtime_binding.borrow().as_ref() {
            let _ = binding.sender.send(EngineCommand::SetGameObjectPosition {
                object_id: binding.object_id,
                position: position.inner,
            });
        }
    }

    /// Get the object's rotation angle.
    ///
    /// Returns the current rotation in **radians** (not degrees). Positive values represent
    /// counter-clockwise rotation.
    ///
    /// # Returns
    /// Float representing rotation angle in **radians**:
    /// - `0.0` = No rotation (facing right/east)
    /// - `π/2` (≈1.571) = 90° counter-clockwise (facing up/north)
    /// - `π` (≈3.142) = 180° (facing left/west)
    /// - `3π/2` (≈4.712) = 270° counter-clockwise (facing down/south)
    /// - `2π` (≈6.283) = 360° (full rotation, back to 0°)
    ///
    /// # Units: Radians
    /// **IMPORTANT:** Rotation uses **radians**, not degrees.
    /// - To convert degrees to radians: Use `math.radians(degrees)`
    /// - To convert radians to degrees: Use `math.degrees(radians)`
    /// - Formula: `radians = degrees × (π / 180)`
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    /// import math
    ///
    /// engine = pyg.Engine()
    /// obj = pyg.GameObject("RotatingObject")
    ///
    /// # Get current rotation
    /// current_rotation = obj.rotation
    /// print(f"Rotation: {current_rotation} radians = {math.degrees(current_rotation)}°")
    ///
    /// # Check if object is upside down
    /// if abs(obj.rotation - math.pi) < 0.1:
    ///     print("Object is approximately upside down")
    /// ```
    ///
    /// # See Also
    /// - `rotation` (setter) - Set rotation angle
    /// - `GameObject.rotation` - Direct property access
    #[getter]
    fn rotation(&self) -> f32 {
        self.inner.transform().rotation()
    }

    /// Set the object's rotation angle.
    ///
    /// Sets the rotation in **radians** (not degrees). Positive values rotate counter-clockwise.
    ///
    /// # Arguments
    /// * `rotation` - Rotation angle in **radians** (not degrees):
    ///   - Positive: Counter-clockwise rotation
    ///   - Negative: Clockwise rotation
    ///   - Range: Typically 0 to 2π, but any value is valid (wraps around)
    ///
    /// # Units: Radians
    /// **IMPORTANT:** Rotation uses **radians**, not degrees!
    /// - Use `math.radians(degrees)` to convert from degrees
    /// - Common angles in radians:
    ///   - 0° = 0.0 rad
    ///   - 45° ≈ 0.785 rad (π/4)
    ///   - 90° ≈ 1.571 rad (π/2)
    ///   - 180° ≈ 3.142 rad (π)
    ///   - 270° ≈ 4.712 rad (3π/2)
    ///   - 360° ≈ 6.283 rad (2π)
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    /// import math
    ///
    /// engine = pyg.Engine()
    /// obj = pyg.GameObject("RotatingObject")
    ///
    /// # Rotate 45 degrees counter-clockwise
    /// obj.rotation = math.radians(45)
    ///
    /// # Rotate 90 degrees clockwise (negative)
    /// obj.rotation = math.radians(-90)
    ///
    /// # Rotate using pi constants
    /// obj.rotation = math.pi / 2  # 90 degrees
    /// obj.rotation = math.pi       # 180 degrees
    /// obj.rotation = 2 * math.pi   # 360 degrees (full rotation)
    /// ```
    ///
    /// # Continuous Rotation
    /// ```python
    /// import pyg_engine as pyg
    /// import math
    ///
    /// engine = pyg.Engine()
    /// obj = pyg.GameObject("Spinner")
    /// engine.add_game_object(obj)
    ///
    /// rotation_speed = math.radians(90)  # 90 degrees per second
    ///
    /// def update(dt, engine, data):
    ///     # Rotate continuously
    ///     obj.rotation += rotation_speed * dt
    ///
    ///     # Optional: Keep rotation in [0, 2π] range
    ///     if obj.rotation > 2 * math.pi:
    ///         obj.rotation -= 2 * math.pi
    ///
    ///     return True
    ///
    /// engine.run(update=update)
    /// ```
    ///
    /// # Rotation Towards Target
    /// ```python
    /// import pyg_engine as pyg
    /// import math
    ///
    /// def rotate_towards(obj, target_pos, dt, turn_speed):
    ///     # Calculate angle to target
    ///     dx = target_pos.x - obj.position.x
    ///     dy = target_pos.y - obj.position.y
    ///     target_angle = math.atan2(dy, dx)
    ///
    ///     # Smoothly rotate towards target
    ///     angle_diff = target_angle - obj.rotation
    ///
    ///     # Normalize to [-π, π]
    ///     while angle_diff > math.pi:
    ///         angle_diff -= 2 * math.pi
    ///     while angle_diff < -math.pi:
    ///         angle_diff += 2 * math.pi
    ///
    ///     # Apply rotation with speed limit
    ///     max_rotation = turn_speed * dt
    ///     obj.rotation += max(-max_rotation, min(max_rotation, angle_diff))
    /// ```
    ///
    /// # Common Mistakes
    /// ```python
    /// # ❌ WRONG - Using degrees directly
    /// obj.rotation = 90  # This is 90 radians, not 90 degrees!
    ///
    /// # ✅ CORRECT - Convert degrees to radians
    /// obj.rotation = math.radians(90)  # 90 degrees
    ///
    /// # ✅ CORRECT - Use pi constants
    /// obj.rotation = math.pi / 2  # 90 degrees
    /// ```
    ///
    /// # Direction Reference
    /// Assuming default orientation (0° = facing right):
    /// - `math.radians(0)` = Right (East)
    /// - `math.radians(90)` = Up (North)
    /// - `math.radians(180)` = Left (West)
    /// - `math.radians(270)` = Down (South)
    ///
    /// # See Also
    /// - `examples/python_game_object_transform_demo.py` - Transform demonstrations
    /// - `rotation` (getter) - Get current rotation
    /// - `GameObject.rotation` - Direct property access
    #[setter]
    fn set_rotation(&mut self, rotation: f32) {
        self.inner.transform_mut().set_rotation(rotation);
        if let Some(binding) = self.runtime_binding.borrow().as_ref() {
            let _ = binding.sender.send(EngineCommand::SetGameObjectRotation {
                object_id: binding.object_id,
                rotation,
            });
        }
    }

    /// Get the scale of this GameObject.
    ///
    /// Returns the object's scale as a `Vec2`. Scale controls the size of the object
    /// relative to its original dimensions. Scale values are multiplicative:
    /// - `1.0` = Original size (100%)
    /// - `2.0` = Double size (200%)
    /// - `0.5` = Half size (50%)
    ///
    /// # Returns
    /// `Vec2` containing the scale factors:
    /// - `x` - Horizontal scale factor
    /// - `y` - Vertical scale factor
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// obj = pyg.GameObject("Sprite")
    /// obj.scale = pyg.Vec2(2.0, 2.0)  # Double size
    ///
    /// # Get current scale
    /// scale = obj.scale
    /// print(f"Scale: {scale.x}x, {scale.y}x")  # "Scale: 2.0x, 2.0x"
    ///
    /// # Check if object is at original size
    /// if obj.scale == pyg.Vec2(1.0, 1.0):
    ///     print("Original size")
    /// ```
    ///
    /// # Non-Uniform Scaling
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// obj = pyg.GameObject("StretchedSprite")
    ///
    /// # Different scale on each axis
    /// obj.scale = pyg.Vec2(2.0, 0.5)  # Wide and short
    /// obj.scale = pyg.Vec2(0.5, 2.0)  # Narrow and tall
    ///
    /// # Flip horizontally (negative scale)
    /// obj.scale = pyg.Vec2(-1.0, 1.0)  # Mirror on X-axis
    /// ```
    ///
    /// # See Also
    /// - `scale` (setter) - Set object scale
    /// - `Vec2` - 2D vector class
    /// - `examples/python_game_object_transform_demo.py` - Transform examples
    #[getter]
    fn scale(&self) -> PyVec2 {
        PyVec2 {
            inner: *self.inner.transform().scale(),
        }
    }

    /// Set the scale of this GameObject.
    ///
    /// Controls the size of the object relative to its original dimensions.
    /// Scale is applied to both the mesh geometry and any child components.
    ///
    /// # Arguments
    /// * `scale` - Scale factors as `Vec2`:
    ///   - `x` - Horizontal scale (1.0 = original width)
    ///   - `y` - Vertical scale (1.0 = original height)
    ///
    /// # Scale Values
    /// - **1.0** = Original size (100%)
    /// - **> 1.0** = Larger (e.g., 2.0 = double size)
    /// - **< 1.0** = Smaller (e.g., 0.5 = half size)
    /// - **0.0** = Invisible (zero size)
    /// - **Negative** = Flipped (mirrored)
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// obj = pyg.GameObject("Sprite")
    ///
    /// # Uniform scaling (same on both axes)
    /// obj.scale = pyg.Vec2(2.0, 2.0)   # Double size
    /// obj.scale = pyg.Vec2(0.5, 0.5)   # Half size
    ///
    /// # Non-uniform scaling
    /// obj.scale = pyg.Vec2(3.0, 1.0)   # Stretched horizontally
    /// obj.scale = pyg.Vec2(1.0, 0.5)   # Compressed vertically
    ///
    /// # Flipping
    /// obj.scale = pyg.Vec2(-1.0, 1.0)  # Flip horizontally
    /// obj.scale = pyg.Vec2(1.0, -1.0)  # Flip vertically
    /// obj.scale = pyg.Vec2(-1.0, -1.0) # Flip both axes
    /// ```
    ///
    /// # Pulsing Effect
    /// ```python
    /// import pyg_engine as pyg
    /// import math
    ///
    /// def update(dt, engine, data):
    ///     elapsed = data['elapsed']
    ///     data['elapsed'] += dt
    ///
    ///     # Pulse between 0.5x and 1.5x size
    ///     pulse = 1.0 + 0.5 * math.sin(elapsed * 2.0)
    ///     data['obj'].scale = pyg.Vec2(pulse, pulse)
    ///
    ///     return True
    ///
    /// engine.run(update=update, user_data={'elapsed': 0.0, 'obj': obj})
    /// ```
    ///
    /// # Grow Over Time
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// def update(dt, engine, data):
    ///     obj = data['obj']
    ///     grow_rate = 0.5  # 50% per second
    ///
    ///     # Grow uniformly
    ///     current_scale = obj.scale.x  # Assuming uniform scale
    ///     new_scale = current_scale + (grow_rate * dt)
    ///     obj.scale = pyg.Vec2(new_scale, new_scale)
    ///
    ///     # Cap maximum size
    ///     if obj.scale.x > 5.0:
    ///         obj.scale = pyg.Vec2(5.0, 5.0)
    ///
    ///     return True
    /// ```
    ///
    /// # See Also
    /// - `scale` (getter) - Get current scale
    /// - `Vec2` - 2D vector operations
    /// - `examples/python_game_object_transform_demo.py` - Transform examples
    #[setter]
    fn set_scale(&mut self, scale: PyVec2) {
        self.inner.transform_mut().set_scale(scale.inner);
        if let Some(binding) = self.runtime_binding.borrow().as_ref() {
            let _ = binding.sender.send(EngineCommand::SetGameObjectScale {
                object_id: binding.object_id,
                scale: scale.inner,
            });
        }
    }

    /// Manually update this GameObject.
    ///
    /// Triggers the object's update lifecycle, including all attached components.
    /// This is typically called automatically by the engine, but can be invoked
    /// manually for custom update logic or testing.
    ///
    /// # Arguments
    /// * `time` - Optional `Time` object for delta time. If `None`, uses current time.
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// obj = pyg.GameObject("Player")
    ///
    /// # Manual update (uses current time)
    /// obj.update()
    ///
    /// # Manual update with specific time
    /// time = pyg.Time()
    /// obj.update(time)
    /// ```
    ///
    /// # Use Cases
    /// - **Testing**: Manually tick objects in unit tests
    /// - **Custom loops**: Control update timing outside the engine loop
    /// - **Selective updates**: Update specific objects independently
    ///
    /// # Note
    /// In normal usage, the engine calls `update()` automatically for all active
    /// GameObjects each frame. Manual calling is rarely needed.
    ///
    /// # See Also
    /// - `GameObject.active` - Control whether object updates automatically
    #[pyo3(signature = (time=None))]
    fn update(&self, time: Option<&PyTime>) {
        if let Some(time) = time {
            self.inner.update(&time.inner);
        } else {
            let local_time = RustTime::new();
            self.inner.update(&local_time);
        }
    }

    /// Check if this GameObject has a mesh component attached.
    ///
    /// Returns `True` if the object has a `MeshComponent` for rendering,
    /// `False` otherwise.
    ///
    /// # Returns
    /// - `True` - Object has a mesh component (is visible)
    /// - `False` - Object has no mesh component (not rendered)
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// obj = pyg.GameObject("Sprite")
    ///
    /// # Check if mesh exists
    /// if obj.has_mesh_component():
    ///     print("Object has rendering")
    /// else:
    ///     print("Object is invisible")
    ///
    /// # Add mesh component
    /// mesh = pyg.MeshComponent()
    /// obj.add_mesh_component(mesh)
    ///
    /// # Now has mesh
    /// assert obj.has_mesh_component() == True
    /// ```
    ///
    /// # See Also
    /// - `add_mesh_component()` - Add a mesh component
    /// - `mesh_component()` - Get the mesh component
    /// - `remove_mesh_component()` - Remove the mesh component
    fn has_mesh_component(&self) -> bool {
        self.inner.has_mesh_component()
    }

    /// Add a mesh component to this GameObject.
    ///
    /// Attaches a `MeshComponent` to make the object visible. If a mesh component
    /// already exists, it will be replaced.
    ///
    /// # Arguments
    /// * `mesh_component` - The `MeshComponent` to attach
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// obj = pyg.GameObject("Sprite")
    ///
    /// # Create and configure mesh
    /// mesh = pyg.MeshComponent()
    /// mesh.set_geometry_rectangle(64.0, 64.0)
    /// mesh.set_fill_color(pyg.Color.RED)
    ///
    /// # Attach to object
    /// obj.add_mesh_component(mesh)
    /// ```
    ///
    /// # Quick Setup
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// # Create object with mesh in one go
    /// obj = pyg.GameObject("Player")
    /// obj.position = pyg.Vec2(0.5, 0.5)
    ///
    /// mesh = pyg.MeshComponent()
    /// mesh.set_geometry_circle(32.0)
    /// mesh.set_image_path("assets/player.png")
    ///
    /// obj.add_mesh_component(mesh)
    /// engine.add_game_object(obj)
    /// ```
    ///
    /// # See Also
    /// - `has_mesh_component()` - Check if mesh exists
    /// - `mesh_component()` - Get the mesh component
    /// - `set_mesh_component()` - Alternative to add (same behavior)
    /// - `MeshComponent` - Mesh component class
    fn add_mesh_component(&mut self, mesh_component: &PyMeshComponent) {
        self.inner.add_mesh_component(mesh_component.inner.clone());
    }

    /// Set the mesh component for this GameObject.
    ///
    /// Attaches or replaces the `MeshComponent` for rendering. This is functionally
    /// identical to `add_mesh_component()`.
    ///
    /// # Arguments
    /// * `mesh_component` - The `MeshComponent` to attach
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// obj = pyg.GameObject("Sprite")
    ///
    /// mesh = pyg.MeshComponent()
    /// mesh.set_geometry_rectangle(64.0, 64.0)
    /// mesh.set_fill_color(pyg.Color.BLUE)
    ///
    /// obj.set_mesh_component(mesh)  # Same as add_mesh_component()
    /// ```
    ///
    /// # See Also
    /// - `add_mesh_component()` - Preferred method (same behavior)
    fn set_mesh_component(&mut self, mesh_component: &PyMeshComponent) {
        self.inner.add_mesh_component(mesh_component.inner.clone());
    }

    /// Remove the mesh component from this GameObject.
    ///
    /// Detaches and returns the object's `MeshComponent`, making it invisible.
    /// If no mesh component exists, returns `None`.
    ///
    /// # Returns
    /// - `Some(MeshComponent)` - The removed mesh component
    /// - `None` - No mesh component was attached
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// obj = pyg.GameObject("Sprite")
    /// mesh = pyg.MeshComponent()
    /// obj.add_mesh_component(mesh)
    ///
    /// # Remove mesh (object becomes invisible)
    /// removed_mesh = obj.remove_mesh_component()
    ///
    /// if removed_mesh:
    ///     print("Mesh removed")
    /// else:
    ///     print("No mesh to remove")
    /// ```
    ///
    /// # Temporary Invisibility
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// obj = pyg.GameObject("Sprite")
    /// mesh = pyg.MeshComponent()
    /// obj.add_mesh_component(mesh)
    ///
    /// # Hide by removing mesh
    /// saved_mesh = obj.remove_mesh_component()
    ///
    /// # Later, restore mesh
    /// if saved_mesh:
    ///     obj.add_mesh_component(saved_mesh)
    /// ```
    ///
    /// # Note
    /// For temporary hiding, consider using `mesh.visible = False` instead,
    /// which is simpler and doesn't require removing/re-adding the component.
    ///
    /// # See Also
    /// - `has_mesh_component()` - Check if mesh exists
    /// - `mesh_component()` - Get mesh without removing
    /// - `MeshComponent.visible` - Toggle visibility without removing
    fn remove_mesh_component(&mut self) -> Option<PyMeshComponent> {
        self.inner
            .remove_mesh_component()
            .map(|inner| PyMeshComponent { inner })
    }

    /// Get a copy of this GameObject's mesh component.
    ///
    /// Returns a copy of the attached `MeshComponent` if one exists.
    /// Changes to the returned mesh will **not** affect the original
    /// (use direct mesh methods on the GameObject for in-place modification).
    ///
    /// # Returns
    /// - `Some(MeshComponent)` - Copy of the mesh component
    /// - `None` - No mesh component attached
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// obj = pyg.GameObject("Sprite")
    /// mesh = pyg.MeshComponent()
    /// obj.add_mesh_component(mesh)
    ///
    /// # Get mesh copy
    /// mesh_copy = obj.mesh_component()
    ///
    /// if mesh_copy:
    ///     print(f"Mesh visible: {mesh_copy.visible}")
    /// ```
    ///
    /// # Note on Copying
    /// ```python
    /// # This returns a COPY, not a reference:
    /// mesh_copy = obj.mesh_component()
    /// mesh_copy.visible = False  # Does NOT affect obj's mesh!
    ///
    /// # To modify the object's mesh directly, use GameObject methods:
    /// obj.set_mesh_visible(False)  # Modifies obj's mesh directly
    /// ```
    ///
    /// # Cloning Objects
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// original = pyg.GameObject("Original")
    /// mesh = pyg.MeshComponent()
    /// mesh.set_geometry_circle(32.0)
    /// original.add_mesh_component(mesh)
    ///
    /// # Clone object with mesh
    /// clone = pyg.GameObject("Clone")
    /// if original.mesh_component():
    ///     clone.add_mesh_component(original.mesh_component())
    /// ```
    ///
    /// # See Also
    /// - `has_mesh_component()` - Check if mesh exists
    /// - `remove_mesh_component()` - Remove and get mesh
    /// - GameObject mesh methods: `set_mesh_visible()`, `set_mesh_fill_color()`, etc.
    fn mesh_component(&self) -> Option<PyMeshComponent> {
        self.inner
            .mesh_component()
            .cloned()
            .map(|inner| PyMeshComponent { inner })
    }

    /// Set the mesh geometry to a rectangle.
    ///
    /// Configures the object's mesh as a rectangle with the specified dimensions.
    /// If no mesh component exists, one is created automatically.
    ///
    /// # Arguments
    /// * `width` - Rectangle width in pixels
    /// * `height` - Rectangle height in pixels
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// obj = pyg.GameObject("Box")
    /// obj.position = pyg.Vec2(0.2, 0.15)
    ///
    /// # Create 100x50 rectangle
    /// obj.set_mesh_geometry_rectangle(100.0, 50.0)
    /// obj.set_mesh_fill_color(pyg.Color.GREEN)
    ///
    /// engine.add_game_object(obj)
    /// ```
    ///
    /// # Quick Sprite Setup
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// sprite = pyg.GameObject("Sprite")
    /// sprite.position = pyg.Vec2(400.0, 300.0)
    /// sprite.set_mesh_geometry_rectangle(64.0, 64.0)  # 64x64 square
    /// sprite.set_mesh_image_path("assets/player.png")
    /// ```
    ///
    /// # See Also
    /// - `set_mesh_geometry_circle()` - Set mesh to circle
    /// - `MeshComponent.set_geometry_rectangle()` - Direct mesh method
    fn set_mesh_geometry_rectangle(&mut self, width: f32, height: f32) {
        let mesh = self.ensure_mesh_component();
        mesh.set_geometry(MeshGeometry::rectangle(width, height));
    }

    /// Set the mesh geometry to a circle.
    ///
    /// Configures the object's mesh as a circle with the specified radius and quality.
    /// If no mesh component exists, one is created automatically.
    ///
    /// # Arguments
    /// * `radius` - Circle radius in pixels
    /// * `segments` - Number of segments for circle quality (default: 32)
    ///   - Higher values = smoother circle, more vertices
    ///   - Typical range: 16-64 segments
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// obj = pyg.GameObject("Ball")
    /// obj.position = pyg.Vec2(200.0, 150.0)
    ///
    /// # Create smooth circle with radius 40
    /// obj.set_mesh_geometry_circle(40.0, segments=64)
    /// obj.set_mesh_fill_color(pyg.Color.RED)
    ///
    /// engine.add_game_object(obj)
    /// ```
    ///
    /// # Segment Quality Comparison
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// # Low quality (fast, jagged)
    /// low_poly = pyg.GameObject("LowPoly")
    /// low_poly.set_mesh_geometry_circle(30.0, segments=8)
    ///
    /// # Medium quality (default)
    /// medium = pyg.GameObject("Medium")
    /// medium.set_mesh_geometry_circle(30.0, segments=32)
    ///
    /// # High quality (smooth, more expensive)
    /// high_poly = pyg.GameObject("HighPoly")
    /// high_poly.set_mesh_geometry_circle(30.0, segments=128)
    /// ```
    ///
    /// # See Also
    /// - `set_mesh_geometry_rectangle()` - Set mesh to rectangle
    /// - `MeshComponent.set_geometry_circle()` - Direct mesh method
    #[pyo3(signature = (radius, segments=32))]
    fn set_mesh_geometry_circle(&mut self, radius: f32, segments: u32) {
        let mesh = self.ensure_mesh_component();
        mesh.set_geometry(MeshGeometry::circle(radius, segments));
    }

    /// Set the fill color of the mesh.
    ///
    /// Sets a solid color for the mesh. If no mesh component exists, one is
    /// created automatically. Pass `None` to remove the color (use image only).
    ///
    /// # Arguments
    /// * `color` - `Color` instance, or `None` to remove fill color
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// obj = pyg.GameObject("ColoredRect")
    /// obj.set_mesh_geometry_rectangle(100.0, 100.0)
    ///
    /// # Set solid color
    /// obj.set_mesh_fill_color(pyg.Color.BLUE)
    ///
    /// # Change color later
    /// obj.set_mesh_fill_color(pyg.Color.RED)
    ///
    /// # Remove color (transparent/image only)
    /// obj.set_mesh_fill_color(None)
    /// ```
    ///
    /// # Color Animation
    /// ```python
    /// import pyg_engine as pyg
    /// import math
    ///
    /// def update(dt, engine, data):
    ///     elapsed = data['elapsed']
    ///     data['elapsed'] += dt
    ///
    ///     # Pulse red channel
    ///     red_value = (math.sin(elapsed * 2.0) + 1.0) / 2.0  # 0.0 to 1.0
    ///     color = pyg.Color.new(red_value, 0.0, 0.0, 1.0)
    ///     data['obj'].set_mesh_fill_color(color)
    ///
    ///     return True
    /// ```
    ///
    /// # See Also
    /// - `mesh_fill_color()` - Get current fill color
    /// - `set_mesh_image_path()` - Set texture image
    /// - `Color` - Color class with creation methods
    fn set_mesh_fill_color(&mut self, color: Option<PyColor>) {
        let mesh = self.ensure_mesh_component();
        let color_inner = color.map(|c| c.inner);
        mesh.set_fill_color(color_inner);

        // Update runtime object if it exists
        if let Some(binding) = self.runtime_binding.borrow().as_ref() {
            let _ = binding.sender.send(EngineCommand::SetGameObjectMeshFillColor {
                object_id: binding.object_id,
                color: color_inner,
            });
        }
    }

    /// Get the fill color of the mesh.
    ///
    /// Returns the mesh's solid fill color, or `None` if no color is set or
    /// no mesh exists.
    ///
    /// # Returns
    /// - `Some(Color)` - The mesh fill color
    /// - `None` - No color set or no mesh component
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// obj = pyg.GameObject("Sprite")
    /// obj.set_mesh_fill_color(pyg.Color.BLUE)
    ///
    /// # Get current color
    /// color = obj.mesh_fill_color()
    /// if color:
    ///     print(f"Fill color: {color}")
    /// ```
    ///
    /// # See Also
    /// - `set_mesh_fill_color()` - Set the fill color
    fn mesh_fill_color(&self) -> Option<PyColor> {
        self.inner
            .mesh_component()
            .and_then(|mesh| mesh.fill_color().copied())
            .map(|inner| PyColor { inner })
    }

    /// Set the image/texture path for the mesh.
    ///
    /// Loads and applies an image texture to the mesh. If no mesh component exists,
    /// one is created automatically. Pass `None` to remove the texture (use color only).
    ///
    /// # Arguments
    /// * `image_path` - Path to image file (PNG, JPG, etc.), or `None` to remove
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// sprite = pyg.GameObject("Sprite")
    /// sprite.set_mesh_geometry_rectangle(64.0, 64.0)
    ///
    /// # Load texture
    /// sprite.set_mesh_image_path("assets/player.png")
    ///
    /// # Change texture
    /// sprite.set_mesh_image_path("assets/enemy.png")
    ///
    /// # Remove texture (use color instead)
    /// sprite.set_mesh_image_path(None)
    /// sprite.set_mesh_fill_color(pyg.Color.BLUE)
    /// ```
    ///
    /// # Sprite Animation
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// class AnimatedSprite:
    ///     def __init__(self, obj, frames):
    ///         self.obj = obj
    ///         self.frames = frames  # ["frame0.png", "frame1.png", ...]
    ///         self.current_frame = 0
    ///         self.frame_time = 0.0
    ///         self.frame_duration = 0.1  # seconds per frame
    ///
    ///     def update(self, dt):
    ///         self.frame_time += dt
    ///         if self.frame_time >= self.frame_duration:
    ///             self.frame_time = 0.0
    ///             self.current_frame = (self.current_frame + 1) % len(self.frames)
    ///             self.obj.set_mesh_image_path(self.frames[self.current_frame])
    /// ```
    ///
    /// # See Also
    /// - `mesh_image_path()` - Get current image path
    /// - `set_mesh_fill_color()` - Set solid color
    fn set_mesh_image_path(&mut self, image_path: Option<String>) {
        let mesh = self.ensure_mesh_component();
        mesh.set_image_path(image_path);
    }

    /// Get the image/texture path of the mesh.
    ///
    /// Returns the path to the mesh's texture image, or `None` if no image is set
    /// or no mesh exists.
    ///
    /// # Returns
    /// - `Some(String)` - Path to the image file
    /// - `None` - No image set or no mesh component
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// sprite = pyg.GameObject("Sprite")
    /// sprite.set_mesh_image_path("assets/player.png")
    ///
    /// # Get current texture path
    /// path = sprite.mesh_image_path()
    /// if path:
    ///     print(f"Current texture: {path}")
    /// ```
    ///
    /// # See Also
    /// - `set_mesh_image_path()` - Set the image path
    fn mesh_image_path(&self) -> Option<String> {
        self.inner
            .mesh_component()
            .and_then(|mesh| mesh.image_path().map(|path| path.to_string()))
    }

    /// Set the visibility of the mesh.
    ///
    /// Controls whether the mesh is rendered. If no mesh component exists, one is
    /// created automatically.
    ///
    /// # Arguments
    /// * `visible` - `True` to show, `False` to hide
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// obj = pyg.GameObject("Sprite")
    /// obj.set_mesh_geometry_rectangle(64.0, 64.0)
    /// obj.set_mesh_fill_color(pyg.Color.RED)
    ///
    /// # Hide mesh
    /// obj.set_mesh_visible(False)
    ///
    /// # Show mesh
    /// obj.set_mesh_visible(True)
    /// ```
    ///
    /// # Blinking Effect
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// def update(dt, engine, data):
    ///     data['blink_timer'] += dt
    ///
    ///     # Blink every 0.5 seconds
    ///     if data['blink_timer'] >= 0.5:
    ///         data['blink_timer'] = 0.0
    ///         current = data['obj'].mesh_visible()
    ///         data['obj'].set_mesh_visible(not current)
    ///
    ///     return True
    ///
    /// engine.run(update=update, user_data={'blink_timer': 0.0, 'obj': obj})
    /// ```
    ///
    /// # See Also
    /// - `mesh_visible()` - Get visibility state
    /// - `GameObject.active` - Control entire object update/render
    fn set_mesh_visible(&mut self, visible: bool) {
        let mesh = self.ensure_mesh_component();
        mesh.set_visible(visible);
    }

    /// Get the visibility state of the mesh.
    ///
    /// Returns whether the mesh is currently visible, or `None` if no mesh exists.
    ///
    /// # Returns
    /// - `Some(True)` - Mesh is visible
    /// - `Some(False)` - Mesh is hidden
    /// - `None` - No mesh component exists
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// obj = pyg.GameObject("Sprite")
    /// obj.set_mesh_visible(False)
    ///
    /// # Check visibility
    /// visible = obj.mesh_visible()
    /// if visible == False:
    ///     print("Mesh is hidden")
    /// ```
    ///
    /// # See Also
    /// - `set_mesh_visible()` - Set visibility
    fn mesh_visible(&self) -> Option<bool> {
        self.inner.mesh_component().map(|mesh| mesh.visible())
    }

    /// Set the draw order (z-index) of the mesh.
    ///
    /// Controls rendering order. Higher values render on top of lower values.
    /// If no mesh component exists, one is created automatically.
    ///
    /// # Arguments
    /// * `draw_order` - Rendering layer (higher = on top)
    ///   - Typical range: -10.0 to 10.0
    ///   - Default: 0.0
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// # Background layer
    /// background = pyg.GameObject("Background")
    /// background.set_mesh_draw_order(-5.0)
    ///
    /// # Game objects (default layer)
    /// player = pyg.GameObject("Player")
    /// player.set_mesh_draw_order(0.0)
    ///
    /// # UI elements (foreground)
    /// ui_panel = pyg.GameObject("UI")
    /// ui_panel.set_mesh_draw_order(10.0)
    /// ```
    ///
    /// # Layering Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// # Create layers from back to front
    /// layers = [
    ///     ("Sky", -10.0),
    ///     ("Background", -5.0),
    ///     ("Terrain", -2.0),
    ///     ("Player", 0.0),
    ///     ("Effects", 2.0),
    ///     ("UI", 10.0),
    /// ]
    ///
    /// for name, depth in layers:
    ///     obj = pyg.GameObject(name)
    ///     obj.set_mesh_geometry_rectangle(800.0, 600.0)
    ///     obj.set_mesh_draw_order(depth)
    ///     engine.add_game_object(obj)
    /// ```
    ///
    /// # See Also
    /// - `mesh_draw_order()` - Get current draw order
    /// - `DrawCommand` - Similar draw order for immediate drawing
    fn set_mesh_draw_order(&mut self, draw_order: f32) {
        let mesh = self.ensure_mesh_component();
        mesh.set_draw_order(draw_order);
    }

    /// Get the draw order (z-index) of the mesh.
    ///
    /// Returns the rendering layer of the mesh, or `None` if no mesh exists.
    ///
    /// # Returns
    /// - `Some(f32)` - The draw order value
    /// - `None` - No mesh component exists
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// obj = pyg.GameObject("Sprite")
    /// obj.set_mesh_draw_order(5.0)
    ///
    /// # Get draw order
    /// order = obj.mesh_draw_order()
    /// if order:
    ///     print(f"Draw order: {order}")
    /// ```
    ///
    /// # See Also
    /// - `set_mesh_draw_order()` - Set the draw order
    fn mesh_draw_order(&self) -> Option<f32> {
        self.inner.mesh_component().map(|mesh| mesh.draw_order())
    }

    /// Set the object type for specialized rendering or behavior.
    ///
    /// Marks the object as a specific type, which affects how the engine handles it.
    /// Most commonly used to mark objects as UI elements for screen-space rendering.
    ///
    /// # Arguments
    /// * `object_type` - Type string (case-sensitive):
    ///   - `"UIObject"` - **UI element** (rendered in screen space, ignores camera)
    ///   - `"GameObject"` - **Standard object** (world space, default)
    ///   - `"ParticleSystem"` - Particle system (reserved for future use)
    ///   - `"Sound"` - Audio source (reserved for future use)
    ///   - `"Light"` - Light source (reserved for future use)
    ///   - `"Camera"` - Camera object (reserved for future use)
    ///
    /// # UI Objects
    ///
    /// **UIObject** is the most commonly used type. UI objects:
    /// - Render in **screen-space** coordinates (pixels from top-left)
    /// - **Ignore camera** position and viewport
    /// - Always rendered **on top** of world objects
    /// - Used for: buttons, panels, labels, menus, HUD elements
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// # Standard world-space object
    /// player = pyg.GameObject("Player")
    /// player.set_object_type("GameObject")  # Default, usually not needed
    ///
    /// # UI button (screen space)
    /// button = pyg.GameObject("PlayButton")
    /// button.set_object_type("UIObject")  # Renders in screen space
    /// ```
    ///
    /// # UI Button Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// engine = pyg.Engine()
    ///
    /// # Create UI button
    /// button_obj = pyg.GameObject("Button")
    /// button_obj.set_object_type("UIObject")  # Mark as UI
    ///
    /// # Add button component
    /// button = pyg.ButtonComponent("PlayButton", 100, 100, 200, 50)
    /// button.set_text("Play Game")
    /// button.set_on_click(lambda: print("Clicked!"))
    /// button_obj.add_component(button)
    ///
    /// engine.add_game_object(button_obj)
    /// engine.run(title="UI Demo")
    /// ```
    ///
    /// # Screen-Space vs World-Space
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// # World-space object (affected by camera)
    /// world_obj = pyg.GameObject("WorldSprite")
    /// world_obj.position = pyg.Vec2(100.0, 100.0)  # World coordinates
    /// world_obj.set_mesh_geometry_rectangle(64.0, 64.0)
    /// world_obj.set_mesh_fill_color(pyg.Color.BLUE)
    /// # Moves with camera, affected by camera zoom
    ///
    /// # Screen-space UI object (ignores camera)
    /// ui_obj = pyg.GameObject("UIPanel")
    /// ui_obj.set_object_type("UIObject")
    /// ui_obj.position = pyg.Vec2(10.0, 10.0)  # Screen pixels from top-left
    /// ui_obj.set_mesh_geometry_rectangle(200.0, 100.0)
    /// ui_obj.set_mesh_fill_color(pyg.Color.rgba(0, 0, 0, 128))
    /// # Always at same screen position, ignores camera
    /// ```
    ///
    /// # See Also
    /// - `examples/ui_demo.py` - Complete UI system example
    /// - `ButtonComponent`, `PanelComponent`, `LabelComponent` - UI components
    fn set_object_type(&mut self, object_type: &str) {
        use crate::core::game_object::ObjectType;
        let obj_type = match object_type {
            "UIObject" => ObjectType::UIObject,
            "ParticleSystem" => ObjectType::ParticleSystem,
            "Sound" => ObjectType::Sound,
            "Light" => ObjectType::Light,
            "Camera" => ObjectType::Camera,
            _ => ObjectType::GameObject,
        };
        self.inner.set_object_type(obj_type);
    }

    /// Add a UI component to this GameObject.
    ///
    /// Attaches a UI component (button, panel, or label) to the object. The object
    /// should typically be marked as a `"UIObject"` for proper screen-space rendering.
    ///
    /// # Supported Components
    /// - `ButtonComponent` - Clickable button with callback
    /// - `PanelComponent` - Rectangular UI container/background
    /// - `LabelComponent` - Text label for UI
    ///
    /// # Arguments
    /// * `component` - The UI component to attach (ButtonComponent, PanelComponent, or LabelComponent)
    ///
    /// # Returns
    /// - `Ok(())` - Component added successfully
    /// - `Err(TypeError)` - Invalid component type
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// # Create UI object
    /// ui_obj = pyg.GameObject("UIElement")
    /// ui_obj.set_object_type("UIObject")
    ///
    /// # Add button component
    /// button = pyg.ButtonComponent("ClickMe", 100, 100, 200, 50)
    /// button.set_text("Click Me!")
    /// button.set_on_click(lambda: print("Clicked!"))
    /// ui_obj.add_component(button)
    ///
    /// engine.add_game_object(ui_obj)
    /// ```
    ///
    /// # Complete UI Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// engine = pyg.Engine()
    ///
    /// # Panel background
    /// panel_obj = pyg.GameObject("Panel")
    /// panel_obj.set_object_type("UIObject")
    /// panel = pyg.PanelComponent("Background", 50, 50, 300, 200)
    /// panel.set_background_color(pyg.Color.rgba(50, 50, 50, 200))
    /// panel_obj.add_component(panel)
    /// engine.add_game_object(panel_obj)
    ///
    /// # Title label
    /// label_obj = pyg.GameObject("Title")
    /// label_obj.set_object_type("UIObject")
    /// label = pyg.LabelComponent("TitleText", 100, 70, "Game Menu")
    /// label.set_font_size(24.0)
    /// label.set_text_color(pyg.Color.WHITE)
    /// label_obj.add_component(label)
    /// engine.add_game_object(label_obj)
    ///
    /// # Play button
    /// button_obj = pyg.GameObject("PlayButton")
    /// button_obj.set_object_type("UIObject")
    /// button = pyg.ButtonComponent("Play", 100, 120, 150, 40)
    /// button.set_text("Play Game")
    /// button.set_on_click(lambda: print("Starting game..."))
    /// button_obj.add_component(button)
    /// engine.add_game_object(button_obj)
    ///
    /// engine.run(title="UI Example")
    /// ```
    ///
    /// # Multiple Components (Not Recommended)
    /// ```python
    /// # While technically possible, avoid adding multiple components of the same type
    /// # to a single GameObject. Instead, create separate GameObjects for each UI element.
    ///
    /// # ❌ DON'T: Multiple buttons on one object
    /// obj = pyg.GameObject("Buttons")
    /// obj.add_component(button1)
    /// obj.add_component(button2)  # Replaces button1!
    ///
    /// # ✅ DO: Separate objects for each button
    /// obj1 = pyg.GameObject("Button1")
    /// obj1.add_component(button1)
    ///
    /// obj2 = pyg.GameObject("Button2")
    /// obj2.add_component(button2)
    /// ```
    ///
    /// # Errors
    /// Raises `TypeError` if the component is not a valid UI component type:
    /// ```python
    /// obj.add_component(mesh)  # TypeError: not a UI component
    /// obj.add_component(button)  # OK: ButtonComponent
    /// ```
    ///
    /// # See Also
    /// - `set_object_type()` - Mark object as UIObject
    /// - `ButtonComponent` - Clickable button
    /// - `PanelComponent` - UI panel/background
    /// - `LabelComponent` - Text label
    /// - `examples/ui_demo.py` - Complete UI examples
    fn add_component(&mut self, component: &Bound<'_, PyAny>) -> PyResult<()> {
        // Try to downcast to each component type
        if let Ok(button) = component.extract::<PyRef<PyButtonComponent>>() {
            self.inner.add_component(Box::new(button.inner.clone()));
            Ok(())
        } else if let Ok(panel) = component.extract::<PyRef<PyPanelComponent>>() {
            self.inner.add_component(Box::new(panel.inner.clone()));
            Ok(())
        } else if let Ok(label) = component.extract::<PyRef<PyLabelComponent>>() {
            self.inner.add_component(Box::new(label.inner.clone()));
            Ok(())
        } else if let Ok(collider) = component.extract::<PyRef<PyCollider>>() {
            self.inner.add_component(Box::new(collider.component.clone()));
            Ok(())
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Component must be ButtonComponent, PanelComponent, LabelComponent, or Collider"
            ))
        }
    }
}

// ========== MeshComponent Bindings ==========

/// Python wrapper for MeshComponent.
/// 2D mesh rendering component for GameObjects.
///
/// `MeshComponent` handles rendering of 2D shapes and images. Attach it to a GameObject
/// to make it visible. Supports:
/// - **Geometry**: Rectangles, circles (custom geometry coming soon)
/// - **Rendering**: Solid colors, images/textures
/// - **Control**: Visibility, draw order (z-index)
///
/// # Basic Usage
///
/// ## Rectangle with Color
/// ```python
/// from pyg_engine import GameObject, MeshComponent, Color, Vec2
///
/// obj = GameObject("ColoredRect")
/// obj.set_position(Vec2(100.0, 100.0))
///
/// mesh = MeshComponent()
/// mesh.set_geometry_rectangle(64.0, 64.0)  # 64x64 square
/// mesh.set_fill_color(Color.BLUE)
/// obj.add_component(mesh)
///
/// engine.add_game_object(obj)
/// ```
///
/// ## Circle with Image
/// ```python
/// from pyg_engine import GameObject, MeshComponent, Vec2
///
/// obj = GameObject("Sprite")
/// obj.set_position(Vec2(200.0, 150.0))
///
/// mesh = MeshComponent()
/// mesh.set_geometry_circle(32.0, segments=64)  # Radius 32, smooth circle
/// mesh.set_image_path("assets/player.png")
/// obj.add_component(mesh)
///
/// engine.add_game_object(obj)
/// ```
///
/// # Draw Order
///
/// Control rendering order with `draw_order` (higher values draw on top):
/// ```python
/// background.draw_order = -5.0  # Draw behind
/// player.draw_order = 0.0       # Default layer
/// ui_element.draw_order = 10.0  # Draw in front
/// ```
///
/// # Visibility
///
/// Toggle visibility without removing the object:
/// ```python
/// mesh.visible = False  # Hide
/// mesh.visible = True   # Show
/// ```
///
/// # See Also
/// - `examples/python_mesh_demo.py` - Complete mesh examples
/// - `examples/python_game_object_transform_demo.py` - Transform + rendering
#[pyclass(name = "MeshComponent")]
#[derive(Clone)]
pub struct PyMeshComponent {
    inner: MeshComponent,
}

#[pymethods]
impl PyMeshComponent {
    #[new]
    #[pyo3(signature = (name=None))]
    fn new(name: Option<String>) -> Self {
        Self {
            inner: MeshComponent::new(name.unwrap_or_else(|| "Mesh Renderer".to_string())),
        }
    }

    #[getter]
    fn name(&self) -> String {
        self.inner.name().to_string()
    }

    fn set_geometry_rectangle(&mut self, width: f32, height: f32) {
        self.inner
            .set_geometry(MeshGeometry::rectangle(width, height));
    }

    #[pyo3(signature = (radius, segments=32))]
    fn set_geometry_circle(&mut self, radius: f32, segments: u32) {
        self.inner
            .set_geometry(MeshGeometry::circle(radius, segments));
    }

    fn set_fill_color(&mut self, color: Option<PyColor>) {
        self.inner.set_fill_color(color.map(|c| c.inner));
    }

    fn fill_color(&self) -> Option<PyColor> {
        self.inner
            .fill_color()
            .copied()
            .map(|inner| PyColor { inner })
    }

    fn set_image_path(&mut self, image_path: Option<String>) {
        self.inner.set_image_path(image_path);
    }

    fn image_path(&self) -> Option<String> {
        self.inner.image_path().map(|path| path.to_string())
    }

    #[getter]
    fn visible(&self) -> bool {
        self.inner.visible()
    }

    #[setter]
    fn set_visible(&mut self, visible: bool) {
        self.inner.set_visible(visible);
    }

    #[getter]
    fn draw_order(&self) -> f32 {
        self.inner.draw_order()
    }

    #[setter]
    fn set_draw_order(&mut self, draw_order: f32) {
        self.inner.set_draw_order(draw_order);
    }
}

// ========== Component Bindings ==========

/// 2D transform component for position, rotation, and scale.
///
/// `TransformComponent` defines the spatial properties of a GameObject in 2D space.
/// Every GameObject has a transform that controls where it appears and how it's oriented.
///
/// # Properties
///
/// - **position**: `Vec2` - Location in world coordinates
/// - **rotation**: `float` - Angle in **radians** (counter-clockwise)
/// - **scale**: `Vec2` - Size multiplier (1.0 = original size)
///
/// # Coordinate System
///
/// - **World space**: Absolute positions in the game world
/// - **Origin**: Typically center of the screen, but configurable via camera
/// - **Y-axis**: Typically up is positive, down is negative
///
/// # Rotation
///
/// Rotation is measured in **radians** (not degrees):
/// - **0 radians**: Facing right (0°)
/// - **π/2 radians**: Facing up (90°)
/// - **π radians**: Facing left (180°)
/// - **3π/2 radians**: Facing down (270°)
///
/// Use `math.radians()` to convert from degrees:
/// ```python
/// import math
/// transform.rotation = math.radians(45)  # 45 degrees
/// ```
///
/// # Examples
///
/// ## Basic Transform
/// ```python
/// from pyg_engine import TransformComponent, Vec2
/// import math
///
/// transform = TransformComponent("Transform")
/// transform.position = Vec2(100.0, 200.0)
/// transform.rotation = math.radians(30)  # 30 degrees
/// transform.scale = Vec2(2.0, 2.0)       # Double size
/// ```
///
/// ## Movement Over Time
/// ```python
/// import math
///
/// def update(dt, engine, data):
///     # Move right at 100 pixels/second
///     data['obj'].position = data['obj'].position + Vec2(100.0 * dt, 0.0)
///
///     # Rotate at 90 degrees/second
///     data['obj'].rotation += math.radians(90) * dt
/// ```
///
/// ## Direction-Based Movement
/// ```python
/// import math
/// from pyg_engine import Vec2
///
/// def update(dt, engine, data):
///     obj = data['obj']
///     speed = 150.0
///
///     # Move in the direction the object is facing
///     angle = obj.rotation
///     direction = Vec2(math.cos(angle), math.sin(angle))
///     obj.position = obj.position + direction * speed * dt
/// ```
///
/// # See Also
/// - `examples/python_game_object_transform_demo.py` - Transform examples
/// - `GameObject` - The container that owns this transform
#[pyclass(name = "TransformComponent")]
pub struct PyTransformComponent {
    inner: TransformComponent,
}

#[pymethods]
impl PyTransformComponent {
    #[new]
    fn new(name: String) -> Self {
        Self {
            inner: TransformComponent::new(name),
        }
    }

    #[getter]
    fn position(&self) -> PyVec2 {
        PyVec2 {
            inner: *self.inner.position(),
        }
    }

    #[setter]
    fn set_position(&mut self, position: PyVec2) {
        self.inner.set_position(position.inner);
    }

    #[getter]
    fn rotation(&self) -> f32 {
        self.inner.rotation()
    }

    #[setter]
    fn set_rotation(&mut self, rotation: f32) {
        self.inner.set_rotation(rotation);
    }

    #[getter]
    fn scale(&self) -> PyVec2 {
        PyVec2 {
            inner: *self.inner.scale(),
        }
    }

    #[setter]
    fn set_scale(&mut self, scale: PyVec2) {
        self.inner.set_scale(scale.inner);
    }

    #[getter]
    fn name(&self) -> String {
        self.inner.name().to_string()
    }
}

// ========== Module Functions ==========

/// Get the engine version (module-level function).
#[pyfunction]
fn version() -> String {
    crate::core::engine::VERSION.to_string()
}

// ========== UI Component Bindings ==========

/// Python wrapper for ButtonComponent.
#[pyclass(name = "ButtonComponent")]
pub struct PyButtonComponent {
    inner: ButtonComponent,
}

#[pymethods]
impl PyButtonComponent {
    #[new]
    #[pyo3(signature = (text="", x=0.0, y=0.0, width=100.0, height=30.0))]
    fn new(text: &str, x: f32, y: f32, width: f32, height: f32) -> Self {
        let button = ButtonComponent::new("Button")
            .with_text(text)
            .with_bounds(x, y, width, height);
        Self { inner: button }
    }

    fn set_text(&mut self, text: &str) {
        self.inner.set_text(text);
    }

    fn get_text(&self) -> String {
        self.inner.text().to_string()
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.inner.set_enabled(enabled);
    }

    fn set_position(&mut self, x: f32, y: f32) {
        let bounds = self.inner.bounds();
        self.inner.set_bounds(Rect::new(x, y, bounds.width, bounds.height));
    }

    fn set_size(&mut self, width: f32, height: f32) {
        let bounds = self.inner.bounds();
        self.inner.set_bounds(Rect::new(bounds.x, bounds.y, width, height));
    }

    fn set_depth(&mut self, depth: f32) {
        self.inner = std::mem::replace(&mut self.inner, ButtonComponent::new("temp"))
            .with_depth(depth);
    }

    /// Set a Python callback for the button click event.
    ///
    /// Registers a function to be called when the button is clicked. The callback executes
    /// on the **main engine thread** during event processing, ensuring safe interaction with
    /// the engine and UI system.
    ///
    /// # Arguments
    /// * `py_callback` - A Python callable that takes **no arguments**. The callback should
    ///   not accept any parameters (not even `self` if it's a method).
    ///
    /// # Callback Signature
    /// The callback must have this signature:
    /// ```python
    /// def callback() -> None:
    ///     # Your code here
    ///     pass
    /// ```
    ///
    /// # Error Handling
    /// If the callback raises an exception:
    /// - The exception is **logged** to the console with stack trace
    /// - The exception **does not crash** the engine
    /// - Other buttons and UI elements continue to function normally
    ///
    /// # Thread Safety
    /// - Callbacks run on the **main engine thread**
    /// - Safe to call any engine methods from the callback
    /// - Safe to modify UI elements and game state
    /// - No manual thread synchronization needed
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// engine = pyg.Engine()
    ///
    /// # Counter for demonstration
    /// click_count = [0]  # Use list for closure modification
    ///
    /// # Define callback (takes no arguments!)
    /// def on_button_clicked():
    ///     click_count[0] += 1
    ///     print(f"Button clicked! Count: {click_count[0]}")
    ///
    /// # Create button with callback
    /// button = pyg.Button(
    ///     "Click Me",
    ///     x=100,
    ///     y=100,
    ///     width=200,
    ///     height=50
    /// )
    /// button.set_on_click(on_button_clicked)
    /// button.set_trigger_on("release")  # Fire on mouse release (default)
    ///
    /// # Add to UI system
    /// engine.ui.add(button)
    ///
    /// # Run engine
    /// engine.run(title="Button Callback Demo")
    /// ```
    ///
    /// # Alternative: Pass Callback in Constructor
    /// ```python
    /// # Shorter syntax using constructor parameter
    /// button = pyg.Button(
    ///     "Click Me",
    ///     x=100,
    ///     y=100,
    ///     width=200,
    ///     height=50,
    ///     on_click=lambda: print("Clicked!")
    /// )
    /// engine.ui.add(button)
    /// ```
    ///
    /// # Closure Example (Modifying External State)
    /// ```python
    /// # Use lists or objects for mutable state in closures
    /// state = {"enabled": True, "count": 0}
    ///
    /// def toggle_button():
    ///     state["enabled"] = not state["enabled"]
    ///     print(f"Enabled: {state['enabled']}")
    ///
    /// button.set_on_click(toggle_button)
    /// ```
    ///
    /// # Updating UI from Callback
    /// ```python
    /// label = pyg.Label("Count: 0", x=100, y=50, font_size=18)
    /// engine.ui.add(label)
    ///
    /// count = [0]
    ///
    /// def increment():
    ///     count[0] += 1
    ///     label.text = f"Count: {count[0]}"  # Safe - on main thread
    ///
    /// button = pyg.Button("Increment", x=100, y=100, width=150, height=40)
    /// button.set_on_click(increment)
    /// engine.ui.add(button)
    /// ```
    ///
    /// # Trigger Timing
    /// Control when the callback fires with `set_trigger_on()`:
    /// ```python
    /// # Fire on mouse release (default - feels like a "click")
    /// button.set_trigger_on("release")
    ///
    /// # Fire immediately on mouse press (more responsive)
    /// button.set_trigger_on("press")
    /// ```
    ///
    /// # Repeat Behavior
    /// Enable continuous firing while button is held:
    /// ```python
    /// # Fire every 100ms while held down
    /// button.set_repeat_interval(100.0)
    ///
    /// # Disable repeating (single fire per click)
    /// button.set_repeat_interval(None)
    /// ```
    ///
    /// # Common Patterns
    ///
    /// **Menu Button:**
    /// ```python
    /// def start_game():
    ///     engine.log("Starting game...")
    ///     # Load game scene, hide menu, etc.
    ///
    /// start_btn = pyg.Button("Start Game", x=400, y=300, width=200, height=60)
    /// start_btn.set_on_click(start_game)
    /// ```
    ///
    /// **Toggle Button:**
    /// ```python
    /// paused = [False]
    /// pause_btn = [None]
    ///
    /// def toggle_pause():
    ///     paused[0] = not paused[0]
    ///     pause_btn[0].text = "Resume" if paused[0] else "Pause"
    ///
    /// pause_btn[0] = pyg.Button("Pause", x=10, y=10, width=100, height=30)
    /// pause_btn[0].set_on_click(toggle_pause)
    /// ```
    ///
    /// # See Also
    /// - `examples/button_features_demo.py` - Button trigger modes and repeat functionality
    /// - `examples/ui_demo.py` - Complete UI system demonstration
    /// - `set_trigger_on()` - Configure when callback fires (press vs release)
    /// - `set_repeat_interval()` - Enable continuous firing while held
    fn set_on_click(&mut self, py_callback: Py<PyAny>) {
        self.inner.set_on_click(move || {
            // Use attach to ensure we have the GIL when calling Python callback
            // from the Rust event loop context
            let _ = pyo3::Python::attach(|py| {
                match py_callback.call0(py) {
                    Ok(_) => {},
                    Err(e) => {
                        e.print(py);

                        logging::log_error(&format!(
                                "Error calling button callback: {:?}",
                                e
                        ));
                    }
                }
            });
        });
    }

    /// Set when the button callback is triggered.
    ///
    /// Controls whether the button's `on_click` callback fires when the mouse button goes
    /// **down** (press) or **up** (release). This affects the button's responsiveness and
    /// "feel" for different use cases.
    ///
    /// # Arguments
    /// * `trigger_on` - Trigger mode string (case-insensitive). Valid values:
    ///   - **`"release"`** (default) - Fire callback when mouse button is released
    ///     - Standard button behavior, feels like a "click"
    ///     - User can cancel by moving mouse away before releasing
    ///     - Aliases: `"up"`, `"click"`
    ///
    ///   - **`"press"`** - Fire callback immediately when mouse button goes down
    ///     - More responsive, instant feedback
    ///     - No way to cancel once pressed
    ///     - Aliases: `"down"`
    ///
    /// Invalid values log a warning and default to `"release"`.
    ///
    /// # Press vs Release
    ///
    /// **Release (default):**
    /// - Traditional button behavior
    /// - User can press, move away, and release to cancel
    /// - Feels like "clicking" the button
    /// - Best for: Menu buttons, confirmations, UI interactions
    ///
    /// **Press:**
    /// - Instant response on mouse down
    /// - No cancel mechanism
    /// - Feels more immediate and game-like
    /// - Best for: Game controls, rapid-fire actions, instant response needs
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// engine = pyg.Engine()
    ///
    /// # Standard release button (default)
    /// menu_btn = pyg.Button("Start Game", x=100, y=100, width=200, height=50)
    /// menu_btn.set_on_click(lambda: print("Game started!"))
    /// menu_btn.set_trigger_on("release")  # Fire on mouse up
    /// engine.ui.add(menu_btn)
    ///
    /// # Instant press button (more responsive)
    /// fire_btn = pyg.Button("Fire!", x=100, y=200, width=200, height=50)
    /// fire_btn.set_on_click(lambda: print("Shooting!"))
    /// fire_btn.set_trigger_on("press")  # Fire on mouse down
    /// engine.ui.add(fire_btn)
    ///
    /// engine.run(title="Button Trigger Demo")
    /// ```
    ///
    /// # Comparison Demo
    /// ```python
    /// press_count = [0]
    /// release_count = [0]
    ///
    /// def on_press():
    ///     press_count[0] += 1
    ///     print(f"Press: {press_count[0]} (fires immediately)")
    ///
    /// def on_release():
    ///     release_count[0] += 1
    ///     print(f"Release: {release_count[0]} (fires on mouse up)")
    ///
    /// press_btn = pyg.Button("Press Mode", x=100, y=100, width=150, height=40)
    /// press_btn.set_on_click(on_press)
    /// press_btn.set_trigger_on("press")
    ///
    /// release_btn = pyg.Button("Release Mode", x=100, y=150, width=150, height=40)
    /// release_btn.set_on_click(on_release)
    /// release_btn.set_trigger_on("release")  # Or omit - this is default
    ///
    /// engine.ui.add(press_btn)
    /// engine.ui.add(release_btn)
    /// ```
    ///
    /// # Use Cases
    ///
    /// **Use "release" for:**
    /// - Menu navigation
    /// - Confirmation dialogs
    /// - Submit buttons
    /// - Any action that should allow cancellation
    ///
    /// **Use "press" for:**
    /// - Game weapon firing
    /// - Jump buttons
    /// - Fast-paced interactions
    /// - When instant response is critical
    ///
    /// # See Also
    /// - `examples/button_features_demo.py` - Trigger mode comparison
    /// - `set_on_click()` - Set button callback
    /// - `set_repeat_interval()` - Enable continuous firing while held
    fn set_trigger_on(&mut self, trigger_on: &str) {
        use crate::core::ui::button::ButtonTrigger;
        let trigger = match trigger_on.to_lowercase().as_str() {
            "press" | "down" => ButtonTrigger::Press,
            "release" | "up" | "click" => ButtonTrigger::Release,
            _ => {
                logging::log_warn(&format!(
                    "Warning: Invalid trigger_on value '{}'. Use 'press' or 'release'. Defaulting to 'release'.",
                    trigger_on
                ));
                ButtonTrigger::Release
            }
        };
        self.inner.set_trigger_on(trigger);
    }

    /// Set the repeat interval in milliseconds for when the button is held down.
    ///
    /// Enables continuous callback firing while the button remains pressed. After the initial
    /// trigger, the callback will fire repeatedly at the specified interval until the button
    /// is released. Useful for increment/decrement buttons, scrolling, or auto-fire mechanics.
    ///
    /// # Arguments
    /// * `interval_ms` - Repeat interval in **milliseconds**, or `None` to disable repeating:
    ///   - **`None`** (default) - Single fire per click, no repeating
    ///   - **`> 0`** - Fire callback every N milliseconds while held
    ///   - Typical values: 50-500ms (50ms = 20Hz, 100ms = 10Hz, 500ms = 2Hz)
    ///
    /// # Behavior
    /// 1. Button is clicked → Callback fires immediately
    /// 2. Button remains held → Wait `interval_ms`
    /// 3. While still held → Callback fires again
    /// 4. Repeat step 3 until button released
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// engine = pyg.Engine()
    ///
    /// counter = [0]
    /// label = [None]
    ///
    /// def increment():
    ///     counter[0] += 1
    ///     if label[0]:
    ///         label[0].text = f"Count: {counter[0]}"
    ///
    /// def decrement():
    ///     counter[0] -= 1
    ///     if label[0]:
    ///         label[0].text = f"Count: {counter[0]}"
    ///
    /// # Counter display
    /// counter_label = pyg.Label(f"Count: {counter[0]}", x=200, y=100, font_size=24)
    /// engine.ui.add(counter_label)
    /// label[0] = counter_label
    ///
    /// # Increment button with repeat (fires every 100ms when held)
    /// inc_btn = pyg.Button("+", x=100, y=150, width=60, height=40)
    /// inc_btn.set_on_click(increment)
    /// inc_btn.set_repeat_interval(100.0)  # Repeat every 100ms
    /// inc_btn.set_trigger_on("press")  # Start immediately on press
    /// engine.ui.add(inc_btn)
    ///
    /// # Decrement button with repeat
    /// dec_btn = pyg.Button("-", x=170, y=150, width=60, height=40)
    /// dec_btn.set_on_click(decrement)
    /// dec_btn.set_repeat_interval(100.0)
    /// dec_btn.set_trigger_on("press")
    /// engine.ui.add(dec_btn)
    ///
    /// engine.run(title="Repeat Button Demo")
    /// ```
    ///
    /// # Faster Repeat for Acceleration
    /// ```python
    /// # Fast repeating button for rapid increment
    /// rapid_btn = pyg.Button("Fast +", x=100, y=200, width=80, height=40)
    /// rapid_btn.set_on_click(increment)
    /// rapid_btn.set_repeat_interval(50.0)  # 20 times per second
    /// engine.ui.add(rapid_btn)
    /// ```
    ///
    /// # Disable Repeating
    /// ```python
    /// # Single fire only (default behavior)
    /// single_btn = pyg.Button("Click Once", x=100, y=250, width=120, height=40)
    /// single_btn.set_on_click(lambda: print("Single fire"))
    /// single_btn.set_repeat_interval(None)  # No repeating
    /// engine.ui.add(single_btn)
    /// ```
    ///
    /// # Scroll Button Example
    /// ```python
    /// scroll_offset = [0]
    ///
    /// def scroll_up():
    ///     scroll_offset[0] -= 10
    ///     print(f"Scroll offset: {scroll_offset[0]}")
    ///
    /// def scroll_down():
    ///     scroll_offset[0] += 10
    ///     print(f"Scroll offset: {scroll_offset[0]}")
    ///
    /// up_btn = pyg.Button("▲", x=700, y=50, width=40, height=40)
    /// up_btn.set_on_click(scroll_up)
    /// up_btn.set_repeat_interval(150.0)  # Smooth scrolling
    /// engine.ui.add(up_btn)
    ///
    /// down_btn = pyg.Button("▼", x=700, y=100, width=40, height=40)
    /// down_btn.set_on_click(scroll_down)
    /// down_btn.set_repeat_interval(150.0)
    /// engine.ui.add(down_btn)
    /// ```
    ///
    /// # Typical Interval Values
    /// - **50-80ms**: Very fast, rapid increment (shooting, fast scrolling)
    /// - **100-150ms**: Fast, comfortable for continuous actions
    /// - **200-300ms**: Moderate, good for volume/brightness controls
    /// - **500ms+**: Slow, deliberate repeating
    ///
    /// # Notes
    /// - First callback fires immediately on trigger (no delay)
    /// - Subsequent callbacks fire at the specified interval
    /// - Works with both `"press"` and `"release"` trigger modes
    /// - Setting `None` disables repeating (reverts to single fire)
    ///
    /// # See Also
    /// - `examples/button_features_demo.py` - Repeat functionality demonstration
    /// - `set_on_click()` - Set button callback
    /// - `set_trigger_on()` - Configure trigger timing
    fn set_repeat_interval(&mut self, interval_ms: Option<f32>) {
        self.inner.set_repeat_interval_ms(interval_ms);
    }

    #[getter]
    fn name(&self) -> String {
        self.inner.name().to_string()
    }
}

/// Python wrapper for PanelComponent.
#[pyclass(name = "PanelComponent")]
pub struct PyPanelComponent {
    inner: PanelComponent,
}

#[pymethods]
impl PyPanelComponent {
    #[new]
    #[pyo3(signature = (x=0.0, y=0.0, width=200.0, height=200.0))]
    fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        let panel = PanelComponent::new("Panel")
            .with_bounds(x, y, width, height);
        Self { inner: panel }
    }

    fn set_position(&mut self, x: f32, y: f32) {
        let bounds = self.inner.bounds();
        self.inner.set_bounds(Rect::new(x, y, bounds.width, bounds.height));
    }

    fn set_size(&mut self, width: f32, height: f32) {
        let bounds = self.inner.bounds();
        self.inner.set_bounds(Rect::new(bounds.x, bounds.y, width, height));
    }

    fn set_depth(&mut self, depth: f32) {
        self.inner = std::mem::replace(&mut self.inner, PanelComponent::new("temp"))
            .with_depth(depth);
    }

    fn set_background_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.inner.style_mut().background_color = [r, g, b, a];
    }

    fn set_border(&mut self, width: f32, r: f32, g: f32, b: f32, a: f32) {
        let style = self.inner.style_mut();
        style.border_width = width;
        style.border_color = [r, g, b, a];
    }

    #[getter]
    fn name(&self) -> String {
        self.inner.name().to_string()
    }
}

/// Python wrapper for LabelComponent.
#[pyclass(name = "LabelComponent")]
pub struct PyLabelComponent {
    inner: LabelComponent,
}

#[pymethods]
impl PyLabelComponent {
    #[new]
    #[pyo3(signature = (text="", x=0.0, y=0.0, font_size=14.0))]
    fn new(text: &str, x: f32, y: f32, font_size: f32) -> Self {
        let mut label = LabelComponent::new("Label")
            .with_text(text)
            .with_position(x, y);
        label.set_font_size(font_size);
        Self { inner: label }
    }

    fn set_text(&mut self, text: &str) {
        self.inner.set_text(text);
    }

    fn get_text(&self) -> String {
        self.inner.text().to_string()
    }

    fn set_position(&mut self, x: f32, y: f32) {
        self.inner.set_position(x, y);
    }

    fn set_font_size(&mut self, size: f32) {
        self.inner.set_font_size(size);
    }

    fn set_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.inner.set_color([r, g, b, a]);
    }

    fn set_align(&mut self, align: &str) {
        let text_align = match align.to_lowercase().as_str() {
            "left" => TextAlign::Left,
            "center" => TextAlign::Center,
            "right" => TextAlign::Right,
            _ => TextAlign::Left,
        };
        self.inner.set_align(text_align);
    }

    fn set_depth(&mut self, depth: f32) {
        self.inner = std::mem::replace(&mut self.inner, LabelComponent::new("temp"))
            .with_depth(depth);
    }

    #[getter]
    fn name(&self) -> String {
        self.inner.name().to_string()
    }
}

// ========== Module Initialization ==========

/// Module initialization function.
#[pymodule]
fn pyg_engine_native(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(version, m)?)?;
    m.add_class::<PyEngine>()?;
    m.add_class::<PyEngineHandle>()?;
    m.add_class::<PyDrawCommand>()?;
    m.add_class::<PyVec2>()?;
    m.add_class::<PyVec3>()?;
    m.add_class::<PyColor>()?;
    m.add_class::<PyTime>()?;
    m.add_class::<PyGameObject>()?;
    m.add_class::<PyMeshComponent>()?;
    m.add_class::<PyTransformComponent>()?;
    m.add_class::<PyButtonComponent>()?;
    m.add_class::<PyPanelComponent>()?;
    m.add_class::<PyLabelComponent>()?;
    m.add_class::<PyCameraAspectMode>()?;
    m.add_class::<PyMouseButton>()?;
    m.add_class::<PyKeys>()?;

    // Register physics bindings
    crate::bindings::physics_bind::register_physics_bindings(m)?;

    Ok(())
}
