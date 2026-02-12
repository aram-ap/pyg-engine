use crate::types::vector::{Vec2, Vec3};
use pyo3::prelude::*;

// ========== Vector Bindings ==========

/// 2D vector for positions, directions, and mathematical operations.
///
/// `Vec2` represents a two-dimensional vector with `x` and `y` components.
/// It supports common vector operations like addition, subtraction, multiplication,
/// normalization, dot/cross products, and linear interpolation.
///
/// # Operator Overloading
///
/// Vec2 supports Python operators for intuitive math operations:
/// - **Addition**: `v1 + v2` (component-wise), `v + scalar` (add to both components)
/// - **Subtraction**: `v1 - v2` (component-wise), `v - scalar` (subtract from both)
/// - **Multiplication**: `v1 * v2` (component-wise), `v * scalar` (scale)
/// - **Division**: `v1 / v2` (component-wise), `v / scalar` (inverse scale)
///
/// # Constants
///
/// Predefined vector constants are available as class attributes:
/// - `Vec2.ZERO` = (0, 0)
/// - `Vec2.UP` = (0, 1)
/// - `Vec2.DOWN` = (0, -1)
/// - `Vec2.LEFT` = (-1, 0)
/// - `Vec2.RIGHT` = (1, 0)
///
/// # Examples
///
/// ## Basic Usage
/// ```python
/// from pyg_engine import Vec2
///
/// # Create vectors
/// pos = Vec2(10.0, 20.0)
/// velocity = Vec2(5.0, -3.0)
///
/// # Access components
/// print(f"Position: ({pos.x}, {pos.y})")
///
/// # Vector arithmetic
/// new_pos = pos + velocity
/// scaled = velocity * 2.0
/// inverted = -velocity  # Same as velocity * -1.0
/// ```
///
/// ## Movement Example
/// ```python
/// from pyg_engine import Vec2
///
/// player_pos = Vec2(100.0, 200.0)
/// direction = Vec2(1.0, 0.0)  # Right
/// speed = 150.0
///
/// # Frame-independent movement
/// def update(dt, engine, data):
///     global player_pos
///     player_pos = player_pos + direction * speed * dt
/// ```
///
/// ## Direction and Distance
/// ```python
/// from pyg_engine import Vec2
///
/// player = Vec2(100.0, 100.0)
/// enemy = Vec2(300.0, 200.0)
///
/// # Calculate direction and distance
/// to_enemy = enemy - player
/// distance = to_enemy.length()
/// direction = to_enemy.normalize()
///
/// print(f"Enemy is {distance:.1f} units away")
///
/// # Move toward enemy
/// speed = 50.0
/// player = player + direction * speed * dt
/// ```
///
/// ## Interpolation
/// ```python
/// from pyg_engine import Vec2
///
/// start = Vec2(0.0, 0.0)
/// end = Vec2(100.0, 100.0)
///
/// # Smooth movement over time
/// t = 0.0
/// def update(dt, engine, data):
///     global t
///     t = min(t + dt * 0.5, 1.0)  # Reach end in 2 seconds
///     pos = start.lerp(end, t)
///     print(f"Position: ({pos.x:.1f}, {pos.y:.1f})")
/// ```
///
/// # See Also
/// - `Vec3` - 3D vector with x, y, z components
/// - `examples/python_game_object_transform_demo.py` - Transform operations
#[pyclass(name = "Vec2")]
#[derive(Clone)]
pub struct PyVec2 {
    pub(crate) inner: Vec2,
}

#[pymethods]
#[allow(non_snake_case)]
impl PyVec2 {
    /// Create a new 2D vector.
    ///
    /// # Arguments
    /// * `x` - X component (horizontal)
    /// * `y` - Y component (vertical)
    #[new]
    fn new(x: f32, y: f32) -> Self {
        Self {
            inner: Vec2::new(x, y),
        }
    }

    /// Get the X component of the vector.
    #[getter]
    fn x(&self) -> f32 {
        self.inner.x()
    }

    /// Get the Y component of the vector.
    #[getter]
    fn y(&self) -> f32 {
        self.inner.y()
    }

