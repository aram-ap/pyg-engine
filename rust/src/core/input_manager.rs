use std::collections::{HashMap, HashSet, VecDeque};
use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::keyboard::{Key, NamedKey};

/// Represents a mouse button state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButtonType {
    Left,
    Right,
    Middle,
    Other(u16),
}

/// Represents a joystick/gamepad button
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JoystickButton {
    pub joystick_id: u32,
    pub button_id: u8,
}

/// Represents a joystick/gamepad axis identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JoystickAxis {
    pub joystick_id: u32,
    pub axis_id: u8,
}

/// Logical mouse axes that can contribute to an input axis
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseAxisType {
    X,
    Y,
    WheelX,
    WheelY,
}

/// Keyboard axis binding: a set of positive and negative keys
#[derive(Debug, Clone)]
pub struct KeyboardAxisBinding {
    /// Keys that contribute a positive value (e.g., D, RightArrow)
    pub positive_keys: Vec<Key>,
    /// Keys that contribute a negative value (e.g., A, LeftArrow)
    pub negative_keys: Vec<Key>,
    /// Multiplier applied to the resulting value from keyboard input
    pub sensitivity: f32,
}

/// Mouse axis binding: maps a mouse axis to a logical axis
#[derive(Debug, Clone)]
pub struct MouseAxisBinding {
    /// Which mouse axis drives this binding
    pub axis: MouseAxisType,
    /// Multiplier applied to the mouse delta
    pub sensitivity: f32,
    /// Whether to invert the axis (useful for Y)
    pub invert: bool,
}

/// Joystick axis binding: maps a joystick axis to a logical axis
#[derive(Debug, Clone)]
pub struct JoystickAxisBinding {
    /// Optional specific joystick; if None, any joystick may contribute
    pub joystick_id: Option<u32>,
    /// Which joystick axis drives this binding
    pub axis: JoystickAxis,
    /// Deadzone below which values are treated as zero
    pub deadzone: f32,
    /// Multiplier applied to the joystick value
    pub sensitivity: f32,
    /// Whether to invert the axis
    pub invert: bool,
}

/// Complete binding configuration for a single logical axis
#[derive(Debug, Clone)]
pub struct AxisBinding {
    /// Optional keyboard contribution for this axis
    pub keyboard: Option<KeyboardAxisBinding>,
    /// Optional mouse contribution for this axis
    pub mouse: Option<MouseAxisBinding>,
    /// Optional joystick contribution for this axis
    pub joystick: Option<JoystickAxisBinding>,
}

/// Input event types that can be queued
#[derive(Debug, Clone)]
pub enum InputEvent {
    KeyPressed {
        key: Key,
    },
    KeyReleased {
        key: Key,
    },
    MouseButtonPressed {
        button: MouseButtonType,
    },
    MouseButtonReleased {
        button: MouseButtonType,
    },
    MouseMoved {
        x: f64,
        y: f64,
    },
    MouseWheel {
        delta_x: f64,
        delta_y: f64,
    },
    JoystickConnected {
        joystick_id: u32,
    },
    JoystickDisconnected {
        joystick_id: u32,
    },
    JoystickButtonPressed {
        joystick_id: u32,
        button_id: u8,
    },
    JoystickButtonReleased {
        joystick_id: u32,
        button_id: u8,
    },
    JoystickAxisMoved {
        joystick_id: u32,
        axis_id: u8,
        value: f32,
    },
}

/// Manages all input from keyboard, mouse, and joysticks
///
/// Tracks current state of all input devices and maintains an event queue
/// for frame-by-frame input processing. Supports both state queries (is key pressed?)
/// and event-driven input (key just pressed this frame?).
#[derive(Debug)]
pub struct InputManager {
    // Keyboard state
    /// Current frame keyboard state - true if key is currently pressed
    keys_current: HashMap<Key, bool>,
    /// Previous frame keyboard state - used for detecting press/release events
    keys_previous: HashMap<Key, bool>,

    // Mouse state
    /// Current mouse position in window coordinates
    mouse_position: (f64, f64),
    /// Previous mouse position
    mouse_position_previous: (f64, f64),
    /// Current frame mouse button states - true if button is currently pressed
    mouse_buttons_current: HashMap<MouseButtonType, bool>,
    /// Previous frame mouse button states
    mouse_buttons_previous: HashMap<MouseButtonType, bool>,
    /// Mouse wheel delta accumulated this frame
    mouse_wheel_delta: (f64, f64),

    // Joystick/Gamepad state
    /// Set of connected joystick IDs
    #[allow(dead_code)]
    connected_joysticks: HashSet<u32>,
    /// Current frame joystick button states - (joystick_id, button_id) -> pressed
    joystick_buttons_current: HashMap<JoystickButton, bool>,
    /// Previous frame joystick button states
    joystick_buttons_previous: HashMap<JoystickButton, bool>,
    /// Current joystick axis values - (joystick_id, axis_id) -> value (-1.0 to 1.0)
    joystick_axes: HashMap<JoystickAxis, f32>,

