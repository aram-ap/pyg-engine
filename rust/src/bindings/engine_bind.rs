use pyo3::prelude::*;
use crate::core::engine::Engine as RustEngine;
use crate::core::time::Time as RustTime;
use crate::core::game_object::GameObject as RustGameObject;
use crate::core::component::{ComponentTrait, TransformComponent};

// Import bindings from separate modules
use super::vector_bind::{PyVec2, PyVec3};
use super::color_bind::PyColor;

// ========== Engine Bindings ==========

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

// ========== Vector Bindings ==========
// Moved to vector_bindings.rs

// ========== Color Bindings ==========
// Moved to color_bindings.rs

// ========== Time Bindings ==========

/// Python wrapper for Time
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

/// Python wrapper for GameObject
#[pyclass(name = "GameObject", unsendable)]
pub struct PyGameObject {
    inner: RustGameObject,
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

    fn set_name(&mut self, name: String) {
        self.inner.set_name(name);
    }

    fn update(&self) {
        self.inner.update();
    }
}

// ========== Component Bindings ==========

/// Python wrapper for TransformComponent
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

// ========== Module Initialization ==========

/// Module initialization function
#[pymodule]
fn pyg_engine_native(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyEngine>()?;
    m.add_class::<PyVec2>()?;
    m.add_class::<PyVec3>()?;
    m.add_class::<PyColor>()?;
    m.add_class::<PyTime>()?;
    m.add_class::<PyGameObject>()?;
    m.add_class::<PyTransformComponent>()?;
    Ok(())
}