    /// Add two vectors component-wise.
    ///
    /// # Example
    /// ```python
    /// v1 = Vec2(10.0, 20.0)
    /// v2 = Vec2(5.0, -3.0)
    /// result = v1.add(v2)  # Vec2(15.0, 17.0)
    /// # Or use operator: result = v1 + v2
    /// ```
    fn add(&self, other: &PyVec2) -> PyVec2 {
        PyVec2 {
            inner: self.inner.add(&other.inner),
        }
    }

    /// Add a scalar value to both components.
    ///
    /// # Example
    /// ```python
    /// v = Vec2(10.0, 20.0)
    /// result = v.add_scalar(5.0)  # Vec2(15.0, 25.0)
    /// # Or use operator: result = v + 5.0
    /// ```
    fn add_scalar(&self, scalar: f32) -> PyVec2 {
        PyVec2 {
            inner: self.inner.add_scalar(scalar),
        }
    }

    /// Subtract one vector from another component-wise.
    ///
    /// # Example
    /// ```python
    /// v1 = Vec2(10.0, 20.0)
    /// v2 = Vec2(5.0, 3.0)
    /// result = v1.subtract(v2)  # Vec2(5.0, 17.0)
    /// # Or use operator: result = v1 - v2
    /// ```
    fn subtract(&self, other: &PyVec2) -> PyVec2 {
        PyVec2 {
            inner: self.inner.subtract(&other.inner),
        }
    }

    /// Subtract a scalar value from both components.
    ///
    /// # Example
    /// ```python
    /// v = Vec2(10.0, 20.0)
    /// result = v.subtract_scalar(5.0)  # Vec2(5.0, 15.0)
    /// # Or use operator: result = v - 5.0
    /// ```
    fn subtract_scalar(&self, scalar: f32) -> PyVec2 {
        PyVec2 {
            inner: self.inner.subtract_scalar(scalar),
        }
    }

    /// Multiply two vectors component-wise.
    ///
    /// # Example
    /// ```python
    /// v1 = Vec2(10.0, 20.0)
    /// v2 = Vec2(2.0, 0.5)
    /// result = v1.multiply(v2)  # Vec2(20.0, 10.0)
    /// # Or use operator: result = v1 * v2
    /// ```
    fn multiply(&self, other: &PyVec2) -> PyVec2 {
        PyVec2 {
            inner: self.inner.multiply(&other.inner),
        }
    }

    /// Multiply both components by a scalar (scale the vector).
    ///
    /// # Example
    /// ```python
    /// v = Vec2(10.0, 20.0)
    /// result = v.multiply_scalar(2.0)  # Vec2(20.0, 40.0)
    /// # Or use operator: result = v * 2.0
    /// ```
    fn multiply_scalar(&self, scalar: f32) -> PyVec2 {
        PyVec2 {
            inner: self.inner.multiply_scalar(scalar),
        }
    }

    /// Divide one vector by another component-wise.
    ///
    /// # Example
    /// ```python
    /// v1 = Vec2(10.0, 20.0)
    /// v2 = Vec2(2.0, 4.0)
    /// result = v1.divide(v2)  # Vec2(5.0, 5.0)
    /// # Or use operator: result = v1 / v2
    /// ```
    fn divide(&self, other: &PyVec2) -> PyVec2 {
        PyVec2 {
            inner: self.inner.divide(&other.inner),
        }
    }

    /// Divide both components by a scalar.
    ///
    /// # Example
    /// ```python
    /// v = Vec2(10.0, 20.0)
    /// result = v.divide_scalar(2.0)  # Vec2(5.0, 10.0)
    /// # Or use operator: result = v / 2.0
    /// ```
    fn divide_scalar(&self, scalar: f32) -> PyVec2 {
        PyVec2 {
            inner: self.inner.divide_scalar(scalar),
        }
    }

