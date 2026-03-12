use super::time::Time;
use super::text::{FontDescriptor, FontStyle, FontWeight, TextStyle};
use crate::types::color::Color;
use crate::types::vector::Vec2;
use std::any::Any;
use std::sync::atomic::{AtomicU32, Ordering};

static COMPONENT_ID: AtomicU32 = AtomicU32::new(0);

pub fn next_component_id() -> u32 {
    COMPONENT_ID.fetch_add(1, Ordering::SeqCst) + 1
}

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

    fn id(&self) -> u32;
    fn component_type(&self) -> &'static str;
    fn is_enabled_self(&self) -> bool;
    fn set_enabled_self(&mut self, enabled: bool);
    fn is_enabled_in_hierarchy(&self) -> bool;
    fn set_enabled_in_hierarchy(&mut self, enabled: bool);

    fn is_effectively_enabled(&self) -> bool {
        self.is_enabled_self() && self.is_enabled_in_hierarchy()
    }

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

    fn clone_component(&self) -> Box<dyn ComponentTrait>;

    /// Downcast to Any for type checking
    fn as_any(&self) -> &dyn Any;

    /// Downcast to Any (mutable) for type checking
    fn as_any_mut(&mut self) -> &mut dyn Any;

    /// Convert boxed component into a concrete Any box.
    fn into_any(self: Box<Self>) -> Box<dyn Any>;

    // Physics callbacks (optional, default implementations do nothing)
    /// Called when a collision starts
    fn on_collision_enter(&self, _other_id: u32, _normal: Vec2, _penetration: f32) {}

    /// Called each frame while a collision is ongoing
    fn on_collision_stay(&self, _other_id: u32, _normal: Vec2, _penetration: f32) {}

    /// Called when a collision ends
    fn on_collision_exit(&self, _other_id: u32) {}
}

impl Clone for Box<dyn ComponentTrait> {
    fn clone(&self) -> Self {
        self.clone_component()
    }
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
    component_id: u32,
    name: String,
    position: Vec2,
    rotation: f32,
    scale: Vec2,
    enabled_self: bool,
    enabled_in_hierarchy: bool,
}

