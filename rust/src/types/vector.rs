#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector<T, const N: usize> {
    data: [T; N],
}

pub type Vec2 = Vector<f32, 2>;
pub type Vec3 = Vector<f32, 3>;
pub type Vec4 = Vector<f32, 4>;

impl<T: Copy, const N: usize> Vector<T, N> {
    pub fn from_array(data: [T; N]) -> Self {
        Self { data }
    }

    pub fn data(&self) -> &[T; N] {
        &self.data
    }
}

impl<T, const N: usize> Vector<T, N>
where
    T: Copy
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>,
{
    pub fn add(&self, other: &Self) -> Self {
        let mut data = self.data;
        for i in 0..N {
            data[i] = data[i] + other.data[i];
        }
        Self { data }
    }

    pub fn add_scalar(&self, scalar: T) -> Self {
        let mut data = self.data;
        for i in 0..N {
            data[i] = data[i] + scalar;
        }
        Self { data }
    }

    pub fn subtract(&self, other: &Self) -> Self {
        let mut data = self.data;
        for i in 0..N {
            data[i] = data[i] - other.data[i];
        }
        Self { data }
    }

    pub fn subtract_scalar(&self, scalar: T) -> Self {
        let mut data = self.data;
        for i in 0..N {
            data[i] = data[i] - scalar;
        }
        Self { data }
    }

    pub fn multiply(&self, other: &Self) -> Self {
        let mut data = self.data;
        for i in 0..N {
            data[i] = data[i] * other.data[i];
        }
        Self { data }
    }

    pub fn multiply_scalar(&self, scalar: T) -> Self {
        let mut data = self.data;
        for i in 0..N {
            data[i] = data[i] * scalar;
        }
        Self { data }
    }

    /// Component-wise division. Division by zero follows IEEE float behavior (Inf/NaN).
    /// Use `try_divide()` for f32/f64 if you need explicit zero-checking.
    pub fn divide(&self, other: &Self) -> Self {
        let mut data = self.data;
        for i in 0..N {
            data[i] = data[i] / other.data[i];
        }
        Self { data }
    }

    /// Scalar division. Division by zero follows IEEE float behavior (Inf/NaN).
    /// Use `try_divide_scalar()` for f32/f64 if you need explicit zero-checking.
    pub fn divide_scalar(&self, scalar: T) -> Self {
        let mut data = self.data;
        for i in 0..N {
            data[i] = data[i] / scalar;
        }
        Self { data }
    }
}

impl<const N: usize> Vector<f32, N> {
    pub fn length(&self) -> f32 {
        let mut sum = 0.0;
        for i in 0..N {
            sum += self.data[i] * self.data[i];
        }
        sum.sqrt()
    }

    pub fn normalize(&self) -> Self {
        let len = self.length();
        let mut data = self.data;
        for i in 0..N {
            data[i] /= len;
        }
        Self { data }
    }

    pub fn dot(&self, other: &Self) -> f32 {
        let mut sum = 0.0;
        for i in 0..N {
            sum += self.data[i] * other.data[i];
        }
        sum
    }

    pub fn angle(&self, other: &Self) -> f32 {
        self.dot(other) / (self.length() * other.length())
    }

    pub fn distance(&self, other: &Self) -> f32 {
        let mut sum = 0.0;
        for i in 0..N {
            let d = self.data[i] - other.data[i];
            sum += d * d;
        }
        sum
    }

    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        let mut data = self.data;
        for i in 0..N {
            data[i] = self.data[i] + (other.data[i] - self.data[i]) * t;
        }
        Self { data }
    }

    pub fn nlerp(&self, other: &Self, t: f32) -> Self {
        self.lerp(other, t).normalize()
    }

    pub fn to_string(&self) -> String {
        let parts: Vec<String> = self.data.iter().map(|v| v.to_string()).collect();
        format!("({})", parts.join(", "))
    }

    pub fn from_string(string: &str) -> Self {
        let cleaned = string.trim_matches(|c| c == '(' || c == ')' || c == ' ');
        let parts: Vec<&str> = cleaned.split(',').collect();
        let mut data = [0.0; N];
        for (i, part) in parts.iter().enumerate().take(N) {
            data[i] = part.trim().parse().unwrap_or(0.0);
        }
        Self { data }
    }

    /// Checked division that returns None if any component of `other` is zero.
    /// Use this when you need explicit zero-checking; otherwise, use `divide()` which follows IEEE behavior (Inf/NaN).
    pub fn try_divide(&self, other: &Self) -> Option<Self> {
        for i in 0..N {
            if other.data[i] == 0.0 {
                return None;
            }
        }
        Some(self.divide(other))
    }

    /// Checked scalar division that returns None if `scalar` is zero.
    /// Use this when you need explicit zero-checking; otherwise, use `divide_scalar()` which follows IEEE behavior (Inf/NaN).
    pub fn try_divide_scalar(&self, scalar: f32) -> Option<Self> {
        if scalar == 0.0 {
            None
        } else {
            Some(self.divide_scalar(scalar))
        }
    }

    /// Checked normalize that returns None if the vector has zero length.
    pub fn try_normalize(&self) -> Option<Self> {
        let len = self.length();
        if len == 0.0 {
            None
        } else {
            Some(self.divide_scalar(len))
        }
    }

    /// Component-wise division with debug-only zero checks.
    /// In debug builds, asserts that no divisor components are zero.
    /// In release builds, follows IEEE float behavior (Inf/NaN) for zero division.
    #[inline]
    pub fn divide_checked(&self, other: &Self) -> Self {
        debug_assert!(
            !other.data.iter().any(|&x| x == 0.0),
            "Vector::divide_checked: Division by zero component"
        );
        self.divide(other)
    }

    /// Scalar division with debug-only zero check.
    /// In debug builds, asserts that the scalar is not zero.
    /// In release builds, follows IEEE float behavior (Inf/NaN) for zero division.
    #[inline]
    pub fn divide_scalar_checked(&self, scalar: f32) -> Self {
        debug_assert!(
            scalar != 0.0,
            "Vector::divide_scalar_checked: Division by zero"
        );
        self.divide_scalar(scalar)
    }

    /// Normalize with debug-only zero length check.
    /// In debug builds, asserts that the vector has non-zero length.
    /// In release builds, may produce NaN/Inf for zero-length vectors.
    #[inline]
    pub fn normalize_checked(&self) -> Self {
        let len = self.length();
        debug_assert!(
            len > 0.0,
            "Vector::normalize_checked: Cannot normalize zero-length vector"
        );
        self.divide_scalar(len)
    }
}

impl Vector<f32, 2> {
    pub fn new(x: f32, y: f32) -> Self {
        Self { data: [x, y] }
    }

    pub fn x(&self) -> f32 {
        self.data[0]
    }

    pub fn y(&self) -> f32 {
        self.data[1]
    }

    pub fn cross(&self, other: &Self) -> f32 {
        self.data[0] * other.data[1] - self.data[1] * other.data[0]
    }
}

impl Vector<f32, 3> {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { data: [x, y, z] }
    }

    pub fn x(&self) -> f32 {
        self.data[0]
    }

    pub fn y(&self) -> f32 {
        self.data[1]
    }

    pub fn z(&self) -> f32 {
        self.data[2]
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            data: [
                self.y() * other.z() - self.z() * other.y(),
                self.z() * other.x() - self.x() * other.z(),
                self.x() * other.y() - self.y() * other.x(),
            ],
        }
    }
}