    /// Calculate the magnitude (length) of the vector.
    ///
    /// Returns the Euclidean length: `sqrt(x² + y²)`
    ///
    /// # Example
    /// ```python
    /// v = Vec2(3.0, 4.0)
    /// length = v.length()  # 5.0
    ///
    /// # Check distance from origin
    /// if v.length() > 100.0:
    ///     print("Too far!")
    /// ```
    fn length(&self) -> f32 {
        self.inner.length()
    }

    /// Return a normalized (unit length) vector in the same direction.
    ///
    /// A normalized vector has length 1.0 but preserves the original direction.
    /// Useful for direction vectors, velocities, and movement.
    ///
    /// # Returns
    /// A new `Vec2` with the same direction but length 1.0.
    /// If the vector has zero length, returns a zero vector.
    ///
    /// # Example
    /// ```python
    /// velocity = Vec2(30.0, 40.0)
    /// direction = velocity.normalize()  # Vec2(0.6, 0.8)
    /// print(f"Length: {direction.length()}")  # 1.0
    ///
    /// # Move in direction at constant speed
    /// speed = 100.0
    /// player_pos = player_pos + direction * speed * dt
    /// ```
    fn normalize(&self) -> PyVec2 {
        PyVec2 {
            inner: self.inner.normalize(),
        }
    }

    /// Calculate the dot product with another vector.
    ///
    /// The dot product is: `self.x * other.x + self.y * other.y`
    ///
    /// Useful for:
    /// - Checking if vectors point in the same direction (positive = same, negative = opposite)
    /// - Calculating projection of one vector onto another
    /// - Determining angle between vectors
    ///
    /// # Returns
    /// A scalar value representing the dot product.
    ///
    /// # Example
    /// ```python
    /// forward = Vec2(1.0, 0.0)
    /// velocity = Vec2(5.0, 3.0)
    ///
    /// # Check if moving forward
    /// dot = forward.dot(velocity)
    /// if dot > 0:
    ///     print("Moving forward!")  # dot = 5.0
    ///
    /// # Check if vectors are perpendicular
    /// right = Vec2(0.0, 1.0)
    /// if abs(forward.dot(right)) < 0.01:
    ///     print("Perpendicular!")  # dot = 0.0
    /// ```
    fn dot(&self, other: &PyVec2) -> f32 {
        self.inner.dot(&other.inner)
    }

    /// Calculate the 2D cross product (z-component) with another vector.
    ///
    /// The 2D cross product returns a scalar: `self.x * other.y - self.y * other.x`
    ///
    /// Useful for:
    /// - Determining which side of a vector another vector is on
    /// - Calculating signed area
    /// - Detecting clockwise/counter-clockwise rotation
    ///
    /// # Returns
    /// - **Positive**: `other` is counter-clockwise from `self`
    /// - **Negative**: `other` is clockwise from `self`
    /// - **Zero**: Vectors are parallel
    ///
    /// # Example
    /// ```python
    /// forward = Vec2(1.0, 0.0)
    /// target = Vec2(5.0, 3.0)
    ///
    /// # Determine rotation direction
    /// cross = forward.cross(target)
    /// if cross > 0:
    ///     print("Turn left!")  # counter-clockwise
    /// elif cross < 0:
    ///     print("Turn right!")  # clockwise
    /// else:
    ///     print("Straight ahead!")  # parallel
    /// ```
    fn cross(&self, other: &PyVec2) -> f32 {
        self.inner.cross(&other.inner)
    }

    /// Calculate the Euclidean distance to another vector.
    ///
    /// Returns the length of the vector from `self` to `other`.
    /// Equivalent to `(other - self).length()`.
    ///
    /// # Example
    /// ```python
    /// player = Vec2(100.0, 100.0)
    /// enemy = Vec2(300.0, 200.0)
    ///
    /// dist = player.distance(enemy)  # ~223.6
    ///
    /// # Check range
    /// if dist < 50.0:
    ///     print("Enemy in range!")
    /// ```
    fn distance(&self, other: &PyVec2) -> f32 {
        self.inner.distance(&other.inner)
    }

