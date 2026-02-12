//! Immediate-mode drawing API for 2D graphics.
//!
//! The draw manager accumulates draw commands each frame which are then batched
//! and rendered by the render manager. All drawing uses screen-space pixel coordinates
//! with origin at top-left (0, 0).
//!
//! # Coordinate System
//!
//! **Screen-space coordinates:**
//! - Origin: Top-left corner (0, 0)
//! - X-axis: Increases to the right
//! - Y-axis: Increases downward
//! - Units: Pixels
//!
//! # Draw Order (Z-Index)
//!
//! All draw commands include a `draw_order` field that controls rendering order:
//! - Higher values render **on top** (closer to camera)
//! - Lower values render **behind** (further from camera)
//! - Default: 0.0
//! - Typical ranges:
//!   - Background: -10.0 to 0.0
//!   - Game objects: 0.0 to 5.0
//!   - UI elements: 10.0 to 20.0
//!
//! # Usage Patterns
//!
//! ## Direct Drawing (via Engine)
//! ```rust
//! # use pyg_engine::Engine;
//! # use pyg_engine::Color;
//! # let mut engine = Engine::new();
//! engine.draw_line(0.0, 0.0, 100.0, 100.0, 2.0, Color::WHITE, 0.0);
//! engine.draw_circle(50.0, 50.0, 25.0, Color::RED, true, 1.0, 32, 0.0);
//! ```
//!
//! ## Bulk Drawing (DrawCommand API)
//! ```rust
//! # use pyg_engine::DrawCommand;
//! # use pyg_engine::Color;
//! let commands = vec![
//!     DrawCommand::Rectangle {
//!         x: 10.0, y: 10.0, width: 80.0, height: 80.0,
//!         color: Color::BLUE, filled: true, thickness: 1.0, draw_order: 1.0,
//!     },
//!     DrawCommand::Circle {
//!         center_x: 50.0, center_y: 50.0, radius: 20.0,
//!         color: Color::RED, filled: true, thickness: 1.0, segments: 32, draw_order: 2.0,
//!     },
//! ];
//! // Submit to draw manager
//! ```
//!
//! # See Also
//! - Python examples: `examples/python_direct_draw_demo.py`, `examples/python_bulk_draw_demo.py`
//! - [`RenderManager`](crate::core::render_manager::RenderManager) - Processes draw commands

use crate::types::Color;
use std::sync::Arc;

/// Immediate-mode draw command for 2D rendering.
///
/// `DrawCommand` variants represent individual drawing operations that can be
/// submitted to the engine for rendering. All coordinates are in screen-space
/// pixels with origin at top-left (0, 0).
///
/// # Draw Order
///
/// All variants include a `draw_order` field that controls rendering order:
/// - **Higher values** render **on top** (z-index behavior)
/// - **Lower values** render **behind**
///
/// # Variants
///
/// - [`Pixel`](DrawCommand::Pixel) - Single pixel
/// - [`Line`](DrawCommand::Line) - Line segment with thickness
/// - [`Rectangle`](DrawCommand::Rectangle) - Filled or outlined rectangle
/// - [`Circle`](DrawCommand::Circle) - Filled or outlined circle with configurable segments
/// - [`GradientRect`](DrawCommand::GradientRect) - Rectangle with gradient between corners
/// - [`Image`](DrawCommand::Image) - Image loaded from file path
/// - [`ImageBytes`](DrawCommand::ImageBytes) - Image from raw RGBA pixel data
/// - [`Text`](DrawCommand::Text) - Text rendered with TrueType font
///
/// # Examples
///
/// ```rust
/// use pyg_engine::DrawCommand;
/// use pyg_engine::Color;
///
/// // Draw a line
/// let line = DrawCommand::Line {
///     start_x: 0.0,
///     start_y: 0.0,
///     end_x: 100.0,
///     end_y: 100.0,
///     thickness: 2.0,
///     color: Color::WHITE,
///     draw_order: 5.0,
/// };
///
/// // Draw a filled circle
/// let circle = DrawCommand::Circle {
///     center_x: 50.0,
///     center_y: 50.0,
///     radius: 25.0,
///     color: Color::RED,
///     filled: true,
///     thickness: 1.0,
///     segments: 32,
///     draw_order: 10.0,
/// };
/// ```
#[derive(Clone, Debug)]
pub enum DrawCommand {
    /// Draw a single pixel at the specified position.
    ///
    /// # Fields
    /// - `x`, `y`: Screen coordinates in pixels
    /// - `color`: Pixel color
    /// - `draw_order`: Rendering layer (higher = on top)
    Pixel {
        x: f32,
        y: f32,
        color: Color,
        draw_order: f32,
    },

