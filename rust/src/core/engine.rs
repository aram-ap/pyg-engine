/// Core engine functionality
use super::logging;
use super::window_manager::{WindowConfig, WindowManager};
use super::render_manager::RenderManager;
use super::object_manager::ObjectManager;
use super::input_manager::InputManager;
use super::game_object::GameObject;
use super::time::Time;
use std::path::PathBuf;
use tracing::Level;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::WindowId;

pub struct Engine {
    version: String,
    window_manager: Option<WindowManager>,
    render_manager: Option<RenderManager>,
    object_manager: Option<ObjectManager>,
    input_manager: Option<InputManager>,
    time: Time,
}

pub const VERSION: &str = "1.2.0";

impl Engine {
    /// Create a new Engine instance with default logging (console only)
    pub fn new() -> Self {
        logging::init_default();
        Self {
            version: VERSION.to_string(),
            window_manager: None,
            render_manager: None,
            object_manager: Some(ObjectManager::new()),
            input_manager: Some(InputManager::new()),
            time: Time::new(),
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

        Self {
            version: VERSION.to_string(),
            window_manager: None,
            render_manager: None,
            object_manager: Some(ObjectManager::new()),
            input_manager: Some(InputManager::new()),
            time: Time::new(),
        }
    }

    /// Run the engine with a window
    /// 
    /// This method takes ownership of the engine and runs the event loop.
    /// It creates a window and render manager, then enters the main game loop.
    pub fn run(self, window_config: WindowConfig) -> Result<(), Box<dyn std::error::Error>> {
        logging::log_info(&format!(
            "Starting PyG Engine v{} with window: {} ({}x{})",
            self.version, window_config.title, window_config.width, window_config.height
        ));

        // Create the event loop
        let event_loop = EventLoop::new()?;
        event_loop.set_control_flow(ControlFlow::Poll);

        // Create application handler
        let mut app = EngineApp {
            engine: self,
            window_config: Some(window_config),
        };

        // Run the event loop
        event_loop.run_app(&mut app)?;

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

    /// Engine update loop
    fn update(&mut self) {
        // ------------------------------------------------------------
        // IF NOT HEADLESS, DO THE FOLLOWING:
        // ------------------------------------------------------------

        // Time step/tick management
        self.time.tick();

        // Input (collect raw input + build an input snapshot)

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
    fn render(&mut self) {
        if let Some(render_manager) = &mut self.render_manager {
            match render_manager.render() {
                Ok(_) => {}
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

/// Application handler for winit event loop
struct EngineApp {
    engine: Engine,
    window_config: Option<WindowConfig>,
}

impl ApplicationHandler for EngineApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Create window and render manager when the application resumes
        if self.engine.window_manager.is_none() {
            if let Some(config) = self.window_config.take() {
                // Extract background color and vsync before config is moved
                let bg_color = config.background_color;
                let vsync = config.vsync;
                match WindowManager::new(event_loop, config) {
                    Ok(window_manager) => {
                        logging::log_info("Window created successfully");
                        
                        // Create render manager with the window Arc
                        let window = window_manager.window_arc();
                        match pollster::block_on(RenderManager::new(window, bg_color, vsync)) {
                            Ok(render_manager) => {
                                logging::log_info("Render manager initialized successfully");
                                self.engine.render_manager = Some(render_manager);
                                self.engine.window_manager = Some(window_manager);
                                
                                // Request initial redraw
                                if let Some(wm) = &self.engine.window_manager {
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
                
                if let Some(window_manager) = &mut self.engine.window_manager {
                    window_manager.update_size(physical_size);
                }
                
                if let Some(render_manager) = &mut self.engine.render_manager {
                    render_manager.resize(physical_size);
                }
            }
            WindowEvent::RedrawRequested => {
                // Update engine state
                self.engine.update();
                
                // Render the frame
                self.engine.render();
                
                // Request next frame
                if let Some(window_manager) = &self.engine.window_manager {
                    window_manager.request_redraw();
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        // This is called when all events have been processed
        // We can use this for continuous rendering
        if let Some(window_manager) = &self.engine.window_manager {
            window_manager.request_redraw();
        }
    }
}
