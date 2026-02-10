use crate::types::Color;

#[derive(Clone, Copy, Debug)]
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
}
