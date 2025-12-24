/// Core engine functionality
use super::logging;
use std::path::PathBuf;
use tracing::Level;

pub struct Engine {
    version: String,
    logging_initialized: bool,
}

pub const VERSION: &str = "1.2.0";

impl Engine {
    /// Create a new Engine instance with default logging (console only)
    pub fn new() -> Self {
        logging::init_default();
        Self {
            version: VERSION.to_string(),
            logging_initialized: true,
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
            logging_initialized: true,
        }
    }

    pub fn open_window(&self, title: &str, width: u32, height: u32) {
        logging::log_info(&format!("Opening window: {} ({}x{})", title, width, height));
    }

    pub fn close_window(&self) {
        logging::log_info("Closing window");
    }

    /*
    Engine update loop
     */
    fn update(&self) {
        // ------------------------------------------------------------
        // IF NOT HEADLESS, DO THE FOLLOWING:
        // ------------------------------------------------------------

        // Time step/tick management

        // Input (collect raw input + build an input snapshot)

        // Event System - enqueue input events

        // UI - input handling / hit-testing (UI gets first right of refusal)

        // Event System - dispatch "unconsumed" gameplay input events

        // GameObjects + Components - pre-physics (gameplay/AI/scripts)

        // **Fixed update:**
            // Physics (often fixed-timestep; may run 0..N steps)

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
