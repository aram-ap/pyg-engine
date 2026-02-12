use super::command::EngineCommand;
use super::draw_manager::{DrawCommand, DrawManager};
use super::game_object::{GameObject, ObjectType};
use super::input_manager::InputManager;
/// Core engine functionality
use super::logging;
use super::object_manager::ObjectManager;
use super::render_manager::{CameraAspectMode, RenderManager};
use super::time::Time;
use super::ui_manager::UIManager;
use super::window_manager::{WindowConfig, WindowManager};
use crate::types::Color;
use crate::types::vector::Vec2;
use crossbeam_channel::{Receiver, Sender, unbounded};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;
use tracing::Level;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Icon, WindowId};

pub struct Engine {
    version: String,
    window_manager: Option<WindowManager>,
    render_manager: Option<RenderManager>,
    object_manager: Option<ObjectManager>,
    pub input_manager: Option<InputManager>,
    pub draw_manager: DrawManager,
    pub time: Time,
    pub ui_manager: Option<UIManager>,

    // Command Queue
    command_receiver: Receiver<EngineCommand>,
    // We keep a sender to give out clones
    command_sender: Sender<EngineCommand>,

    // Window configuration and state
    window_config: Option<WindowConfig>,
    base_window_title: String,
    show_fps_in_title: bool,
    fps_frame_counter: u32,
    fps_last_update: Instant,
    auto_step_on_redraw: bool,
    active_camera_object_id: Option<u32>,
    pending_camera_viewport_size: Option<Vec2>,
    pending_camera_aspect_mode: CameraAspectMode,
    pending_camera_background_color: Option<Color>,
}

pub const VERSION: &str = "1.2.0";

impl Engine {
    /// Create a new Engine instance with default logging (console only)
    pub fn new() -> Self {
        logging::init_default();
        let (sender, receiver) = unbounded();

        let mut engine = Self {
            version: VERSION.to_string(),
            window_manager: None,
            render_manager: None,
            object_manager: Some(ObjectManager::new()),
            input_manager: Some(InputManager::new()),
            draw_manager: DrawManager::new(),
            time: Time::new(),
            ui_manager: None,
            command_receiver: receiver,
            command_sender: sender,
            window_config: None,
            base_window_title: "PyG Engine".to_string(),
            show_fps_in_title: false,
            fps_frame_counter: 0,
            fps_last_update: Instant::now(),
            auto_step_on_redraw: true,
            active_camera_object_id: None,
            pending_camera_viewport_size: None,
            pending_camera_aspect_mode: CameraAspectMode::default(),
            pending_camera_background_color: None,
        };
        engine.ensure_active_camera_object();
        engine
    }

    /// Initialize the engine with custom logging configuration
    pub fn with_logging(enable_file: bool, log_dir: Option<String>, level: Option<String>) -> Self {
        let log_level = level
            .as_deref()
            .and_then(|s| match s.to_uppercase().as_str() {
                "TRACE" => Some(Level::TRACE),
                "DEBUG" => Some(Level::DEBUG),
                "INFO" => Some(Level::INFO),
                "WARN" => Some(Level::WARN),
                "ERROR" => Some(Level::ERROR),
                _ => None,
            })
            .unwrap_or(Level::INFO);

        let config = logging::LogConfig {
            level: log_level,
            enable_file,
            log_dir: log_dir
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("logs")),
            enable_colors: true,
            enable_json: false,
        };

        logging::init_logging(config);