    /// Linearly interpolate between two vectors.
    ///
    /// Returns a vector that is a blend between `self` and `other` based on parameter `t`.
    ///
    /// # Arguments
    /// * `other` - Target vector to interpolate toward
    /// * `t` - Interpolation factor:
    ///   - **0.0** = Returns `self` (start position)
    ///   - **0.5** = Returns midpoint between `self` and `other`
    ///   - **1.0** = Returns `other` (end position)
    ///   - Values outside [0, 1] extrapolate beyond the range
    ///
    /// # Formula
    /// `result = self + (other - self) * t`
    ///
    /// # Example
    /// ```python
    /// start = Vec2(0.0, 0.0)
    /// end = Vec2(100.0, 200.0)
    ///
    /// # Interpolate at 25% of the way
    /// quarter = start.lerp(end, 0.25)  # Vec2(25.0, 50.0)
    ///
    /// # Interpolate at 50% (midpoint)
    /// mid = start.lerp(end, 0.5)  # Vec2(50.0, 100.0)
    ///
    /// # Smooth animation
    /// t = 0.0
    /// def update(dt, engine, data):
    ///     global t
    ///     t = min(t + dt * 0.5, 1.0)  # Animate over 2 seconds
    ///     pos = start.lerp(end, t)
    ///     # Use pos for object position
    /// ```
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
        PyVec2 {
            inner: Vec2::new(0.0, 0.0),
        }
    }

    #[classattr]
    fn UP() -> PyVec2 {
        PyVec2 {
            inner: Vec2::new(0.0, 1.0),
        }
    }

    #[classattr]
    fn DOWN() -> PyVec2 {
        PyVec2 {
            inner: Vec2::new(0.0, -1.0),
        }
    }

    #[classattr]
    fn LEFT() -> PyVec2 {
        PyVec2 {
            inner: Vec2::new(-1.0, 0.0),
        }
    }

    #[classattr]
    fn RIGHT() -> PyVec2 {
        PyVec2 {
            inner: Vec2::new(1.0, 0.0),
        }
    }

    // Python operator overloads
    fn __add__(&self, other: &Bound<'_, pyo3::PyAny>) -> PyResult<PyVec2> {
        if let Ok(vec) = other.extract::<PyVec2>() {
            Ok(self.add(&vec))
        } else if let Ok(scalar) = other.extract::<f32>() {
            Ok(self.add_scalar(scalar))
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err(
                "unsupported operand type(s) for +: 'Vec2' and unknown type",
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
                "unsupported operand type(s) for -: 'Vec2' and unknown type",
            ))
        }
    }

    fn __rsub__(&self, other: &Bound<'_, pyo3::PyAny>) -> PyResult<PyVec2> {
        if let Ok(scalar) = other.extract::<f32>() {
            Ok(PyVec2 {
                inner: Vec2::new(scalar - self.inner.x(), scalar - self.inner.y()),
            })
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err(
                "unsupported operand type(s) for -: unknown type and 'Vec2'",
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
                "unsupported operand type(s) for *: 'Vec2' and unknown type",
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
                "unsupported operand type(s) for /: 'Vec2' and unknown type",
            ))
        }
    }

    fn __rtruediv__(&self, other: &Bound<'_, pyo3::PyAny>) -> PyResult<PyVec2> {
        if let Ok(scalar) = other.extract::<f32>() {
            Ok(PyVec2 {
                inner: Vec2::new(scalar / self.inner.x(), scalar / self.inner.y()),
            })
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err(
                "unsupported operand type(s) for /: unknown type and 'Vec2'",
            ))
        }
    }
}

