use pyo3::prelude::*;
use crate::types::vector::{Vec2, Vec3};

// ========== Vector Bindings ==========

/// Python wrapper for 2D Vector
#[pyclass(name = "Vec2")]
#[derive(Clone)]
pub struct PyVec2 {
    pub(crate) inner: Vec2,
}

#[pymethods]
#[allow(non_snake_case)]
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

    // Vector constants as class attributes
    #[classattr]
    fn ZERO() -> PyVec2 {
        PyVec2 { inner: Vec2::new(0.0, 0.0) }
    }

    #[classattr]
    fn UP() -> PyVec2 {
        PyVec2 { inner: Vec2::new(0.0, 1.0) }
    }

    #[classattr]
    fn DOWN() -> PyVec2 {
        PyVec2 { inner: Vec2::new(0.0, -1.0) }
    }

    #[classattr]
    fn LEFT() -> PyVec2 {
        PyVec2 { inner: Vec2::new(-1.0, 0.0) }
    }

    #[classattr]
    fn RIGHT() -> PyVec2 {
        PyVec2 { inner: Vec2::new(1.0, 0.0) }
    }

    // Python operator overloads
    fn __add__(&self, other: &Bound<'_, pyo3::PyAny>) -> PyResult<PyVec2> {
        if let Ok(vec) = other.extract::<PyVec2>() {
            Ok(self.add(&vec))
        } else if let Ok(scalar) = other.extract::<f32>() {
            Ok(self.add_scalar(scalar))
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err(
                "unsupported operand type(s) for +: 'Vec2' and unknown type"
            ))
        }
    }

    fn __radd__(&self, other: &Bound<'_, pyo3::PyAny>) -> PyResult<PyVec2> {
        self.__add__(other)
    }

    fn __sub__(&self, other: &Bound<'_, pyo3::PyAny>) -> PyResult<PyVec2> {
        if let Ok(vec) = other.extract::<PyVec2>() {
            Ok(self.subtract(&vec))
        } else if let Ok(scalar) = other.extract::<f32>() {
            Ok(self.subtract_scalar(scalar))
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err(
                "unsupported operand type(s) for -: 'Vec2' and unknown type"
            ))
        }
    }

    fn __rsub__(&self, other: &Bound<'_, pyo3::PyAny>) -> PyResult<PyVec2> {
        if let Ok(scalar) = other.extract::<f32>() {
            Ok(PyVec2 {
                inner: Vec2::new(scalar - self.inner.x(), scalar - self.inner.y())
            })
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err(
                "unsupported operand type(s) for -: unknown type and 'Vec2'"
            ))
        }
    }

    fn __mul__(&self, other: &Bound<'_, pyo3::PyAny>) -> PyResult<PyVec2> {
        if let Ok(vec) = other.extract::<PyVec2>() {
            Ok(self.multiply(&vec))
        } else if let Ok(scalar) = other.extract::<f32>() {
            Ok(self.multiply_scalar(scalar))
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err(
                "unsupported operand type(s) for *: 'Vec2' and unknown type"
            ))
        }
    }

    fn __rmul__(&self, other: &Bound<'_, pyo3::PyAny>) -> PyResult<PyVec2> {
        self.__mul__(other)
    }

    fn __truediv__(&self, other: &Bound<'_, pyo3::PyAny>) -> PyResult<PyVec2> {
        if let Ok(vec) = other.extract::<PyVec2>() {
            Ok(self.divide(&vec))
        } else if let Ok(scalar) = other.extract::<f32>() {
            Ok(self.divide_scalar(scalar))
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err(
                "unsupported operand type(s) for /: 'Vec2' and unknown type"
            ))
        }
    }

    fn __rtruediv__(&self, other: &Bound<'_, pyo3::PyAny>) -> PyResult<PyVec2> {
        if let Ok(scalar) = other.extract::<f32>() {
            Ok(PyVec2 {
                inner: Vec2::new(scalar / self.inner.x(), scalar / self.inner.y())
            })
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err(
                "unsupported operand type(s) for /: unknown type and 'Vec2'"
            ))
        }
    }
}

/// Python wrapper for 3D Vector
#[pyclass(name = "Vec3")]
#[derive(Clone)]
pub struct PyVec3 {
    inner: Vec3,
}

