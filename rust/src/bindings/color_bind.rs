use crate::types::color::Color as RustColor;
use pyo3::prelude::*;

// ========== Color Bindings ==========

/// RGBA color representation with float components in range [0.0, 1.0].
///
/// Colors in PyG Engine use **floating-point RGBA values** where each component
/// (red, green, blue, alpha) is in the range **[0.0, 1.0]**. The engine provides
/// convenient factory methods for creating colors from various formats.
///
/// # Color Creation Methods
///
/// ## From Integer RGB/RGBA (0-255)
/// ```python
/// # RGB (alpha defaults to 255)
/// red = Color.rgb(255, 0, 0)
/// semi_transparent_blue = Color.rgba(0, 0, 255, 128)
/// ```
///
/// ## From Float RGBA (0.0-1.0)
/// ```python
/// # Direct constructor
/// green = Color.new(0.0, 1.0, 0.0, 1.0)  # (r, g, b, a)
/// ```
///
/// ## From Hex String
/// ```python
/// # With or without alpha channel
/// orange = Color.from_hex("#FF8000")       # RGB
/// semi_orange = Color.from_hex("#FF800080")  # RGBA
/// ```
///
/// ## From HSV (Hue, Saturation, Value)
/// ```python
/// # H: 0-360 degrees, S/V/A: 0.0-1.0
/// red = Color.from_hsv(0, 1.0, 1.0, 1.0)       # Pure red
/// yellow = Color.from_hsv(60, 1.0, 1.0, 1.0)   # Pure yellow
/// cyan = Color.from_hsv(180, 1.0, 1.0, 1.0)    # Pure cyan
/// ```
///
/// # Predefined Color Constants
///
/// The engine provides predefined colors accessible as class attributes:
///
/// **Basic colors:**
/// `BLACK`, `WHITE`, `RED`, `GREEN`, `BLUE`, `YELLOW`, `CYAN`, `MAGENTA`
///
/// **Additional colors:**
/// `ORANGE`, `PURPLE`, `PINK`, `BROWN`, `LIME`
///
/// **Grayscale:**
/// `GRAY`/`GREY`, `DARK_GRAY`/`DARK_GREY`, `LIGHT_GRAY`/`LIGHT_GREY`
///
/// **Special:**
/// `TRANSPARENT` (fully transparent black)
///
/// ```python
/// # Using predefined colors
/// circle_color = Color.RED
/// background = Color.DARK_GRAY
/// transparent_overlay = Color.TRANSPARENT
/// ```
///
/// # Color Components
///
/// Access and modify RGBA components:
/// ```python
/// color = Color.rgb(255, 128, 64)
///
/// # Get components (returns 0.0-1.0)
/// r = color.r  # Red component
/// g = color.g  # Green component
/// b = color.b  # Blue component
/// a = color.a  # Alpha component
///
/// # Modify components (returns new Color)
/// brighter = color.set_r(1.0)
/// half_alpha = color.with_alpha(0.5)
/// ```
///
/// # Color Operations
///
/// ```python
/// # Color interpolation
/// start = Color.RED
/// end = Color.BLUE
/// middle = start.lerp(end, 0.5)  # Purple (halfway between)
///
/// # Arithmetic (component-wise)
/// c1 = Color.rgb(100, 150, 200)
/// c2 = Color.rgb(50, 50, 50)
/// result = c1 + c2  # Adds each component
/// ```
///
/// # Important: Value Ranges
///
/// - **Internal storage**: RGBA values are **[0.0, 1.0]** floats
/// - **`rgb()` / `rgba()` methods**: Accept integers **[0, 255]** for convenience
/// - **`new()` constructor**: Accepts floats **[0.0, 1.0]**
/// - **`from_hsv()`**: Hue is **[0-360]** degrees, S/V/A are **[0.0-1.0]**
/// - **Component getters** (`r`, `g`, `b`, `a`): Return **[0.0, 1.0]** floats
///
/// # Example
/// ```python
/// import pyg_engine as pyg
///
/// # Create colors using different methods
/// color1 = pyg.Color.rgb(255, 128, 0)          # Orange from RGB
/// color2 = pyg.Color.from_hex("#FF8000")       # Orange from hex
/// color3 = pyg.Color.from_hsv(30, 1.0, 1.0, 1.0)  # Orange from HSV
/// color4 = pyg.Color.ORANGE                    # Predefined orange
///
/// # All create the same color
/// print(color1)  # Color(1.0, 0.5, 0.0, 1.0)
///
/// # Create semi-transparent red
/// red_alpha = pyg.Color.rgba(255, 0, 0, 128)   # 50% transparent
/// # Or
/// red_alpha2 = pyg.Color.RED.with_alpha(0.5)
///
/// # Interpolate between colors
/// start = pyg.Color.BLUE
/// end = pyg.Color.GREEN
/// for i in range(11):
///     t = i / 10.0
///     blended = start.lerp(end, t)
///     print(f"Step {i}: {blended}")
/// ```
///
/// # See Also
/// - `DrawCommand` methods - All accept `Color` parameters
/// - `examples/python_rendering_showcase_demo.py` - Color usage examples
#[pyclass(name = "Color")]
#[derive(Clone)]
pub struct PyColor {
    pub(crate) inner: RustColor,
}