/// 3D vector for positions, directions, and mathematical operations.
///
/// `Vec3` represents a three-dimensional vector with `x`, `y`, and `z` components.
/// It supports the same operations as `Vec2` with added support for 3D-specific
/// operations like 3D cross product.
///
/// # Operator Overloading
///
/// Vec3 supports Python operators for intuitive math operations:
/// - **Addition**: `v1 + v2` (component-wise), `v + scalar` (add to all components)
/// - **Subtraction**: `v1 - v2` (component-wise), `v - scalar` (subtract from all)
/// - **Multiplication**: `v1 * v2` (component-wise), `v * scalar` (scale)
/// - **Division**: `v1 / v2` (component-wise), `v / scalar` (inverse scale)
///
/// # Constants
///
/// Predefined vector constants are available as class attributes:
/// - `Vec3.ZERO` = (0, 0, 0)
/// - `Vec3.UP` = (0, 1, 0)
/// - `Vec3.DOWN` = (0, -1, 0)
/// - `Vec3.LEFT` = (-1, 0, 0)
/// - `Vec3.RIGHT` = (1, 0, 0)
/// - `Vec3.FORWARD` = (0, 0, 1)
/// - `Vec3.BACK` = (0, 0, -1)
///
/// # Examples
///
/// ## Basic Usage
/// ```python
/// from pyg_engine import Vec3
///
/// # Create vectors
/// pos = Vec3(10.0, 20.0, 5.0)
/// velocity = Vec3(5.0, -3.0, 0.0)
///
/// # Access components
/// print(f"Position: ({pos.x}, {pos.y}, {pos.z})")
///
/// # Vector arithmetic
/// new_pos = pos + velocity
/// scaled = velocity * 2.0
/// ```
///
/// ## 3D Movement
/// ```python
/// from pyg_engine import Vec3
///
/// player_pos = Vec3(0.0, 0.0, 0.0)
/// forward = Vec3(0.0, 0.0, 1.0)
/// speed = 10.0
///
/// def update(dt, engine, data):
///     global player_pos
///     player_pos = player_pos + forward * speed * dt
/// ```
///
/// ## Cross Product for Perpendicular Vector
/// ```python
/// from pyg_engine import Vec3
///
/// # Get right vector from forward and up
/// forward = Vec3(0.0, 0.0, 1.0)
/// up = Vec3(0.0, 1.0, 0.0)
/// right = forward.cross(up)  # Vec3(1.0, 0.0, 0.0)
///
/// # Create rotation-based movement
/// strafe_dir = right  # Move right
/// move_dir = forward  # Move forward
/// ```
///
/// # See Also
/// - `Vec2` - 2D vector with x, y components
/// - `examples/python_game_object_transform_demo.py` - 3D transform examples
#[pyclass(name = "Vec3")]
#[derive(Clone)]
pub struct PyVec3 {
    inner: Vec3,
}