    // Unified axis system
    /// Configuration for each logical axis (e.g., \"Horizontal\", \"Vertical\", \"Fire\")
    axis_bindings: HashMap<String, AxisBinding>,
    /// Current frame axis values, after combining keyboard/mouse/joystick
    axis_values_current: HashMap<String, f32>,
    /// Previous frame axis values (for detecting changes / edge triggers)
    axis_values_previous: HashMap<String, f32>,

    // Event queue
    /// Queue of input events that occurred this frame
    event_queue: VecDeque<InputEvent>,

    // Input action mappings (optional - for action-based input)
    /// Maps action names to sets of keys that trigger them
    key_action_mappings: HashMap<String, Vec<Key>>,
    /// Maps action names to sets of mouse buttons that trigger them
    mouse_action_mappings: HashMap<String, Vec<MouseButtonType>>,
    /// Maps action names to sets of joystick buttons that trigger them
    joystick_action_mappings: HashMap<String, Vec<JoystickButton>>,
}

impl InputManager {
    fn normalize_action_name(name: &str) -> String {
        name.trim().to_ascii_lowercase()
    }

    fn find_axis_name_case_insensitive(&self, name: &str) -> Option<String> {
        if self.axis_bindings.contains_key(name) {
            return Some(name.to_string());
        }

        self.axis_bindings
            .keys()
            .find(|axis_name| axis_name.eq_ignore_ascii_case(name))
            .cloned()
    }

    /// Create a new InputManager with default state and default axis bindings.
    ///
    /// The default bindings are:
    /// - "Horizontal": A/D, Left/Right arrows, primary gamepad left-stick X
    /// - "Vertical": W/S, Up/Down arrows, primary gamepad left-stick Y
    /// - "Jump", "Fire1", "Fire2", "Fire3", "Submit", "Cancel",
    ///   "Mouse X", "Mouse Y", "Mouse ScrollWheel", "Escape"
    pub fn new() -> Self {
        let mut manager = Self {
            keys_current: HashMap::new(),
            keys_previous: HashMap::new(),
            mouse_position: (0.0, 0.0),
            mouse_position_previous: (0.0, 0.0),
            mouse_buttons_current: HashMap::new(),
            mouse_buttons_previous: HashMap::new(),
            mouse_wheel_delta: (0.0, 0.0),
            connected_joysticks: HashSet::new(),
            joystick_buttons_current: HashMap::new(),
            joystick_buttons_previous: HashMap::new(),
            joystick_axes: HashMap::new(),
            event_queue: VecDeque::new(),
            key_action_mappings: HashMap::new(),
            mouse_action_mappings: HashMap::new(),
            joystick_action_mappings: HashMap::new(),
            axis_bindings: HashMap::new(),
            axis_values_current: HashMap::new(),
            axis_values_previous: HashMap::new(),
        };

        // Install default axis bindings
        manager.axis_bindings = Self::default_axis_bindings();
        // Install default action mappings (Unity-style names).
        manager.key_action_mappings = Self::default_key_action_mappings();
        manager.mouse_action_mappings = Self::default_mouse_action_mappings();
        manager.joystick_action_mappings = Self::default_joystick_action_mappings();

        manager
    }

    /// Process a winit `WindowEvent` and update the internal input state.
    ///
    /// This method should be called from your window event loop for each event.
    pub fn handle_window_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::Focused(false) => {
                // macOS fullscreen transitions can temporarily drop focus.
                // If we do not clear state here, keys/buttons may remain
                // "stuck" because release events are not guaranteed.
                self.clear_on_focus_lost();
            }
            WindowEvent::KeyboardInput { event, .. } => {
                let key = event.logical_key.clone();
                let pressed = event.state == ElementState::Pressed;
                self.keys_current.insert(key.clone(), pressed);

                if pressed {
                    self.event_queue.push_back(InputEvent::KeyPressed { key });
                } else {
                    self.event_queue.push_back(InputEvent::KeyReleased { key });
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let mapped = Self::map_mouse_button(*button);
                let pressed = *state == ElementState::Pressed;
                self.mouse_buttons_current.insert(mapped, pressed);

                if pressed {
                    self.event_queue
                        .push_back(InputEvent::MouseButtonPressed { button: mapped });
                } else {
                    self.event_queue
                        .push_back(InputEvent::MouseButtonReleased { button: mapped });
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_position = (position.x, position.y);
                self.event_queue.push_back(InputEvent::MouseMoved {
                    x: position.x,
                    y: position.y,
                });
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let (dx, dy) = match delta {
                    MouseScrollDelta::LineDelta(x, y) => (*x as f64, *y as f64),
                    MouseScrollDelta::PixelDelta(pos) => (pos.x, pos.y),
                };

                self.mouse_wheel_delta.0 += dx;
                self.mouse_wheel_delta.1 += dy;

                self.event_queue.push_back(InputEvent::MouseWheel {
                    delta_x: dx,
                    delta_y: dy,
                });
            }
            _ => {}
        }
    }

