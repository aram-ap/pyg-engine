#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Create a color from RGB values in the range 0-255
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: 1.0,
        }
    }

    /// Create a color from RGBA values in the range 0-255
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
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
    
    // Getters
    pub fn r(&self) -> f32 { self.r }
    pub fn g(&self) -> f32 { self.g }
    pub fn b(&self) -> f32 { self.b }
    pub fn a(&self) -> f32 { self.a }
    
    pub fn with_alpha(&self, a: f32) -> Self {
        Self { a, ..*self }
    }
    
    pub fn lerp(&self, other: &Color, t: f32) -> Self {
        Self {
            r: self.r + (other.r - self.r) * t,
            g: self.g + (other.g - self.g) * t,
            b: self.b + (other.b - self.b) * t,
            a: self.a + (other.a - self.a) * t,
        }
    }

    pub fn to_string(&self) -> String {
        format!("Color(r: {}, g: {}, b: {}, a: {})", self.r, self.g, self.b, self.a)
    }

    #[allow(dead_code)]
    pub fn from_string(string: &str) -> Self {
        let parts = string.split(',').collect::<Vec<&str>>();
        Self {
            r: parts[0].parse().unwrap(),
            g: parts[1].parse().unwrap(),
            b: parts[2].parse().unwrap(),
            a: parts[3].parse().unwrap(),
        }
    }

    // ========== Color Constants ==========
    
    // Basic Colors
    #[allow(dead_code)]
    pub const TRANSPARENT: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };
    #[allow(dead_code)]
    pub const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
    #[allow(dead_code)]
    pub const WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    #[allow(dead_code)]
    pub const GRAY: Color = Color { r: 0.5, g: 0.5, b: 0.5, a: 1.0 };
    #[allow(dead_code)]
    pub const GREY: Color = Color { r: 0.5, g: 0.5, b: 0.5, a: 1.0 };
    
    // Primary Colors
    #[allow(dead_code)]
    pub const RED: Color = Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
    #[allow(dead_code)]
    pub const GREEN: Color = Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };
    #[allow(dead_code)]
    pub const BLUE: Color = Color { r: 0.0, g: 0.0, b: 1.0, a: 1.0 };
    
    // Secondary Colors
    #[allow(dead_code)]
    pub const YELLOW: Color = Color { r: 1.0, g: 1.0, b: 0.0, a: 1.0 };
    #[allow(dead_code)]
    pub const CYAN: Color = Color { r: 0.0, g: 1.0, b: 1.0, a: 1.0 };
    #[allow(dead_code)]
    pub const MAGENTA: Color = Color { r: 1.0, g: 0.0, b: 1.0, a: 1.0 };
    
    // Shades of Gray
    #[allow(dead_code)]
    pub const DARK_GRAY: Color = Color { r: 0.25, g: 0.25, b: 0.25, a: 1.0 };
    #[allow(dead_code)]
    pub const DARK_GREY: Color = Color { r: 0.25, g: 0.25, b: 0.25, a: 1.0 };
    #[allow(dead_code)]
    pub const LIGHT_GRAY: Color = Color { r: 0.75, g: 0.75, b: 0.75, a: 1.0 };
    #[allow(dead_code)]
    pub const LIGHT_GREY: Color = Color { r: 0.75, g: 0.75, b: 0.75, a: 1.0 };
    
    // Orange/Brown Tones
    #[allow(dead_code)]
    pub const ORANGE: Color = Color { r: 1.0, g: 0.647, b: 0.0, a: 1.0 };
    #[allow(dead_code)]
    pub const DARK_ORANGE: Color = Color { r: 1.0, g: 0.549, b: 0.0, a: 1.0 };
    #[allow(dead_code)]
    pub const BROWN: Color = Color { r: 0.647, g: 0.165, b: 0.165, a: 1.0 };
    #[allow(dead_code)]
    pub const TAN: Color = Color { r: 0.824, g: 0.706, b: 0.549, a: 1.0 };
    #[allow(dead_code)]
    pub const BEIGE: Color = Color { r: 0.961, g: 0.961, b: 0.863, a: 1.0 };
    
    // Pink/Purple Tones
    #[allow(dead_code)]
    pub const PINK: Color = Color { r: 1.0, g: 0.753, b: 0.796, a: 1.0 };
    #[allow(dead_code)]
    pub const HOT_PINK: Color = Color { r: 1.0, g: 0.412, b: 0.706, a: 1.0 };
    #[allow(dead_code)]
    pub const PURPLE: Color = Color { r: 0.502, g: 0.0, b: 0.502, a: 1.0 };
    #[allow(dead_code)]
    pub const VIOLET: Color = Color { r: 0.933, g: 0.51, b: 0.933, a: 1.0 };
    #[allow(dead_code)]
    pub const INDIGO: Color = Color { r: 0.294, g: 0.0, b: 0.51, a: 1.0 };
    
    // Red Tones
    #[allow(dead_code)]
    pub const CRIMSON: Color = Color { r: 0.863, g: 0.078, b: 0.235, a: 1.0 };
    #[allow(dead_code)]
    pub const MAROON: Color = Color { r: 0.502, g: 0.0, b: 0.0, a: 1.0 };
    #[allow(dead_code)]
    pub const DARK_RED: Color = Color { r: 0.545, g: 0.0, b: 0.0, a: 1.0 };
    #[allow(dead_code)]
    pub const CORAL: Color = Color { r: 1.0, g: 0.498, b: 0.314, a: 1.0 };
    #[allow(dead_code)]
    pub const SALMON: Color = Color { r: 0.98, g: 0.502, b: 0.447, a: 1.0 };
    
    // Green Tones
    #[allow(dead_code)]
    pub const LIME: Color = Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };
    #[allow(dead_code)]
    pub const DARK_GREEN: Color = Color { r: 0.0, g: 0.392, b: 0.0, a: 1.0 };
    #[allow(dead_code)]
    pub const FOREST_GREEN: Color = Color { r: 0.133, g: 0.545, b: 0.133, a: 1.0 };
    #[allow(dead_code)]
    pub const OLIVE: Color = Color { r: 0.502, g: 0.502, b: 0.0, a: 1.0 };
    #[allow(dead_code)]
    pub const SEA_GREEN: Color = Color { r: 0.18, g: 0.545, b: 0.341, a: 1.0 };
    #[allow(dead_code)]
    pub const MINT: Color = Color { r: 0.596, g: 1.0, b: 0.596, a: 1.0 };
    #[allow(dead_code)]
    pub const TEAL: Color = Color { r: 0.0, g: 0.502, b: 0.502, a: 1.0 };
    
    // Blue Tones
    #[allow(dead_code)]
    pub const NAVY: Color = Color { r: 0.0, g: 0.0, b: 0.502, a: 1.0 };
    #[allow(dead_code)]
    pub const DARK_BLUE: Color = Color { r: 0.0, g: 0.0, b: 0.545, a: 1.0 };
    #[allow(dead_code)]
    pub const SKY_BLUE: Color = Color { r: 0.529, g: 0.808, b: 0.922, a: 1.0 };
    #[allow(dead_code)]
    pub const LIGHT_BLUE: Color = Color { r: 0.678, g: 0.847, b: 0.902, a: 1.0 };
    #[allow(dead_code)]
    pub const ROYAL_BLUE: Color = Color { r: 0.255, g: 0.412, b: 0.882, a: 1.0 };
    #[allow(dead_code)]
    pub const STEEL_BLUE: Color = Color { r: 0.275, g: 0.51, b: 0.706, a: 1.0 };
    #[allow(dead_code)]
    pub const TURQUOISE: Color = Color { r: 0.251, g: 0.878, b: 0.816, a: 1.0 };
    
    // Yellow/Gold Tones
    #[allow(dead_code)]
    pub const GOLD: Color = Color { r: 1.0, g: 0.843, b: 0.0, a: 1.0 };
    #[allow(dead_code)]
    pub const KHAKI: Color = Color { r: 0.941, g: 0.902, b: 0.549, a: 1.0 };
    #[allow(dead_code)]
    pub const LEMON: Color = Color { r: 1.0, g: 0.969, b: 0.0, a: 1.0 };
    
    // Special Colors
    #[allow(dead_code)]
    pub const SILVER: Color = Color { r: 0.753, g: 0.753, b: 0.753, a: 1.0 };
    #[allow(dead_code)]
    pub const LAVENDER: Color = Color { r: 0.902, g: 0.902, b: 0.98, a: 1.0 };
    #[allow(dead_code)]
    pub const PEACH: Color = Color { r: 1.0, g: 0.894, b: 0.769, a: 1.0 };
    #[allow(dead_code)]
    pub const CREAM: Color = Color { r: 1.0, g: 0.992, b: 0.816, a: 1.0 };
    
    // Web Standard Colors
    #[allow(dead_code)]
    pub const ALICE_BLUE: Color = Color { r: 0.941, g: 0.973, b: 1.0, a: 1.0 };
    #[allow(dead_code)]
    pub const AQUAMARINE: Color = Color { r: 0.498, g: 1.0, b: 0.831, a: 1.0 };
    #[allow(dead_code)]
    pub const AZURE: Color = Color { r: 0.941, g: 1.0, b: 1.0, a: 1.0 };
    #[allow(dead_code)]
    pub const CHARTREUSE: Color = Color { r: 0.498, g: 1.0, b: 0.0, a: 1.0 };
    #[allow(dead_code)]
    pub const CHOCOLATE: Color = Color { r: 0.824, g: 0.412, b: 0.118, a: 1.0 };
    #[allow(dead_code)]
    pub const CORNFLOWER_BLUE: Color = Color { r: 0.392, g: 0.584, b: 0.929, a: 1.0 };
    #[allow(dead_code)]
    pub const FIREBRICK: Color = Color { r: 0.698, g: 0.133, b: 0.133, a: 1.0 };
    #[allow(dead_code)]
    pub const GAINSBORO: Color = Color { r: 0.863, g: 0.863, b: 0.863, a: 1.0 };
    #[allow(dead_code)]
    pub const HONEYDEW: Color = Color { r: 0.941, g: 1.0, b: 0.941, a: 1.0 };
    #[allow(dead_code)]
    pub const IVORY: Color = Color { r: 1.0, g: 1.0, b: 0.941, a: 1.0 };
    #[allow(dead_code)]
    pub const LAWN_GREEN: Color = Color { r: 0.486, g: 0.988, b: 0.0, a: 1.0 };
    #[allow(dead_code)]
    pub const LINEN: Color = Color { r: 0.98, g: 0.941, b: 0.902, a: 1.0 };
    #[allow(dead_code)]
    pub const MIDNIGHT_BLUE: Color = Color { r: 0.098, g: 0.098, b: 0.439, a: 1.0 };
    #[allow(dead_code)]
    pub const MISTY_ROSE: Color = Color { r: 1.0, g: 0.894, b: 0.882, a: 1.0 };
    #[allow(dead_code)]
    pub const ORCHID: Color = Color { r: 0.855, g: 0.439, b: 0.839, a: 1.0 };
    #[allow(dead_code)]
    pub const PERU: Color = Color { r: 0.804, g: 0.522, b: 0.247, a: 1.0 };
    #[allow(dead_code)]
    pub const PLUM: Color = Color { r: 0.867, g: 0.627, b: 0.867, a: 1.0 };
    #[allow(dead_code)]
    pub const SIENNA: Color = Color { r: 0.627, g: 0.322, b: 0.176, a: 1.0 };
    #[allow(dead_code)]
    pub const SNOW: Color = Color { r: 1.0, g: 0.98, b: 0.98, a: 1.0 };
    #[allow(dead_code)]
    pub const SPRING_GREEN: Color = Color { r: 0.0, g: 1.0, b: 0.498, a: 1.0 };
    #[allow(dead_code)]
    pub const TOMATO: Color = Color { r: 1.0, g: 0.388, b: 0.278, a: 1.0 };
    #[allow(dead_code)]
    pub const WHEAT: Color = Color { r: 0.961, g: 0.871, b: 0.702, a: 1.0 };
}