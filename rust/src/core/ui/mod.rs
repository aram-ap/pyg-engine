use crate::core::component::ComponentTrait;
use crate::core::draw_manager::DrawManager;
use std::any::Any;

pub mod event;
pub mod style;
pub mod layout;
pub mod button;
pub mod panel;
pub mod label;

/// 2D rectangle for bounds and hit detection
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }

    /// Check if a point is inside this rectangle
    pub fn contains(&self, px: f32, py: f32) -> bool {
        px >= self.x && px <= self.x + self.width &&
        py >= self.y && py <= self.y + self.height
    }

    /// Get the center point of the rectangle
    pub fn center(&self) -> (f32, f32) {
        (self.x + self.width / 2.0, self.y + self.height / 2.0)
    }
}

/// Visual state of a UI component
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StyleState {
    Normal,
    Hovered,
    Pressed,
    Focused,
    Disabled,
}

/// Size mode for layout calculations
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SizeMode {
    /// Fixed size in pixels
    Fixed(f32),
    /// Percentage of parent size (0.0 to 1.0)
    Percentage(f32),
    /// Fit to content size
    FitContent,
    /// Fill parent's available space
    FillParent,
}

impl Default for SizeMode {
    fn default() -> Self {
        SizeMode::Fixed(100.0)
    }
}

/// UI-specific component trait extending ComponentTrait
pub trait UIComponentTrait: ComponentTrait {
    /// Get the current bounds of this component
    fn bounds(&self) -> Rect;

    /// Set the bounds of this component
    fn set_bounds(&mut self, bounds: Rect);

    /// Check if a point is within this component
    fn contains_point(&self, x: f64, y: f64) -> bool {
        self.bounds().contains(x as f32, y as f32)
    }

    /// Handle a UI event, return true if consumed
    fn handle_event(&mut self, event: &event::UIEvent) -> bool;

    /// Render this component
    fn render(&self, draw_manager: &mut DrawManager, offset: (f32, f32));

    /// Get the UI depth for rendering order (higher = front)
    fn ui_depth(&self) -> f32 {
        0.0
    }

    /// Check if this component is enabled
    fn is_enabled(&self) -> bool {
        true
    }

    /// Downcast to Any for type checking
    fn as_any(&self) -> &dyn Any;

    /// Downcast to Any (mutable) for type checking
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// 2D vector for offsets and positions
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}
