use super::time::Time;
use crate::types::color::Color;
use crate::types::vector::Vec2;
// use crate::types::texture::Texture;

// Game objects contain components.
// Components are used to add functionality to game objects.
pub trait ComponentTrait: Send + Sync + std::fmt::Debug {
    /**
        Creates a new component.
        @return: The new component.
    */
    fn new(name: String) -> Self
    where
        Self: Sized;

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

#[derive(Clone, Copy, Debug)]
pub struct MeshVertex {
    position: Vec2,
    uv: Vec2,
}

impl MeshVertex {
    pub fn new(position: Vec2, uv: Vec2) -> Self {
        Self { position, uv }
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn uv(&self) -> Vec2 {
        self.uv
    }
}

#[derive(Clone, Debug)]
pub struct MeshGeometry {
    vertices: Vec<MeshVertex>,
    indices: Vec<u32>,
}

impl MeshGeometry {
    pub fn new(vertices: Vec<MeshVertex>, indices: Vec<u32>) -> Self {
        Self { vertices, indices }
    }

    /// Create a unit rectangle centered at origin.
    pub fn rectangle(width: f32, height: f32) -> Self {
        let half_w = width * 0.5;
        let half_h = height * 0.5;

        Self {
            vertices: vec![
                MeshVertex::new(Vec2::new(-half_w, half_h), Vec2::new(0.0, 0.0)),
                MeshVertex::new(Vec2::new(-half_w, -half_h), Vec2::new(0.0, 1.0)),
                MeshVertex::new(Vec2::new(half_w, -half_h), Vec2::new(1.0, 1.0)),
                MeshVertex::new(Vec2::new(half_w, half_h), Vec2::new(1.0, 0.0)),
            ],
            indices: vec![0, 1, 2, 0, 2, 3],
        }
    }

    /// Create a circle centered at origin using a triangle fan.
    pub fn circle(radius: f32, segments: u32) -> Self {
        let segment_count = segments.max(3);
        let safe_radius = radius.abs().max(f32::EPSILON);
        let mut vertices = Vec::with_capacity((segment_count + 1) as usize);
        let mut indices = Vec::with_capacity((segment_count * 3) as usize);

        // Center vertex for triangle fan.
        vertices.push(MeshVertex::new(Vec2::new(0.0, 0.0), Vec2::new(0.5, 0.5)));

        for i in 0..segment_count {
            let angle = (i as f32 / segment_count as f32) * std::f32::consts::TAU;
            let x = safe_radius * angle.cos();
            let y = safe_radius * angle.sin();
            let u = (x / safe_radius) * 0.5 + 0.5;
            let v = 0.5 - (y / safe_radius) * 0.5;
            vertices.push(MeshVertex::new(Vec2::new(x, y), Vec2::new(u, v)));
        }

        for i in 0..segment_count {
            let current = i + 1;
            let next = ((i + 1) % segment_count) + 1;
            indices.extend([0, current, next]);
        }

        Self { vertices, indices }
    }

    pub fn vertices(&self) -> &[MeshVertex] {
        &self.vertices
    }

    pub fn indices(&self) -> &[u32] {
        &self.indices
    }

    pub fn set_vertices(&mut self, vertices: Vec<MeshVertex>) {
        self.vertices = vertices;
    }

    pub fn set_indices(&mut self, indices: Vec<u32>) {
        self.indices = indices;
    }

    pub fn is_valid(&self) -> bool {
        !self.vertices.is_empty() && !self.indices.is_empty()
    }
}

impl Default for MeshGeometry {
    fn default() -> Self {
        Self::rectangle(1.0, 1.0)
    }
}

#[derive(Clone, Debug)]
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
    pub fn new(name: impl Into<String>) -> Self {
        <Self as ComponentTrait>::new(name.into())
    }

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

#[derive(Clone, Debug)]
pub struct MeshComponent {
    name: String,
    geometry: MeshGeometry,
    fill_color: Option<Color>,
    image_path: Option<String>,
    visible: bool,
    layer: i32,
    z_index: f32,
}

impl ComponentTrait for MeshComponent {
    fn new(name: String) -> Self {
        Self {
            name,
            geometry: MeshGeometry::default(),
            fill_color: Some(Color::WHITE),
            image_path: None,
            visible: true,
            layer: 0,
            z_index: 0.0,
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

impl MeshComponent {
    pub fn new(name: impl Into<String>) -> Self {
        <Self as ComponentTrait>::new(name.into())
    }

    pub fn with_geometry(mut self, geometry: MeshGeometry) -> Self {
        self.geometry = geometry;
        self
    }

    pub fn with_fill_color(mut self, fill_color: Option<Color>) -> Self {
        self.fill_color = fill_color;
        self
    }

    pub fn with_image_path(mut self, image_path: Option<String>) -> Self {
        self.image_path = image_path;
        self
    }

    pub fn with_layer(mut self, layer: i32) -> Self {
        self.layer = layer;
        self
    }

    pub fn with_z_index(mut self, z_index: f32) -> Self {
        self.z_index = z_index;
        self
    }

    pub fn geometry(&self) -> &MeshGeometry {
        &self.geometry
    }

    pub fn set_geometry(&mut self, geometry: MeshGeometry) {
        self.geometry = geometry;
    }

    pub fn fill_color(&self) -> Option<&Color> {
        self.fill_color.as_ref()
    }

    pub fn set_fill_color(&mut self, fill_color: Option<Color>) {
        self.fill_color = fill_color;
    }

    pub fn image_path(&self) -> Option<&str> {
        self.image_path.as_deref()
    }

    pub fn set_image_path(&mut self, image_path: Option<String>) {
        self.image_path = image_path;
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn layer(&self) -> i32 {
        self.layer
    }

    pub fn set_layer(&mut self, layer: i32) {
        self.layer = layer;
    }

    pub fn z_index(&self) -> f32 {
        self.z_index
    }

    pub fn set_z_index(&mut self, z_index: f32) {
        self.z_index = z_index;
    }
}

#[derive(Debug)]
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
            name,
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
