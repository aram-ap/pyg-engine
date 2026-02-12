use super::{Rect, UIComponentTrait};
use super::event::UIEvent;
use super::style::UIStyle;
use super::layout::UILayoutComponent;
use crate::core::component::ComponentTrait;
use crate::core::draw_manager::DrawManager;
use crate::core::time::Time;
use crate::types::color::Color;
use std::any::Any;

/// Panel UI component - a container for other UI elements
#[derive(Debug, Clone)]
pub struct PanelComponent {
    name: String,
    bounds: Rect,
    layout: UILayoutComponent,
    style: UIStyle,
    clip_children: bool,
    enabled: bool,
    depth: f32,
}

impl PanelComponent {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            bounds: Rect::new(0.0, 0.0, 200.0, 200.0),
            layout: UILayoutComponent::with_fixed_size(200.0, 200.0),
            style: UIStyle::new(),
            clip_children: false,
            enabled: true,
            depth: 0.0,
        }
    }

    pub fn with_bounds(mut self, x: f32, y: f32, width: f32, height: f32) -> Self {
        self.bounds = Rect::new(x, y, width, height);
        self.layout = UILayoutComponent::with_fixed_size(width, height);
        self
    }

    pub fn with_style(mut self, style: UIStyle) -> Self {
        self.style = style;
        self
    }

    pub fn with_depth(mut self, depth: f32) -> Self {
        self.depth = depth;
        self
    }

    pub fn with_clip_children(mut self, clip: bool) -> Self {
        self.clip_children = clip;
        self
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

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn clip_children(&self) -> bool {
        self.clip_children
    }
}

impl ComponentTrait for PanelComponent {
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

impl UIComponentTrait for PanelComponent {
    fn bounds(&self) -> Rect {
        self.bounds
    }

    fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
    }

    fn handle_event(&mut self, _event: &UIEvent) -> bool {
        // Panels don't handle events directly, but they consume them
        // to prevent clicks from passing through
        true
    }

    fn render(&self, draw_manager: &mut DrawManager, offset: (f32, f32)) {
        let x = self.bounds.x + offset.0;
        let y = self.bounds.y + offset.1;

        // Draw background
        if self.style.background_color[3] > 0.0 {
            let bg_color = Color::new(
                self.style.background_color[0],
                self.style.background_color[1],
                self.style.background_color[2],
                self.style.background_color[3],
            );
            draw_manager.draw_rectangle_with_options(
                x,
                y,
                self.bounds.width,
                self.bounds.height,
                bg_color,
                true,
                1.0,
                self.depth,
            );
        }

        // Draw border
        if self.style.border_width > 0.0 {
            let border_color = Color::new(
                self.style.border_color[0],
                self.style.border_color[1],
                self.style.border_color[2],
                self.style.border_color[3],
            );
            draw_manager.draw_rectangle_with_options(
                x,
                y,
                self.bounds.width,
                self.bounds.height,
                border_color,
                false,
                self.style.border_width,
                self.depth + 0.005,
            );
        }

        // Children will be rendered by the UIManager
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