    /// Draw a line segment between two points.
    ///
    /// # Fields
    /// - `start_x`, `start_y`: Line start point in screen pixels
    /// - `end_x`, `end_y`: Line end point in screen pixels
    /// - `thickness`: Line width in pixels
    /// - `color`: Line color
    /// - `draw_order`: Rendering layer (higher = on top)
    Line {
        start_x: f32,
        start_y: f32,
        end_x: f32,
        end_y: f32,
        thickness: f32,
        color: Color,
        draw_order: f32,
    },

    /// Draw a rectangle at the specified position.
    ///
    /// Position (x, y) represents the **top-left corner**.
    ///
    /// # Fields
    /// - `x`, `y`: Top-left corner position in screen pixels
    /// - `width`, `height`: Rectangle dimensions in pixels
    /// - `color`: Rectangle color
    /// - `filled`: If `true`, fills rectangle; if `false`, draws outline only
    /// - `thickness`: Outline width in pixels (only used when `filled = false`)
    /// - `draw_order`: Rendering layer (higher = on top)
    Rectangle {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color: Color,
        filled: bool,
        thickness: f32,
        draw_order: f32,
    },

    /// Draw a circle at the specified center position.
    ///
    /// # Fields
    /// - `center_x`, `center_y`: Circle center position in screen pixels
    /// - `radius`: Circle radius in pixels
    /// - `color`: Circle color
    /// - `filled`: If `true`, fills circle; if `false`, draws outline only
    /// - `thickness`: Outline width in pixels (only used when `filled = false`)
    /// - `segments`: Number of segments for circle approximation (higher = smoother, default: 32)
    /// - `draw_order`: Rendering layer (higher = on top)
    Circle {
        center_x: f32,
        center_y: f32,
        radius: f32,
        color: Color,
        filled: bool,
        thickness: f32,
        segments: u32,
        draw_order: f32,
    },

    /// Draw a rectangle with gradient colors at each corner.
    ///
    /// Creates smooth color interpolation between the four corners using
    /// bilinear interpolation.
    ///
    /// # Fields
    /// - `x`, `y`: Top-left corner position in screen pixels
    /// - `width`, `height`: Rectangle dimensions in pixels
    /// - `top_left`, `bottom_left`, `bottom_right`, `top_right`: Colors at each corner
    /// - `draw_order`: Rendering layer (higher = on top)
    GradientRect {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        top_left: Color,
        bottom_left: Color,
        bottom_right: Color,
        top_right: Color,
        draw_order: f32,
    },

    /// Draw an image loaded from a file path.
    ///
    /// Loads and renders an image file at the specified position. Images are
    /// automatically cached by path for performance.
    ///
    /// # Fields
    /// - `x`, `y`: Top-left corner position in screen pixels
    /// - `width`, `height`: Display dimensions in pixels (may scale image)
    /// - `texture_path`: File path to image (PNG, JPEG, BMP, etc.)
    /// - `draw_order`: Rendering layer (higher = on top)
    Image {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        texture_path: String,
        draw_order: f32,
    },