        let (sender, receiver) = unbounded();
        let mut engine = Self {
            version: VERSION.to_string(),
            window_manager: None,
            render_manager: None,
            object_manager: Some(ObjectManager::new()),
            input_manager: Some(InputManager::new()),
            draw_manager: DrawManager::new(),
            time: Time::new(),
            ui_manager: None,
            command_receiver: receiver,
            command_sender: sender,
            window_config: None,
            base_window_title: "PyG Engine".to_string(),
            show_fps_in_title: false,
            fps_frame_counter: 0,
            fps_last_update: Instant::now(),
            auto_step_on_redraw: true,
            active_camera_object_id: None,
            pending_camera_viewport_size: None,
            pending_camera_aspect_mode: CameraAspectMode::default(),
            pending_camera_background_color: None,
        };
        engine.ensure_active_camera_object();
        engine
    }

    /// Get a sender for engine commands
    pub fn get_command_sender(&self) -> Sender<EngineCommand> {
        self.command_sender.clone()
    }

    /// Set the window configuration for the engine
    pub fn set_window_config(&mut self, mut config: WindowConfig) {
        if let Some(pending_color) = self.pending_camera_background_color {
            config.background_color = Some(pending_color);
        }
        self.window_config = Some(config);
    }

    /// Configure whether redraw events should automatically step and render.
    ///
    /// This should remain enabled for `run(...)` mode and be disabled when a host
    /// drives the loop manually via `poll_events()`, `update()`, and `render()`.
    pub fn set_auto_step_on_redraw(&mut self, enabled: bool) {
        self.auto_step_on_redraw = enabled;
    }

    /// Set the window title
    pub fn set_window_title(&mut self, title: String) {
        self.base_window_title = title.clone();
        if let Some(window_manager) = &mut self.window_manager {
            if !self.show_fps_in_title {
                window_manager.set_title(&title);
            }
        }
    }

    /// Set the window icon from a decoded icon image.
    ///
    /// If a window is currently open, this updates the live window immediately.
    /// If the window has not been created yet, this updates pending window config.
    pub fn set_window_icon(&mut self, icon: Icon) {
        if let Some(window_manager) = &self.window_manager {
            window_manager.set_icon(icon);
        } else if let Some(config) = &mut self.window_config {
            config.icon = Some(icon);
        }
    }

    /// Get the current display size (window client size) in pixels.
    ///
    /// If the window has not been created yet, this falls back to the configured
    /// size if available; otherwise returns (0, 0).
    pub fn get_display_size(&self) -> (u32, u32) {
        if let Some(window_manager) = &self.window_manager {
            let size = window_manager.size();
            (size.width, size.height)
        } else if let Some(config) = &self.window_config {
            (config.width, config.height)
        } else {
            (0, 0)
        }
    }

    /// Run the engine with a window
    ///
    /// This method takes a mutable reference to the engine and runs the event loop.
    /// It creates a window and render manager, then enters the main game loop.
    pub fn run(&mut self, window_config: WindowConfig) -> Result<(), Box<dyn std::error::Error>> {
        logging::log_info(&format!(
            "Starting PyG Engine v{} with window: {} ({}x{})",
            self.version, window_config.title, window_config.width, window_config.height
        ));

        self.window_config = Some(window_config);

        // Create the event loop.
        // On macOS, force a regular app activation policy so native fullscreen
        // integrates with the standard menu bar behavior.
        let event_loop = {
            #[cfg(target_os = "macos")]
            {
                use winit::platform::macos::{ActivationPolicy, EventLoopBuilderExtMacOS};

                let mut builder = EventLoop::builder();
                builder.with_activation_policy(ActivationPolicy::Regular);
                builder.with_default_menu(true);
                builder.build()?
            }
            #[cfg(not(target_os = "macos"))]
            {
                EventLoop::new()?
            }
        };
        #[cfg(target_os = "macos")]
        event_loop.set_control_flow(ControlFlow::Wait);
        #[cfg(not(target_os = "macos"))]
        event_loop.set_control_flow(ControlFlow::Poll);

        // Run the event loop
        event_loop.run_app(self)?;

        Ok(())
    }

    /// Open a window (legacy method for backwards compatibility)
    pub fn open_window(&self, title: &str, width: u32, height: u32) {
        logging::log_info(&format!("Opening window: {} ({}x{})", title, width, height));
    }

    /// Close the window (legacy method for backwards compatibility)
    pub fn close_window(&self) {
        logging::log_info("Closing window");
    }

    fn create_default_camera_object(&mut self) -> Option<u32> {
        let mut camera = GameObject::new_named("MainCamera".to_string());
        camera.set_object_type(ObjectType::Camera);
        camera.transform_mut().set_position(Vec2::new(0.0, 0.0));
        self.add_game_object(camera)
    }

    fn ensure_active_camera_object(&mut self) -> Option<u32> {
        let camera_is_valid = self
            .active_camera_object_id
            .and_then(|id| {
                self.object_manager
                    .as_ref()
                    .and_then(|object_manager| object_manager.get_object_by_id(id))
                    .map(|_| id)
            })
            .is_some();

        if !camera_is_valid {
            self.active_camera_object_id = self.create_default_camera_object();
        }

        if let Some(render_manager) = &mut self.render_manager {
            render_manager.set_active_camera_object_id(self.active_camera_object_id);
        }

        self.active_camera_object_id
    }

    fn fallback_camera_viewport_size_for_display(&self) -> Vec2 {
        if let Some(viewport_size) = self.pending_camera_viewport_size {
            return viewport_size;
        }

        let (width, height) = self.get_display_size();
        let safe_width = width.max(1) as f32;
        let safe_height = height.max(1) as f32;
        let aspect = safe_width / safe_height;
        Vec2::new(2.0 * aspect, 2.0)
    }

    fn fallback_display_aspect_ratio(&self) -> f32 {
        let (width, height) = self.get_display_size();
        let safe_width = width.max(1) as f32;
        let safe_height = height.max(1) as f32;
        safe_width / safe_height
    }

    fn effective_world_viewport_size_for_mode(&self, viewport: Vec2) -> Vec2 {
        let safe_display_aspect = self.fallback_display_aspect_ratio().max(f32::EPSILON);
        match self.pending_camera_aspect_mode {
            CameraAspectMode::MatchHorizontal => Vec2::new(
                viewport.x().max(f32::EPSILON),
                (viewport.x() / safe_display_aspect).max(f32::EPSILON),
            ),
            CameraAspectMode::MatchVertical => Vec2::new(
                (viewport.y() * safe_display_aspect).max(f32::EPSILON),
                viewport.y().max(f32::EPSILON),
            ),
            _ => viewport,
        }
    }

    fn world_clip_scale_for_mode(&self, viewport: Vec2) -> (f32, f32) {
        let safe_display_aspect = self.fallback_display_aspect_ratio().max(f32::EPSILON);
        let safe_viewport_aspect =
            (viewport.x().max(f32::EPSILON) / viewport.y().max(f32::EPSILON)).max(f32::EPSILON);
        match self.pending_camera_aspect_mode {
            CameraAspectMode::FitBoth => {
                if safe_display_aspect >= safe_viewport_aspect {
                    (safe_viewport_aspect / safe_display_aspect, 1.0)
                } else {
                    (1.0, safe_display_aspect / safe_viewport_aspect)
                }
            }
            CameraAspectMode::FillBoth => {
                if safe_display_aspect >= safe_viewport_aspect {
                    (1.0, safe_display_aspect / safe_viewport_aspect)
                } else {
                    (safe_viewport_aspect / safe_display_aspect, 1.0)
                }
            }
            _ => (1.0, 1.0),
        }
    }

    /// Get the active camera object id.
    pub fn active_camera_object_id(&self) -> Option<u32> {
        self.active_camera_object_id
    }

    /// Get the active camera world position.
    pub fn get_camera_position(&self) -> Vec2 {
        let Some(camera_id) = self.active_camera_object_id else {
            return Vec2::new(0.0, 0.0);
        };

        self.object_manager
            .as_ref()
            .and_then(|object_manager| object_manager.get_object_by_id(camera_id))
            .map(|camera| *camera.transform().position())
            .unwrap_or_else(|| Vec2::new(0.0, 0.0))
    }

    /// Set the active camera world position.
    pub fn set_camera_position(&mut self, position: Vec2) -> bool {
        let Some(camera_id) = self.ensure_active_camera_object() else {
            return false;
        };

        self.set_game_object_position(camera_id, position)
    }

    /// Set the active camera viewport size in world units.
    pub fn set_camera_viewport_size(&mut self, width: f32, height: f32) -> bool {
        if !width.is_finite() || !height.is_finite() || width <= 0.0 || height <= 0.0 {
            return false;
        }

        self.pending_camera_viewport_size = Some(Vec2::new(width, height));
        if let Some(render_manager) = &mut self.render_manager {
            render_manager.set_camera_viewport_size(width, height);
        }
        self.request_render_redraw();
        true
    }

    /// Set how the active camera handles display aspect ratio changes.
    pub fn set_camera_aspect_mode(&mut self, mode: CameraAspectMode) -> bool {
        self.pending_camera_aspect_mode = mode;
        if let Some(render_manager) = &mut self.render_manager {
            render_manager.set_camera_aspect_mode(mode);
        }
        self.request_render_redraw();
        true
    }

    /// Get the active camera aspect policy.
    pub fn camera_aspect_mode(&self) -> CameraAspectMode {
        if let Some(render_manager) = &self.render_manager {
            return render_manager.camera_aspect_mode();
        }
        self.pending_camera_aspect_mode
    }

    /// Get the active camera viewport size in world units.
    pub fn camera_viewport_size(&self) -> (f32, f32) {
        if let Some(render_manager) = &self.render_manager {
            return render_manager.camera_viewport_size();
        }

        let viewport = self.fallback_camera_viewport_size_for_display();
        (viewport.x(), viewport.y())
    }

    /// Set the camera clear/background color.
    pub fn set_camera_background_color(&mut self, color: Color) {
        self.pending_camera_background_color = Some(color);
        if let Some(window_config) = &mut self.window_config {
            window_config.background_color = Some(color);
        }
        if let Some(render_manager) = &mut self.render_manager {
            render_manager.set_background_color(color);
        }
    }

    /// Get the camera clear/background color.
    pub fn camera_background_color(&self) -> Color {
        if let Some(render_manager) = &self.render_manager {
            return render_manager.background_color();
        }
        if let Some(color) = self.pending_camera_background_color {
            return color;
        }
        if let Some(window_config) = &self.window_config
            && let Some(color) = window_config.background_color
        {
            return color;
        }
        Color::BLACK
    }

    /// Convert world-space coordinates to screen-space pixel coordinates.
    pub fn world_to_screen(&self, world_position: Vec2) -> (f32, f32) {
        let camera_position = self.get_camera_position();
        if let Some(render_manager) = &self.render_manager {
            return render_manager.world_to_screen(world_position, camera_position);
        }

        let viewport = self.fallback_camera_viewport_size_for_display();
        let (display_width, display_height) = self.get_display_size();
        let width = display_width.max(1) as f32;
        let height = display_height.max(1) as f32;
        let effective_viewport = self.effective_world_viewport_size_for_mode(viewport);
        let half_w = (effective_viewport.x() * 0.5).max(f32::EPSILON);
        let half_h = (effective_viewport.y() * 0.5).max(f32::EPSILON);
        let normalized_x = (world_position.x() - camera_position.x()) / half_w;
        let normalized_y = (world_position.y() - camera_position.y()) / half_h;
        let (clip_scale_x, clip_scale_y) = self.world_clip_scale_for_mode(viewport);
        let clip_x = normalized_x * clip_scale_x;
        let clip_y = normalized_y * clip_scale_y;

        let screen_x = (clip_x + 1.0) * 0.5 * width;
        let screen_y = (1.0 - clip_y) * 0.5 * height;
        (screen_x, screen_y)
    }

    /// Convert screen-space pixel coordinates to world-space coordinates.
    pub fn screen_to_world(&self, screen_x: f32, screen_y: f32) -> Vec2 {
        let camera_position = self.get_camera_position();
        if let Some(render_manager) = &self.render_manager {
            return render_manager.screen_to_world(screen_x, screen_y, camera_position);
        }

        let viewport = self.fallback_camera_viewport_size_for_display();
        let (display_width, display_height) = self.get_display_size();
        let width = display_width.max(1) as f32;
        let height = display_height.max(1) as f32;

        let clip_x = (screen_x / width) * 2.0 - 1.0;
        let clip_y = 1.0 - (screen_y / height) * 2.0;
        let (clip_scale_x, clip_scale_y) = self.world_clip_scale_for_mode(viewport);
        let normalized_x = clip_x / clip_scale_x.max(f32::EPSILON);
        let normalized_y = clip_y / clip_scale_y.max(f32::EPSILON);
        let effective_viewport = self.effective_world_viewport_size_for_mode(viewport);
        let world_x = camera_position.x() + normalized_x * (effective_viewport.x() * 0.5);
        let world_y = camera_position.y() + normalized_y * (effective_viewport.y() * 0.5);
        Vec2::new(world_x, world_y)
    }

    /// Add a game object to the engine
    ///
    /// Takes ownership of the GameObject and adds it to the engine's object manager.
    /// Returns the ID of the added object, or None if the object manager is not initialized.
    pub fn add_game_object(&mut self, object: GameObject) -> Option<u32> {
        let object_type = object.get_object_type();
        if let Some(object_manager) = &mut self.object_manager {
            let object_id = object_manager.add_object(object);
            if object_type == ObjectType::Camera && self.active_camera_object_id.is_none() {
                self.active_camera_object_id = object_id;
            }
            if let Some(render_manager) = &mut self.render_manager {
                render_manager.set_active_camera_object_id(self.active_camera_object_id);
            }
            object_id
        } else {
            logging::log_warn("Cannot add game object: object manager is not initialized");
            None
        }
    }

    /// Create a new GameObject and add it to the engine
    ///
    /// Creates a new GameObject with a default name and adds it to the engine's object manager.
    /// Returns the ID of the created object, or None if the object manager is not initialized.
    pub fn create_game_object(&mut self) -> Option<u32> {
        self.add_game_object(GameObject::new())
    }

    /// Create a new named GameObject and add it to the engine
    ///
    /// Creates a new GameObject with the specified name and adds it to the engine's object manager.
    /// Returns the ID of the created object, or None if the object manager is not initialized.
    pub fn create_game_object_named(&mut self, name: String) -> Option<u32> {
        self.add_game_object(GameObject::new_named(name))
    }

    /// Get an immutable reference to a game object by id.
    pub fn get_game_object(&self, id: u32) -> Option<&GameObject> {
        self.object_manager.as_ref()?.get_object_by_id(id)
    }

    /// Get a mutable reference to a game object by id.
    pub fn get_game_object_mut(&mut self, id: u32) -> Option<&mut GameObject> {
        self.object_manager.as_mut()?.get_object_by_id_mut(id)
    }

    /// Remove a game object by id.
    pub fn remove_game_object(&mut self, id: u32) {
        if let Some(object_manager) = &mut self.object_manager {
            object_manager.remove_object(id);
        }
        if self.active_camera_object_id == Some(id) {
            self.active_camera_object_id = None;
            self.ensure_active_camera_object();
        }
        if let Some(render_manager) = &mut self.render_manager {
            render_manager.set_active_camera_object_id(self.active_camera_object_id);
        }
    }

    /// Update a runtime GameObject position by id.
    pub fn set_game_object_position(&mut self, id: u32, position: Vec2) -> bool {
        let Some(object_manager) = &mut self.object_manager else {
            return false;
        };

        let Some(object) = object_manager.get_object_by_id_mut(id) else {
            return false;
        };

        object.transform_mut().set_position(position);
        self.request_render_redraw();
        true
    }

    /// Update a runtime GameObject rotation by id.
    pub fn set_game_object_rotation(&mut self, id: u32, rotation: f32) -> bool {
        let Some(object_manager) = &mut self.object_manager else {
            return false;
        };

        let Some(object) = object_manager.get_object_by_id_mut(id) else {
            return false;
        };

        object.transform_mut().set_rotation(rotation);
        self.request_render_redraw();
        true
    }

    /// Update a runtime GameObject scale by id.
    pub fn set_game_object_scale(&mut self, id: u32, scale: Vec2) -> bool {
        let Some(object_manager) = &mut self.object_manager else {
            return false;
        };

        let Some(object) = object_manager.get_object_by_id_mut(id) else {
            return false;
        };

        object.transform_mut().set_scale(scale);
        self.request_render_redraw();
        true
    }

    fn request_render_redraw(&mut self) {
        if let Some(render_manager) = &mut self.render_manager {
            render_manager.request_redraw();
        }
    }

    /// Clears all direct draw commands.
    pub fn clear_draw_commands(&mut self) {
        self.draw_manager.clear();
        // Reset UI command tracking to avoid truncating wrong commands
        if let Some(ui_manager) = &mut self.ui_manager {
            ui_manager.reset_command_tracking();
        }
        self.request_render_redraw();
    }

    /// Draw a single pixel at window coordinates.
    pub fn draw_pixel(&mut self, x: u32, y: u32, color: Color) {
        self.draw_manager.draw_pixel(x, y, color);
        self.request_render_redraw();
    }

    /// Draw a single pixel with explicit draw ordering.
    pub fn draw_pixel_with_order(&mut self, x: u32, y: u32, color: Color, draw_order: f32) {
        self.draw_manager
            .draw_pixel_with_order(x, y, color, draw_order);
        self.request_render_redraw();
    }

    /// Draw a line in window pixel coordinates.
    pub fn draw_line(&mut self, start_x: f32, start_y: f32, end_x: f32, end_y: f32, color: Color) {
        self.draw_manager
            .draw_line(start_x, start_y, end_x, end_y, color);
        self.request_render_redraw();
    }

    /// Draw a line with explicit thickness/draw-order options.
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
        self.draw_manager
            .draw_line_with_options(start_x, start_y, end_x, end_y, thickness, color, draw_order);
        self.request_render_redraw();
    }

    /// Draw a filled rectangle in window pixel coordinates.
    pub fn draw_rectangle(&mut self, x: f32, y: f32, width: f32, height: f32, color: Color) {
        self.draw_manager.draw_rectangle(x, y, width, height, color);
        self.request_render_redraw();
    }

    /// Draw a rectangle outline in window pixel coordinates.
    pub fn draw_rectangle_outline(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        thickness: f32,
        color: Color,
    ) {
        self.draw_manager
            .draw_rectangle_outline(x, y, width, height, thickness, color);
        self.request_render_redraw();
    }

    /// Draw a rectangle with explicit fill/draw-order options.
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
        self.draw_manager
            .draw_rectangle_with_options(x, y, width, height, color, filled, thickness, draw_order);
        self.request_render_redraw();
    }

    /// Draw a filled circle in window pixel coordinates.
    pub fn draw_circle(&mut self, center_x: f32, center_y: f32, radius: f32, color: Color) {
        self.draw_manager
            .draw_circle(center_x, center_y, radius, color);
        self.request_render_redraw();
    }

    /// Draw a circle outline in window pixel coordinates.
    pub fn draw_circle_outline(
        &mut self,
        center_x: f32,
        center_y: f32,
        radius: f32,
        thickness: f32,
        color: Color,
    ) {
        self.draw_manager
            .draw_circle_outline(center_x, center_y, radius, thickness, color);
        self.request_render_redraw();
    }

    /// Draw a circle with explicit fill/tessellation/draw-order options.
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
        self.draw_manager.draw_circle_with_options(
            center_x, center_y, radius, color, filled, thickness, segments, draw_order,
        );
        self.request_render_redraw();
    }

    /// Draw a gradient rectangle with per-corner colors.
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
        self.draw_manager.draw_gradient_rect_with_options(
            x,
            y,
            width,
            height,
            top_left,
            bottom_left,
            bottom_right,
            top_right,
            draw_order,
        );
        self.request_render_redraw();
    }

    /// Draw an image from a file path in window pixel coordinates.
    pub fn draw_image_with_options(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        texture_path: String,
        draw_order: f32,
    ) {
        self.draw_manager
            .draw_image_with_options(x, y, width, height, texture_path, draw_order);
        self.request_render_redraw();
    }

    /// Draw an image from RGBA bytes in window pixel coordinates.
    pub fn draw_image_from_bytes_with_options(
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
    ) -> Result<(), String> {
        self.draw_image_from_bytes_with_options_shared(
            x,
            y,
            width,
            height,
            texture_key,
            Arc::from(rgba),
            texture_width,
            texture_height,
            draw_order,
        )
    }

    fn draw_image_from_bytes_with_options_shared(
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
        self.draw_manager.draw_image_from_bytes_with_options(
            x,
            y,
            width,
            height,
            texture_key,
            rgba,
            texture_width,
            texture_height,
            draw_order,
        )?;
        self.request_render_redraw();
        Ok(())
    }

    /// Draw text with optional custom font path and spacing controls.
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
        self.draw_manager.draw_text_with_options(
            text,
            x,
            y,
            font_size,
            color,
            font_path,
            letter_spacing,
            line_spacing,
            draw_order,
        );
        self.request_render_redraw();
    }

    /// Push a fully-custom direct draw command.
    pub fn add_draw_command(&mut self, command: DrawCommand) {
        self.draw_manager.add_command(command);
        self.request_render_redraw();
    }

    /// Push many direct draw commands in one batch.
    pub fn add_draw_commands(&mut self, commands: Vec<DrawCommand>) {
        if commands.is_empty() {
            return;
        }
        self.draw_manager.add_commands(commands);
        self.request_render_redraw();
    }

    /// Process all queued commands
    fn process_commands(&mut self) {
        while let Ok(command) = self.command_receiver.try_recv() {
            match command {
                EngineCommand::AddGameObject(object) => {
                    self.add_game_object(object);
                }
                EngineCommand::RemoveGameObject(id) => {
                    self.remove_game_object(id);
                }
                EngineCommand::SetGameObjectPosition {
                    object_id,
                    position,
                } => {
                    let _ = self.set_game_object_position(object_id, position);
                }
                EngineCommand::SetGameObjectRotation {
                    object_id,
                    rotation,
                } => {
                    let _ = self.set_game_object_rotation(object_id, rotation);
                }
                EngineCommand::SetGameObjectScale { object_id, scale } => {
                    let _ = self.set_game_object_scale(object_id, scale);
                }
                EngineCommand::SetCameraPosition { position } => {
                    let _ = self.set_camera_position(position);
                }
                EngineCommand::SetCameraViewportSize { width, height } => {
                    let _ = self.set_camera_viewport_size(width, height);
                }
                EngineCommand::SetCameraAspectMode { mode } => {
                    let _ = self.set_camera_aspect_mode(mode);
                }
                EngineCommand::SetCameraBackgroundColor { color } => {
                    self.set_camera_background_color(color);
                }
                EngineCommand::ClearDrawCommands => {
                    self.clear_draw_commands();
                }
                EngineCommand::AddDrawCommand(command) => {
                    self.add_draw_command(command);
                }
                EngineCommand::AddDrawCommands(commands) => {
                    self.add_draw_commands(commands);
                }
                EngineCommand::DrawPixel {
                    x,
                    y,
                    color,
                    draw_order,
                } => {
                    self.draw_pixel_with_order(x, y, color, draw_order);
                }
                EngineCommand::DrawLine {
                    start_x,
                    start_y,
                    end_x,
                    end_y,
                    thickness,
                    color,
                    draw_order,
                } => {
                    self.draw_line_with_options(
                        start_x, start_y, end_x, end_y, thickness, color, draw_order,
                    );
                }
                EngineCommand::DrawRectangle {
                    x,
                    y,
                    width,
                    height,
                    color,
                    filled,
                    thickness,
                    draw_order,
                } => {
                    self.draw_rectangle_with_options(
                        x, y, width, height, color, filled, thickness, draw_order,
                    );
                }
                EngineCommand::DrawCircle {
                    center_x,
                    center_y,
                    radius,
                    color,
                    filled,
                    thickness,
                    segments,
                    draw_order,
                } => {
                    self.draw_circle_with_options(
                        center_x, center_y, radius, color, filled, thickness, segments, draw_order,
                    );
                }
                EngineCommand::DrawGradientRect {
                    x,
                    y,
                    width,
                    height,
                    top_left,
                    bottom_left,
                    bottom_right,
                    top_right,
                    draw_order,
                } => {
                    self.draw_gradient_rect_with_options(
                        x,
                        y,
                        width,
                        height,
                        top_left,
                        bottom_left,
                        bottom_right,
                        top_right,
                        draw_order,
                    );
                }
                EngineCommand::DrawImage {
                    x,
                    y,
                    width,
                    height,
                    texture_path,
                    draw_order,
                } => {
                    self.draw_image_with_options(x, y, width, height, texture_path, draw_order);
                }
                EngineCommand::DrawImageBytes {
                    x,
                    y,
                    width,
                    height,
                    texture_key,
                    rgba,
                    texture_width,
                    texture_height,
                    draw_order,
                } => {
                    if let Err(err) = self.draw_image_from_bytes_with_options_shared(
                        x,
                        y,
                        width,
                        height,
                        texture_key,
                        rgba,
                        texture_width,
                        texture_height,
                        draw_order,
                    ) {
                        logging::log_warn(&format!("Dropped DrawImageBytes command: {err}"));
                    }
                }
                EngineCommand::DrawText {
                    text,
                    x,
                    y,
                    font_size,
                    color,
                    font_path,
                    letter_spacing,
                    line_spacing,
                    draw_order,
                } => {
                    self.draw_text_with_options(
                        text,
                        x,
                        y,
                        font_size,
                        color,
                        font_path,
                        letter_spacing,
                        line_spacing,
                        draw_order,
                    );
                }
                EngineCommand::UpdateUILabelText { object_id, text } => {
                    if let Some(object_manager) = &mut self.object_manager {
                        if let Some(obj) = object_manager.get_object_by_id_mut(object_id) {
                            if let Some(comp) = obj.get_component_by_name_mut("Label") {
                                if let Some(label) = comp.as_any_mut().downcast_mut::<crate::core::ui::label::LabelComponent>() {
                                    label.set_text(text);
                                }
                            }
                        }
                    }
                }
                EngineCommand::UpdateUIButtonText { object_id, text } => {
                    if let Some(object_manager) = &mut self.object_manager {
                        if let Some(obj) = object_manager.get_object_by_id_mut(object_id) {
                            if let Some(comp) = obj.get_component_by_name_mut("Button") {
                                if let Some(btn) = comp.as_any_mut().downcast_mut::<crate::core::ui::button::ButtonComponent>() {
                                    btn.set_text(text);
                                }
                            }
                        }
                    }
                }
                EngineCommand::LogTrace(message) => {
                    logging::log_trace(&message);
                }
                EngineCommand::LogDebug(message) => {
                    logging::log_debug(&message);
                }
                EngineCommand::LogInfo(message) => {
                    logging::log_info(&message);
                }
                EngineCommand::LogWarn(message) => {
                    logging::log_warn(&message);
                }
                EngineCommand::LogError(message) => {
                    logging::log_error(&message);
                }
            }
        }
    }

    /// Engine update loop
    pub fn update(&mut self) {
        if let Some(render_manager) = &mut self.render_manager {
            // `about_to_wait` can precompute a signature for redraw checks.
            // Simulation updates can change scene state, so invalidate it.
            render_manager.invalidate_precomputed_scene_signature();
        }

        // ------------------------------------------------------------
        // Process Commands First
        // ------------------------------------------------------------
        self.process_commands();
        self.ensure_active_camera_object();

        // ------------------------------------------------------------
        // IF NOT HEADLESS, DO THE FOLLOWING:
        // ------------------------------------------------------------

        // Time step/tick management
        self.time.tick();

        // Input (collect raw input + build an input snapshot)
        if let Some(input_manager) = &mut self.input_manager {
            input_manager.update();
        }

        // Event System - enqueue input events

        // UI - input handling / hit-testing (UI gets first right of refusal)
        if let (Some(ui_manager), Some(input_manager), Some(object_manager)) =
            (&mut self.ui_manager, &self.input_manager, &mut self.object_manager)
        {
            ui_manager.update(input_manager, object_manager);

            // If UI consumed input, mark scene dirty and request redraw
            // This ensures UI updates continue even in redraw_on_change_only mode
            if ui_manager.is_input_consumed() {
                if let Some(object_manager) = &mut self.object_manager {
                    object_manager.mark_scene_dirty();
                }
                self.request_render_redraw();
            }
        }

        // Event System - dispatch "unconsumed" gameplay input events

        // GameObjects + Components - pre-physics (gameplay/AI/scripts)
        if let Some(object_manager) = &mut self.object_manager {
            if object_manager.get_total_objects() > 0 {
                // Component updates can mutate object state through interior mutability.
                // Mark scene state as potentially changed before running user updates.
                object_manager.mark_scene_dirty();
            }

            for &key in object_manager.get_keys() {
                if let Some(object) = object_manager.get_object_by_id(key) {
                    object.update(&self.time);
                }
            }
        }

        // **Fixed update:**
        // Physics (often fixed-timestep; may run 0..N steps)
        let (is_fixed_time, fixed_time) = self.time.tick_fixed();
        if is_fixed_time && let Some(object_manager) = &mut self.object_manager {
            if object_manager.get_total_objects() > 0 {
                // Fixed-step systems may also update transforms/mesh state.
                object_manager.mark_scene_dirty();
            }

            for &key in object_manager.get_keys() {
                if let Some(object) = object_manager.get_object_by_id(key) {
                    object.fixed_update(&self.time, fixed_time);
                }
            }
        }

        // Event System - enqueue physics events (collisions/triggers)

        // GameObjects + Components - post-physics / late update (react, sync transforms, camera, attachments)

        // UI - update layout/animations/data-binding (using final game state)

        // **Frame rate limiting (optional)**
        // Rendering - world
        // Rendering - UI

        // ------------------------------------------------------------
        // IF HEADLESS, DO THE FOLLOWING:
        // ------------------------------------------------------------

        // Time step/tick management, (i.e., delta time is not based on system time, but rather a fixed timestep)
        // Input ("virtual" input: network commands, bots, scripted tests)
        // Event system - enqueue input events
        // GameObjects + Components - pre-physics (gameplay/AI/scripts)
        // Physics (often fixed-timestep; may run 0..N steps)
        // Event System - enqueue physics events (collisions/triggers)
        // GameObjects + Components - post-physics / late update
        // Event System - dispatch deffered events (end-of-tick)
        // Networking/persistance (optional but common): replicate state, process outgoing packets, write snapshots

        // ^^^ Note: Key differences are no rendering, UI is disabled, simulation runs at fixed timestep
    }

    /// Render a frame
    pub fn render(&mut self) {
        self.ensure_active_camera_object();

        // Render UI elements
        if let (Some(ui_manager), Some(object_manager)) = (&mut self.ui_manager, &self.object_manager) {
            ui_manager.render(&mut self.draw_manager, object_manager);
        }

        if let Some(render_manager) = &mut self.render_manager {
            match render_manager.render(&self.object_manager, Some(&self.draw_manager)) {
                Ok(_) => {
                    // Update FPS counter on successful render
                    if self.show_fps_in_title {
                        self.update_fps_counter();
                    }
                }
                Err(wgpu::SurfaceError::Lost) => {
                    // Surface is lost, need to reconfigure
                    if let Some(window_manager) = &self.window_manager {
                        render_manager.resize(window_manager.size());
                    }
                }
                Err(wgpu::SurfaceError::Outdated) => {
                    // Surface is outdated, reconfigure with current size
                    if let Some(window_manager) = &self.window_manager {
                        render_manager.resize(window_manager.size());
                    }
                }
                Err(wgpu::SurfaceError::OutOfMemory) => {
                    logging::log_error("Out of memory!");
                }
                Err(e) => {
                    logging::log_warn(&format!("Surface error: {:?}", e));
                }
            }
        }
    }

    /// Synchronize window and renderer with a new physical size.
    ///
    /// This is used for both direct resize events and scale-factor changes,
    /// which can happen during macOS fullscreen transitions.
    fn apply_window_resize(&mut self, physical_size: winit::dpi::PhysicalSize<u32>) {
        logging::log_debug(&format!(
            "Window resized to: {}x{}",
            physical_size.width, physical_size.height
        ));

        if let Some(window_manager) = &mut self.window_manager {
            window_manager.update_size(physical_size);
        }

        if let Some(render_manager) = &mut self.render_manager {
            render_manager.resize(physical_size);
        }

        if let Some(ui_manager) = &mut self.ui_manager {
            ui_manager.resize(physical_size.width as f32, physical_size.height as f32);
        }

        if let Some(window_manager) = &self.window_manager {
            window_manager.request_redraw();
        }
    }

    /// Update FPS counter in window title
    fn update_fps_counter(&mut self) {
        self.fps_frame_counter += 1;
        let elapsed_seconds = self.fps_last_update.elapsed().as_secs_f64();
        if elapsed_seconds >= 0.5 {
            let fps = self.fps_frame_counter as f64 / elapsed_seconds;
            if let Some(window_manager) = &self.window_manager {
                window_manager.set_title(&format!("{} | FPS: {:.1}", self.base_window_title, fps));
            }
            self.fps_frame_counter = 0;
            self.fps_last_update = Instant::now();
        }
    }

    /// Log a message at INFO level
    pub fn log(&self, message: &str) {
        logging::log_info(message);
    }

    /// Log a message at TRACE level
    pub fn log_trace(&self, message: &str) {
        logging::log_trace(message);
    }

    /// Log a message at DEBUG level
    pub fn log_debug(&self, message: &str) {
        logging::log_debug(message);
    }

    /// Log a message at INFO level
    pub fn log_info(&self, message: &str) {
        logging::log_info(message);
    }

    /// Log a message at WARN level
    pub fn log_warn(&self, message: &str) {
        logging::log_warn(message);
    }

    /// Log a message at ERROR level
    pub fn log_error(&self, message: &str) {
        logging::log_error(message);
    }

    /// Get the engine version
    pub fn version(&self) -> &str {
        &self.version
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

impl ApplicationHandler for Engine {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Create window and render manager when the application resumes
        if self.window_manager.is_none() {
            if let Some(config) = self.window_config.take() {
                // Extract background color and vsync before config is moved
                let mut bg_color = config.background_color;
                if let Some(pending_color) = self.pending_camera_background_color {
                    bg_color = Some(pending_color);
                }
                let vsync = config.vsync;
                let redraw_on_change_only = config.redraw_on_change_only;
                self.base_window_title = config.title.clone();
                self.show_fps_in_title = config.show_fps_in_title;
                self.fps_frame_counter = 0;
                self.fps_last_update = Instant::now();
                match WindowManager::new(event_loop, config) {
                    Ok(window_manager) => {
                        logging::log_info("Window created successfully");

                        // Create render manager with the window Arc
                        let window = window_manager.window_arc();
                        match pollster::block_on(RenderManager::new(
                            window,
                            bg_color,
                            vsync,
                            redraw_on_change_only,
                        )) {
                            Ok(render_manager) => {
                                logging::log_info("Render manager initialized successfully");
                                self.render_manager = Some(render_manager);

                                // Initialize UI manager with window size and scale factor
                                let window_size = window_manager.size();
                                let scale_factor = window_manager.scale_factor() as f32;
                                self.ui_manager = Some(UIManager::new(
                                    window_size.width as f32,
                                    window_size.height as f32,
                                    scale_factor,
                                ));
                                logging::log_info("UI manager initialized");

                                self.window_manager = Some(window_manager);
                                self.ensure_active_camera_object();

                                if let Some(viewport_size) = self.pending_camera_viewport_size
                                    && let Some(render_manager) = &mut self.render_manager
                                {
                                    render_manager.set_camera_viewport_size(
                                        viewport_size.x(),
                                        viewport_size.y(),
                                    );
                                }

                                if let Some(render_manager) = &mut self.render_manager {
                                    render_manager
                                        .set_camera_aspect_mode(self.pending_camera_aspect_mode);
                                }

                                // Request initial redraw
                                if let Some(wm) = &self.window_manager {
                                    wm.request_redraw();
                                }
                            }
                            Err(e) => {
                                logging::log_error(&format!(
                                    "Failed to create render manager: {}",
                                    e
                                ));
                                event_loop.exit();
                            }
                        }
                    }
                    Err(e) => {
                        logging::log_error(&format!("Failed to create window: {}", e));
                        event_loop.exit();
                    }
                }
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        // Forward all window events to the input manager so it can update input state.
        if let Some(input_manager) = &mut self.input_manager {
            input_manager.handle_window_event(&event);
        }

        match event {
            WindowEvent::CloseRequested => {
                logging::log_info("Close requested, shutting down engine");
                event_loop.exit();
            }
            WindowEvent::Resized(physical_size) => {
                self.apply_window_resize(physical_size);
            }
            WindowEvent::ScaleFactorChanged { .. } => {
                if let Some(window_manager) = &self.window_manager {
                    // Pull the post-scale physical size directly from winit.
                    // On macOS fullscreen transitions this path may fire
                    // without a corresponding Resized event.
                    let physical_size = window_manager.window().inner_size();
                    self.apply_window_resize(physical_size);
                }
            }
            WindowEvent::Focused(focused) => {
                logging::log_debug(&format!("Window focus changed: {}", focused));

                if focused && let Some(window_manager) = &self.window_manager {
                    // Ensure the OS key window / first-responder chain is restored
                    // after fullscreen transition focus hops.
                    window_manager.window().focus_window();
                    window_manager.request_redraw();
                }
            }
            WindowEvent::RedrawRequested => {
                if self.auto_step_on_redraw {
                    // Update engine state
                    self.update();

                    // Render the frame
                    self.render();
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window_manager) = &self.window_manager {
            if self.show_fps_in_title {
                window_manager.request_redraw();
                return;
            }

            // If there are UI objects, continuously update to process input
            // This ensures UI works in redraw_on_change_only mode without callbacks
            if let Some(object_manager) = &self.object_manager {
                if object_manager.has_ui_objects() {
                    window_manager.request_redraw();
                    return;
                }
            }
        }

        if let Some(window_manager) = &self.window_manager
            && let Some(render_manager) = &mut self.render_manager
            && render_manager.should_request_redraw(&self.object_manager, Some(&self.draw_manager))
        {
            window_manager.request_redraw();
        }
    }
}