#[pymethods]
#[allow(non_snake_case)]
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

    // Vector constants as class attributes
    #[classattr]
    fn ZERO() -> PyVec3 {
        PyVec3 { inner: Vec3::new(0.0, 0.0, 0.0) }
    }

    #[classattr]
    fn UP() -> PyVec3 {
        PyVec3 { inner: Vec3::new(0.0, 1.0, 0.0) }
    }

    #[classattr]
    fn DOWN() -> PyVec3 {
        PyVec3 { inner: Vec3::new(0.0, -1.0, 0.0) }
    }

    #[classattr]
    fn LEFT() -> PyVec3 {
        PyVec3 { inner: Vec3::new(-1.0, 0.0, 0.0) }
    }

    #[classattr]
    fn RIGHT() -> PyVec3 {
        PyVec3 { inner: Vec3::new(1.0, 0.0, 0.0) }
    }

    #[classattr]
    fn FORWARD() -> PyVec3 {
        PyVec3 { inner: Vec3::new(0.0, 0.0, 1.0) }
    }

    #[classattr]
    fn BACK() -> PyVec3 {
        PyVec3 { inner: Vec3::new(0.0, 0.0, -1.0) }
    }

    // Python operator overloads
    fn __add__(&self, other: &Bound<'_, pyo3::PyAny>) -> PyResult<PyVec3> {
        if let Ok(vec) = other.extract::<PyVec3>() {
            Ok(self.add(&vec))
        } else if let Ok(scalar) = other.extract::<f32>() {
            Ok(self.add_scalar(scalar))
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err(
                "unsupported operand type(s) for +: 'Vec3' and unknown type"
            ))
        }
    }

    fn __radd__(&self, other: &Bound<'_, pyo3::PyAny>) -> PyResult<PyVec3> {
        self.__add__(other)
    }

    fn __sub__(&self, other: &Bound<'_, pyo3::PyAny>) -> PyResult<PyVec3> {
        if let Ok(vec) = other.extract::<PyVec3>() {
            Ok(self.subtract(&vec))
        } else if let Ok(scalar) = other.extract::<f32>() {
            Ok(self.subtract_scalar(scalar))
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err(
                "unsupported operand type(s) for -: 'Vec3' and unknown type"
            ))
        }
    }

    fn __rsub__(&self, other: &Bound<'_, pyo3::PyAny>) -> PyResult<PyVec3> {
        if let Ok(scalar) = other.extract::<f32>() {
            Ok(PyVec3 {
                inner: Vec3::new(
                    scalar - self.inner.x(),
                    scalar - self.inner.y(),
                    scalar - self.inner.z()
                )
            })
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err(
                "unsupported operand type(s) for -: unknown type and 'Vec3'"
            ))
        }
    }

    fn __mul__(&self, other: &Bound<'_, pyo3::PyAny>) -> PyResult<PyVec3> {
        if let Ok(vec) = other.extract::<PyVec3>() {
            Ok(self.multiply(&vec))
        } else if let Ok(scalar) = other.extract::<f32>() {
            Ok(self.multiply_scalar(scalar))
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err(
                "unsupported operand type(s) for *: 'Vec3' and unknown type"
            ))
        }
    }

    fn __rmul__(&self, other: &Bound<'_, pyo3::PyAny>) -> PyResult<PyVec3> {
        self.__mul__(other)
    }

    fn __truediv__(&self, other: &Bound<'_, pyo3::PyAny>) -> PyResult<PyVec3> {
        if let Ok(vec) = other.extract::<PyVec3>() {
            Ok(self.divide(&vec))
        } else if let Ok(scalar) = other.extract::<f32>() {
            Ok(self.divide_scalar(scalar))
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err(
                "unsupported operand type(s) for /: 'Vec3' and unknown type"
            ))
        }
    }

    fn __rtruediv__(&self, other: &Bound<'_, pyo3::PyAny>) -> PyResult<PyVec3> {
        if let Ok(scalar) = other.extract::<f32>() {
            Ok(PyVec3 {
                inner: Vec3::new(
                    scalar / self.inner.x(),
                    scalar / self.inner.y(),
                    scalar / self.inner.z()
                )
            })
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err(
                "unsupported operand type(s) for /: unknown type and 'Vec3'"
            ))
        }
    }
}