impl ComponentTrait for TransformComponent {
    fn new(name: String) -> Self {
        Self {
            component_id: next_component_id(),
            name,
            position: Vec2::new(0.0, 0.0),
            rotation: 0.0,
            scale: Vec2::new(1.0, 1.0),
            enabled_self: true,
            enabled_in_hierarchy: true,
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn id(&self) -> u32 {
        self.component_id
    }

    fn component_type(&self) -> &'static str {
        "Transform"
    }

    fn is_enabled_self(&self) -> bool {
        self.enabled_self
    }

    fn set_enabled_self(&mut self, enabled: bool) {
        self.enabled_self = enabled;
    }

    fn is_enabled_in_hierarchy(&self) -> bool {
        self.enabled_in_hierarchy
    }

    fn set_enabled_in_hierarchy(&mut self, enabled: bool) {
        self.enabled_in_hierarchy = enabled;
    }

    fn update(&self, _time: &Time) {}
    fn fixed_update(&self, _time: &Time, _fixed_time: f32) {}

    fn on_start(&self) {}
    fn on_destroy(&self) {}
    fn on_enable(&self) {}
    fn on_disable(&self) {}

    fn clone_component(&self) -> Box<dyn ComponentTrait> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
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
    component_id: u32,
    name: String,
    geometry: MeshGeometry,
    fill_color: Option<Color>,
    image_path: Option<String>,
    visible: bool,
    draw_order: f32,
    enabled_self: bool,
    enabled_in_hierarchy: bool,
}

impl ComponentTrait for MeshComponent {
    fn new(name: String) -> Self {
        Self {
            component_id: next_component_id(),
            name,
            geometry: MeshGeometry::default(),
            fill_color: Some(Color::WHITE),
            image_path: None,
            visible: true,
            draw_order: 0.0,
            enabled_self: true,
            enabled_in_hierarchy: true,
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn id(&self) -> u32 {
        self.component_id
    }

    fn component_type(&self) -> &'static str {
        "Mesh"
    }

    fn is_enabled_self(&self) -> bool {
        self.enabled_self
    }

    fn set_enabled_self(&mut self, enabled: bool) {
        self.enabled_self = enabled;
    }

    fn is_enabled_in_hierarchy(&self) -> bool {
        self.enabled_in_hierarchy
    }

    fn set_enabled_in_hierarchy(&mut self, enabled: bool) {
        self.enabled_in_hierarchy = enabled;
    }

    fn update(&self, _time: &Time) {}
    fn fixed_update(&self, _time: &Time, _fixed_time: f32) {}
    fn on_start(&self) {}
    fn on_destroy(&self) {}
    fn on_enable(&self) {}
    fn on_disable(&self) {}

    fn clone_component(&self) -> Box<dyn ComponentTrait> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
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

    pub fn with_draw_order(mut self, draw_order: f32) -> Self {
        self.draw_order = draw_order;
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

    pub fn draw_order(&self) -> f32 {
        self.draw_order
    }

    pub fn set_draw_order(&mut self, draw_order: f32) {
        self.draw_order = draw_order;
    }
}

#[derive(Clone, Debug)]
pub struct TextMeshComponent {
    component_id: u32,
    name: String,
    text: String,
    color: Color,
    text_style: TextStyle,
    visible: bool,
    draw_order: f32,
    enabled_self: bool,
    enabled_in_hierarchy: bool,
}

impl ComponentTrait for TextMeshComponent {
    fn new(name: String) -> Self {
        Self {
            component_id: next_component_id(),
            name,
            text: String::new(),
            color: Color::WHITE,
            text_style: TextStyle::new(24.0),
            visible: true,
            draw_order: 0.0,
            enabled_self: true,
            enabled_in_hierarchy: true,
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn id(&self) -> u32 {
        self.component_id
    }

    fn component_type(&self) -> &'static str {
        "TextMesh"
    }

    fn is_enabled_self(&self) -> bool {
        self.enabled_self
    }

    fn set_enabled_self(&mut self, enabled: bool) {
        self.enabled_self = enabled;
    }

    fn is_enabled_in_hierarchy(&self) -> bool {
        self.enabled_in_hierarchy
    }

    fn set_enabled_in_hierarchy(&mut self, enabled: bool) {
        self.enabled_in_hierarchy = enabled;
    }

    fn update(&self, _time: &Time) {}
    fn fixed_update(&self, _time: &Time, _fixed_time: f32) {}
    fn on_start(&self) {}
    fn on_destroy(&self) {}
    fn on_enable(&self) {}
    fn on_disable(&self) {}

    fn clone_component(&self) -> Box<dyn ComponentTrait> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

impl TextMeshComponent {
    pub fn new(name: impl Into<String>) -> Self {
        <Self as ComponentTrait>::new(name.into())
    }

    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn with_font_size(mut self, font_size: f32) -> Self {
        self.text_style.font_size = font_size.max(1.0);
        self
    }

    pub fn with_font_path(mut self, font_path: Option<String>) -> Self {
        self.text_style.font.set_path(font_path);
        self
    }

    pub fn with_font_family(mut self, font_family: Option<String>) -> Self {
        self.text_style.font.set_family(font_family);
        self
    }

    pub fn with_font_weight(mut self, font_weight: FontWeight) -> Self {
        self.text_style.font.set_weight(font_weight);
        self
    }

    pub fn with_font_style(mut self, font_style: FontStyle) -> Self {
        self.text_style.font.set_style(font_style);
        self
    }

    pub fn with_kerning(mut self, kerning: bool) -> Self {
        self.text_style.kerning = kerning;
        self
    }

    pub fn with_letter_spacing(mut self, letter_spacing: f32) -> Self {
        self.text_style.letter_spacing = letter_spacing;
        self
    }

    pub fn with_line_spacing(mut self, line_spacing: f32) -> Self {
        self.text_style.line_spacing = line_spacing;
        self
    }

    pub fn with_draw_order(mut self, draw_order: f32) -> Self {
        self.draw_order = draw_order;
        self
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, text: impl Into<String>) {
        self.text = text.into();
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn font_size(&self) -> f32 {
        self.text_style.font_size
    }

    pub fn set_font_size(&mut self, font_size: f32) {
        self.text_style.font_size = font_size.max(1.0);
    }

    pub fn font_path(&self) -> Option<&str> {
        self.text_style.font.path()
    }

    pub fn set_font_path(&mut self, font_path: Option<String>) {
        self.text_style.font.set_path(font_path);
    }

    pub fn font_family(&self) -> Option<&str> {
        self.text_style.font.family()
    }

    pub fn set_font_family(&mut self, font_family: Option<String>) {
        self.text_style.font.set_family(font_family);
    }

    pub fn font_weight(&self) -> FontWeight {
        self.text_style.font.weight()
    }

    pub fn set_font_weight(&mut self, font_weight: FontWeight) {
        self.text_style.font.set_weight(font_weight);
    }

    pub fn font_style(&self) -> FontStyle {
        self.text_style.font.style()
    }

    pub fn set_font_style(&mut self, font_style: FontStyle) {
        self.text_style.font.set_style(font_style);
    }

    pub fn kerning(&self) -> bool {
        self.text_style.kerning
    }

    pub fn set_kerning(&mut self, kerning: bool) {
        self.text_style.kerning = kerning;
    }

    pub fn letter_spacing(&self) -> f32 {
        self.text_style.letter_spacing
    }

    pub fn set_letter_spacing(&mut self, letter_spacing: f32) {
        self.text_style.letter_spacing = letter_spacing;
    }

    pub fn line_spacing(&self) -> f32 {
        self.text_style.line_spacing
    }

    pub fn set_line_spacing(&mut self, line_spacing: f32) {
        self.text_style.line_spacing = line_spacing;
    }

    pub fn text_style(&self) -> &TextStyle {
        &self.text_style
    }

    pub fn set_text_style(&mut self, text_style: TextStyle) {
        self.text_style = text_style;
        self.text_style.font_size = self.text_style.font_size.max(1.0);
    }

    pub fn font_descriptor(&self) -> &FontDescriptor {
        &self.text_style.font
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn draw_order(&self) -> f32 {
        self.draw_order
    }

    pub fn set_draw_order(&mut self, draw_order: f32) {
        self.draw_order = draw_order;
    }
}

#[derive(Debug)]
pub struct SpriteComponent {
    component_id: u32,
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
    enabled_self: bool,
    enabled_in_hierarchy: bool,
}

impl ComponentTrait for SpriteComponent {
    fn new(name: String) -> Self {
        Self {
            component_id: next_component_id(),
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
            enabled_self: true,
            enabled_in_hierarchy: true,
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn id(&self) -> u32 {
        self.component_id
    }

    fn component_type(&self) -> &'static str {
        "Sprite"
    }

    fn is_enabled_self(&self) -> bool {
        self.enabled_self
    }

    fn set_enabled_self(&mut self, enabled: bool) {
        self.enabled_self = enabled;
    }

    fn is_enabled_in_hierarchy(&self) -> bool {
        self.enabled_in_hierarchy
    }

    fn set_enabled_in_hierarchy(&mut self, enabled: bool) {
        self.enabled_in_hierarchy = enabled;
    }

    fn update(&self, _time: &Time) {}
    fn fixed_update(&self, _time: &Time, _fixed_time: f32) {}
    fn on_start(&self) {}
    fn on_destroy(&self) {}
    fn on_enable(&self) {}
    fn on_disable(&self) {}

    fn clone_component(&self) -> Box<dyn ComponentTrait> {
        Box::new(Self {
            component_id: self.component_id,
            name: self.name.clone(),
            offset: self.offset,
            scale: self.scale,
            rotation: self.rotation,
            pivot: self.pivot,
            origin: self.origin,
            flip_x: self.flip_x,
            flip_y: self.flip_y,
            z_index: self.z_index,
            color: self.color,
            visible: self.visible,
            layer: self.layer,
            enabled_self: self.enabled_self,
            enabled_in_hierarchy: self.enabled_in_hierarchy,
        })
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
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
