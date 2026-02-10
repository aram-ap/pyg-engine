/// Core engine functionality
use super::logging;
use super::window_manager::{WindowConfig, WindowManager};
use super::render_manager::RenderManager;
use super::object_manager::ObjectManager;
use super::input_manager::InputManager;
use super::draw_manager::{DrawCommand, DrawManager};
use super::game_object::GameObject;
use super::time::Time;
use super::command::EngineCommand;
use crate::types::Color;
use std::path::PathBuf;
use std::time::Instant;
use tracing::Level;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::WindowId;
use crossbeam_channel::{unbounded, Receiver, Sender};

pub struct Engine {
    version: String,
    window_manager: Option<WindowManager>,
    render_manager: Option<RenderManager>,
    object_manager: Option<ObjectManager>,
    pub input_manager: Option<InputManager>,
    pub draw_manager: DrawManager,
    pub time: Time,
    
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
}

pub const VERSION: &str = "1.2.0";

impl Engine {
    /// Create a new Engine instance with default logging (console only)
    pub fn new() -> Self {
        logging::init_default();
        let (sender, receiver) = unbounded();
        
        Self {
            version: VERSION.to_string(),
            window_manager: None,
            render_manager: None,
            object_manager: Some(ObjectManager::new()),
            input_manager: Some(InputManager::new()),
            draw_manager: DrawManager::new(),
            time: Time::new(),
            command_receiver: receiver,
            command_sender: sender,
            window_config: None,
            base_window_title: "PyG Engine".to_string(),
            show_fps_in_title: false,
            fps_frame_counter: 0,
            fps_last_update: Instant::now(),
        }
    }

    /// Initialize the engine with custom logging configuration
    pub fn with_logging(
        enable_file: bool,
        log_dir: Option<String>,
        level: Option<String>,
    ) -> Self {
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
            log_dir: log_dir.map(PathBuf::from).unwrap_or_else(|| PathBuf::from("logs")),
            enable_colors: true,
            enable_json: false,
        };

        logging::init_logging(config);

        let (sender, receiver) = unbounded();

