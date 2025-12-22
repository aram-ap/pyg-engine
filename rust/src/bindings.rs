use pyo3::prelude::*;
use crate::engine::Engine as RustEngine;

/// Python wrapper for the Rust Engine
#[pyclass(name = "Engine")]
pub struct PyEngine {
    inner: RustEngine,
}

#[pymethods]
impl PyEngine {
    /// Create a new Engine instance with default logging (console only, INFO level)
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

        Self { inner }
    }

    /// Log a message at INFO level (default log method)
    fn log(&self, message: &str) {
        self.inner.log(message);
    }

    /// Log a message at TRACE level (most verbose)
    fn log_trace(&self, message: &str) {
        self.inner.log_trace(message);
    }

    /// Log a message at DEBUG level
    fn log_debug(&self, message: &str) {
        self.inner.log_debug(message);
    }

    /// Log a message at INFO level
    fn log_info(&self, message: &str) {
        self.inner.log_info(message);
    }

    /// Log a message at WARN level
    fn log_warn(&self, message: &str) {
        self.inner.log_warn(message);
    }

    /// Log a message at ERROR level
    fn log_error(&self, message: &str) {
        self.inner.log_error(message);
    }

    /// Get the engine version
    #[getter]
    fn version(&self) -> String {
        self.inner.version().to_string()
    }
}

/// Module initialization function
#[pymodule]
fn pyg_engine_native(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyEngine>()?;
    Ok(())
}
