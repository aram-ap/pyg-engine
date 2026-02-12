use crate::types::Color;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub enum DrawCommand {
    Pixel {
        x: f32,
        y: f32,
        color: Color,
        draw_order: f32,
    },
    Line {
        start_x: f32,
        start_y: f32,
        end_x: f32,
        end_y: f32,
        thickness: f32,
        color: Color,
        draw_order: f32,
    },
    Rectangle {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color: Color,
        filled: bool,
        thickness: f32,
        draw_order: f32,
    },
    Circle {
        center_x: f32,
        center_y: f32,
        radius: f32,
        color: Color,
        filled: bool,
        thickness: f32,
        segments: u32,
        draw_order: f32,
    },
    GradientRect {
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
    Image {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        texture_path: String,
        draw_order: f32,
    },
    ImageBytes {
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
    Text {
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
}

#[derive(Default)]
pub struct DrawManager {
    commands: Vec<DrawCommand>,
    scene_version: u64,
}

impl DrawManager {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            scene_version: 0,
        }
    }

    pub fn clear(&mut self) {
        if self.commands.is_empty() {
            return;
        }

        self.commands.clear();
        self.bump_scene_version();
    }

    pub fn commands(&self) -> &[DrawCommand] {
        &self.commands
    }

    pub fn scene_version(&self) -> u64 {
        self.scene_version
    }

    fn bump_scene_version(&mut self) {
        self.scene_version = self.scene_version.wrapping_add(1);
    }

    fn push_command(&mut self, command: DrawCommand) {
        self.commands.push(command);
        self.bump_scene_version();
    }

    pub fn add_command(&mut self, command: DrawCommand) {
        self.push_command(command);
    }

    pub fn add_commands(&mut self, mut commands: Vec<DrawCommand>) {
        if commands.is_empty() {
            return;
        }

        self.commands.append(&mut commands);
        self.bump_scene_version();
    }

    /// Remove all draw commands from index `start` onward.
    /// Used by UIManager to clear previous frame's UI commands before re-rendering.
    pub fn truncate_from(&mut self, start: usize) {
        if start < self.commands.len() {
            self.commands.truncate(start);
            self.bump_scene_version();
        }
    }

    /// Scale all draw commands from index `start` onward by `scale`.
    /// Used to convert UI coordinates from logical to physical pixels.
    pub fn scale_commands_from(&mut self, start: usize, scale: f32) {
        for cmd in self.commands[start..].iter_mut() {
            match cmd {
                DrawCommand::Rectangle { x, y, width, height, thickness, .. } => {
                    *x *= scale;
                    *y *= scale;
                    *width *= scale;
                    *height *= scale;
                    *thickness *= scale;
                }
                DrawCommand::Text { x, y, font_size, .. } => {
                    *x *= scale;
                    *y *= scale;
                    *font_size *= scale;
                }
                DrawCommand::Line { start_x, start_y, end_x, end_y, thickness, .. } => {
                    *start_x *= scale;
                    *start_y *= scale;
                    *end_x *= scale;
                    *end_y *= scale;
                    *thickness *= scale;
                }
                DrawCommand::Pixel { x, y, .. } => {
                    *x *= scale;
                    *y *= scale;
                }
                DrawCommand::Circle { center_x, center_y, radius, thickness, .. } => {
                    *center_x *= scale;
                    *center_y *= scale;
                    *radius *= scale;
                    *thickness *= scale;
                }
                DrawCommand::GradientRect { x, y, width, height, .. } => {
                    *x *= scale;
                    *y *= scale;
                    *width *= scale;
                    *height *= scale;
                }
                DrawCommand::Image { x, y, width, height, .. } => {
                    *x *= scale;
                    *y *= scale;
                    *width *= scale;
                    *height *= scale;
                }
                DrawCommand::ImageBytes { x, y, width, height, .. } => {
                    *x *= scale;
                    *y *= scale;
                    *width *= scale;
                    *height *= scale;
                }
            }
        }
        self.bump_scene_version();
    }

    pub fn draw_pixel(&mut self, x: u32, y: u32, color: Color) {
        self.draw_pixel_with_order(x, y, color, 0.0);
    }

    pub fn draw_pixel_with_order(&mut self, x: u32, y: u32, color: Color, draw_order: f32) {
        self.push_command(DrawCommand::Pixel {
            x: x as f32,
            y: y as f32,
            color,
            draw_order,
        });
    }

    pub fn draw_line(&mut self, start_x: f32, start_y: f32, end_x: f32, end_y: f32, color: Color) {
        self.draw_line_with_options(start_x, start_y, end_x, end_y, 1.0, color, 0.0);
    }

    pub fn draw_line_with_options(
        &mut self,
        start_x: f32,
        start_y: f32,
        end_x: f32,
        end_y: f32,
        thickness: f32,
        color: Color,
        draw_order: f32,
    ) {
        self.push_command(DrawCommand::Line {
            start_x,
            start_y,
            end_x,
            end_y,
            thickness,
            color,
            draw_order,
        });
    }

    pub fn draw_rectangle(&mut self, x: f32, y: f32, width: f32, height: f32, color: Color) {
        self.draw_rectangle_with_options(x, y, width, height, color, true, 1.0, 0.0);
    }

    pub fn draw_rectangle_outline(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        thickness: f32,
        color: Color,
    ) {
        self.draw_rectangle_with_options(x, y, width, height, color, false, thickness, 0.0);
    }

    pub fn draw_rectangle_with_options(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color: Color,
        filled: bool,
        thickness: f32,
        draw_order: f32,
    ) {
        self.push_command(DrawCommand::Rectangle {
            x,
            y,
            width,
            height,
            color,
            filled,
            thickness,
            draw_order,
        });
    }

    pub fn draw_circle(&mut self, center_x: f32, center_y: f32, radius: f32, color: Color) {
        self.draw_circle_with_options(center_x, center_y, radius, color, true, 1.0, 32, 0.0);
    }

    pub fn draw_circle_outline(
        &mut self,
        center_x: f32,
        center_y: f32,
        radius: f32,
        thickness: f32,
        color: Color,
    ) {
        self.draw_circle_with_options(center_x, center_y, radius, color, false, thickness, 32, 0.0);
    }

    pub fn draw_circle_with_options(
        &mut self,
        center_x: f32,
        center_y: f32,
        radius: f32,
        color: Color,
        filled: bool,
        thickness: f32,
        segments: u32,
        draw_order: f32,
    ) {
        self.push_command(DrawCommand::Circle {
            center_x,
            center_y,
            radius,
            color,
            filled,
            thickness,
            segments,
            draw_order,
        });
    }

    pub fn draw_gradient_rect_with_options(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        top_left: Color,
        bottom_left: Color,
        bottom_right: Color,
        top_right: Color,
        draw_order: f32,
    ) {
        self.push_command(DrawCommand::GradientRect {
            x,
            y,
            width,
            height,
            top_left,
            bottom_left,
            bottom_right,
            top_right,
            draw_order,
        });
    }

    pub fn draw_image_with_options(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        texture_path: String,
        draw_order: f32,
    ) {
        self.push_command(DrawCommand::Image {
            x,
            y,
            width,
            height,
            texture_path,
            draw_order,
        });
    }

    pub fn draw_image_from_bytes_with_options(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        texture_key: String,
        rgba: Arc<[u8]>,
        texture_width: u32,
        texture_height: u32,
        draw_order: f32,
    ) -> Result<(), String> {
        let expected_size = (texture_width as usize)
            .checked_mul(texture_height as usize)
            .and_then(|value| value.checked_mul(4))
            .ok_or_else(|| "texture size overflow while validating RGBA buffer".to_string())?;

        if rgba.len() != expected_size {
            return Err(format!(
                "texture byte size mismatch for key '{texture_key}': expected {expected_size} bytes ({}x{} RGBA), got {} bytes",
                texture_width,
                texture_height,
                rgba.len()
            ));
        }

        self.push_command(DrawCommand::ImageBytes {
            x,
            y,
            width,
            height,
            texture_key,
            rgba,
            texture_width,
            texture_height,
            draw_order,
        });

        Ok(())
    }

    pub fn draw_text(&mut self, text: String, x: f32, y: f32, color: Color) {
        self.draw_text_with_options(text, x, y, 24.0, color, None, 0.0, 0.0, 0.0);
    }

    #[allow(clippy::too_many_arguments)]
    pub fn draw_text_with_options(
        &mut self,
        text: String,
        x: f32,
        y: f32,
        font_size: f32,
        color: Color,
        font_path: Option<String>,
        letter_spacing: f32,
        line_spacing: f32,
        draw_order: f32,
    ) {
        self.push_command(DrawCommand::Text {
            text,
            x,
            y,
            font_size,
            color,
            font_path,
            letter_spacing,
            line_spacing,
            draw_order,
        });
    }
}