    /// Clear transient input state when window focus is lost.
    fn clear_on_focus_lost(&mut self) {
        self.keys_current.clear();
        self.mouse_buttons_current.clear();
        self.mouse_wheel_delta = (0.0, 0.0);
        self.mouse_position_previous = self.mouse_position;
    }

    /// Build the default axis bindings used by `new`.
    fn default_axis_bindings() -> HashMap<String, AxisBinding> {
        let mut bindings = HashMap::new();

        // Horizontal axis: A/D, Left/Right arrows, gamepad left-stick X (axis 0)
        let horizontal_keyboard = KeyboardAxisBinding {
            positive_keys: vec![Key::Character("d".into()), Key::Named(NamedKey::ArrowRight)],
            negative_keys: vec![Key::Character("a".into()), Key::Named(NamedKey::ArrowLeft)],
            sensitivity: 1.0,
        };
        let horizontal_joystick = JoystickAxisBinding {
            joystick_id: None,
            axis: JoystickAxis {
                joystick_id: 0,
                axis_id: 0,
            },
            deadzone: 0.15,
            sensitivity: 1.0,
            invert: false,
        };
        bindings.insert(
            "Horizontal".to_string(),
            AxisBinding {
                keyboard: Some(horizontal_keyboard),
                mouse: None,
                joystick: Some(horizontal_joystick),
            },
        );

        // Vertical axis: W/S, Up/Down arrows, gamepad left-stick Y (axis 1)
        let vertical_keyboard = KeyboardAxisBinding {
            positive_keys: vec![Key::Character("w".into()), Key::Named(NamedKey::ArrowUp)],
            negative_keys: vec![Key::Character("s".into()), Key::Named(NamedKey::ArrowDown)],
            sensitivity: 1.0,
        };
        let vertical_joystick = JoystickAxisBinding {
            joystick_id: None,
            axis: JoystickAxis {
                joystick_id: 0,
                axis_id: 1,
            },
            deadzone: 0.15,
            sensitivity: 1.0,
            invert: false,
        };
        bindings.insert(
            "Vertical".to_string(),
            AxisBinding {
                keyboard: Some(vertical_keyboard),
                mouse: None,
                joystick: Some(vertical_joystick),
            },
        );

        // Unity-style button-like axes.
        bindings.insert(
            "Jump".to_string(),
            AxisBinding {
                keyboard: Some(KeyboardAxisBinding {
                    positive_keys: vec![Key::Named(NamedKey::Space)],
                    negative_keys: Vec::new(),
                    sensitivity: 1.0,
                }),
                mouse: None,
                joystick: None,
            },
        );
        bindings.insert(
            "Fire1".to_string(),
            AxisBinding {
                keyboard: Some(KeyboardAxisBinding {
                    positive_keys: vec![Key::Named(NamedKey::Control)],
                    negative_keys: Vec::new(),
                    sensitivity: 1.0,
                }),
                mouse: None,
                joystick: None,
            },
        );
        bindings.insert(
            "Fire2".to_string(),
            AxisBinding {
                keyboard: Some(KeyboardAxisBinding {
                    positive_keys: vec![Key::Named(NamedKey::Alt)],
                    negative_keys: Vec::new(),
                    sensitivity: 1.0,
                }),
                mouse: None,
                joystick: None,
            },
        );
        bindings.insert(
            "Fire3".to_string(),
            AxisBinding {
                keyboard: Some(KeyboardAxisBinding {
                    positive_keys: vec![Key::Named(NamedKey::Super), Key::Character("j".into())],
                    negative_keys: Vec::new(),
                    sensitivity: 1.0,
                }),
                mouse: None,
                joystick: None,
            },
        );
        bindings.insert(
            "Escape".to_string(),
            AxisBinding {
                keyboard: Some(KeyboardAxisBinding {
                    positive_keys: vec![Key::Named(NamedKey::Escape)],
                    negative_keys: Vec::new(),
                    sensitivity: 1.0,
                }),
                mouse: None,
                joystick: None,
            },
        );
        bindings.insert(
            "Submit".to_string(),
            AxisBinding {
                keyboard: Some(KeyboardAxisBinding {
                    positive_keys: vec![Key::Named(NamedKey::Enter), Key::Named(NamedKey::Space)],
                    negative_keys: Vec::new(),
                    sensitivity: 1.0,
                }),
                mouse: None,
                joystick: None,
            },
        );
        bindings.insert(
            "Cancel".to_string(),
            AxisBinding {
                keyboard: Some(KeyboardAxisBinding {
                    positive_keys: vec![Key::Named(NamedKey::Escape)],
                    negative_keys: Vec::new(),
                    sensitivity: 1.0,
                }),
                mouse: None,
                joystick: None,
            },
        );
        bindings.insert(
            "Mouse X".to_string(),
            AxisBinding {
                keyboard: None,
                mouse: Some(MouseAxisBinding {
                    axis: MouseAxisType::X,
                    sensitivity: 1.0,
                    invert: false,
                }),
                joystick: None,
            },
        );
        bindings.insert(
            "Mouse Y".to_string(),
            AxisBinding {
                keyboard: None,
                mouse: Some(MouseAxisBinding {
                    axis: MouseAxisType::Y,
                    sensitivity: 1.0,
                    invert: false,
                }),
                joystick: None,
            },
        );
        bindings.insert(
            "Mouse ScrollWheel".to_string(),
            AxisBinding {
                keyboard: None,
                mouse: Some(MouseAxisBinding {
                    axis: MouseAxisType::WheelY,
                    sensitivity: 1.0,
                    invert: false,
                }),
                joystick: None,
            },
        );

        bindings
    }

