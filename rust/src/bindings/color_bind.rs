use pyo3::prelude::*;
use crate::types::color::Color as RustColor;

// ========== Color Bindings ==========

/// Python wrapper for Color
#[pyclass(name = "Color")]
#[derive(Clone)]
pub struct PyColor {
    inner: RustColor,
}

#[pymethods]
#[allow(non_snake_case)]
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