#[pymethods]
#[allow(non_snake_case)]
impl PyVec3 {
    /// Create a new 3D vector.
    ///
    /// # Arguments
    /// * `x` - X component (horizontal, typically left-right)
    /// * `y` - Y component (vertical, typically up-down)
    /// * `z` - Z component (depth, typically forward-back)
    #[new]
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            inner: Vec3::new(x, y, z),
        }
    }

    /// Get the X component of the vector.
    #[getter]
    fn x(&self) -> f32 {
        self.inner.x()
    }

    /// Get the Y component of the vector.
    #[getter]
    fn y(&self) -> f32 {
        self.inner.y()
    }

    /// Get the Z component of the vector.
    #[getter]
    fn z(&self) -> f32 {
        self.inner.z()
    }

    /// Add two vectors component-wise.
    ///
    /// # Example
    /// ```python
    /// v1 = Vec3(10.0, 20.0, 5.0)
    /// v2 = Vec3(5.0, -3.0, 2.0)
    /// result = v1.add(v2)  # Vec3(15.0, 17.0, 7.0)
    /// # Or use operator: result = v1 + v2
    /// ```
    fn add(&self, other: &PyVec3) -> PyVec3 {
        PyVec3 {
            inner: self.inner.add(&other.inner),
        }
    }

    /// Add a scalar value to all components.
    ///
    /// # Example
    /// ```python
    /// v = Vec3(10.0, 20.0, 5.0)
    /// result = v.add_scalar(5.0)  # Vec3(15.0, 25.0, 10.0)
    /// # Or use operator: result = v + 5.0
    /// ```
    fn add_scalar(&self, scalar: f32) -> PyVec3 {
        PyVec3 {
            inner: self.inner.add_scalar(scalar),
        }
    }

    /// Subtract one vector from another component-wise.
    fn subtract(&self, other: &PyVec3) -> PyVec3 {
        PyVec3 {
            inner: self.inner.subtract(&other.inner),
        }
    }

    /// Subtract a scalar value from all components.
    fn subtract_scalar(&self, scalar: f32) -> PyVec3 {
        PyVec3 {
            inner: self.inner.subtract_scalar(scalar),
        }
    }

    /// Multiply two vectors component-wise.
    fn multiply(&self, other: &PyVec3) -> PyVec3 {
        PyVec3 {
            inner: self.inner.multiply(&other.inner),
        }
    }

    /// Multiply all components by a scalar (scale the vector).
    fn multiply_scalar(&self, scalar: f32) -> PyVec3 {
        PyVec3 {
            inner: self.inner.multiply_scalar(scalar),
        }
    }

    /// Divide one vector by another component-wise.
    fn divide(&self, other: &PyVec3) -> PyVec3 {
        PyVec3 {
            inner: self.inner.divide(&other.inner),
        }
    }

    /// Divide all components by a scalar.
    fn divide_scalar(&self, scalar: f32) -> PyVec3 {
        PyVec3 {
            inner: self.inner.divide_scalar(scalar),
        }
    }

    /// Calculate the magnitude (length) of the vector.
    ///
    /// Returns the Euclidean length: `sqrt(x² + y² + z²)`
    ///
    /// # Example
    /// ```python
    /// v = Vec3(2.0, 3.0, 6.0)
    /// length = v.length()  # 7.0
    /// ```
    fn length(&self) -> f32 {
        self.inner.length()
    }

    /// Return a normalized (unit length) vector in the same direction.
    ///
    /// A normalized vector has length 1.0 but preserves the original direction.
    ///
    /// # Example
    /// ```python
    /// velocity = Vec3(3.0, 4.0, 0.0)
    /// direction = velocity.normalize()
    /// print(f"Length: {direction.length()}")  # 1.0
    /// ```
    fn normalize(&self) -> PyVec3 {
        PyVec3 {
            inner: self.inner.normalize(),
        }
    }

    /// Calculate the dot product with another vector.
    ///
    /// The dot product is: `self.x * other.x + self.y * other.y + self.z * other.z`
    ///
    /// # Example
    /// ```python
    /// forward = Vec3(0.0, 0.0, 1.0)
    /// velocity = Vec3(5.0, 0.0, 3.0)
    ///
    /// dot = forward.dot(velocity)  # 3.0
    /// if dot > 0:
    ///     print("Moving forward!")
    /// ```
    fn dot(&self, other: &PyVec3) -> f32 {
        self.inner.dot(&other.inner)
    }

    /// Calculate the 3D cross product with another vector.
    ///
    /// The cross product returns a vector **perpendicular** to both input vectors.
    /// The magnitude of the result is proportional to the sine of the angle between
    /// the vectors.
    ///
    /// # Right-Hand Rule
    /// The direction follows the right-hand rule:
    /// - Point fingers along `self`
    /// - Curl them toward `other`
    /// - Thumb points in direction of result
    ///
    /// # Properties
    /// - `self.cross(other)` = `-other.cross(self)` (anti-commutative)
    /// - `self.cross(self)` = `Vec3.ZERO` (parallel vectors)
    /// - Result is perpendicular to both input vectors
    ///
    /// # Example
    /// ```python
    /// # Standard 3D basis vectors
    /// x = Vec3(1.0, 0.0, 0.0)
    /// y = Vec3(0.0, 1.0, 0.0)
    /// z = x.cross(y)  # Vec3(0.0, 0.0, 1.0)
    ///
    /// # Get perpendicular vector
    /// forward = Vec3(0.0, 0.0, 1.0)
    /// up = Vec3(0.0, 1.0, 0.0)
    /// right = forward.cross(up)  # Vec3(1.0, 0.0, 0.0)
    /// ```
    ///
    /// # Use Cases
    /// - Calculate surface normals
    /// - Find perpendicular directions
    /// - Determine rotation axis
    /// - Compute torque in physics
    fn cross(&self, other: &PyVec3) -> PyVec3 {
        PyVec3 {
            inner: self.inner.cross(&other.inner),
        }
    }

    /// Calculate the Euclidean distance to another vector.
    ///
    /// # Example
    /// ```python
    /// player = Vec3(100.0, 100.0, 0.0)
    /// enemy = Vec3(300.0, 200.0, 0.0)
    ///
    /// dist = player.distance(enemy)
    /// if dist < 50.0:
    ///     print("Enemy in range!")
    /// ```
    fn distance(&self, other: &PyVec3) -> f32 {
        self.inner.distance(&other.inner)
    }

    /// Linearly interpolate between two vectors.
    ///
    /// Returns a vector that is a blend between `self` and `other` based on parameter `t`.
    ///
    /// # Arguments
    /// * `other` - Target vector to interpolate toward
    /// * `t` - Interpolation factor:
    ///   - **0.0** = Returns `self` (start)
    ///   - **0.5** = Returns midpoint
    ///   - **1.0** = Returns `other` (end)
    ///
    /// # Example
    /// ```python
    /// start = Vec3(0.0, 0.0, 0.0)
    /// end = Vec3(100.0, 200.0, 50.0)
    ///
    /// mid = start.lerp(end, 0.5)  # Vec3(50.0, 100.0, 25.0)
    /// ```
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
        PyVec3 {
            inner: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    #[classattr]
    fn UP() -> PyVec3 {
        PyVec3 {
            inner: Vec3::new(0.0, 1.0, 0.0),
        }
    }

    #[classattr]
    fn DOWN() -> PyVec3 {
        PyVec3 {
            inner: Vec3::new(0.0, -1.0, 0.0),
        }
    }

    #[classattr]
    fn LEFT() -> PyVec3 {
        PyVec3 {
            inner: Vec3::new(-1.0, 0.0, 0.0),
        }
    }

    #[classattr]
    fn RIGHT() -> PyVec3 {
        PyVec3 {
            inner: Vec3::new(1.0, 0.0, 0.0),
        }
    }

    #[classattr]
    fn FORWARD() -> PyVec3 {
        PyVec3 {
            inner: Vec3::new(0.0, 0.0, 1.0),
        }
    }

    #[classattr]
    fn BACK() -> PyVec3 {
        PyVec3 {
            inner: Vec3::new(0.0, 0.0, -1.0),
        }
    }

    // Python operator overloads
    fn __add__(&self, other: &Bound<'_, pyo3::PyAny>) -> PyResult<PyVec3> {
        if let Ok(vec) = other.extract::<PyVec3>() {
            Ok(self.add(&vec))
        } else if let Ok(scalar) = other.extract::<f32>() {
            Ok(self.add_scalar(scalar))
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err(
                "unsupported operand type(s) for +: 'Vec3' and unknown type",
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
                "unsupported operand type(s) for -: 'Vec3' and unknown type",
            ))
        }
    }

    fn __rsub__(&self, other: &Bound<'_, pyo3::PyAny>) -> PyResult<PyVec3> {
        if let Ok(scalar) = other.extract::<f32>() {
            Ok(PyVec3 {
                inner: Vec3::new(
                    scalar - self.inner.x(),
                    scalar - self.inner.y(),
                    scalar - self.inner.z(),
                ),
            })
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err(
                "unsupported operand type(s) for -: unknown type and 'Vec3'",
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
                "unsupported operand type(s) for *: 'Vec3' and unknown type",
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
                "unsupported operand type(s) for /: 'Vec3' and unknown type",
            ))
        }
    }

    fn __rtruediv__(&self, other: &Bound<'_, pyo3::PyAny>) -> PyResult<PyVec3> {
        if let Ok(scalar) = other.extract::<f32>() {
            Ok(PyVec3 {
                inner: Vec3::new(
                    scalar / self.inner.x(),
                    scalar / self.inner.y(),
                    scalar / self.inner.z(),
                ),
            })
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err(
                "unsupported operand type(s) for /: unknown type and 'Vec3'",
            ))
        }
    }
}
