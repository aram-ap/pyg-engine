//! Logging system using tracing for structured, async-ready logging
//!
//! This module provides a comprehensive logging system with:
//! - Multiple log levels (trace, debug, info, warn, error)
//! - Timestamps
//! - Optional file output with daily rotation
//! - Async-friendly non-blocking file writes
//! - Structured logging support

use once_cell::sync::OnceCell;
use std::path::PathBuf;
use tracing::{debug, error, info, trace, warn, Level};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{
    fmt::{self, time::ChronoLocal},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter, Layer,
};

/// Global logger guard to keep file writer alive
static LOGGER_GUARD: OnceCell<Option<WorkerGuard>> = OnceCell::new();

/// Configuration for the logging system
#[derive(Debug, Clone)]
pub struct LogConfig {
    /// Minimum log level to display
    pub level: Level,
    /// Enable file logging
    pub enable_file: bool,
    /// Directory for log files
    pub log_dir: PathBuf,
    /// Enable colored output (for terminals)
    pub enable_colors: bool,
    /// Enable JSON formatting (for structured logs)
    pub enable_json: bool,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: Level::INFO,
            enable_file: false,
            log_dir: PathBuf::from("logs"),
            enable_colors: true,
            enable_json: false,
        }
    }
}

/// Initialize the logging system
///
/// This should be called once at the start of the application.
/// Subsequent calls will be ignored.
pub fn init_logging(config: LogConfig) {
    // Only initialize once
    if LOGGER_GUARD.get().is_some() {
        return;
    }

    let mut layers = Vec::new();
    let mut guard = None;

    // File logging layer
    if config.enable_file {
        // Create log directory if it doesn't exist
        let _ = std::fs::create_dir_all(&config.log_dir);

        let file_appender = tracing_appender::rolling::daily(
            config.log_dir.clone(),
            "pyg_engine.log",
        );
        let (non_blocking, worker_guard) = tracing_appender::non_blocking(file_appender);
        guard = Some(worker_guard);

        let file_layer = if config.enable_json {
            fmt::layer()
                .json()
                .with_writer(non_blocking)
                .with_timer(ChronoLocal::rfc_3339())
                .with_target(true)
                .with_thread_ids(true)
                .with_file(true)
                .with_line_number(true)
                .boxed()
        } else {
            fmt::layer()
                .with_writer(non_blocking)
                .with_timer(ChronoLocal::rfc_3339())
                .with_target(true)
                .with_ansi(false)
                .boxed()
        };

        layers.push(file_layer);
    }

    // Console logging layer
    let console_layer = fmt::layer()
        .with_timer(ChronoLocal::rfc_3339())
        .with_target(false)
        .with_ansi(config.enable_colors)
        .compact()
        .boxed();

    layers.push(console_layer);

    // Build the subscriber with env filter
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(config.level.as_str()));

    tracing_subscriber::registry()
        .with(env_filter)
        .with(layers)
        .init();

    // Store the guard to keep file writer alive
    let _ = LOGGER_GUARD.set(guard);

    info!("Logging system initialized with level: {}", config.level);
}

/// Initialize with default configuration (console only, INFO level)
pub fn init_default() {
    init_logging(LogConfig::default());
}

/// Log at TRACE level
#[inline]
pub fn log_trace(message: &str) {
    trace!("{}", message);
}

/// Log at DEBUG level
#[inline]
pub fn log_debug(message: &str) {
    debug!("{}", message);
}

/// Log at INFO level
#[inline]
pub fn log_info(message: &str) {
    info!("{}", message);
}

/// Log at WARN level
#[inline]
pub fn log_warn(message: &str) {
    warn!("{}", message);
}

/// Log at ERROR level
#[inline]
pub fn log_error(message: &str) {
    error!("{}", message);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = LogConfig::default();
        assert_eq!(config.level, Level::INFO);
        assert!(!config.enable_file);
        assert!(config.enable_colors);
    }

    #[test]
    fn test_logging_functions() {
        init_default();
        log_trace("test trace");
        log_debug("test debug");
        log_info("test info");
        log_warn("test warn");
        log_error("test error");
    }
}