    /// Draw an image from raw RGBA pixel data.
    ///
    /// Creates a texture from a byte array of RGBA pixel data. Useful for
    /// procedurally generated textures or video frames.
    ///
    /// # Fields
    /// - `x`, `y`: Top-left corner position in screen pixels
    /// - `width`, `height`: Display dimensions in pixels (may scale from texture size)
    /// - `texture_key`: Unique identifier for caching (e.g., `"procedural_1"`)
    /// - `rgba`: Byte array of RGBA pixel data (must be `texture_width × texture_height × 4` bytes)
    /// - `texture_width`, `texture_height`: Source texture dimensions in pixels
    /// - `draw_order`: Rendering layer (higher = on top)
    ///
    /// # RGBA Format
    /// Each pixel is 4 bytes: R, G, B, A (each 0-255), in row-major order.
    ImageBytes {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        texture_key: String,
        rgba: Arc<[u8]>,
        texture_width: u32,
        texture_height: u32,
        draw_order: f32,
    },

    /// Draw text at the specified position.
    ///
    /// Renders text using a TrueType font. Position (x, y) represents the
    /// **top-left** corner of the text's bounding box.
    ///
    /// # Fields
    /// - `text`: String to render (supports newlines `\n`)
    /// - `x`, `y`: Top-left position in screen pixels
    /// - `font_size`: Font size in pixels
    /// - `color`: Text color
    /// - `font_path`: Optional path to TTF font file (uses default if `None`)
    /// - `letter_spacing`: Extra spacing between characters in pixels
    /// - `line_spacing`: Extra spacing between lines in pixels
    /// - `draw_order`: Rendering layer (higher = on top)
    Text {
        text: String,
        x: f32,
        y: f32,
        font_size: f32,
        color: Color,
        font_path: Option<String>,
        letter_spacing: f32,
        line_spacing: f32,
        draw_order: f32,
    },
}

/// Manages immediate-mode draw commands for 2D rendering.
///
/// The `DrawManager` accumulates draw commands each frame which are then processed
/// by the render manager. It tracks a scene version number that increments whenever
/// the command list changes, enabling change detection for conditional rendering.
///
/// # Usage
///
/// ```rust
/// use pyg_engine::DrawManager;
/// use pyg_engine::DrawCommand;
/// use pyg_engine::Color;
///
/// let mut draw_manager = DrawManager::new();
///
/// // Clear previous frame's commands
/// draw_manager.clear();
///
/// // Add draw commands
/// draw_manager.draw_line(0.0, 0.0, 100.0, 100.0, Color::WHITE);
/// draw_manager.draw_circle(50.0, 50.0, 25.0, Color::RED);
///
/// // Or add commands directly
/// draw_manager.add_command(DrawCommand::Rectangle {
///     x: 10.0, y: 10.0, width: 80.0, height: 80.0,
///     color: Color::BLUE, filled: true, thickness: 1.0, draw_order: 0.0,
/// });
///
/// // Get commands for rendering
/// let commands = draw_manager.commands();
/// ```
#[derive(Default)]
pub struct DrawManager {
    commands: Vec<DrawCommand>,
    scene_version: u64,
}

