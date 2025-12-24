use super::time::Time;
use crate::types::vector::Vec2;
use crate::types::color::Color;
// use crate::types::texture::Texture;

// Game objects contain components.
// Components are used to add functionality to game objects.
pub trait ComponentTrait: Send + Sync {
    /**
        Creates a new component.
        @return: The new component.
    */
    fn new(name: String) -> Self where Self: Sized;

    /**
        Gets the name of the component.
        @return: The name of the component.
    */
    fn name(&self) -> &str;

    /**
        Updates the component.
        @param delta_time: The time since the last update.
    */
    fn update(&self, time: &Time);
    fn fixed_update(&self, time: &Time, fixed_time: f32);
    fn on_start(&self);
    fn on_destroy(&self);
    fn on_enable(&self);
    fn on_disable(&self);
}

pub struct TransformComponent {
    name: String,
    position: Vec2,
    rotation: f32,
    scale: Vec2,
}

impl ComponentTrait for TransformComponent {
    fn new(name: String) -> Self {
        Self {
            name,
            position: Vec2::new(0.0, 0.0),
            rotation: 0.0,
            scale: Vec2::new(1.0, 1.0),
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn update(&self, _time: &Time) {}
    fn fixed_update(&self, _time: &Time, _fixed_time: f32) {}

    fn on_start(&self) {}
    fn on_destroy(&self) {}
    fn on_enable(&self) {}
    fn on_disable(&self) {}
}

impl TransformComponent {
    pub fn position(&self) -> &Vec2 {
        &self.position
    }

    pub fn set_position(&mut self, position: Vec2) {
        self.position = position;
    }

    pub fn rotation(&self) -> f32 {
        self.rotation
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }

    pub fn scale(&self) -> &Vec2 {
        &self.scale
    }

    pub fn set_scale(&mut self, scale: Vec2) {
        self.scale = scale;
    }
}

pub struct SpriteComponent {
    name: String,
    // texture: Option<Texture>,
    offset: Vec2,
    scale: Vec2,
    rotation: f32,
    pivot: Vec2,
    origin: Vec2,

    flip_x: bool,
    flip_y: bool,

    z_index: u32,

    color: Color,
    visible: bool,
    layer: u32,
}

impl ComponentTrait for SpriteComponent {
    fn new(name: String) -> Self {
        Self {
            name: "Sprite Renderer".to_string(),
            // texture: None,
            color: Color::new(1.0, 1.0, 1.0, 1.0),
            visible: true,
            layer: 0,
            z_index: 0,
            flip_x: false,
            flip_y: false,
            offset: Vec2::new(0.0, 0.0),
            scale: Vec2::new(1.0, 1.0),
            rotation: 0.0,
            pivot: Vec2::new(0.5, 0.5),
            origin: Vec2::new(0.5, 0.5),
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn update(&self, _time: &Time) {}
    fn fixed_update(&self, _time: &Time, _fixed_time: f32) {}
    fn on_start(&self) {}
    fn on_destroy(&self) {}
    fn on_enable(&self) {}
    fn on_disable(&self) {}
}

impl SpriteComponent {
    pub fn visible(&self) -> bool {
        self.visible
    }
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
    pub fn layer(&self) -> u32 {
        self.layer
    }
    pub fn set_layer(&mut self, layer: u32) {
        self.layer = layer;
    }
    pub fn z_index(&self) -> u32 {
        self.z_index
    }
    pub fn set_z_index(&mut self, z_index: u32) {
        self.z_index = z_index;
    }
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }
    pub fn color(&self) -> &Color {
        &self.color
    }
    pub fn offset(&self) -> &Vec2 {
        &self.offset
    }
    pub fn set_offset(&mut self, offset: Vec2) {
        self.offset = offset;
    }
    pub fn scale(&self) -> &Vec2 {
        &self.scale
    }
    pub fn set_scale(&mut self, scale: Vec2) {
        self.scale = scale;
    }
    pub fn rotation(&self) -> f32 {
        self.rotation
    }
    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }
    pub fn pivot(&self) -> &Vec2 {
        &self.pivot
    }
    pub fn set_pivot(&mut self, pivot: Vec2) {
        self.pivot = pivot;
    }
    pub fn origin(&self) -> &Vec2 {
        &self.origin
    }
    pub fn set_origin(&mut self, origin: Vec2) {
        self.origin = origin;
    }
    pub fn flip_x(&self) -> bool {
        self.flip_x
    }
    pub fn set_flip_x(&mut self, flip_x: bool) {
        self.flip_x = flip_x;
    }
    pub fn flip_y(&self) -> bool {
        self.flip_y
    }
    pub fn set_flip_y(&mut self, flip_y: bool) {
        self.flip_y = flip_y;
    }
}