    fn default_key_action_mappings() -> HashMap<String, Vec<Key>> {
        let mut mappings = HashMap::new();
        mappings.insert("jump".to_string(), vec![Key::Named(NamedKey::Space)]);
        mappings.insert(
            "fire1".to_string(),
            vec![Key::Named(NamedKey::Control), Key::Character("z".into())],
        );
        mappings.insert("fire2".to_string(), vec![Key::Named(NamedKey::Alt)]);
        mappings.insert(
            "fire3".to_string(),
            vec![Key::Named(NamedKey::Super), Key::Character("j".into())],
        );
        mappings.insert(
            "submit".to_string(),
            vec![Key::Named(NamedKey::Enter), Key::Named(NamedKey::Space)],
        );
        mappings.insert("cancel".to_string(), vec![Key::Named(NamedKey::Escape)]);
        mappings.insert("escape".to_string(), vec![Key::Named(NamedKey::Escape)]);
        mappings.insert("sprint".to_string(), vec![Key::Named(NamedKey::Shift)]);
        mappings
    }

    fn default_mouse_action_mappings() -> HashMap<String, Vec<MouseButtonType>> {
        let mut mappings = HashMap::new();
        mappings.insert("fire1".to_string(), vec![MouseButtonType::Left]);
        mappings.insert("fire2".to_string(), vec![MouseButtonType::Right]);
        mappings.insert("fire3".to_string(), vec![MouseButtonType::Middle]);
        mappings
    }

    fn default_joystick_action_mappings() -> HashMap<String, Vec<JoystickButton>> {
        let mut mappings = HashMap::new();
        mappings.insert(
            "jump".to_string(),
            vec![JoystickButton {
                joystick_id: 0,
                button_id: 0,
            }],
        );
        mappings.insert(
            "submit".to_string(),
            vec![JoystickButton {
                joystick_id: 0,
                button_id: 0,
            }],
        );
        mappings.insert(
            "cancel".to_string(),
            vec![JoystickButton {
                joystick_id: 0,
                button_id: 1,
            }],
        );
        mappings.insert(
            "fire1".to_string(),
            vec![JoystickButton {
                joystick_id: 0,
                button_id: 0,
            }],
        );
        mappings.insert(
            "fire2".to_string(),
            vec![JoystickButton {
                joystick_id: 0,
                button_id: 1,
            }],
        );
        mappings.insert(
            "fire3".to_string(),
            vec![JoystickButton {
                joystick_id: 0,
                button_id: 2,
            }],
        );
        mappings
    }

    /// Replace the full binding for a logical axis.
    ///
    /// If the axis does not exist yet it will be created; otherwise, its
    /// binding will be overwritten.
    pub fn set_axis_binding<S: Into<String>>(&mut self, name: S, binding: AxisBinding) {
        let requested_name = name.into();
        let resolved_name = self
            .find_axis_name_case_insensitive(&requested_name)
            .unwrap_or(requested_name);
        self.axis_bindings.insert(resolved_name, binding);
    }

    /// Update or create the keyboard binding for a logical axis.
    pub fn set_axis_keyboard_binding<S: Into<String>>(
        &mut self,
        name: S,
        keyboard: KeyboardAxisBinding,
    ) {
        let requested_name = name.into();
        let name = self
            .find_axis_name_case_insensitive(&requested_name)
            .unwrap_or(requested_name);
        self.axis_bindings
            .entry(name)
            .and_modify(|axis| axis.keyboard = Some(keyboard.clone()))
            .or_insert_with(|| AxisBinding {
                keyboard: Some(keyboard),
                mouse: None,
                joystick: None,
            });
    }