impl DrawManager {
    /// Create a new empty DrawManager.
    ///
    /// Initializes with an empty command list and scene version 0.
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            scene_version: 0,
        }
    }

    /// Clear all draw commands.
    ///
    /// Removes all pending draw commands and increments the scene version,
    /// triggering a redraw if `redraw_on_change_only` is enabled.
    ///
    /// This is typically called at the start of each frame to clear the
    /// previous frame's drawing operations.
    pub fn clear(&mut self) {
        if self.commands.is_empty() {
            return;
        }

        self.commands.clear();
        self.bump_scene_version();
    }

    /// Get a slice of all current draw commands.
    ///
    /// Returns an immutable reference to the internal command list for rendering.
    pub fn commands(&self) -> &[DrawCommand] {
        &self.commands
    }

    /// Get the current scene version number.
    ///
    /// The scene version increments whenever the command list changes. This can be
    /// used for change detection to optimize rendering (e.g., skip rendering if
    /// the scene hasn't changed).
    ///
    /// # Returns
    /// A monotonically increasing version number (wraps on overflow)
    pub fn scene_version(&self) -> u64 {
        self.scene_version
    }

    fn bump_scene_version(&mut self) {
        self.scene_version = self.scene_version.wrapping_add(1);
    }

    fn push_command(&mut self, command: DrawCommand) {
        self.commands.push(command);
        self.bump_scene_version();
    }

    /// Add a single draw command.
    ///
    /// Appends the command to the internal list and increments the scene version.
    ///
    /// # Arguments
    /// * `command` - The draw command to add
    pub fn add_command(&mut self, command: DrawCommand) {
        self.push_command(command);
    }

    /// Add multiple draw commands in bulk.
    ///
    /// More efficient than calling `add_command()` repeatedly as it only
    /// increments the scene version once.
    ///
    /// # Arguments
    /// * `commands` - Vector of draw commands to add
    pub fn add_commands(&mut self, mut commands: Vec<DrawCommand>) {
        if commands.is_empty() {
            return;
        }

        self.commands.append(&mut commands);
        self.bump_scene_version();
    }

    /// Remove all draw commands from index `start` onward.
    /// Used by UIManager to clear previous frame's UI commands before re-rendering.
    pub fn truncate_from(&mut self, start: usize) {
        if start < self.commands.len() {
            self.commands.truncate(start);
            self.bump_scene_version();
        }
    }

    /// Scale all draw commands from index `start` onward by `scale`.
    /// Used to convert UI coordinates from logical to physical pixels.
    pub fn scale_commands_from(&mut self, start: usize, scale: f32) {
        for cmd in self.commands[start..].iter_mut() {
            match cmd {
                DrawCommand::Rectangle { x, y, width, height, thickness, .. } => {
                    *x *= scale;
                    *y *= scale;
                    *width *= scale;
                    *height *= scale;
                    *thickness *= scale;
                }
                DrawCommand::Text { x, y, font_size, .. } => {
                    *x *= scale;
                    *y *= scale;
                    *font_size *= scale;
                }
                DrawCommand::Line { start_x, start_y, end_x, end_y, thickness, .. } => {
                    *start_x *= scale;
                    *start_y *= scale;
                    *end_x *= scale;
                    *end_y *= scale;
                    *thickness *= scale;
                }
                DrawCommand::Pixel { x, y, .. } => {
                    *x *= scale;
                    *y *= scale;
                }
                DrawCommand::Circle { center_x, center_y, radius, thickness, .. } => {
                    *center_x *= scale;
                    *center_y *= scale;
                    *radius *= scale;
                    *thickness *= scale;
                }
                DrawCommand::GradientRect { x, y, width, height, .. } => {
                    *x *= scale;
                    *y *= scale;
                    *width *= scale;
                    *height *= scale;
                }
                DrawCommand::Image { x, y, width, height, .. } => {
                    *x *= scale;
                    *y *= scale;
                    *width *= scale;
                    *height *= scale;
                }
                DrawCommand::ImageBytes { x, y, width, height, .. } => {
                    *x *= scale;
                    *y *= scale;
                    *width *= scale;
                    *height *= scale;
                }
            }
        }
        self.bump_scene_version();
    }

    pub fn draw_pixel(&mut self, x: u32, y: u32, color: Color) {
        self.draw_pixel_with_order(x, y, color, 0.0);
    }

    pub fn draw_pixel_with_order(&mut self, x: u32, y: u32, color: Color, draw_order: f32) {
        self.push_command(DrawCommand::Pixel {
            x: x as f32,
            y: y as f32,
            color,
            draw_order,
        });
    }

    pub fn draw_line(&mut self, start_x: f32, start_y: f32, end_x: f32, end_y: f32, color: Color) {
        self.draw_line_with_options(start_x, start_y, end_x, end_y, 1.0, color, 0.0);
    }

    pub fn draw_line_with_options(
        &mut self,
        start_x: f32,
        start_y: f32,
        end_x: f32,
        end_y: f32,
        thickness: f32,
        color: Color,
        draw_order: f32,
    ) {
        self.push_command(DrawCommand::Line {
            start_x,
            start_y,
            end_x,
            end_y,
            thickness,
            color,
            draw_order,
        });
    }

    pub fn draw_rectangle(&mut self, x: f32, y: f32, width: f32, height: f32, color: Color) {
        self.draw_rectangle_with_options(x, y, width, height, color, true, 1.0, 0.0);
    }

    pub fn draw_rectangle_outline(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        thickness: f32,
        color: Color,
    ) {
        self.draw_rectangle_with_options(x, y, width, height, color, false, thickness, 0.0);
    }

    pub fn draw_rectangle_with_options(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color: Color,
        filled: bool,
        thickness: f32,
        draw_order: f32,
    ) {
        self.push_command(DrawCommand::Rectangle {
            x,
            y,
            width,
            height,
            color,
            filled,
            thickness,
            draw_order,
        });
    }

    pub fn draw_circle(&mut self, center_x: f32, center_y: f32, radius: f32, color: Color) {
        self.draw_circle_with_options(center_x, center_y, radius, color, true, 1.0, 32, 0.0);
    }

    pub fn draw_circle_outline(
        &mut self,
        center_x: f32,
        center_y: f32,
        radius: f32,
        thickness: f32,
        color: Color,
    ) {
        self.draw_circle_with_options(center_x, center_y, radius, color, false, thickness, 32, 0.0);
    }

    pub fn draw_circle_with_options(
        &mut self,
        center_x: f32,
        center_y: f32,
        radius: f32,
        color: Color,
        filled: bool,
        thickness: f32,
        segments: u32,
        draw_order: f32,
    ) {
        self.push_command(DrawCommand::Circle {
            center_x,
            center_y,
            radius,
            color,
            filled,
            thickness,
            segments,
            draw_order,
        });
    }

    pub fn draw_gradient_rect_with_options(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        top_left: Color,
        bottom_left: Color,
        bottom_right: Color,
        top_right: Color,
        draw_order: f32,
    ) {
        self.push_command(DrawCommand::GradientRect {
            x,
            y,
            width,
            height,
            top_left,
            bottom_left,
            bottom_right,
            top_right,
            draw_order,
        });
    }

    pub fn draw_image_with_options(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        texture_path: String,
        draw_order: f32,
    ) {
        self.push_command(DrawCommand::Image {
            x,
            y,
            width,
            height,
            texture_path,
            draw_order,
        });
    }

    pub fn draw_image_from_bytes_with_options(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        texture_key: String,
        rgba: Arc<[u8]>,
        texture_width: u32,
        texture_height: u32,
        draw_order: f32,
    ) -> Result<(), String> {
        let expected_size = (texture_width as usize)
            .checked_mul(texture_height as usize)
            .and_then(|value| value.checked_mul(4))
            .ok_or_else(|| "texture size overflow while validating RGBA buffer".to_string())?;

        if rgba.len() != expected_size {
            return Err(format!(
                "texture byte size mismatch for key '{texture_key}': expected {expected_size} bytes ({}x{} RGBA), got {} bytes",
                texture_width,
                texture_height,
                rgba.len()
            ));
        }

        self.push_command(DrawCommand::ImageBytes {
            x,
            y,
            width,
            height,
            texture_key,
            rgba,
            texture_width,
            texture_height,
            draw_order,
        });

        Ok(())
    }

    pub fn draw_text(&mut self, text: String, x: f32, y: f32, color: Color) {
        self.draw_text_with_options(text, x, y, 24.0, color, None, 0.0, 0.0, 0.0);
    }

    #[allow(clippy::too_many_arguments)]
    pub fn draw_text_with_options(
        &mut self,
        text: String,
        x: f32,
        y: f32,
        font_size: f32,
        color: Color,
        font_path: Option<String>,
        letter_spacing: f32,
        line_spacing: f32,
        draw_order: f32,
    ) {
        self.push_command(DrawCommand::Text {
            text,
            x,
            y,
            font_size,
            color,
            font_path,
            letter_spacing,
            line_spacing,
            draw_order,
        });
    }
}
