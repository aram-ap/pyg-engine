use std::path::Path;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum FontWeight {
    #[default]
    Regular,
    Bold,
}

impl FontWeight {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Regular => "regular",
            Self::Bold => "bold",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "regular" | "normal" | "400" => Some(Self::Regular),
            "bold" | "700" => Some(Self::Bold),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum FontStyle {
    #[default]
    Normal,
    Italic,
}

impl FontStyle {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Italic => "italic",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "normal" | "roman" | "regular" => Some(Self::Normal),
            "italic" | "oblique" => Some(Self::Italic),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum TextAlign {
    #[default]
    Left,
    Center,
    Right,
}

impl TextAlign {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Center => "center",
            Self::Right => "right",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum VerticalTextAlign {
    #[default]
    Top,
    Center,
    Bottom,
}

impl VerticalTextAlign {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Center => "center",
            Self::Bottom => "bottom",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct FontDescriptor {
    family: Option<String>,
    path: Option<String>,
    weight: FontWeight,
    style: FontStyle,
}

impl FontDescriptor {
    pub fn default_font() -> Self {
        Self::default()
    }

    pub fn from_path(path: impl Into<String>) -> Self {
        let mut descriptor = Self::default();
        descriptor.set_path(Some(path.into()));
        descriptor
    }

    pub fn from_family(
        family: impl Into<String>,
        weight: FontWeight,
        style: FontStyle,
    ) -> Self {
        let mut descriptor = Self::default();
        descriptor.set_family(Some(family.into()));
        descriptor.set_weight(weight);
        descriptor.set_style(style);
        descriptor
    }

    pub fn family(&self) -> Option<&str> {
        self.family.as_deref()
    }

    pub fn set_family(&mut self, family: Option<String>) {
        self.family = family.filter(|value| !value.trim().is_empty());
        if self.family.is_some() {
            self.path = None;
        }
    }

    pub fn path(&self) -> Option<&str> {
        self.path.as_deref()
    }

    pub fn set_path(&mut self, path: Option<String>) {
        self.path = path.filter(|value| !value.trim().is_empty());
        if self.path.is_some() {
            self.family = None;
        }
    }

    pub fn weight(&self) -> FontWeight {
        self.weight
    }

    pub fn set_weight(&mut self, weight: FontWeight) {
        self.weight = weight;
    }

    pub fn style(&self) -> FontStyle {
        self.style
    }

    pub fn set_style(&mut self, style: FontStyle) {
        self.style = style;
    }

    pub fn family_key(&self) -> Option<String> {
        self.family
            .as_ref()
            .map(|family| normalize_font_family_key(family))
    }

    pub fn has_custom_font(&self) -> bool {
        self.path.is_some() || self.family.is_some()
    }

    pub fn cache_key(&self) -> String {
        if let Some(path) = &self.path {
            format!("path:{}", normalize_font_path(path))
        } else if let Some(family) = self.family_key() {
            format!(
                "family:{}:{}:{}",
                family,
                self.weight.as_str(),
                self.style.as_str()
            )
        } else {
            "default".to_string()
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TextStyle {
    pub font: FontDescriptor,
    pub font_size: f32,
    pub letter_spacing: f32,
    pub line_spacing: f32,
    pub kerning: bool,
}

impl TextStyle {
    pub fn new(font_size: f32) -> Self {
        Self {
            font: FontDescriptor::default(),
            font_size,
            letter_spacing: 0.0,
            line_spacing: 0.0,
            kerning: true,
        }
    }
}

impl Default for TextStyle {
    fn default() -> Self {
        Self::new(24.0)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TextLayoutOptions {
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub horizontal_align: TextAlign,
    pub vertical_align: VerticalTextAlign,
}

impl Default for TextLayoutOptions {
    fn default() -> Self {
        Self {
            width: None,
            height: None,
            horizontal_align: TextAlign::Left,
            vertical_align: VerticalTextAlign::Top,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FontFamilyDefinition {
    pub regular: Option<String>,
    pub bold: Option<String>,
    pub italic: Option<String>,
    pub bold_italic: Option<String>,
}

impl FontFamilyDefinition {
    pub fn new(
        regular: Option<String>,
        bold: Option<String>,
        italic: Option<String>,
        bold_italic: Option<String>,
    ) -> Self {
        Self {
            regular: regular.filter(|value| !value.trim().is_empty()),
            bold: bold.filter(|value| !value.trim().is_empty()),
            italic: italic.filter(|value| !value.trim().is_empty()),
            bold_italic: bold_italic.filter(|value| !value.trim().is_empty()),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.regular.is_none()
            && self.bold.is_none()
            && self.italic.is_none()
            && self.bold_italic.is_none()
    }

    pub fn resolve(&self, weight: FontWeight, style: FontStyle) -> Option<&str> {
        let primary = match (weight, style) {
            (FontWeight::Regular, FontStyle::Normal) => self.regular.as_deref(),
            (FontWeight::Bold, FontStyle::Normal) => self.bold.as_deref(),
            (FontWeight::Regular, FontStyle::Italic) => self.italic.as_deref(),
            (FontWeight::Bold, FontStyle::Italic) => self.bold_italic.as_deref(),
        };

        primary
            .or_else(|| {
                if style == FontStyle::Italic {
                    self.italic.as_deref()
                } else {
                    None
                }
            })
            .or_else(|| {
                if weight == FontWeight::Bold {
                    self.bold.as_deref()
                } else {
                    None
                }
            })
            .or(self.regular.as_deref())
            .or(self.bold_italic.as_deref())
            .or(self.italic.as_deref())
            .or(self.bold.as_deref())
    }
}

pub fn normalize_font_family_key(value: &str) -> String {
    value.trim().to_ascii_lowercase()
}

pub fn normalize_font_path(value: &str) -> String {
    Path::new(value)
        .to_string_lossy()
        .replace('\\', "/")
}

#[cfg(test)]
mod tests {
    use super::{FontFamilyDefinition, FontStyle, FontWeight, normalize_font_family_key};

    #[test]
    fn resolves_exact_variant_first() {
        let family = FontFamilyDefinition::new(
            Some("regular.ttf".to_string()),
            Some("bold.ttf".to_string()),
            Some("italic.ttf".to_string()),
            Some("bold_italic.ttf".to_string()),
        );
        assert_eq!(
            family.resolve(FontWeight::Bold, FontStyle::Italic),
            Some("bold_italic.ttf")
        );
    }

    #[test]
    fn falls_back_to_nearest_available_variant() {
        let family = FontFamilyDefinition::new(
            Some("regular.ttf".to_string()),
            None,
            Some("italic.ttf".to_string()),
            None,
        );
        assert_eq!(family.resolve(FontWeight::Bold, FontStyle::Italic), Some("italic.ttf"));
        assert_eq!(family.resolve(FontWeight::Bold, FontStyle::Normal), Some("regular.ttf"));
    }

    #[test]
    fn normalizes_family_keys() {
        assert_eq!(normalize_font_family_key("  Inter UI "), "inter ui");
    }
}
