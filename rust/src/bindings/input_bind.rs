use pyo3::prelude::*;
use winit::keyboard::{Key, NamedKey, NativeKey};

use crate::core::input_manager::MouseButtonType;

#[pyclass(name = "MouseButton")]
pub struct PyMouseButton;

#[pymethods]
impl PyMouseButton {
    #[classattr]
    const LEFT: &'static str = "left";
    #[classattr]
    const RIGHT: &'static str = "right";
    #[classattr]
    const MIDDLE: &'static str = "middle";
}

/// Helper to parse a mouse button name from Python into an engine mouse button.
pub fn parse_mouse_button(button_name: &str) -> MouseButtonType {
    match button_name.to_lowercase().as_str() {
        "left" => MouseButtonType::Left,
        "right" => MouseButtonType::Right,
        "middle" => MouseButtonType::Middle,
        // Fallback for unsupported names.
        _ => MouseButtonType::Other(0),
    }
}

/// Helper to parse a string key code from Python into a winit Key
pub fn parse_key(key_name: &str) -> Key {
    match key_name.to_lowercase().as_str() {
        // Named keys
        "escape" | "esc" => Key::Named(NamedKey::Escape),
        "enter" | "return" => Key::Named(NamedKey::Enter),
        "space" => Key::Named(NamedKey::Space),
        "backspace" => Key::Named(NamedKey::Backspace),
        "tab" => Key::Named(NamedKey::Tab),
        "arrowup" | "up" => Key::Named(NamedKey::ArrowUp),
        "arrowdown" | "down" => Key::Named(NamedKey::ArrowDown),
        "arrowleft" | "left" => Key::Named(NamedKey::ArrowLeft),
        "arrowright" | "right" => Key::Named(NamedKey::ArrowRight),
        "shift" => Key::Named(NamedKey::Shift),
        "control" | "ctrl" => Key::Named(NamedKey::Control),
        "alt" => Key::Named(NamedKey::Alt),
        "capslock" => Key::Named(NamedKey::CapsLock),
        "f1" => Key::Named(NamedKey::F1),
        "f2" => Key::Named(NamedKey::F2),
        "f3" => Key::Named(NamedKey::F3),
        "f4" => Key::Named(NamedKey::F4),
        "f5" => Key::Named(NamedKey::F5),
        "f6" => Key::Named(NamedKey::F6),
        "f7" => Key::Named(NamedKey::F7),
        "f8" => Key::Named(NamedKey::F8),
        "f9" => Key::Named(NamedKey::F9),
        "f10" => Key::Named(NamedKey::F10),
        "f11" => Key::Named(NamedKey::F11),
        "f12" => Key::Named(NamedKey::F12),
        
        // Single characters
        c if c.len() == 1 => Key::Character(c.into()),
        
        // Fallback
        _ => Key::Unidentified(NativeKey::Unidentified),
    }
}

#[pyclass(name = "Keys")]
pub struct PyKeys;

#[pymethods]
impl PyKeys {
    #[classattr]
    const ESCAPE: &'static str = "Escape";
    #[classattr]
    const ENTER: &'static str = "Enter";
    #[classattr]
    const SPACE: &'static str = "Space";
    #[classattr]
    const BACKSPACE: &'static str = "Backspace";
    #[classattr]
    const TAB: &'static str = "Tab";
    #[classattr]
    const ARROW_UP: &'static str = "ArrowUp";
    #[classattr]
    const ARROW_DOWN: &'static str = "ArrowDown";
    #[classattr]
    const ARROW_LEFT: &'static str = "ArrowLeft";
    #[classattr]
    const ARROW_RIGHT: &'static str = "ArrowRight";
    #[classattr]
    const SHIFT: &'static str = "Shift";
    #[classattr]
    const CONTROL: &'static str = "Control";
    #[classattr]
    const ALT: &'static str = "Alt";
    #[classattr]
    const CAPS_LOCK: &'static str = "CapsLock";
}
