// Collision event system

use super::sat::CollisionManifold;

/// Types of collision events
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollisionEventType {
    /// Collision just started this frame
    Enter,
    /// Collision is ongoing (was present last frame and this frame)
    Stay,
    /// Collision just ended this frame
    Exit,
}

/// Collision event data
#[derive(Debug, Clone)]
pub struct CollisionEvent {
    /// The first object ID involved in the collision
    pub object_id_a: u32,
    /// The second object ID involved in the collision
    pub object_id_b: u32,
    /// The type of collision event
    pub event_type: CollisionEventType,
    /// Collision manifold (None for Exit events)
    pub manifold: Option<CollisionManifold>,
}

impl CollisionEvent {
    pub fn new(
        object_id_a: u32,
        object_id_b: u32,
        event_type: CollisionEventType,
        manifold: Option<CollisionManifold>,
    ) -> Self {
        Self {
            object_id_a,
            object_id_b,
            event_type,
            manifold,
        }
    }

    pub fn enter(object_id_a: u32, object_id_b: u32, manifold: CollisionManifold) -> Self {
        Self::new(object_id_a, object_id_b, CollisionEventType::Enter, Some(manifold))
    }

    pub fn stay(object_id_a: u32, object_id_b: u32, manifold: CollisionManifold) -> Self {
        Self::new(object_id_a, object_id_b, CollisionEventType::Stay, Some(manifold))
    }

    pub fn exit(object_id_a: u32, object_id_b: u32) -> Self {
        Self::new(object_id_a, object_id_b, CollisionEventType::Exit, None)
    }
}

/// Trait for components that want to receive collision callbacks
pub trait CollisionCallbacks {
    fn on_collision_enter(&self, _other_id: u32, _manifold: &CollisionManifold) {}
    fn on_collision_stay(&self, _other_id: u32, _manifold: &CollisionManifold) {}
    fn on_collision_exit(&self, _other_id: u32) {}
}
