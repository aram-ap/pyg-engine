// Collision detection world manager
// Detects collisions and dispatches events, but does not simulate physics

use super::aabb_tree::AABBTree;
use super::collider::ColliderComponent;
use super::events::{CollisionEvent, CollisionEventType};
use super::sat::SAT;
use crate::core::object_manager::ObjectManager;
use std::collections::HashSet;

/// Collision pair identifier (always ordered: smaller ID first)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CollisionPair(u32, u32);

impl CollisionPair {
    fn new(id_a: u32, id_b: u32) -> Self {
        if id_a < id_b {
            CollisionPair(id_a, id_b)
        } else {
            CollisionPair(id_b, id_a)
        }
    }
}

/// Collision detection world - tracks collisions without physics simulation
pub struct CollisionWorld {
    aabb_tree: AABBTree,

    // Track collision pairs across frames
    collision_pairs: HashSet<CollisionPair>,

    // Events to dispatch
    collision_events: Vec<CollisionEvent>,
}

impl CollisionWorld {
    pub fn new() -> Self {
        Self {
            aabb_tree: AABBTree::new(),
            collision_pairs: HashSet::new(),
            collision_events: Vec::new(),
        }
    }

    /// Run collision detection step
    pub fn step(&mut self, object_manager: &ObjectManager) {
        // Clear previous frame's events
        self.collision_events.clear();

        // 1. Update broad-phase (sync AABB tree with transforms)
        self.update_broadphase(object_manager);

        // 2. Find collision pairs using broad-phase
        let potential_pairs = self.find_collision_pairs(object_manager);

        // Track new collision pairs for this frame
        let mut new_collision_pairs = HashSet::new();

        // 3. Narrow-phase collision detection
        for (id_a, id_b) in potential_pairs {
            let pair = CollisionPair::new(id_a, id_b);

            // Get objects and components
            let (obj_a, obj_b) = match (
                object_manager.get_object_by_id(id_a),
                object_manager.get_object_by_id(id_b),
            ) {
                (Some(a), Some(b)) => (a, b),
                _ => continue,
            };

            let (collider_a, collider_b) = match (
                obj_a.get_component::<ColliderComponent>(),
                obj_b.get_component::<ColliderComponent>(),
            ) {
                (Some(a), Some(b)) => (a, b),
                _ => continue,
            };

            // Check layer filtering
            if !collider_a.should_collide_with(&collider_b) {
                continue;
            }

            // Narrow-phase collision detection
            let manifold = SAT::test_collision(
                collider_a.shape(),
                obj_a.position(),
                obj_a.rotation(),
                obj_a.scale(),
                collider_b.shape(),
                obj_b.position(),
                obj_b.rotation(),
                obj_b.scale(),
            );

            if let Some(manifold) = manifold {
                new_collision_pairs.insert(pair);

                // Determine event type
                let event_type = if self.collision_pairs.contains(&pair) {
                    CollisionEventType::Stay
                } else {
                    CollisionEventType::Enter
                };

                // Create collision event
                let event = CollisionEvent::new(id_a, id_b, event_type, Some(manifold));
                self.collision_events.push(event);
            }
        }

        // 4. Handle collision exit events
        for pair in &self.collision_pairs {
            if !new_collision_pairs.contains(pair) {
                let event = CollisionEvent::exit(pair.0, pair.1);
                self.collision_events.push(event);
            }
        }

        // Update collision pairs for next frame
        self.collision_pairs = new_collision_pairs;

        // 5. Dispatch collision callbacks to components
        self.dispatch_collision_callbacks(object_manager);
    }

    /// Get collision events from the last step
    pub fn collision_events(&self) -> &[CollisionEvent] {
        &self.collision_events
    }

    fn update_broadphase(&mut self, object_manager: &ObjectManager) {
        // Get all objects with colliders
        let all_objects = object_manager.get_keys();
        let mut tracked_objects = HashSet::new();

        for &object_id in all_objects {
            if let Some(obj) = object_manager.get_object_by_id(object_id) {
                if let Some(collider) = obj.get_component::<ColliderComponent>() {
                    tracked_objects.insert(object_id);

                    let aabb = collider.compute_aabb(
                        obj.position(),
                        obj.rotation(),
                        obj.scale(),
                    );

                    // Update or insert in AABB tree
                    if !self.aabb_tree.update(object_id, aabb) {
                        // Object not in tree, insert it
                        self.aabb_tree.insert(object_id, aabb);
                    }
                }
            }
        }

        // Remove objects that no longer have colliders
        let tree_objects: HashSet<u32> = self.aabb_tree.get_all_objects().into_iter().collect();
        for &object_id in tree_objects.iter() {
            if !tracked_objects.contains(&object_id) {
                self.aabb_tree.remove(object_id);
            }
        }
    }

    fn find_collision_pairs(&self, object_manager: &ObjectManager) -> Vec<(u32, u32)> {
        let mut pairs = Vec::new();
        let all_objects = self.aabb_tree.get_all_objects();

        for &object_id in &all_objects {
            if let Some(obj) = object_manager.get_object_by_id(object_id) {
                if let Some(collider) = obj.get_component::<ColliderComponent>() {
                    let aabb = collider.compute_aabb(
                        obj.position(),
                        obj.rotation(),
                        obj.scale(),
                    );

                    let overlapping = self.aabb_tree.query(&aabb);

                    for &other_id in &overlapping {
                        if other_id > object_id {
                            pairs.push((object_id, other_id));
                        }
                    }
                }
            }
        }

        pairs
    }

    fn dispatch_collision_callbacks(&self, object_manager: &ObjectManager) {
        use crate::core::component::ComponentTrait;

        // Dispatch callbacks for all collision events
        for event in &self.collision_events {
            let obj_a = object_manager.get_object_by_id(event.object_id_a);
            let obj_b = object_manager.get_object_by_id(event.object_id_b);

            if let (Some(obj_a), Some(obj_b)) = (obj_a, obj_b) {
                // Get all components from both objects
                let components_a: Vec<&dyn ComponentTrait> = obj_a
                    .components_iter()
                    .map(|c| c.as_ref() as &dyn ComponentTrait)
                    .collect();

                let components_b: Vec<&dyn ComponentTrait> = obj_b
                    .components_iter()
                    .map(|c| c.as_ref() as &dyn ComponentTrait)
                    .collect();

                // Extract collision info
                let (normal, penetration) = if let Some(ref manifold) = event.manifold {
                    (manifold.normal, manifold.penetration_depth)
                } else {
                    (crate::types::vector::Vec2::new(0.0, 0.0), 0.0)
                };

                // Call callbacks on all components
                match event.event_type {
                    CollisionEventType::Enter => {
                        for component in &components_a {
                            component.on_collision_enter(event.object_id_b, normal, penetration);
                        }
                        for component in &components_b {
                            component.on_collision_enter(event.object_id_a, normal.multiply_scalar(-1.0), penetration);
                        }
                    }
                    CollisionEventType::Stay => {
                        for component in &components_a {
                            component.on_collision_stay(event.object_id_b, normal, penetration);
                        }
                        for component in &components_b {
                            component.on_collision_stay(event.object_id_a, normal.multiply_scalar(-1.0), penetration);
                        }
                    }
                    CollisionEventType::Exit => {
                        for component in &components_a {
                            component.on_collision_exit(event.object_id_b);
                        }
                        for component in &components_b {
                            component.on_collision_exit(event.object_id_a);
                        }
                    }
                }
            }
        }
    }
}

impl Default for CollisionWorld {
    fn default() -> Self {
        Self::new()
    }
}
