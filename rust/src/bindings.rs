use pyo3::prelude::*;
use crate::engine::Engine as RustEngine;
use crate::vector::{Vec2, Vec3};
use crate::time::Time as RustTime;
use crate::color::Color as RustColor;
use crate::game_object::GameObject as RustGameObject;
use crate::component::{ComponentTrait, TransformComponent};

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

/// Python wrapper for 2D Vector
#[pyclass(name = "Vec2")]
#[derive(Clone)]
pub struct PyVec2 {
    inner: Vec2,
}

#[pymethods]
impl PyVec2 {
    #[new]
    fn new(x: f32, y: f32) -> Self {
        Self {
            inner: Vec2::new(x, y),
        }
    }

    #[getter]
    fn x(&self) -> f32 {
        self.inner.x()
    }

    #[getter]
    fn y(&self) -> f32 {
        self.inner.y()
    }

    fn add(&self, other: &PyVec2) -> PyVec2 {
        PyVec2 {
            inner: self.inner.add(&other.inner),
        }
    }

    fn add_scalar(&self, scalar: f32) -> PyVec2 {
        PyVec2 {
            inner: self.inner.add_scalar(scalar),
        }
    }

    fn subtract(&self, other: &PyVec2) -> PyVec2 {
        PyVec2 {
            inner: self.inner.subtract(&other.inner),
        }
    }

    fn subtract_scalar(&self, scalar: f32) -> PyVec2 {
        PyVec2 {
            inner: self.inner.subtract_scalar(scalar),
        }
    }

    fn multiply(&self, other: &PyVec2) -> PyVec2 {
        PyVec2 {
            inner: self.inner.multiply(&other.inner),
        }
    }

    fn multiply_scalar(&self, scalar: f32) -> PyVec2 {
        PyVec2 {
            inner: self.inner.multiply_scalar(scalar),
        }
    }

    fn divide(&self, other: &PyVec2) -> PyVec2 {
        PyVec2 {
            inner: self.inner.divide(&other.inner),
        }
    }

    fn divide_scalar(&self, scalar: f32) -> PyVec2 {
        PyVec2 {
            inner: self.inner.divide_scalar(scalar),
        }
    }

    fn length(&self) -> f32 {
        self.inner.length()
    }

    fn normalize(&self) -> PyVec2 {
        PyVec2 {
            inner: self.inner.normalize(),
        }
    }

    fn dot(&self, other: &PyVec2) -> f32 {
        self.inner.dot(&other.inner)
    }

    fn cross(&self, other: &PyVec2) -> f32 {
        self.inner.cross(&other.inner)
    }

    fn distance(&self, other: &PyVec2) -> f32 {
        self.inner.distance(&other.inner)
    }

    fn lerp(&self, other: &PyVec2, t: f32) -> PyVec2 {
        PyVec2 {
            inner: self.inner.lerp(&other.inner, t),
        }
    }

    fn __str__(&self) -> String {
        self.inner.to_string()
    }

    fn __repr__(&self) -> String {
        format!("Vec2{}", self.inner.to_string())
    }
}

/// Python wrapper for 3D Vector
#[pyclass(name = "Vec3")]
#[derive(Clone)]
pub struct PyVec3 {
    inner: Vec3,
}

#[pymethods]
impl PyVec3 {
    #[new]
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            inner: Vec3::new(x, y, z),
        }
    }

    #[getter]
    fn x(&self) -> f32 {
        self.inner.x()
    }

    #[getter]
    fn y(&self) -> f32 {
        self.inner.y()
    }

    #[getter]
    fn z(&self) -> f32 {
        self.inner.z()
    }

    fn add(&self, other: &PyVec3) -> PyVec3 {
        PyVec3 {
            inner: self.inner.add(&other.inner),
        }
    }

    fn add_scalar(&self, scalar: f32) -> PyVec3 {
        PyVec3 {
            inner: self.inner.add_scalar(scalar),
        }
    }

    fn subtract(&self, other: &PyVec3) -> PyVec3 {
        PyVec3 {
            inner: self.inner.subtract(&other.inner),
        }
    }

    fn subtract_scalar(&self, scalar: f32) -> PyVec3 {
        PyVec3 {
            inner: self.inner.subtract_scalar(scalar),
        }
    }

    fn multiply(&self, other: &PyVec3) -> PyVec3 {
        PyVec3 {
            inner: self.inner.multiply(&other.inner),
        }
    }

    fn multiply_scalar(&self, scalar: f32) -> PyVec3 {
        PyVec3 {
            inner: self.inner.multiply_scalar(scalar),
        }
    }

    fn divide(&self, other: &PyVec3) -> PyVec3 {
        PyVec3 {
            inner: self.inner.divide(&other.inner),
        }
    }

    fn divide_scalar(&self, scalar: f32) -> PyVec3 {
        PyVec3 {
            inner: self.inner.divide_scalar(scalar),
        }
    }

    fn length(&self) -> f32 {
        self.inner.length()
    }

    fn normalize(&self) -> PyVec3 {
        PyVec3 {
            inner: self.inner.normalize(),
        }
    }

    fn dot(&self, other: &PyVec3) -> f32 {
        self.inner.dot(&other.inner)
    }

    fn cross(&self, other: &PyVec3) -> PyVec3 {
        PyVec3 {
            inner: self.inner.cross(&other.inner),
        }
    }

    fn distance(&self, other: &PyVec3) -> f32 {
        self.inner.distance(&other.inner)
    }

    fn lerp(&self, other: &PyVec3, t: f32) -> PyVec3 {
        PyVec3 {
            inner: self.inner.lerp(&other.inner, t),
        }
    }

    fn __str__(&self) -> String {
        self.inner.to_string()
    }

    fn __repr__(&self) -> String {
        format!("Vec3{}", self.inner.to_string())
    }
}