    /// Update or create the mouse binding for a logical axis.
    pub fn set_axis_mouse_binding<S: Into<String>>(&mut self, name: S, mouse: MouseAxisBinding) {
        let requested_name = name.into();
        let name = self
            .find_axis_name_case_insensitive(&requested_name)
            .unwrap_or(requested_name);
        self.axis_bindings
            .entry(name)
            .and_modify(|axis| axis.mouse = Some(mouse.clone()))
            .or_insert_with(|| AxisBinding {
                keyboard: None,
                mouse: Some(mouse),
                joystick: None,
            });
    }

    /// Update or create the joystick binding for a logical axis.
    pub fn set_axis_joystick_binding<S: Into<String>>(
        &mut self,
        name: S,
        joystick: JoystickAxisBinding,
    ) {
        let requested_name = name.into();
        let name = self
            .find_axis_name_case_insensitive(&requested_name)
            .unwrap_or(requested_name);
        self.axis_bindings
            .entry(name)
            .and_modify(|axis| axis.joystick = Some(joystick.clone()))
            .or_insert_with(|| AxisBinding {
                keyboard: None,
                mouse: None,
                joystick: Some(joystick),
            });
    }

    /// Configure keyboard keys for a logical axis (replaces existing keys).
    pub fn set_axis_keyboard_keys<S: Into<String>>(
        &mut self,
        name: S,
        positive_keys: Vec<Key>,
        negative_keys: Vec<Key>,
        sensitivity: f32,
    ) {
        self.set_axis_keyboard_binding(
            name,
            KeyboardAxisBinding {
                positive_keys,
                negative_keys,
                sensitivity,
            },
        );
    }

    /// Add a positive keyboard key to a logical axis.
    pub fn add_axis_positive_key<S: Into<String>>(&mut self, name: S, key: Key) {
        let requested_name = name.into();
        let name = self
            .find_axis_name_case_insensitive(&requested_name)
            .unwrap_or(requested_name);
        let axis = self
            .axis_bindings
            .entry(name)
            .or_insert_with(|| AxisBinding {
                keyboard: None,
                mouse: None,
                joystick: None,
            });
        let keyboard = axis.keyboard.get_or_insert_with(|| KeyboardAxisBinding {
            positive_keys: Vec::new(),
            negative_keys: Vec::new(),
            sensitivity: 1.0,
        });
        if !keyboard.positive_keys.contains(&key) {
            keyboard.positive_keys.push(key);
        }
    }

    /// Add a negative keyboard key to a logical axis.
    pub fn add_axis_negative_key<S: Into<String>>(&mut self, name: S, key: Key) {
        let requested_name = name.into();
        let name = self
            .find_axis_name_case_insensitive(&requested_name)
            .unwrap_or(requested_name);
        let axis = self
            .axis_bindings
            .entry(name)
            .or_insert_with(|| AxisBinding {
                keyboard: None,
                mouse: None,
                joystick: None,
            });
        let keyboard = axis.keyboard.get_or_insert_with(|| KeyboardAxisBinding {
            positive_keys: Vec::new(),
            negative_keys: Vec::new(),
            sensitivity: 1.0,
        });
        if !keyboard.negative_keys.contains(&key) {
            keyboard.negative_keys.push(key);
        }
    }

    /// Remove a positive keyboard key from a logical axis.
    pub fn remove_axis_positive_key(&mut self, axis_name: &str, key: &Key) -> bool {
        let Some(name) = self.find_axis_name_case_insensitive(axis_name) else {
            return false;
        };
        let Some(axis) = self.axis_bindings.get_mut(&name) else {
            return false;
        };
        let Some(keyboard) = axis.keyboard.as_mut() else {
            return false;
        };
        if let Some(index) = keyboard
            .positive_keys
            .iter()
            .position(|existing| existing == key)
        {
            keyboard.positive_keys.swap_remove(index);
            return true;
        }
        false
    }

    /// Remove a negative keyboard key from a logical axis.
    pub fn remove_axis_negative_key(&mut self, axis_name: &str, key: &Key) -> bool {
        let Some(name) = self.find_axis_name_case_insensitive(axis_name) else {
            return false;
        };
        let Some(axis) = self.axis_bindings.get_mut(&name) else {
            return false;
        };
        let Some(keyboard) = axis.keyboard.as_mut() else {
            return false;
        };
        if let Some(index) = keyboard
            .negative_keys
            .iter()
            .position(|existing| existing == key)
        {
            keyboard.negative_keys.swap_remove(index);
            return true;
        }
        false
    }

    /// Remove an axis binding by name.
    pub fn remove_axis(&mut self, axis_name: &str) -> bool {
        let Some(name) = self.find_axis_name_case_insensitive(axis_name) else {
            return false;
        };
        self.axis_bindings.remove(&name).is_some()
    }

    /// Return all logical axis names in sorted order.
    pub fn axis_names(&self) -> Vec<String> {
        let mut names: Vec<String> = self.axis_bindings.keys().cloned().collect();
        names.sort();
        names
    }

