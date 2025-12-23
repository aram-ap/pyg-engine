use crate::time::Time;
use crate::vector::Vec2;

// Game objects contain components.
// Components are used to add functionality to game objects.
pub trait ComponentTrait {
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