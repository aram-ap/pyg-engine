use crossbeam_channel::Sender;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::pump_events::{EventLoopExtPumpEvents, PumpStatus};

use crate::core::command::EngineCommand;
use crate::core::component::{ComponentTrait, MeshComponent, MeshGeometry, TransformComponent};
use crate::core::draw_manager::DrawCommand;
use crate::core::engine::Engine as RustEngine;
use crate::core::game_object::GameObject as RustGameObject;
use crate::core::time::Time as RustTime;
use crate::core::window_manager::{FullscreenMode, WindowConfig};

// Import bindings from separate modules
use super::color_bind::PyColor;
use super::input_bind::{PyKeys, PyMouseButton, parse_key, parse_mouse_button};
use super::vector_bind::{PyVec2, PyVec3};

// ========== Engine Bindings ==========

/// Python-side draw command builder used for bulk submission.
#[pyclass(name = "DrawCommand")]
#[derive(Clone)]
pub struct PyDrawCommand {
    inner: DrawCommand,
}

#[pymethods]
impl PyDrawCommand {
    #[staticmethod]
    #[pyo3(signature = (x, y, color, layer=0, z_index=0.0))]
    fn pixel(x: u32, y: u32, color: &PyColor, layer: i32, z_index: f32) -> Self {
        Self {
            inner: DrawCommand::Pixel {
                x: x as f32,
                y: y as f32,
                color: color.inner,
                layer,
                z_index,
            },
        }
    }

    #[staticmethod]
    #[pyo3(signature = (start_x, start_y, end_x, end_y, color, thickness=1.0, layer=0, z_index=0.0))]
    fn line(
        start_x: f32,
        start_y: f32,
        end_x: f32,
        end_y: f32,
        color: &PyColor,
        thickness: f32,
        layer: i32,
        z_index: f32,
    ) -> Self {
        Self {
            inner: DrawCommand::Line {
                start_x,
                start_y,
                end_x,
                end_y,
                thickness,
                color: color.inner,
                layer,
                z_index,
            },
        }
    }