    /// Update input state and recompute all logical axis values for this frame.
    ///
    /// This should be called once per frame, after any raw input events have
    /// been applied to the underlying keyboard/mouse/joystick state.
    pub fn update(&mut self) {
        // Reuse axis buffers by swapping snapshots before recomputation.
        std::mem::swap(
            &mut self.axis_values_previous,
            &mut self.axis_values_current,
        );

        // Recompute axis values from current device state. The previous-axis
        // snapshot was swapped above so edge detection remains valid.
        self.axis_values_current.clear();
        for (name, binding) in &self.axis_bindings {
            let mut value: f32 = 0.0;

            // Keyboard contribution
            if let Some(kb) = &binding.keyboard {
                value += self.compute_keyboard_axis(kb);
            }

            // Mouse contribution
            if let Some(mouse) = &binding.mouse {
                value += self.compute_mouse_axis(mouse);
            }

            // Joystick contribution
            if let Some(js) = &binding.joystick {
                value += self.compute_joystick_axis(js);
            }

            // Clamp combined value to [-1.0, 1.0]
            let clamped = value.clamp(-1.0, 1.0);
            self.axis_values_current.insert(name.clone(), clamped);
        }

        // Clear per-frame accumulators that should not persist
        self.event_queue.clear();
        self.mouse_wheel_delta = (0.0, 0.0);

        // Carry over current state for next-frame edge detection.
        self.keys_previous.clone_from(&self.keys_current);
        self.mouse_position_previous = self.mouse_position;
        self.mouse_buttons_previous
            .clone_from(&self.mouse_buttons_current);
        self.joystick_buttons_previous
            .clone_from(&self.joystick_buttons_current);
    }

    /// Get the current value of a logical axis.
    ///
    /// Returns 0.0 if the axis is not defined.
    pub fn axis(&self, name: &str) -> f32 {
        if let Some(value) = self.axis_values_current.get(name) {
            return *value;
        }

        self.axis_values_current
            .iter()
            .find_map(|(axis_name, value)| {
                if axis_name.eq_ignore_ascii_case(name) {
                    Some(*value)
                } else {
                    None
                }
            })
            .unwrap_or(0.0)
    }

    /// Get the previous frame's value of a logical axis.
    ///
    /// Returns 0.0 if the axis is not defined.
    pub fn axis_previous(&self, name: &str) -> f32 {
        if let Some(value) = self.axis_values_previous.get(name) {
            return *value;
        }

        self.axis_values_previous
            .iter()
            .find_map(|(axis_name, value)| {
                if axis_name.eq_ignore_ascii_case(name) {
                    Some(*value)
                } else {
                    None
                }
            })
            .unwrap_or(0.0)
    }

    /// Check if an action is currently active (held).
    pub fn action_down(&self, action_name: &str) -> bool {
        let action = Self::normalize_action_name(action_name);
        if let Some(keys) = self.key_action_mappings.get(&action)
            && keys.iter().any(|key| self.key_down(key))
        {
            return true;
        }

        if let Some(buttons) = self.mouse_action_mappings.get(&action)
            && buttons.iter().any(|button| self.mouse_button_down(*button))
        {
            return true;
        }

        if let Some(buttons) = self.joystick_action_mappings.get(&action)
            && buttons
                .iter()
                .any(|button| self.joystick_button_down(button.joystick_id, button.button_id))
        {
            return true;
        }

        false
    }

    /// Check if an action was pressed this frame.
    pub fn action_pressed(&self, action_name: &str) -> bool {
        let action = Self::normalize_action_name(action_name);
        if let Some(keys) = self.key_action_mappings.get(&action)
            && keys.iter().any(|key| self.key_pressed(key))
        {
            return true;
        }

        if let Some(buttons) = self.mouse_action_mappings.get(&action)
            && buttons
                .iter()
                .any(|button| self.mouse_button_pressed(*button))
        {
            return true;
        }

        if let Some(buttons) = self.joystick_action_mappings.get(&action)
            && buttons
                .iter()
                .any(|button| self.joystick_button_pressed(button.joystick_id, button.button_id))
        {
            return true;
        }

        false
    }

    /// Check if an action was released this frame.
    pub fn action_released(&self, action_name: &str) -> bool {
        let action = Self::normalize_action_name(action_name);
        if let Some(keys) = self.key_action_mappings.get(&action)
            && keys.iter().any(|key| self.key_released(key))
        {
            return true;
        }

        if let Some(buttons) = self.mouse_action_mappings.get(&action)
            && buttons
                .iter()
                .any(|button| self.mouse_button_released(*button))
        {
            return true;
        }

        if let Some(buttons) = self.joystick_action_mappings.get(&action)
            && buttons
                .iter()
                .any(|button| self.joystick_button_released(button.joystick_id, button.button_id))
        {
            return true;
        }

        false
    }

