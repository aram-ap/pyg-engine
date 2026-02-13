// Collision detection module for pyg_engine
// Provides collision detection without physics simulation
// Physics response system can be added in the future

pub mod shapes;
pub mod collider;
pub mod aabb_tree;
pub mod sat;
pub mod layers;
pub mod events;
pub mod collision_world;

// Re-export commonly used types
pub use shapes::{ColliderShape, AABB};
pub use collider::ColliderComponent;
pub use aabb_tree::AABBTree;
pub use sat::{SAT, CollisionManifold};
pub use layers::PhysicsLayers;
pub use events::{CollisionEvent, CollisionEventType};
pub use collision_world::CollisionWorld;
