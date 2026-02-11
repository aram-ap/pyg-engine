use super::draw_manager::DrawCommand;
use super::game_object::GameObject;
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
    SetGameObjectPosition {
        object_id: u32,
        position: Vec2,
    },

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
        layer: i32,
        z_index: f32,
    },

    /// Draw a line (helper wrapper around AddDrawCommand)
    DrawLine {
        start_x: f32,
        start_y: f32,
        end_x: f32,
        end_y: f32,
        thickness: f32,
        color: Color,
        layer: i32,
        z_index: f32,
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
        layer: i32,
        z_index: f32,
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
        layer: i32,
        z_index: f32,
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
        layer: i32,
        z_index: f32,
    },

    /// Draw an image from disk path (helper wrapper around AddDrawCommand)
    DrawImage {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        texture_path: String,
        layer: i32,
        z_index: f32,
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
        layer: i32,
        z_index: f32,
    },

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
        layer: i32,
        z_index: f32,
    },
}