    /// Return all action names in sorted order.
    pub fn action_names(&self) -> Vec<String> {
        let mut names = HashSet::new();
        names.extend(self.key_action_mappings.keys().cloned());
        names.extend(self.mouse_action_mappings.keys().cloned());
        names.extend(self.joystick_action_mappings.keys().cloned());
        let mut ordered: Vec<String> = names.into_iter().collect();
        ordered.sort();
        ordered
    }

    /// Replace keyboard bindings for an action.
    pub fn set_action_keys<S: Into<String>>(&mut self, action_name: S, keys: Vec<Key>) {
        self.key_action_mappings
            .insert(Self::normalize_action_name(&action_name.into()), keys);
    }

    /// Add one keyboard key to an action binding.
    pub fn add_action_key<S: Into<String>>(&mut self, action_name: S, key: Key) {
        let action = Self::normalize_action_name(&action_name.into());
        let keys = self.key_action_mappings.entry(action).or_default();
        if !keys.contains(&key) {
            keys.push(key);
        }
    }

    /// Remove one keyboard key from an action binding.
    pub fn remove_action_key(&mut self, action_name: &str, key: &Key) -> bool {
        let action = Self::normalize_action_name(action_name);
        let Some(keys) = self.key_action_mappings.get_mut(&action) else {
            return false;
        };
        if let Some(index) = keys.iter().position(|existing| existing == key) {
            keys.swap_remove(index);
            return true;
        }
        false
    }

    /// Replace mouse button bindings for an action.
    pub fn set_action_mouse_buttons<S: Into<String>>(
        &mut self,
        action_name: S,
        buttons: Vec<MouseButtonType>,
    ) {
        self.mouse_action_mappings
            .insert(Self::normalize_action_name(&action_name.into()), buttons);
    }

    /// Add one mouse button to an action binding.
    pub fn add_action_mouse_button<S: Into<String>>(
        &mut self,
        action_name: S,
        button: MouseButtonType,
    ) {
        let action = Self::normalize_action_name(&action_name.into());
        let buttons = self.mouse_action_mappings.entry(action).or_default();
        if !buttons.contains(&button) {
            buttons.push(button);
        }
    }

    /// Remove one mouse button from an action binding.
    pub fn remove_action_mouse_button(
        &mut self,
        action_name: &str,
        button: MouseButtonType,
    ) -> bool {
        let action = Self::normalize_action_name(action_name);
        let Some(buttons) = self.mouse_action_mappings.get_mut(&action) else {
            return false;
        };
        if let Some(index) = buttons.iter().position(|existing| *existing == button) {
            buttons.swap_remove(index);
            return true;
        }
        false
    }

    /// Remove all bindings for an action.
    pub fn clear_action_bindings(&mut self, action_name: &str) {
        let action = Self::normalize_action_name(action_name);
        self.key_action_mappings.remove(&action);
        self.mouse_action_mappings.remove(&action);
        self.joystick_action_mappings.remove(&action);
    }

    /// Restore default axis and action bindings.
    pub fn reset_input_bindings_to_defaults(&mut self) {
        self.axis_bindings = Self::default_axis_bindings();
        self.key_action_mappings = Self::default_key_action_mappings();
        self.mouse_action_mappings = Self::default_mouse_action_mappings();
        self.joystick_action_mappings = Self::default_joystick_action_mappings();
    }

    /// Check if a keyboard key is currently held down.
    pub fn key_down(&self, key: &Key) -> bool {
        *self.keys_current.get(key).unwrap_or(&false)
    }

    /// Check if a keyboard key was pressed this frame (up last frame, down now).
    pub fn key_pressed(&self, key: &Key) -> bool {
        let now = *self.keys_current.get(key).unwrap_or(&false);
        let before = *self.keys_previous.get(key).unwrap_or(&false);
        now && !before
    }

    /// Check if a keyboard key was released this frame (down last frame, up now).
    pub fn key_released(&self, key: &Key) -> bool {
        let now = *self.keys_current.get(key).unwrap_or(&false);
        let before = *self.keys_previous.get(key).unwrap_or(&false);
        !now && before
    }

    /// Check if a mouse button is currently held down.
    pub fn mouse_button_down(&self, button: MouseButtonType) -> bool {
        *self.mouse_buttons_current.get(&button).unwrap_or(&false)
    }

    /// Check if a mouse button was pressed this frame.
    pub fn mouse_button_pressed(&self, button: MouseButtonType) -> bool {
        let now = *self.mouse_buttons_current.get(&button).unwrap_or(&false);
        let before = *self.mouse_buttons_previous.get(&button).unwrap_or(&false);
        now && !before
    }

    /// Check if a mouse button was released this frame.
    pub fn mouse_button_released(&self, button: MouseButtonType) -> bool {
        let now = *self.mouse_buttons_current.get(&button).unwrap_or(&false);
        let before = *self.mouse_buttons_previous.get(&button).unwrap_or(&false);
        !now && before
    }

    /// Get the current mouse position in window coordinates.
    pub fn mouse_position(&self) -> (f64, f64) {
        self.mouse_position
    }

