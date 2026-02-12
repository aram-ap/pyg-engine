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

/// Determines when a button callback is triggered.
///
/// Controls whether the button's `on_click` callback fires when the mouse button
/// goes down (Press) or up (Release). This affects the button's responsiveness
/// and "feel" for different use cases.
///
/// # Variants
///
/// ## Release (Default)
/// Triggers when the mouse button is **released** over the button. This is the
/// standard behavior for UI buttons, allowing users to cancel by moving the
/// mouse away before releasing.
///
/// - **Pros**: Feels like a "click", users can cancel, industry standard
/// - **Cons**: Slightly less responsive than Press
/// - **Use cases**: Menu buttons, confirmations, UI interactions
///
/// ## Press
/// Triggers immediately when the mouse button goes **down** over the button.
/// More responsive but doesn't allow cancellation.
///
/// - **Pros**: Instant feedback, more game-like
/// - **Cons**: No way to cancel, less forgiving
/// - **Use cases**: Game controls, rapid-fire actions, instant response needs
///
/// # Examples
///
/// ```rust
/// use pyg_engine::ButtonComponent;
/// use pyg_engine::ButtonTrigger;
///
/// // Standard click button (release)
/// let menu_button = ButtonComponent::new("StartButton")
///     .with_text("Start Game")
///     .with_trigger_on(ButtonTrigger::Release);  // Default
///
/// // Instant response button (press)
/// let fire_button = ButtonComponent::new("FireButton")
///     .with_text("Fire!")
///     .with_trigger_on(ButtonTrigger::Press);
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ButtonTrigger {
    /// Trigger on mouse button **release** (click) - default behavior.
    ///
    /// Standard button behavior. Callback fires when the user releases the
    /// mouse button while over the button. Users can cancel by moving the
    /// mouse away before releasing.
    Release,

    /// Trigger on mouse button **press** (down).
    ///
    /// Immediate response. Callback fires as soon as the user presses down
    /// on the button, with no way to cancel.
    Press,
}