// ========== Color Bindings ==========

/// Python wrapper for Color
#[pyclass(name = "Color")]
#[derive(Clone)]
pub struct PyColor {
    inner: RustColor,
}

#[pymethods]
impl PyColor {
    #[new]
    fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            inner: RustColor::new(r, g, b, a),
        }
    }

    #[staticmethod]
    fn rgb(r: u8, g: u8, b: u8) -> PyColor {
        PyColor {
            inner: RustColor::rgb(r, g, b),
        }
    }

    #[staticmethod]
    fn rgba(r: u8, g: u8, b: u8, a: u8) -> PyColor {
        PyColor {
            inner: RustColor::rgba(r, g, b, a),
        }
    }

    #[staticmethod]
    fn from_hex(hex: &str) -> PyColor {
        PyColor {
            inner: RustColor::from_hex(hex),
        }
    }

    #[getter]
    fn r(&self) -> f32 {
        self.inner.r()
    }

    #[getter]
    fn g(&self) -> f32 {
        self.inner.g()
    }

    #[getter]
    fn b(&self) -> f32 {
        self.inner.b()
    }

    #[getter]
    fn a(&self) -> f32 {
        self.inner.a()
    }

    fn with_alpha(&self, a: f32) -> PyColor {
        PyColor {
            inner: self.inner.with_alpha(a),
        }
    }

    fn lerp(&self, other: &PyColor, t: f32) -> PyColor {
        PyColor {
            inner: self.inner.lerp(&other.inner, t),
        }
    }

    fn __str__(&self) -> String {
        self.inner.to_string()
    }

    fn __repr__(&self) -> String {
        self.inner.to_string()
    }

    // Color constants as class attributes
    #[classattr]
    fn TRANSPARENT() -> PyColor {
        PyColor { inner: RustColor::TRANSPARENT }
    }

    #[classattr]
    fn BLACK() -> PyColor {
        PyColor { inner: RustColor::BLACK }
    }

    #[classattr]
    fn WHITE() -> PyColor {
        PyColor { inner: RustColor::WHITE }
    }

    #[classattr]
    fn GRAY() -> PyColor {
        PyColor { inner: RustColor::GRAY }
    }

    #[classattr]
    fn GREY() -> PyColor {
        PyColor { inner: RustColor::GREY }
    }

    #[classattr]
    fn RED() -> PyColor {
        PyColor { inner: RustColor::RED }
    }

    #[classattr]
    fn GREEN() -> PyColor {
        PyColor { inner: RustColor::GREEN }
    }

    #[classattr]
    fn BLUE() -> PyColor {
        PyColor { inner: RustColor::BLUE }
    }

    #[classattr]
    fn YELLOW() -> PyColor {
        PyColor { inner: RustColor::YELLOW }
    }

    #[classattr]
    fn CYAN() -> PyColor {
        PyColor { inner: RustColor::CYAN }
    }

    #[classattr]
    fn MAGENTA() -> PyColor {
        PyColor { inner: RustColor::MAGENTA }
    }

    #[classattr]
    fn ORANGE() -> PyColor {
        PyColor { inner: RustColor::ORANGE }
    }

    #[classattr]
    fn PINK() -> PyColor {
        PyColor { inner: RustColor::PINK }
    }

    #[classattr]
    fn PURPLE() -> PyColor {
        PyColor { inner: RustColor::PURPLE }
    }

    #[classattr]
    fn BROWN() -> PyColor {
        PyColor { inner: RustColor::BROWN }
    }
}

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
