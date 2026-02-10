use crate::types::color::Color as RustColor;
use pyo3::prelude::*;

// ========== Color Bindings ==========

/// Python wrapper for Color
#[pyclass(name = "Color")]
#[derive(Clone)]
pub struct PyColor {
    pub(crate) inner: RustColor,
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

    #[staticmethod]
    fn from_hsv(h: f32, s: f32, v: f32, a: f32) -> PyColor {
        PyColor {
            inner: RustColor::from_hsv(h, s, v, a),
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

    fn set_r(&self, r: f32) -> PyColor {
        PyColor {
            inner: self.inner.set_r(r),
        }
    }

    fn set_g(&self, g: f32) -> PyColor {
        PyColor {
            inner: self.inner.set_g(g),
        }
    }

    fn set_b(&self, b: f32) -> PyColor {
        PyColor {
            inner: self.inner.set_b(b),
        }
    }

    fn set_a(&self, a: f32) -> PyColor {
        PyColor {
            inner: self.inner.set_a(a),
        }
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

    fn __add__(&self, other: &PyColor) -> PyColor {
        PyColor {
            inner: self.inner + other.inner,
        }
    }

    fn __sub__(&self, other: &PyColor) -> PyColor {
        PyColor {
            inner: self.inner - other.inner,
        }
    }

    fn __mul__(&self, other: &PyColor) -> PyColor {
        PyColor {
            inner: self.inner * other.inner,
        }
    }

    fn __truediv__(&self, other: &PyColor) -> PyColor {
        PyColor {
            inner: self.inner / other.inner,
        }
    }

    fn __eq__(&self, other: &PyColor) -> bool {
        self.inner.approx_eq_default(&other.inner)
    }

    fn __ne__(&self, other: &PyColor) -> bool {
        !self.inner.approx_eq_default(&other.inner)
    }

    // Color constants as class attributes
    #[classattr]
    fn TRANSPARENT() -> PyColor {
        PyColor {
            inner: RustColor::TRANSPARENT,
        }
    }

    #[classattr]
    fn BLACK() -> PyColor {
        PyColor {
            inner: RustColor::BLACK,
        }
    }

    #[classattr]
    fn WHITE() -> PyColor {
        PyColor {
            inner: RustColor::WHITE,
        }
    }

    #[classattr]
    fn GRAY() -> PyColor {
        PyColor {
            inner: RustColor::GRAY,
        }
    }

    #[classattr]
    fn GREY() -> PyColor {
        PyColor {
            inner: RustColor::GREY,
        }
    }

    #[classattr]
    fn DARK_GRAY() -> PyColor {
        PyColor {
            inner: RustColor::DARK_GRAY,
        }
    }

    #[classattr]
    fn DARK_GREY() -> PyColor {
        PyColor {
            inner: RustColor::DARK_GREY,
        }
    }

    #[classattr]
    fn LIGHT_GRAY() -> PyColor {
        PyColor {
            inner: RustColor::LIGHT_GRAY,
        }
    }

    #[classattr]
    fn LIGHT_GREY() -> PyColor {
        PyColor {
            inner: RustColor::LIGHT_GREY,
        }
    }

    #[classattr]
    fn RED() -> PyColor {
        PyColor {
            inner: RustColor::RED,
        }
    }

    #[classattr]
    fn GREEN() -> PyColor {
        PyColor {
            inner: RustColor::GREEN,
        }
    }

    #[classattr]
    fn LIME() -> PyColor {
        PyColor {
            inner: RustColor::LIME,
        }
    }

    #[classattr]
    fn BLUE() -> PyColor {
        PyColor {
            inner: RustColor::BLUE,
        }
    }

    #[classattr]
    fn YELLOW() -> PyColor {
        PyColor {
            inner: RustColor::YELLOW,
        }
    }

    #[classattr]
    fn CYAN() -> PyColor {
        PyColor {
            inner: RustColor::CYAN,
        }
    }

    #[classattr]
    fn MAGENTA() -> PyColor {
        PyColor {
            inner: RustColor::MAGENTA,
        }
    }

    #[classattr]
    fn ORANGE() -> PyColor {
        PyColor {
            inner: RustColor::ORANGE,
        }
    }

    #[classattr]
    fn PINK() -> PyColor {
        PyColor {
            inner: RustColor::PINK,
        }
    }

    #[classattr]
    fn PURPLE() -> PyColor {
        PyColor {
            inner: RustColor::PURPLE,
        }
    }

    #[classattr]
    fn BROWN() -> PyColor {
        PyColor {
            inner: RustColor::BROWN,
        }
    }
}