/// Button UI component for clickable buttons.
///
/// A clickable button with customizable appearance, text label, and callback.
/// Supports hover states, press states, enabled/disabled states, and optional
/// continuous firing while held.
///
/// # Features
///
/// - **Callback system**: Register functions to be called on click
/// - **Trigger modes**: Fire on press or release
/// - **Repeat mode**: Optionally fire continuously while held
/// - **Visual states**: Normal, hovered, pressed, disabled
/// - **Styling**: Customizable colors, borders, fonts
/// - **Builder pattern**: Fluent API for construction
///
/// # Builder Pattern
///
/// ```rust
/// use pyg_engine::ButtonComponent;
/// use pyg_engine::ButtonTrigger;
///
/// let button = ButtonComponent::new("PlayButton")
///     .with_text("Play Game")
///     .with_bounds(100.0, 200.0, 200.0, 50.0)
///     .with_depth(10.0)
///     .with_trigger_on(ButtonTrigger::Release);
///
/// // Set callback later
/// // button.set_on_click(|| println!("Button clicked!"));
/// ```
///
/// # Callback Registration
///
/// ```rust
/// use pyg_engine::ButtonComponent;
///
/// let mut button = ButtonComponent::new("MyButton");
///
/// // Register callback
/// button.set_on_click(|| {
///     println!("Button was clicked!");
/// });
/// ```
///
/// # Repeat Mode
///
/// Enable continuous firing while the button is held:
///
/// ```rust
/// use pyg_engine::ButtonComponent;
///
/// let mut button = ButtonComponent::new("IncrementButton")
///     .with_text("+");
///
/// // Fire every 100ms while held
/// button.set_repeat_interval_ms(Some(100.0));
/// ```
///
/// # See Also
/// - Python examples: `examples/button_features_demo.py`, `examples/ui_demo.py`
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
    trigger_on: ButtonTrigger,
    repeat_interval_ms: Option<f32>,
    last_repeat_time: Arc<Mutex<Option<f32>>>,
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
    /// Create a new button with the given name.
    ///
    /// Creates a button with default properties:
    /// - Position: (0, 0)
    /// - Size: 100×30 pixels
    /// - Text: Empty
    /// - Trigger: Release
    /// - Enabled: true
    /// - Depth: 0.0
    ///
    /// Use builder methods to customize.
    ///
    /// # Arguments
    /// * `name` - Unique identifier for this button
    ///
    /// # Examples
    /// ```rust
    /// use pyg_engine::ButtonComponent;
    ///
    /// let button = ButtonComponent::new("PlayButton")
    ///     .with_text("Play Game")
    ///     .with_bounds(100.0, 200.0, 200.0, 50.0);
    /// ```
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
            trigger_on: ButtonTrigger::Release,
            repeat_interval_ms: None,
            last_repeat_time: Arc::new(Mutex::new(None)),
        }
    }

    /// Set the button text (builder pattern).
    ///
    /// # Arguments
    /// * `text` - Display text for the button
    ///
    /// # Returns
    /// Self for method chaining
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.label = text.into();
        self
    }

    /// Set the button position and size (builder pattern).
    ///
    /// # Arguments
    /// * `x` - Left edge X coordinate in pixels
    /// * `y` - Top edge Y coordinate in pixels
    /// * `width` - Button width in pixels
    /// * `height` - Button height in pixels
    ///
    /// # Returns
    /// Self for method chaining
    pub fn with_bounds(mut self, x: f32, y: f32, width: f32, height: f32) -> Self {
        self.bounds = Rect::new(x, y, width, height);
        self.layout = UILayoutComponent::with_fixed_size(width, height);
        self
    }

    /// Set the button style (builder pattern).
    ///
    /// Customize colors, borders, fonts, and other visual properties.
    ///
    /// # Arguments
    /// * `style` - StyleSet defining button appearance
    ///
    /// # Returns
    /// Self for method chaining
    pub fn with_style(mut self, style: StyleSet) -> Self {
        self.style = style;
        self
    }

    /// Set the button depth/z-index (builder pattern).
    ///
    /// Controls rendering order. Higher depth values render on top.
    ///
    /// # Arguments
    /// * `depth` - Depth value (higher = on top)
    ///
    /// # Returns
    /// Self for method chaining
    pub fn with_depth(mut self, depth: f32) -> Self {
        self.depth = depth;
        self
    }

    /// Set the button text.
    ///
    /// # Arguments
    /// * `text` - New display text
    pub fn set_text(&mut self, text: impl Into<String>) {
        self.label = text.into();
    }

    /// Get the current button text.
    ///
    /// # Returns
    /// String slice containing the button text
    pub fn text(&self) -> &str {
        &self.label
    }

    /// Enable or disable the button.
    ///
    /// Disabled buttons don't respond to clicks and typically display
    /// with a "disabled" visual state.
    ///
    /// # Arguments
    /// * `enabled` - `true` to enable, `false` to disable
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

    pub fn set_trigger_on(&mut self, trigger: ButtonTrigger) {
        self.trigger_on = trigger;
    }

    pub fn set_repeat_interval_ms(&mut self, interval_ms: Option<f32>) {
        self.repeat_interval_ms = interval_ms;
        if interval_ms.is_none() {
            *self.last_repeat_time.lock().unwrap() = None;
        }
    }

    fn trigger_callback(&mut self) {
        if let Ok(mut guard) = self.on_click.lock() {
            if let Some(callback) = guard.as_mut() {
                callback();
            }
        }
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

    fn update(&self, time: &Time) {
        // Handle repeat functionality when button is held down
        if !self.enabled || !self.is_pressed {
            return;
        }

        if let Some(interval_ms) = self.repeat_interval_ms {
            let current_time = time.elapsed_time();

            // Check if it's time to repeat
            if let Ok(mut last_time_guard) = self.last_repeat_time.lock() {
                match *last_time_guard {
                    Some(last_time) => {
                        let elapsed_ms = (current_time - last_time) * 1000.0;
                        if elapsed_ms >= interval_ms {
                            // Trigger callback and update last repeat time
                            *last_time_guard = Some(current_time);
                            drop(last_time_guard); // Release lock before calling callback

                            // Trigger the callback
                            if let Ok(mut guard) = self.on_click.lock() {
                                if let Some(callback) = guard.as_mut() {
                                    callback();
                                }
                            }
                        }
                    }
                    None => {
                        // Initialize the repeat timer - button just started being pressed
                        *last_time_guard = Some(current_time);
                    }
                }
            }
        }
    }

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
                // Clear repeat timer when mouse exits
                if self.repeat_interval_ms.is_some() {
                    *self.last_repeat_time.lock().unwrap() = None;
                }
                true
            }
            UIEvent::MouseDown { .. } => {
                self.is_pressed = true;
                self.update_state();

                // If trigger on press, fire callback immediately
                if self.trigger_on == ButtonTrigger::Press {
                    crate::core::logging::log_debug(&format!("Button '{}' pressed!", self.label));
                    self.trigger_callback();
                }
                true
            }
            UIEvent::MouseUp { .. } => {
                self.is_pressed = false;
                self.update_state();
                // Clear repeat timer when button is released
                if self.repeat_interval_ms.is_some() {
                    *self.last_repeat_time.lock().unwrap() = None;
                }
                true
            }
            UIEvent::Click { .. } => {
                // Only trigger on click if trigger mode is Release
                if self.trigger_on == ButtonTrigger::Release {
                    crate::core::logging::log_debug(&format!("Button '{}' clicked!", self.label));
                    self.trigger_callback();
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
