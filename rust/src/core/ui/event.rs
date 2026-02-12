use super::UIComponentTrait;
use crate::core::input_manager::{InputManager, MouseButtonType};
use std::time::Instant;

/// UI event types
#[derive(Debug, Clone, PartialEq)]
pub enum UIEvent {
    MouseEnter { x: f64, y: f64 },
    MouseExit { x: f64, y: f64 },
    MouseMove { x: f64, y: f64, dx: f64, dy: f64 },
    MouseDown { x: f64, y: f64, button: MouseButtonType },
    MouseUp { x: f64, y: f64, button: MouseButtonType },
    Click { x: f64, y: f64, button: MouseButtonType },
    DoubleClick { x: f64, y: f64, button: MouseButtonType },
    FocusGained,
    FocusLost,
}

/// Manages UI events and input processing
pub struct UIEventManager {
    /// Currently hovered component ID
    hovered_component: Option<u32>,
    /// Currently focused component ID
    focused_component: Option<u32>,
    /// Currently pressed component ID
    pressed_component: Option<u32>,
    /// Last mouse position
    last_mouse_pos: (f64, f64),
    /// Last click time for double-click detection
    last_click_time: Option<Instant>,
    /// Last clicked component for double-click
    last_clicked_component: Option<u32>,
    /// Whether input was consumed this frame
    input_consumed: bool,
    /// Mouse button states from previous frame
    prev_mouse_buttons: [bool; 3],
}

impl UIEventManager {
    pub fn new() -> Self {
        Self {
            hovered_component: None,
            focused_component: None,
            pressed_component: None,
            last_mouse_pos: (0.0, 0.0),
            last_click_time: None,
            last_clicked_component: None,
            input_consumed: false,
            prev_mouse_buttons: [false; 3],
        }
    }

    /// Process input and generate UI events
    /// Returns vector of (component_id, event) pairs
    ///
    /// # Arguments
    /// * `input` - Input manager containing mouse/keyboard state
    /// * `ui_components` - List of UI components with their IDs and depths
    /// * `scale_factor` - HiDPI scale factor (physical pixels / logical pixels)
    pub fn process_input(
        &mut self,
        input: &InputManager,
        ui_components: &[(u32, &dyn UIComponentTrait, f64)], // (id, component, depth)
        scale_factor: f32,
    ) -> Vec<(u32, UIEvent)> {
        let mut events = Vec::new();
        self.input_consumed = false;

        // Get mouse position in physical pixels and convert to logical pixels
        // UI components are positioned in logical pixels, but mouse events are in physical pixels
        let mouse_pos = input.mouse_position();
        let mouse_x = mouse_pos.0 / scale_factor as f64;
        let mouse_y = mouse_pos.1 / scale_factor as f64;

        // Sort components by depth (front to back for hit testing)
        let mut sorted_components: Vec<_> = ui_components.iter().collect();
        sorted_components.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));

        // Hit test to find hovered component
        let mut new_hovered = None;
        for &&(id, component, _depth) in &sorted_components {
            if component.is_enabled() && component.contains_point(mouse_x, mouse_y) {
                new_hovered = Some(id);
                break; // Only the frontmost component
            }
        }

        // Handle hover changes
        if new_hovered != self.hovered_component {
            // Mouse exit from previous component
            if let Some(old_id) = self.hovered_component {
                events.push((old_id, UIEvent::MouseExit { x: mouse_x, y: mouse_y }));
            }

            // Mouse enter new component
            if let Some(new_id) = new_hovered {
                events.push((new_id, UIEvent::MouseEnter { x: mouse_x, y: mouse_y }));
                self.input_consumed = true;
            }

            self.hovered_component = new_hovered;
        }

        // Handle mouse movement
        if (mouse_x, mouse_y) != self.last_mouse_pos {
            if let Some(hovered_id) = self.hovered_component {
                let dx = mouse_x - self.last_mouse_pos.0;
                let dy = mouse_y - self.last_mouse_pos.1;
                events.push((hovered_id, UIEvent::MouseMove { x: mouse_x, y: mouse_y, dx, dy }));
                self.input_consumed = true;
            }
            self.last_mouse_pos = (mouse_x, mouse_y);
        }

        // Handle mouse buttons
        let buttons = [MouseButtonType::Left, MouseButtonType::Right, MouseButtonType::Middle];
        for (idx, &button) in buttons.iter().enumerate() {
            let is_down = input.mouse_button_down(button);
            let was_down = self.prev_mouse_buttons[idx];

            // Mouse down
            if is_down && !was_down {
                if let Some(hovered_id) = self.hovered_component {
                    events.push((hovered_id, UIEvent::MouseDown { x: mouse_x, y: mouse_y, button }));
                    self.pressed_component = Some(hovered_id);
                    self.input_consumed = true;

                    // Update focus
                    if self.focused_component != Some(hovered_id) {
                        if let Some(old_focused) = self.focused_component {
                            events.push((old_focused, UIEvent::FocusLost));
                        }
                        events.push((hovered_id, UIEvent::FocusGained));
                        self.focused_component = Some(hovered_id);
                    }
                }
            }

            // Mouse up
            if !is_down && was_down {
                if let Some(pressed_id) = self.pressed_component {
                    events.push((pressed_id, UIEvent::MouseUp { x: mouse_x, y: mouse_y, button }));

                    // Check for click (mouse up on same component)
                    if Some(pressed_id) == self.hovered_component {
                        // Check for double-click
                        let now = Instant::now();
                        let is_double_click = if let Some(last_time) = self.last_click_time {
                            if let Some(last_comp) = self.last_clicked_component {
                                last_comp == pressed_id && now.duration_since(last_time).as_millis() < 500
                            } else {
                                false
                            }
                        } else {
                            false
                        };

                        if is_double_click {
                            crate::core::logging::log_debug(&format!("UI: Double-click event on component {}", pressed_id));
                            events.push((pressed_id, UIEvent::DoubleClick { x: mouse_x, y: mouse_y, button }));
                            self.last_click_time = None; // Reset after double-click
                            self.last_clicked_component = None;
                        } else {
                            crate::core::logging::log_debug(&format!("UI: Click event on component {} at ({}, {})", pressed_id, mouse_x, mouse_y));
                            events.push((pressed_id, UIEvent::Click { x: mouse_x, y: mouse_y, button }));
                            self.last_click_time = Some(now);
                            self.last_clicked_component = Some(pressed_id);
                        }
                    }

                    self.pressed_component = None;
                    self.input_consumed = true;
                }
            }

            self.prev_mouse_buttons[idx] = is_down;
        }

        events
    }

    /// Check if input was consumed by UI this frame
    pub fn is_input_consumed(&self) -> bool {
        self.input_consumed
    }

    /// Get currently hovered component
    pub fn hovered_component(&self) -> Option<u32> {
        self.hovered_component
    }

    /// Get currently focused component
    pub fn focused_component(&self) -> Option<u32> {
        self.focused_component
    }

    /// Clear focus
    pub fn clear_focus(&mut self) -> Option<u32> {
        self.focused_component.take()
    }
}

impl Default for UIEventManager {
    fn default() -> Self {
        Self::new()
    }
}
