use crate::types::Color;
use super::game_object::GameObject;
use super::draw_manager::DrawCommand;

/// Commands that can be sent to the engine from any thread
#[derive(Debug)]
pub enum EngineCommand {
    /// Add a new game object to the scene
    AddGameObject(GameObject),
    
    /// Remove a game object by ID
    RemoveGameObject(u32),
    
    /// Clear all immediate-mode draw commands
    ClearDrawCommands,
    
    /// Add a direct draw command
    AddDrawCommand(DrawCommand),
    
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
}
