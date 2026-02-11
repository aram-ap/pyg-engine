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
    let normalized: String = key_name
        .trim()
        .chars()
        .flat_map(|ch| ch.to_lowercase())
        .filter(|ch| !matches!(ch, ' ' | '_' | '-'))
        .collect();

    if normalized.len() == 1 {
        return Key::Character(normalized.into());
    }

    if let Some(rest) = normalized.strip_prefix('f')
        && let Ok(index) = rest.parse::<u8>()
    {
        return match index {
            1 => Key::Named(NamedKey::F1),
            2 => Key::Named(NamedKey::F2),
            3 => Key::Named(NamedKey::F3),
            4 => Key::Named(NamedKey::F4),
            5 => Key::Named(NamedKey::F5),
            6 => Key::Named(NamedKey::F6),
            7 => Key::Named(NamedKey::F7),
            8 => Key::Named(NamedKey::F8),
            9 => Key::Named(NamedKey::F9),
            10 => Key::Named(NamedKey::F10),
            11 => Key::Named(NamedKey::F11),
            12 => Key::Named(NamedKey::F12),
            13 => Key::Named(NamedKey::F13),
            14 => Key::Named(NamedKey::F14),
            15 => Key::Named(NamedKey::F15),
            16 => Key::Named(NamedKey::F16),
            17 => Key::Named(NamedKey::F17),
            18 => Key::Named(NamedKey::F18),
            19 => Key::Named(NamedKey::F19),
            20 => Key::Named(NamedKey::F20),
            21 => Key::Named(NamedKey::F21),
            22 => Key::Named(NamedKey::F22),
            23 => Key::Named(NamedKey::F23),
            24 => Key::Named(NamedKey::F24),
            _ => Key::Unidentified(NativeKey::Unidentified),
        };
    }

    match normalized.as_str() {
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
        "insert" => Key::Named(NamedKey::Insert),
        "delete" | "del" => Key::Named(NamedKey::Delete),
        "home" => Key::Named(NamedKey::Home),
        "end" => Key::Named(NamedKey::End),
        "pageup" | "pgup" => Key::Named(NamedKey::PageUp),
        "pagedown" | "pgdown" => Key::Named(NamedKey::PageDown),
        "numlock" => Key::Named(NamedKey::NumLock),
        "scrolllock" => Key::Named(NamedKey::ScrollLock),
        "pause" => Key::Named(NamedKey::Pause),
        "printscreen" | "prtsc" | "snapshot" => Key::Named(NamedKey::PrintScreen),
        "shift" => Key::Named(NamedKey::Shift),
        "leftshift" | "lshift" | "rightshift" | "rshift" => Key::Named(NamedKey::Shift),
        "control" | "ctrl" => Key::Named(NamedKey::Control),
        "leftcontrol" | "lcontrol" | "leftctrl" | "lctrl" => Key::Named(NamedKey::Control),
        "rightcontrol" | "rcontrol" | "rightctrl" | "rctrl" => Key::Named(NamedKey::Control),
        "alt" => Key::Named(NamedKey::Alt),
        "leftalt" | "lalt" | "rightalt" | "ralt" | "altgr" | "option" => Key::Named(NamedKey::Alt),
        "super" | "meta" | "command" | "cmd" | "win" | "windows" | "os" => {
            Key::Named(NamedKey::Super)
        }
        "capslock" => Key::Named(NamedKey::CapsLock),
        "menu" | "contextmenu" => Key::Named(NamedKey::ContextMenu),

        // Fallback
        _ => Key::Unidentified(NativeKey::Unidentified),
    }
}

#[pyclass(name = "Keys")]
pub struct PyKeys;

