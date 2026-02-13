// Collider component

use super::layers::{all, should_collide};
use super::shapes::{ColliderShape, AABB};
use crate::core::component::ComponentTrait;
use crate::core::time::Time;
use crate::types::vector::Vec2;
use std::any::Any;
use std::sync::{Arc, Mutex, RwLock};

/// Type alias for collision callbacks
pub type CollisionCallback = Arc<Mutex<Option<Box<dyn FnMut(u32, Vec2, f32) + Send + Sync>>>>;

/// Collider component for collision detection
pub struct ColliderComponent {
    name: String,
    shape: ColliderShape,
    offset: Vec2,
    layer: u32,
    collision_mask: u32,
    is_trigger: bool,
    // Cached AABB for broad-phase optimization
    cached_aabb: RwLock<Option<AABB>>,
    aabb_dirty: RwLock<bool>,
    // Collision callbacks
    on_collision_enter: CollisionCallback,
    on_collision_stay: CollisionCallback,
    on_collision_exit: Arc<Mutex<Option<Box<dyn FnMut(u32) + Send + Sync>>>>,
}

impl std::fmt::Debug for ColliderComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ColliderComponent")
            .field("name", &self.name)
            .field("shape", &self.shape)
            .field("layer", &self.layer)
            .field("is_trigger", &self.is_trigger)
            .finish()
    }
}

impl Clone for ColliderComponent {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            shape: self.shape.clone(),
            offset: self.offset,
            layer: self.layer,
            collision_mask: self.collision_mask,
            is_trigger: self.is_trigger,
            // Clone the cached AABB by reading the lock
            cached_aabb: RwLock::new(self.cached_aabb.read().unwrap().clone()),
            aabb_dirty: RwLock::new(*self.aabb_dirty.read().unwrap()),
            // Clone callback Arc pointers (shared ownership)
            on_collision_enter: Arc::clone(&self.on_collision_enter),
            on_collision_stay: Arc::clone(&self.on_collision_stay),
            on_collision_exit: Arc::clone(&self.on_collision_exit),
        }
    }
}