    #[staticmethod]
    #[pyo3(signature = (x, y, width, height, color, filled=true, thickness=1.0, layer=0, z_index=0.0))]
    fn rectangle(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color: &PyColor,
        filled: bool,
        thickness: f32,
        layer: i32,
        z_index: f32,
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
                layer,
                z_index,
            },
        }
    }

    #[staticmethod]
    #[pyo3(signature = (
        center_x,
        center_y,
        radius,
        color,
        filled=true,
        thickness=1.0,
        segments=32,
        layer=0,
        z_index=0.0
    ))]
    fn circle(
        center_x: f32,
        center_y: f32,
        radius: f32,
        color: &PyColor,
        filled: bool,
        thickness: f32,
        segments: u32,
        layer: i32,
        z_index: f32,
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
                layer,
                z_index,
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
        layer=0,
        z_index=0.0
    ))]
    fn gradient_rect(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        top_left: &PyColor,
        bottom_left: &PyColor,
        bottom_right: &PyColor,
        top_right: &PyColor,
        layer: i32,
        z_index: f32,
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
                layer,
                z_index,
            },
        }
    }

    #[staticmethod]
    #[pyo3(signature = (x, y, width, height, texture_path, layer=0, z_index=0.0))]
    fn image(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        texture_path: String,
        layer: i32,
        z_index: f32,
    ) -> Self {
        Self {
            inner: DrawCommand::Image {
                x,
                y,
                width,
                height,
                texture_path,
                layer,
                z_index,
            },
        }
    }

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
        layer=0,
        z_index=0.0
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
        layer: i32,
        z_index: f32,
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
                rgba,
                texture_width,
                texture_height,
                layer,
                z_index,
            },
        })
    }

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
        layer=0,
        z_index=0.0
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
        layer: i32,
        z_index: f32,
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
                layer,
                z_index,
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
        show_fps_in_title=false
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
    ) -> PyResult<()> {
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
                "Engine not initialized. Call initialize() first.",
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
        show_fps_in_title=false
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
    ) -> PyResult<()> {
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

        self.inner.set_auto_step_on_redraw(true);
        self.inner
            .run(config)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }

    /// Add a GameObject to the engine.
    ///
    /// The object is copied into the runtime scene using current transform + mesh state.
    fn add_game_object(&mut self, game_object: &PyGameObject) -> Option<u32> {
        self.inner
            .add_game_object(game_object.to_runtime_game_object())
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
    #[pyo3(signature = (x, y, color, layer=0, z_index=0.0))]
    fn draw_pixel(&mut self, x: u32, y: u32, color: &PyColor, layer: i32, z_index: f32) {
        self.inner
            .draw_pixel_with_order(x, y, color.inner, layer, z_index);
    }

    /// Draw a line at window coordinates.
    #[pyo3(signature = (start_x, start_y, end_x, end_y, color, thickness=1.0, layer=0, z_index=0.0))]
    fn draw_line(
        &mut self,
        start_x: f32,
        start_y: f32,
        end_x: f32,
        end_y: f32,
        color: &PyColor,
        thickness: f32,
        layer: i32,
        z_index: f32,
    ) {
        self.inner.draw_line_with_options(
            start_x,
            start_y,
            end_x,
            end_y,
            thickness,
            color.inner,
            layer,
            z_index,
        );
    }

    /// Draw a rectangle at window coordinates.
    #[pyo3(signature = (x, y, width, height, color, filled=true, thickness=1.0, layer=0, z_index=0.0))]
    fn draw_rectangle(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color: &PyColor,
        filled: bool,
        thickness: f32,
        layer: i32,
        z_index: f32,
    ) {
        self.inner.draw_rectangle_with_options(
            x,
            y,
            width,
            height,
            color.inner,
            filled,
            thickness,
            layer,
            z_index,
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
        layer=0,
        z_index=0.0
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
        layer: i32,
        z_index: f32,
    ) {
        self.inner.draw_circle_with_options(
            center_x,
            center_y,
            radius,
            color.inner,
            filled,
            thickness,
            segments,
            layer,
            z_index,
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
        layer=0,
        z_index=0.0
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
        layer: i32,
        z_index: f32,
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
            layer,
            z_index,
        );
    }

    /// Draw an image from a filesystem path at window coordinates.
    #[pyo3(signature = (x, y, width, height, texture_path, layer=0, z_index=0.0))]
    fn draw_image(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        texture_path: String,
        layer: i32,
        z_index: f32,
    ) {
        self.inner
            .draw_image_with_options(x, y, width, height, texture_path, layer, z_index);
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
        layer=0,
        z_index=0.0
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
        layer: i32,
        z_index: f32,
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
                layer,
                z_index,
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
        layer=0,
        z_index=0.0
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
        layer: i32,
        z_index: f32,
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
            layer,
            z_index,
        );
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
    #[getter]
    fn delta_time(&self) -> f32 {
        self.inner.time.delta_time()
    }

    /// Get the total elapsed time in seconds since the engine started.
    #[getter]
    fn elapsed_time(&self) -> f32 {
        self.inner.time.elapsed_time()
    }

    // ========== Input Methods ==========

    /// Check if a keyboard key is currently held down.
    fn key_down(&self, key_name: &str) -> bool {
        if let Some(input) = &self.inner.input_manager {
            input.key_down(&parse_key(key_name))
        } else {
            false
        }
    }

    /// Check if a keyboard key was pressed this frame.
    fn key_pressed(&self, key_name: &str) -> bool {
        if let Some(input) = &self.inner.input_manager {
            input.key_pressed(&parse_key(key_name))
        } else {
            false
        }
    }

    /// Check if a keyboard key was released this frame.
    fn key_released(&self, key_name: &str) -> bool {
        if let Some(input) = &self.inner.input_manager {
            input.key_released(&parse_key(key_name))
        } else {
            false
        }
    }

    /// Check if a mouse button is currently held down.
    fn mouse_button_down(&self, button: &str) -> bool {
        if let Some(input) = &self.inner.input_manager {
            input.mouse_button_down(parse_mouse_button(button))
        } else {
            false
        }
    }

    /// Check if a mouse button was pressed this frame.
    fn mouse_button_pressed(&self, button: &str) -> bool {
        if let Some(input) = &self.inner.input_manager {
            input.mouse_button_pressed(parse_mouse_button(button))
        } else {
            false
        }
    }

    /// Check if a mouse button was released this frame.
    fn mouse_button_released(&self, button: &str) -> bool {
        if let Some(input) = &self.inner.input_manager {
            input.mouse_button_released(parse_mouse_button(button))
        } else {
            false
        }
    }

    /// Get the current mouse position in window coordinates.
    fn mouse_position(&self) -> (f64, f64) {
        if let Some(input) = &self.inner.input_manager {
            input.mouse_position()
        } else {
            (0.0, 0.0)
        }
    }

    /// Get the mouse movement delta for this frame.
    fn mouse_delta(&self) -> (f64, f64) {
        if let Some(input) = &self.inner.input_manager {
            input.mouse_delta()
        } else {
            (0.0, 0.0)
        }
    }

    /// Get the mouse wheel delta accumulated this frame.
    fn mouse_wheel(&self) -> (f64, f64) {
        if let Some(input) = &self.inner.input_manager {
            input.mouse_wheel()
        } else {
            (0.0, 0.0)
        }
    }

    /// Get the current value of a logical axis.
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
    #[pyo3(signature = (x, y, color, layer=0, z_index=0.0))]
    fn draw_pixel(&self, x: u32, y: u32, color: &PyColor, layer: i32, z_index: f32) {
        let _ = self.sender.send(EngineCommand::DrawPixel {
            x,
            y,
            color: color.inner,
            layer,
            z_index,
        });
    }

    /// Draw a line at window coordinates via command queue.
    #[pyo3(signature = (start_x, start_y, end_x, end_y, color, thickness=1.0, layer=0, z_index=0.0))]
    fn draw_line(
        &self,
        start_x: f32,
        start_y: f32,
        end_x: f32,
        end_y: f32,
        color: &PyColor,
        thickness: f32,
        layer: i32,
        z_index: f32,
    ) {
        let _ = self.sender.send(EngineCommand::DrawLine {
            start_x,
            start_y,
            end_x,
            end_y,
            thickness,
            color: color.inner,
            layer,
            z_index,
        });
    }

    /// Draw a rectangle at window coordinates via command queue.
    #[pyo3(signature = (x, y, width, height, color, filled=true, thickness=1.0, layer=0, z_index=0.0))]
    fn draw_rectangle(
        &self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color: &PyColor,
        filled: bool,
        thickness: f32,
        layer: i32,
        z_index: f32,
    ) {
        let _ = self.sender.send(EngineCommand::DrawRectangle {
            x,
            y,
            width,
            height,
            color: color.inner,
            filled,
            thickness,
            layer,
            z_index,
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
        layer=0,
        z_index=0.0
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
        layer: i32,
        z_index: f32,
    ) {
        let _ = self.sender.send(EngineCommand::DrawCircle {
            center_x,
            center_y,
            radius,
            color: color.inner,
            filled,
            thickness,
            segments,
            layer,
            z_index,
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
        layer=0,
        z_index=0.0
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
        layer: i32,
        z_index: f32,
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
            layer,
            z_index,
        });
    }

    /// Draw an image from a filesystem path via command queue.
    #[pyo3(signature = (x, y, width, height, texture_path, layer=0, z_index=0.0))]
    fn draw_image(
        &self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        texture_path: String,
        layer: i32,
        z_index: f32,
    ) {
        let _ = self.sender.send(EngineCommand::DrawImage {
            x,
            y,
            width,
            height,
            texture_path,
            layer,
            z_index,
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
        layer=0,
        z_index=0.0
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
        layer: i32,
        z_index: f32,
    ) {
        let _ = self.sender.send(EngineCommand::DrawImageBytes {
            x,
            y,
            width,
            height,
            texture_key,
            rgba,
            texture_width,
            texture_height,
            layer,
            z_index,
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
        layer=0,
        z_index=0.0
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
        layer: i32,
        z_index: f32,
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
            layer,
            z_index,
        });
    }
}

/// Python wrapper for Time.
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

    #[getter]
    fn delta_time(&self) -> f32 {
        self.inner.delta_time()
    }

    #[getter]
    fn elapsed_time(&self) -> f32 {
        self.inner.elapsed_time()
    }
}

// ========== GameObject Bindings ==========

/// Python wrapper for GameObject.
#[pyclass(name = "GameObject", unsendable)]
pub struct PyGameObject {
    inner: RustGameObject,
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

        runtime
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
        Self { inner }
    }

    #[getter]
    fn id(&self) -> u32 {
        self.inner.get_id()
    }

    #[getter]
    fn name(&self) -> Option<String> {
        self.inner.name().map(|name| name.to_string())
    }

    fn set_name(&mut self, name: String) {
        self.inner.set_name(name);
    }

    #[getter]
    fn active(&self) -> bool {
        self.inner.is_active()
    }

    #[setter]
    fn set_active(&mut self, active: bool) {
        self.inner.set_active(active);
    }

    #[getter]
    fn position(&self) -> PyVec2 {
        PyVec2 {
            inner: *self.inner.transform().position(),
        }
    }

    #[setter]
    fn set_position(&mut self, position: PyVec2) {
        self.inner.transform_mut().set_position(position.inner);
    }

    #[getter]
    fn rotation(&self) -> f32 {
        self.inner.transform().rotation()
    }

    #[setter]
    fn set_rotation(&mut self, rotation: f32) {
        self.inner.transform_mut().set_rotation(rotation);
    }

    #[getter]
    fn scale(&self) -> PyVec2 {
        PyVec2 {
            inner: *self.inner.transform().scale(),
        }
    }

    #[setter]
    fn set_scale(&mut self, scale: PyVec2) {
        self.inner.transform_mut().set_scale(scale.inner);
    }

    /// Update this object manually with an optional Time value.
    #[pyo3(signature = (time=None))]
    fn update(&self, time: Option<&PyTime>) {
        if let Some(time) = time {
            self.inner.update(&time.inner);
        } else {
            let local_time = RustTime::new();
            self.inner.update(&local_time);
        }
    }

    fn has_mesh_component(&self) -> bool {
        self.inner.has_mesh_component()
    }

    fn add_mesh_component(&mut self, mesh_component: &PyMeshComponent) {
        self.inner.add_mesh_component(mesh_component.inner.clone());
    }

    fn set_mesh_component(&mut self, mesh_component: &PyMeshComponent) {
        self.inner.add_mesh_component(mesh_component.inner.clone());
    }

    fn remove_mesh_component(&mut self) -> Option<PyMeshComponent> {
        self.inner
            .remove_mesh_component()
            .map(|inner| PyMeshComponent { inner })
    }

    fn mesh_component(&self) -> Option<PyMeshComponent> {
        self.inner
            .mesh_component()
            .cloned()
            .map(|inner| PyMeshComponent { inner })
    }

    fn set_mesh_geometry_rectangle(&mut self, width: f32, height: f32) {
        let mesh = self.ensure_mesh_component();
        mesh.set_geometry(MeshGeometry::rectangle(width, height));
    }

    fn set_mesh_fill_color(&mut self, color: Option<PyColor>) {
        let mesh = self.ensure_mesh_component();
        mesh.set_fill_color(color.map(|c| c.inner));
    }

    fn mesh_fill_color(&self) -> Option<PyColor> {
        self.inner
            .mesh_component()
            .and_then(|mesh| mesh.fill_color().copied())
            .map(|inner| PyColor { inner })
    }

    fn set_mesh_image_path(&mut self, image_path: Option<String>) {
        let mesh = self.ensure_mesh_component();
        mesh.set_image_path(image_path);
    }

    fn mesh_image_path(&self) -> Option<String> {
        self.inner
            .mesh_component()
            .and_then(|mesh| mesh.image_path().map(|path| path.to_string()))
    }

    fn set_mesh_visible(&mut self, visible: bool) {
        let mesh = self.ensure_mesh_component();
        mesh.set_visible(visible);
    }

    fn mesh_visible(&self) -> Option<bool> {
        self.inner.mesh_component().map(|mesh| mesh.visible())
    }

    fn set_mesh_layer(&mut self, layer: i32) {
        let mesh = self.ensure_mesh_component();
        mesh.set_layer(layer);
    }

    fn mesh_layer(&self) -> Option<i32> {
        self.inner.mesh_component().map(|mesh| mesh.layer())
    }

    fn set_mesh_z_index(&mut self, z_index: f32) {
        let mesh = self.ensure_mesh_component();
        mesh.set_z_index(z_index);
    }

    fn mesh_z_index(&self) -> Option<f32> {
        self.inner.mesh_component().map(|mesh| mesh.z_index())
    }
}

// ========== MeshComponent Bindings ==========

/// Python wrapper for MeshComponent.
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
    fn layer(&self) -> i32 {
        self.inner.layer()
    }

    #[setter]
    fn set_layer(&mut self, layer: i32) {
        self.inner.set_layer(layer);
    }

    #[getter]
    fn z_index(&self) -> f32 {
        self.inner.z_index()
    }

    #[setter]
    fn set_z_index(&mut self, z_index: f32) {
        self.inner.set_z_index(z_index);
    }
}

// ========== Component Bindings ==========

/// Python wrapper for TransformComponent.
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
    m.add_class::<PyMouseButton>()?;
    m.add_class::<PyKeys>()?;
    Ok(())
}