        Self {
            version: VERSION.to_string(),
            window_manager: None,
            render_manager: None,
            object_manager: Some(ObjectManager::new()),
            input_manager: Some(InputManager::new()),
            draw_manager: DrawManager::new(),
            time: Time::new(),
            command_receiver: receiver,
            command_sender: sender,
            window_config: None,
            base_window_title: "PyG Engine".to_string(),
            show_fps_in_title: false,
            fps_frame_counter: 0,
            fps_last_update: Instant::now(),
        }
    }

    /// Get a sender for engine commands
    pub fn get_command_sender(&self) -> Sender<EngineCommand> {
        self.command_sender.clone()
    }

    /// Set the window configuration for the engine
    pub fn set_window_config(&mut self, config: WindowConfig) {
        self.window_config = Some(config);
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

        // Create the event loop
        let event_loop = EventLoop::new()?;
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

    /// Add a game object to the engine
    /// 
    /// Takes ownership of the GameObject and adds it to the engine's object manager.
    /// Returns the ID of the added object, or None if the object manager is not initialized.
    pub fn add_game_object(&mut self, object: GameObject) -> Option<u32> {
        if let Some(object_manager) = &mut self.object_manager {
            object_manager.add_object(object)
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
    }

    fn request_render_redraw(&mut self) {
        if let Some(render_manager) = &mut self.render_manager {
            render_manager.request_redraw();
        }
    }

    /// Clears all direct draw commands.
    pub fn clear_draw_commands(&mut self) {
        self.draw_manager.clear();
        self.request_render_redraw();
    }

    /// Draw a single pixel at window coordinates.
    pub fn draw_pixel(&mut self, x: u32, y: u32, color: Color) {
        self.draw_manager.draw_pixel(x, y, color);
        self.request_render_redraw();
    }

    /// Draw a single pixel with explicit layer/z ordering.
    pub fn draw_pixel_with_order(
        &mut self,
        x: u32,
        y: u32,
        color: Color,
        layer: i32,
        z_index: f32,
    ) {
        self.draw_manager
            .draw_pixel_with_order(x, y, color, layer, z_index);
        self.request_render_redraw();
    }

    /// Draw a line in window pixel coordinates.
    pub fn draw_line(&mut self, start_x: f32, start_y: f32, end_x: f32, end_y: f32, color: Color) {
        self.draw_manager
            .draw_line(start_x, start_y, end_x, end_y, color);
        self.request_render_redraw();
    }

    /// Draw a line with explicit thickness/layer/z options.
    pub fn draw_line_with_options(
        &mut self,
        start_x: f32,
        start_y: f32,
        end_x: f32,
        end_y: f32,
        thickness: f32,
        color: Color,
        layer: i32,
        z_index: f32,
    ) {
        self.draw_manager.draw_line_with_options(
            start_x, start_y, end_x, end_y, thickness, color, layer, z_index,
        );
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

    /// Draw a rectangle with explicit fill/layer/z options.
    pub fn draw_rectangle_with_options(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color: Color,
        filled: bool,
        thickness: f32,
        layer: i32,
        z_index: f32,
    ) {
        self.draw_manager.draw_rectangle_with_options(
            x, y, width, height, color, filled, thickness, layer, z_index,
        );
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

    /// Draw a circle with explicit fill/tessellation/layer/z options.
    pub fn draw_circle_with_options(
        &mut self,
        center_x: f32,
        center_y: f32,
        radius: f32,
        color: Color,
        filled: bool,
        thickness: f32,
        segments: u32,
        layer: i32,
        z_index: f32,
    ) {
        self.draw_manager.draw_circle_with_options(
            center_x, center_y, radius, color, filled, thickness, segments, layer, z_index,
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
        layer: i32,
        z_index: f32,
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
            layer,
            z_index,
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
        layer: i32,
        z_index: f32,
    ) {
        self.draw_manager
            .draw_image_with_options(x, y, width, height, texture_path, layer, z_index);
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
        layer: i32,
        z_index: f32,
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
            layer,
            z_index,
        )?;
        self.request_render_redraw();
        Ok(())
    }

    /// Push a fully-custom direct draw command.
    pub fn add_draw_command(&mut self, command: DrawCommand) {
        self.draw_manager.add_command(command);
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
                EngineCommand::ClearDrawCommands => {
                    self.clear_draw_commands();
                }
                EngineCommand::AddDrawCommand(command) => {
                    self.add_draw_command(command);
                }
                EngineCommand::DrawPixel { x, y, color, layer, z_index } => {
                    self.draw_pixel_with_order(x, y, color, layer, z_index);
                }
                EngineCommand::DrawLine { start_x, start_y, end_x, end_y, thickness, color, layer, z_index } => {
                    self.draw_line_with_options(start_x, start_y, end_x, end_y, thickness, color, layer, z_index);
                }
                EngineCommand::DrawRectangle { x, y, width, height, color, filled, thickness, layer, z_index } => {
                    self.draw_rectangle_with_options(x, y, width, height, color, filled, thickness, layer, z_index);
                }
                EngineCommand::DrawCircle { center_x, center_y, radius, color, filled, thickness, segments, layer, z_index } => {
                    self.draw_circle_with_options(center_x, center_y, radius, color, filled, thickness, segments, layer, z_index);
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
                    layer,
                    z_index,
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
                        layer,
                        z_index,
                    );
                }
                EngineCommand::DrawImage {
                    x,
                    y,
                    width,
                    height,
                    texture_path,
                    layer,
                    z_index,
                } => {
                    self.draw_image_with_options(x, y, width, height, texture_path, layer, z_index);
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
                    layer,
                    z_index,
                } => {
                    if let Err(err) = self.draw_image_from_bytes_with_options(
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
                    ) {
                        logging::log_warn(&format!("Dropped DrawImageBytes command: {err}"));
                    }
                }
            }
        }
    }

    /// Engine update loop
    pub fn update(&mut self) {
        // ------------------------------------------------------------
        // Process Commands First
        // ------------------------------------------------------------
        self.process_commands();
    
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

        // Event System - dispatch "unconsumed" gameplay input events

        // GameObjects + Components - pre-physics (gameplay/AI/scripts)
        if let Some(object_manager) = &mut self.object_manager {
            for key in object_manager.get_keys() {
                if let Some(object) = object_manager.get_object_by_id(key) {
                    object.update(&self.time);
                }
            }
        }

        // **Fixed update:**
            // Physics (often fixed-timestep; may run 0..N steps)
        let (is_fixed_time, fixed_time) = self.time.tick_fixed();
        if is_fixed_time 
            && let Some(object_manager) = &mut self.object_manager {

            for key in object_manager.get_keys() {
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

    /// Update FPS counter in window title
    fn update_fps_counter(&mut self) {
        self.fps_frame_counter += 1;
        let elapsed_seconds = self.fps_last_update.elapsed().as_secs_f64();
        if elapsed_seconds >= 0.5 {
            let fps = self.fps_frame_counter as f64 / elapsed_seconds;
            if let Some(window_manager) = &self.window_manager {
                window_manager.set_title(&format!(
                    "{} | FPS: {:.1}",
                    self.base_window_title, fps
                ));
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
                let bg_color = config.background_color;
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
                                self.window_manager = Some(window_manager);
                                
                                // Request initial redraw
                                if let Some(wm) = &self.window_manager {
                                    wm.request_redraw();
                                }
                            }
                            Err(e) => {
                                logging::log_error(&format!("Failed to create render manager: {}", e));
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
            }
            WindowEvent::RedrawRequested => {
                // Update engine state
                self.update();
                
                // Render the frame
                self.render();
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
        }

        if let (Some(window_manager), Some(render_manager)) =
            (&self.window_manager, &self.render_manager)
        {
            if render_manager.should_request_redraw(
                &self.object_manager,
                Some(&self.draw_manager),
            ) {
                window_manager.request_redraw();
            }
        }
    }
}
