use super::StyleState;

/// Padding for UI elements
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Padding {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl Padding {
    pub fn new(left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Self { left, right, top, bottom }
    }

    pub fn uniform(value: f32) -> Self {
        Self {
            left: value,
            right: value,
            top: value,
            bottom: value,
        }
    }

    pub fn zero() -> Self {
        Self::uniform(0.0)
    }
}

impl Default for Padding {
    fn default() -> Self {
        Self::zero()
    }
}

/// Style properties for a UI component
#[derive(Debug, Clone, PartialEq)]
pub struct UIStyle {
    pub background_color: [f32; 4],  // RGBA
    pub border_color: [f32; 4],      // RGBA
    pub text_color: [f32; 4],        // RGBA
    pub border_width: f32,
    pub border_radius: f32,
    pub padding: Padding,
    pub margin: Padding,
    pub font_size: f32,
    pub font_path: Option<String>,
}

impl UIStyle {
    pub fn new() -> Self {
        Self {
            background_color: [1.0, 1.0, 1.0, 1.0], // White
            border_color: [0.0, 0.0, 0.0, 1.0],     // Black
            text_color: [0.0, 0.0, 0.0, 1.0],       // Black
            border_width: 0.0,
            border_radius: 0.0,
            padding: Padding::zero(),
            margin: Padding::zero(),
            font_size: 16.0,
            font_path: None,
        }
    }

    /// Create a transparent style
    pub fn transparent() -> Self {
        Self {
            background_color: [0.0, 0.0, 0.0, 0.0],
            ..Self::new()
        }
    }
}

impl Default for UIStyle {
    fn default() -> Self {
        Self::new()
    }
}

/// Style set containing styles for different states
#[derive(Debug, Clone, PartialEq)]
pub struct StyleSet {
    pub normal: UIStyle,
    pub hovered: UIStyle,
    pub pressed: UIStyle,
    pub focused: UIStyle,
    pub disabled: UIStyle,
}

impl StyleSet {
    pub fn new(base_style: UIStyle) -> Self {
        Self {
            normal: base_style.clone(),
            hovered: base_style.clone(),
            pressed: base_style.clone(),
            focused: base_style.clone(),
            disabled: base_style,
        }
    }

    /// Get the style for a given state
    pub fn get_style(&self, state: StyleState) -> &UIStyle {
        match state {
            StyleState::Normal => &self.normal,
            StyleState::Hovered => &self.hovered,
            StyleState::Pressed => &self.pressed,
            StyleState::Focused => &self.focused,
            StyleState::Disabled => &self.disabled,
        }
    }

    /// Get mutable style for a given state
    pub fn get_style_mut(&mut self, state: StyleState) -> &mut UIStyle {
        match state {
            StyleState::Normal => &mut self.normal,
            StyleState::Hovered => &mut self.hovered,
            StyleState::Pressed => &mut self.pressed,
            StyleState::Focused => &mut self.focused,
            StyleState::Disabled => &mut self.disabled,
        }
    }
}

impl Default for StyleSet {
    fn default() -> Self {
        Self::new(UIStyle::default())
    }
}

/// UI theme containing default styles for components
#[derive(Debug, Clone)]
pub struct UITheme {
    pub button_style: StyleSet,
    pub panel_style: UIStyle,
    pub label_style: UIStyle,
}

impl UITheme {
    /// Create the default light theme
    pub fn default_light() -> Self {
        // Button styles
        let mut button_normal = UIStyle::new();
        button_normal.background_color = [0.90, 0.90, 0.90, 1.0]; // #e5e5e5
        button_normal.border_color = [0.6, 0.6, 0.6, 1.0];
        button_normal.border_width = 1.0;
        button_normal.border_radius = 4.0;
        button_normal.padding = Padding::uniform(8.0);
        button_normal.text_color = [0.0, 0.0, 0.0, 1.0];
        button_normal.font_size = 14.0;

        let mut button_hovered = button_normal.clone();
        button_hovered.background_color = [0.95, 0.95, 0.95, 1.0]; // #f2f2f2

        let mut button_pressed = button_normal.clone();
        button_pressed.background_color = [0.80, 0.80, 0.80, 1.0]; // #cccccc

        let button_focused = button_hovered.clone();

        let mut button_disabled = button_normal.clone();
        button_disabled.background_color = [0.85, 0.85, 0.85, 1.0];
        button_disabled.text_color = [0.5, 0.5, 0.5, 1.0];

        let button_style = StyleSet {
            normal: button_normal,
            hovered: button_hovered,
            pressed: button_pressed,
            focused: button_focused,
            disabled: button_disabled,
        };

        // Panel style
        let mut panel_style = UIStyle::new();
        panel_style.background_color = [1.0, 1.0, 1.0, 1.0]; // White
        panel_style.border_color = [0.8, 0.8, 0.8, 1.0];
        panel_style.border_width = 1.0;
        panel_style.border_radius = 0.0;
        panel_style.padding = Padding::uniform(4.0);

        // Label style
        let mut label_style = UIStyle::transparent();
        label_style.text_color = [0.0, 0.0, 0.0, 1.0];
        label_style.font_size = 14.0;

        Self {
            button_style,
            panel_style,
            label_style,
        }
    }
}

impl Default for UITheme {
    fn default() -> Self {
        Self::default_light()
    }
}