impl ComponentTrait for ColliderComponent {
    fn new(name: String) -> Self {
        Self {
            name,
            shape: ColliderShape::circle(0.5),
            offset: Vec2::new(0.0, 0.0),
            layer: 0,
            collision_mask: all(),
            is_trigger: false,
            cached_aabb: RwLock::new(None),
            aabb_dirty: RwLock::new(true),
            on_collision_enter: Arc::new(Mutex::new(None)),
            on_collision_stay: Arc::new(Mutex::new(None)),
            on_collision_exit: Arc::new(Mutex::new(None)),
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

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    /// Override collision enter callback
    fn on_collision_enter(&self, other_id: u32, normal: Vec2, penetration: f32) {
        self.invoke_on_collision_enter(other_id, normal, penetration);
    }

    /// Override collision stay callback
    fn on_collision_stay(&self, other_id: u32, normal: Vec2, penetration: f32) {
        self.invoke_on_collision_stay(other_id, normal, penetration);
    }

    /// Override collision exit callback
    fn on_collision_exit(&self, other_id: u32) {
        self.invoke_on_collision_exit(other_id);
    }
}

impl ColliderComponent {
    /// Create a new collider with default settings
    pub fn new(name: impl Into<String>) -> Self {
        <Self as ComponentTrait>::new(name.into())
    }

    /// Set the collider shape
    pub fn with_shape(mut self, shape: ColliderShape) -> Self {
        self.shape = shape;
        self.mark_aabb_dirty();
        self
    }

    /// Set the offset from the GameObject center
    pub fn with_offset(mut self, offset: Vec2) -> Self {
        self.offset = offset;
        self.mark_aabb_dirty();
        self
    }

    /// Set the physics layer (0-31)
    pub fn with_layer(mut self, layer: u32) -> Self {
        self.layer = layer.min(31);
        self
    }

    /// Set the collision mask (bitfield of layers this collider can collide with)
    pub fn with_mask(mut self, mask: u32) -> Self {
        self.collision_mask = mask;
        self
    }

    /// Set whether this is a trigger (detects collisions but doesn't resolve physics)
    pub fn as_trigger(mut self, is_trigger: bool) -> Self {
        self.is_trigger = is_trigger;
        self
    }

    /// Get the collider shape
    pub fn shape(&self) -> &ColliderShape {
        &self.shape
    }

    /// Set the collider shape
    pub fn set_shape(&mut self, shape: ColliderShape) {
        self.shape = shape;
        self.mark_aabb_dirty();
    }

    /// Get the offset
    pub fn offset(&self) -> Vec2 {
        self.offset
    }

    /// Set the offset
    pub fn set_offset(&mut self, offset: Vec2) {
        self.offset = offset;
        self.mark_aabb_dirty();
    }

    /// Get the physics layer
    pub fn layer(&self) -> u32 {
        self.layer
    }

    /// Set the physics layer
    pub fn set_layer(&mut self, layer: u32) {
        self.layer = layer.min(31);
    }

    /// Get the collision mask
    pub fn collision_mask(&self) -> u32 {
        self.collision_mask
    }

    /// Set the collision mask
    pub fn set_collision_mask(&mut self, mask: u32) {
        self.collision_mask = mask;
    }

    /// Check if this is a trigger
    pub fn is_trigger(&self) -> bool {
        self.is_trigger
    }

    /// Set whether this is a trigger
    pub fn set_trigger(&mut self, is_trigger: bool) {
        self.is_trigger = is_trigger;
    }

    /// Check if this collider should collide with another
    pub fn should_collide_with(&self, other: &ColliderComponent) -> bool {
        should_collide(self.layer, self.collision_mask, other.layer, other.collision_mask)
    }

    /// Compute the AABB for this collider given a transform
    pub fn compute_aabb(&self, position: Vec2, rotation: f32, scale: Vec2) -> AABB {
        let world_position = position.add(&self.offset);
        self.shape.compute_aabb(world_position, rotation, scale)
    }

    /// Get the cached AABB (computes if dirty)
    pub fn get_aabb(&self, position: Vec2, rotation: f32, scale: Vec2) -> AABB {
        if *self.aabb_dirty.read().unwrap() {
            let aabb = self.compute_aabb(position, rotation, scale);
            *self.cached_aabb.write().unwrap() = Some(aabb);
            *self.aabb_dirty.write().unwrap() = false;
            aabb
        } else {
            self.cached_aabb.read().unwrap().unwrap()
        }
    }

    /// Mark the AABB as dirty (needs recomputation)
    pub fn mark_aabb_dirty(&self) {
        *self.aabb_dirty.write().unwrap() = true;
    }

    /// Set the collision enter callback
    /// Called when this collider first overlaps with another
    /// Parameters: other_id (u32), normal (Vec2), penetration (f32)
    pub fn set_on_collision_enter<F>(&mut self, callback: F)
    where
        F: FnMut(u32, Vec2, f32) + Send + Sync + 'static,
    {
        *self.on_collision_enter.lock().unwrap() = Some(Box::new(callback));
    }

    /// Set the collision stay callback
    /// Called each frame while this collider overlaps with another
    /// Parameters: other_id (u32), normal (Vec2), penetration (f32)
    pub fn set_on_collision_stay<F>(&mut self, callback: F)
    where
        F: FnMut(u32, Vec2, f32) + Send + Sync + 'static,
    {
        *self.on_collision_stay.lock().unwrap() = Some(Box::new(callback));
    }

    /// Set the collision exit callback
    /// Called when this collider stops overlapping with another
    /// Parameters: other_id (u32)
    pub fn set_on_collision_exit<F>(&mut self, callback: F)
    where
        F: FnMut(u32) + Send + Sync + 'static,
    {
        *self.on_collision_exit.lock().unwrap() = Some(Box::new(callback));
    }

    /// Invoke the collision enter callback (internal use)
    pub(crate) fn invoke_on_collision_enter(&self, other_id: u32, normal: Vec2, penetration: f32) {
        if let Ok(mut guard) = self.on_collision_enter.lock() {
            if let Some(callback) = guard.as_mut() {
                callback(other_id, normal, penetration);
            }
        }
    }

    /// Invoke the collision stay callback (internal use)
    pub(crate) fn invoke_on_collision_stay(&self, other_id: u32, normal: Vec2, penetration: f32) {
        if let Ok(mut guard) = self.on_collision_stay.lock() {
            if let Some(callback) = guard.as_mut() {
                callback(other_id, normal, penetration);
            }
        }
    }

    /// Invoke the collision exit callback (internal use)
    pub(crate) fn invoke_on_collision_exit(&self, other_id: u32) {
        if let Ok(mut guard) = self.on_collision_exit.lock() {
            if let Some(callback) = guard.as_mut() {
                callback(other_id);
            }
        }
    }
}
