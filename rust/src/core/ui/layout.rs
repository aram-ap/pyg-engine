use super::{Rect, SizeMode};

/// Anchor positions for UI layout
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Anchor {
    TopLeft,
    TopCenter,
    TopRight,
    MiddleLeft,
    MiddleCenter,
    MiddleRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

impl Default for Anchor {
    fn default() -> Self {
        Anchor::TopLeft
    }
}

/// Offset from anchor point
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AnchorOffset {
    pub x: f32,
    pub y: f32,
}

impl AnchorOffset {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

impl Default for AnchorOffset {
    fn default() -> Self {
        Self::zero()
    }
}

/// Layout component for positioning UI elements
#[derive(Debug, Clone, PartialEq)]
pub struct UILayoutComponent {
    /// Anchor point
    pub anchor: Anchor,
    /// Offset from anchor
    pub offset: AnchorOffset,
    /// Pivot point (0.0 to 1.0, where 0,0 is top-left of element)
    pub pivot: (f32, f32),
    /// Width mode
    pub width_mode: SizeMode,
    /// Height mode
    pub height_mode: SizeMode,
    /// Cached computed bounds
    computed_bounds: Option<Rect>,
    /// Dirty flag for layout recalculation
    layout_dirty: bool,
}

impl UILayoutComponent {
    pub fn new() -> Self {
        Self {
            anchor: Anchor::TopLeft,
            offset: AnchorOffset::zero(),
            pivot: (0.0, 0.0),
            width_mode: SizeMode::Fixed(100.0),
            height_mode: SizeMode::Fixed(100.0),
            computed_bounds: None,
            layout_dirty: true,
        }
    }

    /// Create with specific anchor and offset
    pub fn with_anchor(anchor: Anchor, offset: AnchorOffset) -> Self {
        Self {
            anchor,
            offset,
            ..Self::new()
        }
    }

    /// Create with fixed size
    pub fn with_fixed_size(width: f32, height: f32) -> Self {
        Self {
            width_mode: SizeMode::Fixed(width),
            height_mode: SizeMode::Fixed(height),
            ..Self::new()
        }
    }

    /// Mark layout as dirty
    pub fn mark_dirty(&mut self) {
        self.layout_dirty = true;
    }

    /// Get computed bounds (if available)
    pub fn get_bounds(&self) -> Option<Rect> {
        self.computed_bounds
    }

    /// Calculate bounds based on parent bounds
    pub fn calculate_bounds(&mut self, parent_bounds: Rect) -> Rect {
        // Calculate size
        let width = match self.width_mode {
            SizeMode::Fixed(w) => w,
            SizeMode::Percentage(p) => parent_bounds.width * p,
            SizeMode::FitContent => 100.0, // TODO: Calculate from content
            SizeMode::FillParent => parent_bounds.width,
        };

        let height = match self.height_mode {
            SizeMode::Fixed(h) => h,
            SizeMode::Percentage(p) => parent_bounds.height * p,
            SizeMode::FitContent => 100.0, // TODO: Calculate from content
            SizeMode::FillParent => parent_bounds.height,
        };

        // Calculate anchor point in parent space
        let anchor_point = self.calculate_anchor_point(&parent_bounds);

        // Calculate position with pivot offset
        let x = anchor_point.0 + self.offset.x - (width * self.pivot.0);
        let y = anchor_point.1 + self.offset.y - (height * self.pivot.1);

        let bounds = Rect::new(x, y, width, height);
        self.computed_bounds = Some(bounds);
        self.layout_dirty = false;

        bounds
    }

    /// Calculate the anchor point in parent space
    fn calculate_anchor_point(&self, parent: &Rect) -> (f32, f32) {
        match self.anchor {
            Anchor::TopLeft => (parent.x, parent.y),
            Anchor::TopCenter => (parent.x + parent.width / 2.0, parent.y),
            Anchor::TopRight => (parent.x + parent.width, parent.y),
            Anchor::MiddleLeft => (parent.x, parent.y + parent.height / 2.0),
            Anchor::MiddleCenter => (parent.x + parent.width / 2.0, parent.y + parent.height / 2.0),
            Anchor::MiddleRight => (parent.x + parent.width, parent.y + parent.height / 2.0),
            Anchor::BottomLeft => (parent.x, parent.y + parent.height),
            Anchor::BottomCenter => (parent.x + parent.width / 2.0, parent.y + parent.height),
            Anchor::BottomRight => (parent.x + parent.width, parent.y + parent.height),
        }
    }

    /// Check if layout needs recalculation
    pub fn is_dirty(&self) -> bool {
        self.layout_dirty
    }
}

impl Default for UILayoutComponent {
    fn default() -> Self {
        Self::new()
    }
}
