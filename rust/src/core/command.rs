use super::draw_manager::DrawCommand;
use super::game_object::GameObject;
use super::render_manager::CameraAspectMode;
use crate::types::Color;
use crate::types::vector::Vec2;
use std::sync::Arc;

/// Commands that can be sent to the engine from any thread
#[derive(Debug)]
pub enum EngineCommand {
    /// Add a new game object to the scene
    AddGameObject(GameObject),

    /// Remove a game object by ID
    RemoveGameObject(u32),

    /// Update a runtime GameObject position by id
    SetGameObjectPosition { object_id: u32, position: Vec2 },

    /// Update a runtime GameObject rotation by id
    SetGameObjectRotation { object_id: u32, rotation: f32 },

    /// Update a runtime GameObject scale by id
    SetGameObjectScale { object_id: u32, scale: Vec2 },

    /// Update a runtime GameObject mesh fill color by id
    SetGameObjectMeshFillColor { object_id: u32, color: Option<Color> },

    /// Update the active camera world position
    SetCameraPosition { position: Vec2 },

    /// Set the active camera viewport size in world units
    SetCameraViewportSize { width: f32, height: f32 },

    /// Set how the camera handles window/display aspect changes
    SetCameraAspectMode { mode: CameraAspectMode },

    /// Set the active camera background clear color
    SetCameraBackgroundColor { color: Color },

    /// Clear all immediate-mode draw commands
    ClearDrawCommands,

    /// Add a direct draw command
    AddDrawCommand(DrawCommand),

    /// Add many direct draw commands in one batch
    AddDrawCommands(Vec<DrawCommand>),

    /// Draw a pixel (helper wrapper around AddDrawCommand)
    DrawPixel {
        x: u32,
        y: u32,
        color: Color,
        draw_order: f32,
    },

    /// Draw a line (helper wrapper around AddDrawCommand)
    DrawLine {
        start_x: f32,
        start_y: f32,
        end_x: f32,
        end_y: f32,
        thickness: f32,
        color: Color,
        draw_order: f32,
    },

    /// Draw a rectangle (helper wrapper around AddDrawCommand)
    DrawRectangle {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color: Color,
        filled: bool,
        thickness: f32,
        draw_order: f32,
    },

    /// Draw a circle (helper wrapper around AddDrawCommand)
    DrawCircle {
        center_x: f32,
        center_y: f32,
        radius: f32,
        color: Color,
        filled: bool,
        thickness: f32,
        segments: u32,
        draw_order: f32,
    },

    /// Draw a gradient rectangle (helper wrapper around AddDrawCommand)
    DrawGradientRect {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        top_left: Color,
        bottom_left: Color,
        bottom_right: Color,
        top_right: Color,
        draw_order: f32,
    },

    /// Draw an image from disk path (helper wrapper around AddDrawCommand)
    DrawImage {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        texture_path: String,
        draw_order: f32,
    },

    /// Draw an image from raw RGBA bytes (helper wrapper around AddDrawCommand)
    DrawImageBytes {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        texture_key: String,
        rgba: Arc<[u8]>,
        texture_width: u32,
        texture_height: u32,
        draw_order: f32,
    },

    /// Update a UI label's text by object ID
    UpdateUILabelText { object_id: u32, text: String },

    /// Update a UI button's text by object ID
    UpdateUIButtonText { object_id: u32, text: String },

    /// Draw text with optional custom font (helper wrapper around AddDrawCommand)
    DrawText {
        text: String,
        x: f32,
        y: f32,
        font_size: f32,
        color: Color,
        font_path: Option<String>,
        letter_spacing: f32,
        line_spacing: f32,
        draw_order: f32,
    },

    /// Log a message at TRACE level
    LogTrace(String),

    /// Log a message at DEBUG level
    LogDebug(String),

    /// Log a message at INFO level
    LogInfo(String),

    /// Log a message at WARN level
    LogWarn(String),

    /// Log a message at ERROR level
    LogError(String),
}
