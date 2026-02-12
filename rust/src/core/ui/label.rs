use super::{Rect, UIComponentTrait};
use super::event::UIEvent;
use super::style::UIStyle;
use super::layout::UILayoutComponent;
use crate::core::component::ComponentTrait;
use crate::core::draw_manager::DrawManager;
use crate::core::time::Time;
use crate::types::color::Color;
use std::any::Any;

/// Text alignment for labels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}

impl Default for TextAlign {
    fn default() -> Self {
        TextAlign::Left
    }
}

/// Label UI component for displaying text
#[derive(Debug, Clone)]
pub struct LabelComponent {
    name: String,
    bounds: Rect,
    layout: UILayoutComponent,
    text: String,
    style: UIStyle,
    text_align: TextAlign,
    enabled: bool,
    depth: f32,
}

impl LabelComponent {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            bounds: Rect::new(0.0, 0.0, 100.0, 20.0),
            layout: UILayoutComponent::with_fixed_size(100.0, 20.0),
            text: String::new(),
            style: UIStyle::transparent(),
            text_align: TextAlign::Left,
            enabled: true,
            depth: 0.0,
        }
    }

    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }

    pub fn with_bounds(mut self, x: f32, y: f32, width: f32, height: f32) -> Self {
        self.bounds = Rect::new(x, y, width, height);
        self.layout = UILayoutComponent::with_fixed_size(width, height);
        self
    }

    pub fn with_position(mut self, x: f32, y: f32) -> Self {
        self.bounds.x = x;
        self.bounds.y = y;
        self
    }

    pub fn with_style(mut self, style: UIStyle) -> Self {
        self.style = style;
        self
    }

    pub fn with_align(mut self, align: TextAlign) -> Self {
        self.text_align = align;
        self
    }

    pub fn with_depth(mut self, depth: f32) -> Self {
        self.depth = depth;
        self
    }

    pub fn set_text(&mut self, text: impl Into<String>) {
        self.text = text.into();
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.bounds.x = x;
        self.bounds.y = y;
    }

    pub fn set_font_size(&mut self, size: f32) {
        self.style.font_size = size;
    }

    pub fn set_color(&mut self, color: [f32; 4]) {
        self.style.text_color = color;
    }

    pub fn set_align(&mut self, align: TextAlign) {
        self.text_align = align;
    }

    pub fn set_style(&mut self, style: UIStyle) {
        self.style = style;
    }

    pub fn style(&self) -> &UIStyle {
        &self.style
    }

    pub fn style_mut(&mut self) -> &mut UIStyle {
        &mut self.style
    }

    /// Estimate text width using font8x8 metrics (8px base glyph width).
    fn estimate_text_width(&self) -> f32 {
        let scale = (self.style.font_size / 8.0).max(1.0).round();
        let glyph_width = 8.0 * scale;
        self.text.len() as f32 * glyph_width
    }
}

impl ComponentTrait for LabelComponent {
    fn new(name: String) -> Self {
        Self::new(name)
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn update(&self, _time: &Time) {}
    fn fixed_update(&self, _time: &Time, _fixed_time: f32) {}
    fn on_start(&self) {}
    fn on_destroy(&self) {}
    fn on_enable(&self) {}
    fn on_disable(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl UIComponentTrait for LabelComponent {
    fn bounds(&self) -> Rect {
        self.bounds
    }

    fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
    }

    fn handle_event(&mut self, _event: &UIEvent) -> bool {
        // Labels don't handle events
        false
    }

    fn render(&self, draw_manager: &mut DrawManager, offset: (f32, f32)) {
        if self.text.is_empty() {
            return;
        }

        let x = self.bounds.x + offset.0;
        let y = self.bounds.y + offset.1;

        // Calculate text position based on alignment
        // x is the anchor point: left edge for Left, center for Center, right edge for Right
        let text_width = self.estimate_text_width();
        let text_x = match self.text_align {
            TextAlign::Left => x,
            TextAlign::Center => x - text_width / 2.0,
            TextAlign::Right => x - text_width,
        };

        let text_y = y;

        let text_color = Color::new(
            self.style.text_color[0],
            self.style.text_color[1],
            self.style.text_color[2],
            self.style.text_color[3],
        );

        draw_manager.draw_text_with_options(
            self.text.clone(),
            text_x,
            text_y,
            self.style.font_size,
            text_color,
            None,
            0.0,
            0.0,
            self.depth + 0.01,
        );
    }

    fn ui_depth(&self) -> f32 {
        self.depth
    }

    fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