#[pymethods]
impl PyKeys {
    #[classattr]
    const A: &'static str = "a";
    #[classattr]
    const B: &'static str = "b";
    #[classattr]
    const C: &'static str = "c";
    #[classattr]
    const D: &'static str = "d";
    #[classattr]
    const E: &'static str = "e";
    #[classattr]
    const F: &'static str = "f";
    #[classattr]
    const G: &'static str = "g";
    #[classattr]
    const H: &'static str = "h";
    #[classattr]
    const I: &'static str = "i";
    #[classattr]
    const J: &'static str = "j";
    #[classattr]
    const K: &'static str = "k";
    #[classattr]
    const L: &'static str = "l";
    #[classattr]
    const M: &'static str = "m";
    #[classattr]
    const N: &'static str = "n";
    #[classattr]
    const O: &'static str = "o";
    #[classattr]
    const P: &'static str = "p";
    #[classattr]
    const Q: &'static str = "q";
    #[classattr]
    const R: &'static str = "r";
    #[classattr]
    const S: &'static str = "s";
    #[classattr]
    const T: &'static str = "t";
    #[classattr]
    const U: &'static str = "u";
    #[classattr]
    const V: &'static str = "v";
    #[classattr]
    const W: &'static str = "w";
    #[classattr]
    const X: &'static str = "x";
    #[classattr]
    const Y: &'static str = "y";
    #[classattr]
    const Z: &'static str = "z";

    #[classattr]
    const NUM_0: &'static str = "0";
    #[classattr]
    const NUM_1: &'static str = "1";
    #[classattr]
    const NUM_2: &'static str = "2";
    #[classattr]
    const NUM_3: &'static str = "3";
    #[classattr]
    const NUM_4: &'static str = "4";
    #[classattr]
    const NUM_5: &'static str = "5";
    #[classattr]
    const NUM_6: &'static str = "6";
    #[classattr]
    const NUM_7: &'static str = "7";
    #[classattr]
    const NUM_8: &'static str = "8";
    #[classattr]
    const NUM_9: &'static str = "9";

    #[classattr]
    const MINUS: &'static str = "-";
    #[classattr]
    const EQUALS: &'static str = "=";
    #[classattr]
    const LEFT_BRACKET: &'static str = "[";
    #[classattr]
    const RIGHT_BRACKET: &'static str = "]";
    #[classattr]
    const BACKSLASH: &'static str = "\\";
    #[classattr]
    const SEMICOLON: &'static str = ";";
    #[classattr]
    const APOSTROPHE: &'static str = "'";
    #[classattr]
    const COMMA: &'static str = ",";
    #[classattr]
    const PERIOD: &'static str = ".";
    #[classattr]
    const SLASH: &'static str = "/";
    #[classattr]
    const GRAVE: &'static str = "`";

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
    const SUPER: &'static str = "Super";
    #[classattr]
    const CAPS_LOCK: &'static str = "CapsLock";
    #[classattr]
    const NUM_LOCK: &'static str = "NumLock";
    #[classattr]
    const SCROLL_LOCK: &'static str = "ScrollLock";
    #[classattr]
    const INSERT: &'static str = "Insert";
    #[classattr]
    const DELETE: &'static str = "Delete";
    #[classattr]
    const HOME: &'static str = "Home";
    #[classattr]
    const END: &'static str = "End";
    #[classattr]
    const PAGE_UP: &'static str = "PageUp";
    #[classattr]
    const PAGE_DOWN: &'static str = "PageDown";
    #[classattr]
    const PRINT_SCREEN: &'static str = "PrintScreen";
    #[classattr]
    const PAUSE: &'static str = "Pause";
    #[classattr]
    const CONTEXT_MENU: &'static str = "ContextMenu";

    #[classattr]
    const F1: &'static str = "F1";
    #[classattr]
    const F2: &'static str = "F2";
    #[classattr]
    const F3: &'static str = "F3";
    #[classattr]
    const F4: &'static str = "F4";
    #[classattr]
    const F5: &'static str = "F5";
    #[classattr]
    const F6: &'static str = "F6";
    #[classattr]
    const F7: &'static str = "F7";
    #[classattr]
    const F8: &'static str = "F8";
    #[classattr]
    const F9: &'static str = "F9";
    #[classattr]
    const F10: &'static str = "F10";
    #[classattr]
    const F11: &'static str = "F11";
    #[classattr]
    const F12: &'static str = "F12";
}