    /// Get the mouse movement delta for this frame.
    pub fn mouse_delta(&self) -> (f64, f64) {
        (
            self.mouse_position.0 - self.mouse_position_previous.0,
            self.mouse_position.1 - self.mouse_position_previous.1,
        )
    }

    /// Get the mouse wheel delta accumulated this frame.
    pub fn mouse_wheel(&self) -> (f64, f64) {
        self.mouse_wheel_delta
    }

    /// Check if a joystick button is currently held down.
    pub fn joystick_button_down(&self, joystick_id: u32, button_id: u8) -> bool {
        let key = JoystickButton {
            joystick_id,
            button_id,
        };
        *self.joystick_buttons_current.get(&key).unwrap_or(&false)
    }

    /// Check if a joystick button was pressed this frame.
    pub fn joystick_button_pressed(&self, joystick_id: u32, button_id: u8) -> bool {
        let key = JoystickButton {
            joystick_id,
            button_id,
        };
        let now = *self.joystick_buttons_current.get(&key).unwrap_or(&false);
        let before = *self.joystick_buttons_previous.get(&key).unwrap_or(&false);
        now && !before
    }

    /// Check if a joystick button was released this frame.
    pub fn joystick_button_released(&self, joystick_id: u32, button_id: u8) -> bool {
        let key = JoystickButton {
            joystick_id,
            button_id,
        };
        let now = *self.joystick_buttons_current.get(&key).unwrap_or(&false);
        let before = *self.joystick_buttons_previous.get(&key).unwrap_or(&false);
        !now && before
    }

    /// Get the current value of a joystick axis (-1.0 to 1.0).
    pub fn joystick_axis(&self, joystick_id: u32, axis_id: u8) -> f32 {
        let key = JoystickAxis {
            joystick_id,
            axis_id,
        };
        *self.joystick_axes.get(&key).unwrap_or(&0.0)
    }

    /// Compute the keyboard contribution to a logical axis.
    fn compute_keyboard_axis(&self, binding: &KeyboardAxisBinding) -> f32 {
        let mut value: f32 = 0.0;

        for key in &binding.positive_keys {
            if *self.keys_current.get(key).unwrap_or(&false) {
                value += 1.0;
            }
        }

        for key in &binding.negative_keys {
            if *self.keys_current.get(key).unwrap_or(&false) {
                value -= 1.0;
            }
        }

        // Normalize (if multiple keys are held) and apply sensitivity
        if value > 1.0 {
            value = 1.0;
        } else if value < -1.0 {
            value = -1.0;
        }

        value * binding.sensitivity
    }

    /// Compute the mouse contribution to a logical axis.
    ///
    /// Mouse X/Y are based on position delta; WheelX/WheelY use accumulated scroll.
    fn compute_mouse_axis(&self, binding: &MouseAxisBinding) -> f32 {
        let raw = match binding.axis {
            MouseAxisType::X => (self.mouse_position.0 - self.mouse_position_previous.0) as f32,
            MouseAxisType::Y => (self.mouse_position.1 - self.mouse_position_previous.1) as f32,
            MouseAxisType::WheelX => self.mouse_wheel_delta.0 as f32,
            MouseAxisType::WheelY => self.mouse_wheel_delta.1 as f32,
        };

        let mut value = raw * binding.sensitivity;
        if binding.invert {
            value = -value;
        }

        value
    }

    /// Compute the joystick contribution to a logical axis.
    fn compute_joystick_axis(&self, binding: &JoystickAxisBinding) -> f32 {
        // Resolve which joystick axis value to read
        let axis_value = if let Some(joy_id) = binding.joystick_id {
            // Specific joystick requested
            let key = JoystickAxis {
                joystick_id: joy_id,
                axis_id: binding.axis.axis_id,
            };
            *self.joystick_axes.get(&key).unwrap_or(&0.0)
        } else {
            // Any joystick may contribute: use the first matching axis_id
            self.joystick_axes
                .iter()
                .find_map(|(axis, value)| {
                    if axis.axis_id == binding.axis.axis_id {
                        Some(*value)
                    } else {
                        None
                    }
                })
                .unwrap_or(0.0)
        };

        // Apply deadzone
        let mut value = if axis_value.abs() < binding.deadzone {
            0.0
        } else {
            axis_value
        };

        // Apply inversion and sensitivity
        if binding.invert {
            value = -value;
        }

        value * binding.sensitivity
    }

    /// Map a winit mouse button to the engine's `MouseButtonType`.
    fn map_mouse_button(button: MouseButton) -> MouseButtonType {
        match button {
            MouseButton::Left => MouseButtonType::Left,
            MouseButton::Right => MouseButtonType::Right,
            MouseButton::Middle => MouseButtonType::Middle,
            MouseButton::Other(id) => MouseButtonType::Other(id),
            _ => MouseButtonType::Other(0),
        }
    }
}
