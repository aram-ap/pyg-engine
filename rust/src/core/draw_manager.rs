use crate::types::Color;

#[derive(Clone, Debug)]
pub enum DrawCommand {
    Pixel {
        x: f32,
        y: f32,
        color: Color,
        layer: i32,
        z_index: f32,
    },
    Line {
        start_x: f32,
        start_y: f32,
        end_x: f32,
        end_y: f32,
        thickness: f32,
        color: Color,
        layer: i32,
        z_index: f32,
    },
    Rectangle {
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
    Circle {
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
    GradientRect {
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
    Image {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        texture_path: String,
        layer: i32,
        z_index: f32,
    },
    ImageBytes {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        texture_key: String,
        rgba: Vec<u8>,
        texture_width: u32,
        texture_height: u32,
        layer: i32,
        z_index: f32,
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
        layer: i32,
        z_index: f32,
    },
}

#[derive(Default)]
pub struct DrawManager {
    commands: Vec<DrawCommand>,
}

impl DrawManager {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.commands.clear();
    }

    pub fn commands(&self) -> &[DrawCommand] {
        &self.commands
    }

    pub fn add_command(&mut self, command: DrawCommand) {
        self.commands.push(command);
    }

    pub fn add_commands(&mut self, mut commands: Vec<DrawCommand>) {
        self.commands.append(&mut commands);
    }

    pub fn draw_pixel(&mut self, x: u32, y: u32, color: Color) {
        self.draw_pixel_with_order(x, y, color, 0, 0.0);
    }

    pub fn draw_pixel_with_order(
        &mut self,
        x: u32,
        y: u32,
        color: Color,
        layer: i32,
        z_index: f32,
    ) {
        self.commands.push(DrawCommand::Pixel {
            x: x as f32,
            y: y as f32,
            color,
            layer,
            z_index,
        });
    }

    pub fn draw_line(&mut self, start_x: f32, start_y: f32, end_x: f32, end_y: f32, color: Color) {
        self.draw_line_with_options(start_x, start_y, end_x, end_y, 1.0, color, 0, 0.0);
    }

    pub fn draw_line_with_options(
        &mut self,
        start_x: f32,
        start_y: f32,
        end_x: f32,
        end_y: f32,
        thickness: f32,
        color: Color,
        layer: i32,
        z_index: f32,
    ) {
        self.commands.push(DrawCommand::Line {
            start_x,
            start_y,
            end_x,
            end_y,
            thickness,
            color,
            layer,
            z_index,
        });
    }

    pub fn draw_rectangle(&mut self, x: f32, y: f32, width: f32, height: f32, color: Color) {
        self.draw_rectangle_with_options(x, y, width, height, color, true, 1.0, 0, 0.0);
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
        self.draw_rectangle_with_options(x, y, width, height, color, false, thickness, 0, 0.0);
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
        layer: i32,
        z_index: f32,
    ) {
        self.commands.push(DrawCommand::Rectangle {
            x,
            y,
            width,
            height,
            color,
            filled,
            thickness,
            layer,
            z_index,
        });
    }

    pub fn draw_circle(&mut self, center_x: f32, center_y: f32, radius: f32, color: Color) {
        self.draw_circle_with_options(center_x, center_y, radius, color, true, 1.0, 32, 0, 0.0);
    }

    pub fn draw_circle_outline(
        &mut self,
        center_x: f32,
        center_y: f32,
        radius: f32,
        thickness: f32,
        color: Color,
    ) {
        self.draw_circle_with_options(
            center_x, center_y, radius, color, false, thickness, 32, 0, 0.0,
        );
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
        layer: i32,
        z_index: f32,
    ) {
        self.commands.push(DrawCommand::Circle {
            center_x,
            center_y,
            radius,
            color,
            filled,
            thickness,
            segments,
            layer,
            z_index,
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
        layer: i32,
        z_index: f32,
    ) {
        self.commands.push(DrawCommand::GradientRect {
            x,
            y,
            width,
            height,
            top_left,
            bottom_left,
            bottom_right,
            top_right,
            layer,
            z_index,
        });
    }

    pub fn draw_image_with_options(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        texture_path: String,
        layer: i32,
        z_index: f32,
    ) {
        self.commands.push(DrawCommand::Image {
            x,
            y,
            width,
            height,
            texture_path,
            layer,
            z_index,
        });
    }

    pub fn draw_image_from_bytes_with_options(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        texture_key: String,
        rgba: Vec<u8>,
        texture_width: u32,
        texture_height: u32,
        layer: i32,
        z_index: f32,
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

        self.commands.push(DrawCommand::ImageBytes {
            x,
            y,
            width,
            height,
            texture_key,
            rgba,
            texture_width,
            texture_height,
            layer,
            z_index,
        });

        Ok(())
    }

    pub fn draw_text(&mut self, text: String, x: f32, y: f32, color: Color) {
        self.draw_text_with_options(text, x, y, 24.0, color, None, 0.0, 0.0, 0, 0.0);
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
        layer: i32,
        z_index: f32,
    ) {
        self.commands.push(DrawCommand::Text {
            text,
            x,
            y,
            font_size,
            color,
            font_path,
            letter_spacing,
            line_spacing,
            layer,
            z_index,
        });
    }
}
