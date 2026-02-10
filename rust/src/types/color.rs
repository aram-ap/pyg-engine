#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            r: r.clamp(0.0, 1.0),
            g: g.clamp(0.0, 1.0),
            b: b.clamp(0.0, 1.0),
            a: a.clamp(0.0, 1.0),
        }
    }

    /// Create a color from RGB values in the range 0-255
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        let r_clamped = r.clamp(0, 255);
        let g_clamped = g.clamp(0, 255);
        let b_clamped = b.clamp(0, 255);
        Self {
            r: r_clamped as f32 / 255.0,
            g: g_clamped as f32 / 255.0,
            b: b_clamped as f32 / 255.0,
            a: 1.0,
        }
    }

    /// Create a color from RGBA values in the range 0-255
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        let r_clamped = r.clamp(0, 255);
        let g_clamped = g.clamp(0, 255);
        let b_clamped = b.clamp(0, 255);
        let a_clamped = a.clamp(0, 255);
        Self {
            r: r_clamped as f32 / 255.0,
            g: g_clamped as f32 / 255.0,
            b: b_clamped as f32 / 255.0,
            a: a_clamped as f32 / 255.0,
        }
    }

    /// Create a color from a hex string (e.g., "#FF0000" or "FF0000")
    pub fn from_hex(hex: &str) -> Self {
        let hex = hex.trim_start_matches('#');
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
        Self::rgb(r, g, b)
    }

    /// Create a color from HSV values
    /// - h: hue in degrees (0-360), will be wrapped
    /// - s: saturation (0.0-1.0), will be clamped
    /// - v: value/brightness (0.0-1.0), will be clamped
    /// - a: alpha (0.0-1.0), will be clamped
    pub fn from_hsv(h: f32, s: f32, v: f32, a: f32) -> Self {
        let h = h % 360.0;
        let s = s.clamp(0.0, 1.0);
        let v = v.clamp(0.0, 1.0);

        let a_clamped = a.clamp(0.0, 1.0);

        let c = v * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = v - c;

        let (r, g, b) = if h < 60.0 {
            (c, x, 0.0)
        } else if h < 120.0 {
            (x, c, 0.0)
        } else if h < 180.0 {
            (0.0, c, x)
        } else if h < 240.0 {
            (0.0, x, c)
        } else if h < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        Self::new(r + m, g + m, b + m, a_clamped)
    }

    // Getters
    pub fn r(&self) -> f32 {
        self.r
    }
    pub fn g(&self) -> f32 {
        self.g
    }
    pub fn b(&self) -> f32 {
        self.b
    }
    pub fn a(&self) -> f32 {
        self.a
    }

    // Setters
    pub fn set_r(&self, r: f32) -> Self {
        Self {
            r: r.clamp(0.0, 1.0),
            ..*self
        }
    }

    pub fn set_g(&self, g: f32) -> Self {
        Self {
            g: g.clamp(0.0, 1.0),
            ..*self
        }
    }

    pub fn set_b(&self, b: f32) -> Self {
        Self {
            b: b.clamp(0.0, 1.0),
            ..*self
        }
    }

    pub fn set_a(&self, a: f32) -> Self {
        Self {
            a: a.clamp(0.0, 1.0),
            ..*self
        }
    }

    pub fn with_alpha(&self, a: f32) -> Self {
        Self {
            a: a.clamp(0.0, 1.0),
            ..*self
        }
    }

    pub fn lerp(&self, other: &Color, t: f32) -> Self {
        Self::new(
            self.r + (other.r - self.r) * t,
            self.g + (other.g - self.g) * t,
            self.b + (other.b - self.b) * t,
            self.a + (other.a - self.a) * t,
        )
    }

    /// Check if two colors are approximately equal within an epsilon tolerance
    /// Default epsilon is 1e-5 (0.00001)
    pub fn approx_eq(&self, other: &Color, epsilon: f32) -> bool {
        (self.r - other.r).abs() < epsilon
            && (self.g - other.g).abs() < epsilon
            && (self.b - other.b).abs() < epsilon
            && (self.a - other.a).abs() < epsilon
    }

    /// Check if two colors are approximately equal with default epsilon (1e-5)
    pub fn approx_eq_default(&self, other: &Color) -> bool {
        self.approx_eq(other, 1e-5)
    }

    pub fn to_string(&self) -> String {
        format!(
            "Color(r: {}, g: {}, b: {}, a: {})",
            self.r, self.g, self.b, self.a
        )
    }

    #[allow(dead_code)]
    pub fn from_string(string: &str) -> Self {
        let parts = string.split(',').collect::<Vec<&str>>();
        Self::new(
            parts[0].parse().unwrap_or(0.0),
            parts[1].parse().unwrap_or(0.0),
            parts[2].parse().unwrap_or(0.0),
            parts[3].parse().unwrap_or(1.0),
        )
    }

    /// Convert this Color to a wgpu::Color
    pub fn to_wgpu(&self) -> wgpu::Color {
        wgpu::Color {
            r: self.r as f64,
            g: self.g as f64,
            b: self.b as f64,
            a: self.a as f64,
        }
    }

    // ========== Color Constants ==========

    // Basic Colors
    #[allow(dead_code)]
    pub const TRANSPARENT: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    };
    #[allow(dead_code)]
    pub const BLACK: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const WHITE: Color = Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const GRAY: Color = Color {
        r: 0.5,
        g: 0.5,
        b: 0.5,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const GREY: Color = Color {
        r: 0.5,
        g: 0.5,
        b: 0.5,
        a: 1.0,
    };

    // Primary Colors
    #[allow(dead_code)]
    pub const RED: Color = Color {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const GREEN: Color = Color {
        r: 0.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const BLUE: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };

    // Secondary Colors
    #[allow(dead_code)]
    pub const YELLOW: Color = Color {
        r: 1.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const CYAN: Color = Color {
        r: 0.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const MAGENTA: Color = Color {
        r: 1.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };

    // Shades of Gray
    #[allow(dead_code)]
    pub const DARK_GRAY: Color = Color {
        r: 0.25,
        g: 0.25,
        b: 0.25,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const DARK_GREY: Color = Color {
        r: 0.25,
        g: 0.25,
        b: 0.25,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const LIGHT_GRAY: Color = Color {
        r: 0.75,
        g: 0.75,
        b: 0.75,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const LIGHT_GREY: Color = Color {
        r: 0.75,
        g: 0.75,
        b: 0.75,
        a: 1.0,
    };

    // Orange/Brown Tones
    #[allow(dead_code)]
    pub const ORANGE: Color = Color {
        r: 1.0,
        g: 0.647,
        b: 0.0,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const DARK_ORANGE: Color = Color {
        r: 1.0,
        g: 0.549,
        b: 0.0,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const BROWN: Color = Color {
        r: 0.647,
        g: 0.165,
        b: 0.165,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const TAN: Color = Color {
        r: 0.824,
        g: 0.706,
        b: 0.549,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const BEIGE: Color = Color {
        r: 0.961,
        g: 0.961,
        b: 0.863,
        a: 1.0,
    };

    // Pink/Purple Tones
    #[allow(dead_code)]
    pub const PINK: Color = Color {
        r: 1.0,
        g: 0.753,
        b: 0.796,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const HOT_PINK: Color = Color {
        r: 1.0,
        g: 0.412,
        b: 0.706,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const PURPLE: Color = Color {
        r: 0.502,
        g: 0.0,
        b: 0.502,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const VIOLET: Color = Color {
        r: 0.933,
        g: 0.51,
        b: 0.933,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const INDIGO: Color = Color {
        r: 0.294,
        g: 0.0,
        b: 0.51,
        a: 1.0,
    };

    // Red Tones
    #[allow(dead_code)]
    pub const CRIMSON: Color = Color {
        r: 0.863,
        g: 0.078,
        b: 0.235,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const MAROON: Color = Color {
        r: 0.502,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const DARK_RED: Color = Color {
        r: 0.545,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const CORAL: Color = Color {
        r: 1.0,
        g: 0.498,
        b: 0.314,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const SALMON: Color = Color {
        r: 0.98,
        g: 0.502,
        b: 0.447,
        a: 1.0,
    };

    // Green Tones
    #[allow(dead_code)]
    pub const LIME: Color = Color {
        r: 0.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const DARK_GREEN: Color = Color {
        r: 0.0,
        g: 0.392,
        b: 0.0,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const FOREST_GREEN: Color = Color {
        r: 0.133,
        g: 0.545,
        b: 0.133,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const OLIVE: Color = Color {
        r: 0.502,
        g: 0.502,
        b: 0.0,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const SEA_GREEN: Color = Color {
        r: 0.18,
        g: 0.545,
        b: 0.341,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const MINT: Color = Color {
        r: 0.596,
        g: 1.0,
        b: 0.596,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const TEAL: Color = Color {
        r: 0.0,
        g: 0.502,
        b: 0.502,
        a: 1.0,
    };

    // Blue Tones
    #[allow(dead_code)]
    pub const NAVY: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.502,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const DARK_BLUE: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.545,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const SKY_BLUE: Color = Color {
        r: 0.529,
        g: 0.808,
        b: 0.922,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const LIGHT_BLUE: Color = Color {
        r: 0.678,
        g: 0.847,
        b: 0.902,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const ROYAL_BLUE: Color = Color {
        r: 0.255,
        g: 0.412,
        b: 0.882,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const STEEL_BLUE: Color = Color {
        r: 0.275,
        g: 0.51,
        b: 0.706,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const TURQUOISE: Color = Color {
        r: 0.251,
        g: 0.878,
        b: 0.816,
        a: 1.0,
    };

    // Yellow/Gold Tones
    #[allow(dead_code)]
    pub const GOLD: Color = Color {
        r: 1.0,
        g: 0.843,
        b: 0.0,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const KHAKI: Color = Color {
        r: 0.941,
        g: 0.902,
        b: 0.549,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const LEMON: Color = Color {
        r: 1.0,
        g: 0.969,
        b: 0.0,
        a: 1.0,
    };

    // Special Colors
    #[allow(dead_code)]
    pub const SILVER: Color = Color {
        r: 0.753,
        g: 0.753,
        b: 0.753,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const LAVENDER: Color = Color {
        r: 0.902,
        g: 0.902,
        b: 0.98,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const PEACH: Color = Color {
        r: 1.0,
        g: 0.894,
        b: 0.769,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const CREAM: Color = Color {
        r: 1.0,
        g: 0.992,
        b: 0.816,
        a: 1.0,
    };

    // Web Standard Colors
    #[allow(dead_code)]
    pub const ALICE_BLUE: Color = Color {
        r: 0.941,
        g: 0.973,
        b: 1.0,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const AQUAMARINE: Color = Color {
        r: 0.498,
        g: 1.0,
        b: 0.831,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const AZURE: Color = Color {
        r: 0.941,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const CHARTREUSE: Color = Color {
        r: 0.498,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const CHOCOLATE: Color = Color {
        r: 0.824,
        g: 0.412,
        b: 0.118,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const CORNFLOWER_BLUE: Color = Color {
        r: 0.392,
        g: 0.584,
        b: 0.929,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const FIREBRICK: Color = Color {
        r: 0.698,
        g: 0.133,
        b: 0.133,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const GAINSBORO: Color = Color {
        r: 0.863,
        g: 0.863,
        b: 0.863,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const HONEYDEW: Color = Color {
        r: 0.941,
        g: 1.0,
        b: 0.941,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const IVORY: Color = Color {
        r: 1.0,
        g: 1.0,
        b: 0.941,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const LAWN_GREEN: Color = Color {
        r: 0.486,
        g: 0.988,
        b: 0.0,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const LINEN: Color = Color {
        r: 0.98,
        g: 0.941,
        b: 0.902,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const MIDNIGHT_BLUE: Color = Color {
        r: 0.098,
        g: 0.098,
        b: 0.439,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const MISTY_ROSE: Color = Color {
        r: 1.0,
        g: 0.894,
        b: 0.882,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const ORCHID: Color = Color {
        r: 0.855,
        g: 0.439,
        b: 0.839,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const PERU: Color = Color {
        r: 0.804,
        g: 0.522,
        b: 0.247,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const PLUM: Color = Color {
        r: 0.867,
        g: 0.627,
        b: 0.867,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const SIENNA: Color = Color {
        r: 0.627,
        g: 0.322,
        b: 0.176,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const SNOW: Color = Color {
        r: 1.0,
        g: 0.98,
        b: 0.98,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const SPRING_GREEN: Color = Color {
        r: 0.0,
        g: 1.0,
        b: 0.498,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const TOMATO: Color = Color {
        r: 1.0,
        g: 0.388,
        b: 0.278,
        a: 1.0,
    };
    #[allow(dead_code)]
    pub const WHEAT: Color = Color {
        r: 0.961,
        g: 0.871,
        b: 0.702,
        a: 1.0,
    };
}

// ========== Operator Overloads ==========

use std::ops::{Add, Div, Mul, Sub};

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(
            self.r + other.r,
            self.g + other.g,
            self.b + other.b,
            self.a + other.a,
        )
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(
            self.r - other.r,
            self.g - other.g,
            self.b - other.b,
            self.a - other.a,
        )
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::new(
            self.r * other.r,
            self.g * other.g,
            self.b * other.b,
            self.a * other.a,
        )
    }
}

impl Div for Color {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self::new(
            if other.r != 0.0 {
                self.r / other.r
            } else {
                0.0
            },
            if other.g != 0.0 {
                self.g / other.g
            } else {
                0.0
            },
            if other.b != 0.0 {
                self.b / other.b
            } else {
                0.0
            },
            if other.a != 0.0 {
                self.a / other.a
            } else {
                0.0
            },
        )
    }
}