#[pymethods]
#[allow(non_snake_case)]
impl PyColor {
    /// Create a color from RGBA float values in range [0.0, 1.0].
    ///
    /// # Arguments
    /// * `r` - Red component (0.0 to 1.0)
    /// * `g` - Green component (0.0 to 1.0)
    /// * `b` - Blue component (0.0 to 1.0)
    /// * `a` - Alpha/opacity component (0.0 = transparent, 1.0 = opaque)
    ///
    /// # Example
    /// ```python
    /// # Create semi-transparent orange
    /// color = Color.new(1.0, 0.5, 0.0, 0.75)
    /// ```
    #[new]
    fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            inner: RustColor::new(r, g, b, a),
        }
    }

    /// Create an opaque color from RGB integer values in range [0, 255].
    ///
    /// Convenience method for creating colors with integer RGB values (0-255).
    /// Alpha defaults to 255 (fully opaque).
    ///
    /// # Arguments
    /// * `r` - Red component (0 to 255)
    /// * `g` - Green component (0 to 255)
    /// * `b` - Blue component (0 to 255)
    ///
    /// # Example
    /// ```python
    /// red = Color.rgb(255, 0, 0)
    /// orange = Color.rgb(255, 165, 0)
    /// custom = Color.rgb(127, 63, 200)
    /// ```
    #[staticmethod]
    fn rgb(r: u8, g: u8, b: u8) -> PyColor {
        PyColor {
            inner: RustColor::rgb(r, g, b),
        }
    }

    /// Create a color from RGBA integer values in range [0, 255].
    ///
    /// Allows specifying alpha/opacity using integer values (0-255).
    ///
    /// # Arguments
    /// * `r` - Red component (0 to 255)
    /// * `g` - Green component (0 to 255)
    /// * `b` - Blue component (0 to 255)
    /// * `a` - Alpha component (0 = transparent, 255 = opaque)
    ///
    /// # Example
    /// ```python
    /// # 50% transparent red
    /// semi_red = Color.rgba(255, 0, 0, 128)
    ///
    /// # 25% transparent blue
    /// faint_blue = Color.rgba(0, 0, 255, 64)
    /// ```
    #[staticmethod]
    fn rgba(r: u8, g: u8, b: u8, a: u8) -> PyColor {
        PyColor {
            inner: RustColor::rgba(r, g, b, a),
        }
    }

    /// Create a color from a hex string.
    ///
    /// Supports standard web color formats with or without alpha channel.
    ///
    /// # Arguments
    /// * `hex` - Hex color string in format:
    ///   - `"#RRGGBB"` - RGB (alpha defaults to FF)
    ///   - `"#RRGGBBAA"` - RGBA (includes alpha)
    ///   - `"RRGGBB"` - Without hash prefix also supported
    ///
    /// # Example
    /// ```python
    /// red = Color.from_hex("#FF0000")
    /// orange = Color.from_hex("#FF8000")
    /// semi_blue = Color.from_hex("#0000FF80")  # 50% transparent
    ///
    /// # Without # prefix also works
    /// green = Color.from_hex("00FF00")
    /// ```
    #[staticmethod]
    fn from_hex(hex: &str) -> PyColor {
        PyColor {
            inner: RustColor::from_hex(hex),
        }
    }

    /// Create a color from HSV (Hue, Saturation, Value) color space.
    ///
    /// HSV is often more intuitive than RGB for generating colors programmatically,
    /// especially for color wheels, rainbows, or gradients.
    ///
    /// # Arguments
    /// * `h` - Hue in degrees (0-360):
    ///   - 0° = Red
    ///   - 60° = Yellow
    ///   - 120° = Green
    ///   - 180° = Cyan
    ///   - 240° = Blue
    ///   - 300° = Magenta
    ///   - 360° = Red (wraps around)
    /// * `s` - Saturation (0.0 to 1.0):
    ///   - 0.0 = Grayscale (no color)
    ///   - 1.0 = Full saturation (vibrant color)
    /// * `v` - Value/Brightness (0.0 to 1.0):
    ///   - 0.0 = Black
    ///   - 1.0 = Full brightness
    /// * `a` - Alpha/opacity (0.0 to 1.0):
    ///   - 0.0 = Transparent
    ///   - 1.0 = Opaque
    ///
    /// # Example
    /// ```python
    /// # Pure colors
    /// red = Color.from_hsv(0, 1.0, 1.0, 1.0)
    /// green = Color.from_hsv(120, 1.0, 1.0, 1.0)
    /// blue = Color.from_hsv(240, 1.0, 1.0, 1.0)
    ///
    /// # Pastel colors (low saturation)
    /// pastel_pink = Color.from_hsv(330, 0.3, 1.0, 1.0)
    ///
    /// # Dark colors (low value)
    /// dark_red = Color.from_hsv(0, 1.0, 0.3, 1.0)
    ///
    /// # Rainbow gradient
    /// import pyg_engine as pyg
    /// for i in range(360):
    ///     hue = i
    ///     color = pyg.Color.from_hsv(hue, 1.0, 1.0, 1.0)
    ///     # Draw with color
    /// ```
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

    /// Create a new color with a different red component.
    ///
    /// # Arguments
    /// * `r` - New red value (0.0 to 1.0)
    ///
    /// # Returns
    /// New Color with modified red component
    ///
    /// # Example
    /// ```python
    /// color = pyg.Color.rgb(100, 200, 50)
    /// brighter = color.set_r(1.0)  # Max red
    /// ```
    fn set_r(&self, r: f32) -> PyColor {
        PyColor {
            inner: self.inner.set_r(r),
        }
    }

    /// Create a new color with a different green component.
    ///
    /// # Arguments
    /// * `g` - New green value (0.0 to 1.0)
    ///
    /// # Returns
    /// New Color with modified green component
    ///
    /// # Example
    /// ```python
    /// color = pyg.Color.rgb(100, 200, 50)
    /// greener = color.set_g(1.0)  # Max green
    /// ```
    fn set_g(&self, g: f32) -> PyColor {
        PyColor {
            inner: self.inner.set_g(g),
        }
    }

    /// Create a new color with a different blue component.
    ///
    /// # Arguments
    /// * `b` - New blue value (0.0 to 1.0)
    ///
    /// # Returns
    /// New Color with modified blue component
    ///
    /// # Example
    /// ```python
    /// color = pyg.Color.rgb(100, 200, 50)
    /// bluer = color.set_b(1.0)  # Max blue
    /// ```
    fn set_b(&self, b: f32) -> PyColor {
        PyColor {
            inner: self.inner.set_b(b),
        }
    }

    /// Create a new color with a different alpha component.
    ///
    /// # Arguments
    /// * `a` - New alpha value (0.0 to 1.0)
    ///
    /// # Returns
    /// New Color with modified alpha component
    ///
    /// # Example
    /// ```python
    /// color = pyg.Color.RED
    /// transparent = color.set_a(0.5)  # 50% transparent
    /// ```
    fn set_a(&self, a: f32) -> PyColor {
        PyColor {
            inner: self.inner.set_a(a),
        }
    }

    /// Create a new color with a different alpha value.
    ///
    /// Returns a copy of this color with the specified alpha, leaving RGB unchanged.
    ///
    /// # Arguments
    /// * `a` - New alpha value (0.0 to 1.0)
    ///
    /// # Returns
    /// New Color with modified alpha
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// # Start with opaque red
    /// red = pyg.Color.RED  # alpha = 1.0
    ///
    /// # Create semi-transparent versions
    /// red_75 = red.with_alpha(0.75)  # 75% opaque
    /// red_50 = red.with_alpha(0.50)  # 50% opaque
    /// red_25 = red.with_alpha(0.25)  # 25% opaque
    ///
    /// # Draw overlapping circles with transparency
    /// engine.draw_circle(100, 100, 50, red_75, filled=True)
    /// engine.draw_circle(130, 100, 50, red_50, filled=True)
    /// engine.draw_circle(160, 100, 50, red_25, filled=True)
    /// ```
    fn with_alpha(&self, a: f32) -> PyColor {
        PyColor {
            inner: self.inner.with_alpha(a),
        }
    }

    /// Linearly interpolate between two colors.
    ///
    /// Returns a color that is a blend between this color and another,
    /// controlled by parameter `t`. Useful for smooth color transitions,
    /// gradients, and animations.
    ///
    /// # Arguments
    /// * `other` - Target color to interpolate towards
    /// * `t` - Interpolation factor (0.0 to 1.0):
    ///   - 0.0 = This color (start)
    ///   - 0.5 = Halfway between
    ///   - 1.0 = Other color (end)
    ///
    /// # Returns
    /// Interpolated color
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// engine = pyg.Engine()
    /// engine.start_manual()
    ///
    /// start_color = pyg.Color.BLUE
    /// end_color = pyg.Color.RED
    ///
    /// while engine.poll_events():
    ///     engine.clear_draw_commands()
    ///
    ///     # Draw gradient using lerp
    ///     for i in range(10):
    ///         t = i / 9.0  # 0.0 to 1.0
    ///         color = start_color.lerp(end_color, t)
    ///         engine.draw_rectangle(i * 80, 100, 80, 200, color, filled=True)
    ///
    ///     engine.update()
    ///     engine.render()
    /// ```
    ///
    /// # Color Pulsing Animation
    /// ```python
    /// import math
    /// t = engine.elapsed_time
    /// pulse = (math.sin(t * 2.0) + 1.0) / 2.0  # 0.0 to 1.0
    /// color = pyg.Color.WHITE.lerp(pyg.Color.RED, pulse)
    /// engine.draw_circle(400, 300, 50, color, filled=True)
    /// ```
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
