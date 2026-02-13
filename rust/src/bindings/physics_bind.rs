use crate::core::physics::*;
use crate::types::vector::Vec2;
use pyo3::prelude::*;

// ========== Collision Detection Bindings ==========

/// Physics layer constants for collision filtering
#[pyclass(name = "PhysicsLayers")]
#[derive(Clone)]
pub struct PyPhysicsLayers;

#[pymethods]
impl PyPhysicsLayers {
    #[classattr]
    const DEFAULT: u32 = PhysicsLayers::DEFAULT;

    #[classattr]
    const PLAYER: u32 = PhysicsLayers::PLAYER;

    #[classattr]
    const ENEMY: u32 = PhysicsLayers::ENEMY;

    #[classattr]
    const PROJECTILE: u32 = PhysicsLayers::PROJECTILE;

    #[classattr]
    const ENVIRONMENT: u32 = PhysicsLayers::ENVIRONMENT;

    #[classattr]
    const TRIGGER: u32 = PhysicsLayers::TRIGGER;

    #[classattr]
    const UI: u32 = PhysicsLayers::UI;

    #[classattr]
    const PICKUP: u32 = PhysicsLayers::PICKUP;

    /// Create a collision mask from a list of layers
    #[staticmethod]
    fn create_mask(layers: Vec<u32>) -> u32 {
        layers::create_mask(&layers)
    }

    /// Create a mask that collides with all layers
    #[staticmethod]
    fn all() -> u32 {
        layers::all()
    }

    /// Create a mask that collides with no layers
    #[staticmethod]
    fn none() -> u32 {
        layers::none()
    }
}

/// Collider shape builder
#[pyclass(name = "ColliderShape")]
#[derive(Clone)]
pub struct PyColliderShape {
    pub(crate) shape: ColliderShape,
}

#[pymethods]
impl PyColliderShape {
    /// Create a circle collider
    #[staticmethod]
    fn circle(radius: f32) -> Self {
        Self {
            shape: ColliderShape::circle(radius),
        }
    }

    /// Create a box collider
    #[staticmethod]
    fn box_shape(half_width: f32, half_height: f32) -> Self {
        Self {
            shape: ColliderShape::box_shape(Vec2::new(half_width, half_height)),
        }
    }

    /// Create an oriented box collider
    #[staticmethod]
    fn obb(half_width: f32, half_height: f32, local_rotation: f32) -> Self {
        Self {
            shape: ColliderShape::obb(Vec2::new(half_width, half_height), local_rotation),
        }
    }

    /// Create a polygon collider
    #[staticmethod]
    fn polygon(vertices: Vec<(f32, f32)>) -> Self {
        let verts: Vec<Vec2> = vertices.iter().map(|&(x, y)| Vec2::new(x, y)).collect();
        Self {
            shape: ColliderShape::polygon(verts),
        }
    }
}

/// Collider component for collision detection
#[pyclass(name = "Collider")]
pub struct PyCollider {
    pub(crate) component: ColliderComponent,
}

#[pymethods]
impl PyCollider {
    /// Create a new collider component
    #[new]
    fn new(name: String) -> Self {
        Self {
            component: ColliderComponent::new(name),
        }
    }

    /// Set the collider shape
    fn set_shape(&mut self, shape: PyColliderShape) {
        self.component.set_shape(shape.shape);
    }

    /// Set the offset from the GameObject center
    fn set_offset(&mut self, x: f32, y: f32) {
        self.component.set_offset(Vec2::new(x, y));
    }

    /// Set the physics layer (0-31)
    fn set_layer(&mut self, layer: u32) {
        self.component.set_layer(layer);
    }

    /// Set the collision mask
    fn set_collision_mask(&mut self, mask: u32) {
        self.component.set_collision_mask(mask);
    }

    /// Set whether this is a trigger
    fn set_trigger(&mut self, is_trigger: bool) {
        self.component.set_trigger(is_trigger);
    }

    /// Get the physics layer
    #[getter]
    fn layer(&self) -> u32 {
        self.component.layer()
    }

    /// Get the collision mask
    #[getter]
    fn collision_mask(&self) -> u32 {
        self.component.collision_mask()
    }

    /// Check if this is a trigger
    #[getter]
    fn is_trigger(&self) -> bool {
        self.component.is_trigger()
    }
}

/// Register collision detection bindings with Python
pub fn register_physics_bindings(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyPhysicsLayers>()?;
    m.add_class::<PyColliderShape>()?;
    m.add_class::<PyCollider>()?;
    Ok(())
}
