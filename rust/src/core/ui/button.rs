use super::{Rect, StyleState, UIComponentTrait};
use super::event::UIEvent;
use super::style::StyleSet;
use super::layout::UILayoutComponent;
use crate::core::component::ComponentTrait;
use crate::core::draw_manager::DrawManager;
use crate::core::time::Time;
use crate::types::color::Color;
use std::any::Any;
use std::sync::{Arc, Mutex};

/// Button UI component
#[derive(Clone)]
pub struct ButtonComponent {
    name: String,
    bounds: Rect,
    layout: UILayoutComponent,
    style: StyleSet,
    current_state: StyleState,
    label: String,
    on_click: Arc<Mutex<Option<Box<dyn FnMut() + Send + Sync>>>>,
    is_hovered: bool,
    is_pressed: bool,
    enabled: bool,
    depth: f32,
}

impl std::fmt::Debug for ButtonComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ButtonComponent")
            .field("name", &self.name)
            .field("bounds", &self.bounds)
            .field("label", &self.label)
            .field("enabled", &self.enabled)
            .finish()
    }
}

impl ButtonComponent {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            bounds: Rect::new(0.0, 0.0, 100.0, 30.0),
            layout: UILayoutComponent::with_fixed_size(100.0, 30.0),
            style: StyleSet::default(),
            current_state: StyleState::Normal,
            label: String::new(),
            on_click: Arc::new(Mutex::new(None)),
            is_hovered: false,
            is_pressed: false,
            enabled: true,
            depth: 0.0,
        }
    }

    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.label = text.into();
        self
    }

    pub fn with_bounds(mut self, x: f32, y: f32, width: f32, height: f32) -> Self {
        self.bounds = Rect::new(x, y, width, height);
        self.layout = UILayoutComponent::with_fixed_size(width, height);
        self
    }

    pub fn with_style(mut self, style: StyleSet) -> Self {
        self.style = style;
        self
    }

    pub fn with_depth(mut self, depth: f32) -> Self {
        self.depth = depth;
        self
    }

    pub fn set_text(&mut self, text: impl Into<String>) {
        self.label = text.into();
    }

    pub fn text(&self) -> &str {
        &self.label
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        self.current_state = if enabled {
            StyleState::Normal
        } else {
            StyleState::Disabled
        };
    }

    pub fn set_style(&mut self, style: StyleSet) {
        self.style = style;
    }

    pub fn set_on_click<F>(&mut self, callback: F)
    where
        F: FnMut() + Send + Sync + 'static,
    {
        *self.on_click.lock().unwrap() = Some(Box::new(callback));
    }

    fn update_state(&mut self) {
        if !self.enabled {
            self.current_state = StyleState::Disabled;
        } else if self.is_pressed {
            self.current_state = StyleState::Pressed;
        } else if self.is_hovered {
            self.current_state = StyleState::Hovered;
        } else {
            self.current_state = StyleState::Normal;
        }
    }
}

impl ComponentTrait for ButtonComponent {
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

impl UIComponentTrait for ButtonComponent {
    fn bounds(&self) -> Rect {
        self.bounds
    }

    fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
    }

    fn handle_event(&mut self, event: &UIEvent) -> bool {
        if !self.enabled {
            return false;
        }

        match event {
            UIEvent::MouseEnter { .. } => {
                self.is_hovered = true;
                self.update_state();
                true
            }
            UIEvent::MouseExit { .. } => {
                self.is_hovered = false;
                self.is_pressed = false;
                self.update_state();
                true
            }
            UIEvent::MouseDown { .. } => {
                self.is_pressed = true;
                self.update_state();
                true
            }
            UIEvent::MouseUp { .. } => {
                self.is_pressed = false;
                self.update_state();
                true
            }
            UIEvent::Click { .. } | UIEvent::DoubleClick { .. } => {
                // Trigger callback for both single and double clicks
                // This allows rapid clicking without missing every other click
                crate::core::logging::log_debug(&format!("Button '{}' clicked!", self.label));
                if let Ok(mut guard) = self.on_click.lock() {
                    if let Some(callback) = guard.as_mut() {
                        crate::core::logging::log_debug("Invoking button callback...");
                        callback();
                        crate::core::logging::log_debug("Button callback completed");
                    } else {
                        crate::core::logging::log_debug("No callback set for button");
                    }
                }
                true
            }
            _ => false,
        }
    }

    fn render(&self, draw_manager: &mut DrawManager, offset: (f32, f32)) {
        let style = self.style.get_style(self.current_state);
        let x = self.bounds.x + offset.0;
        let y = self.bounds.y + offset.1;

        // Draw background
        if style.background_color[3] > 0.0 {
            let bg_color = Color::new(
                style.background_color[0],
                style.background_color[1],
                style.background_color[2],
                style.background_color[3],
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
        if style.border_width > 0.0 {
            let border_color = Color::new(
                style.border_color[0],
                style.border_color[1],
                style.border_color[2],
                style.border_color[3],
            );
            draw_manager.draw_rectangle_with_options(
                x,
                y,
                self.bounds.width,
                self.bounds.height,
                border_color,
                false,
                style.border_width,
                self.depth + 0.005,
            );
        }

        // Draw text (centered)
        if !self.label.is_empty() {
            // Estimate text width using font8x8 metrics (24px default font size, 8px base glyph)
            let font_size = 14.0_f32; // Default button font size
            let scale = (font_size / 8.0).max(1.0).round();
            let glyph_width = 8.0 * scale;
            let glyph_height = 8.0 * scale;
            let text_width = self.label.len() as f32 * glyph_width;
            let text_x = x + (self.bounds.width - text_width) / 2.0;
            let text_y = y + (self.bounds.height - glyph_height) / 2.0;
            let text_color = Color::new(
                style.text_color[0],
                style.text_color[1],
                style.text_color[2],
                style.text_color[3],
            );

            draw_manager.draw_text_with_options(
                self.label.clone(),
                text_x,
                text_y,
                font_size,
                text_color,
                None,
                0.0,
                0.0,
                self.depth + 0.01,
            );
        }
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
