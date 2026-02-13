// Collider shape definitions

use crate::types::vector::Vec2;

/// Axis-Aligned Bounding Box
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AABB {
    pub min: Vec2,
    pub max: Vec2,
}

impl AABB {
    pub fn new(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }

    pub fn from_center_size(center: Vec2, half_extents: Vec2) -> Self {
        Self {
            min: center.subtract(&half_extents),
            max: center.add(&half_extents),
        }
    }

    pub fn center(&self) -> Vec2 {
        self.min.add(&self.max).multiply_scalar(0.5)
    }

    pub fn half_extents(&self) -> Vec2 {
        self.max.subtract(&self.min).multiply_scalar(0.5)
    }

    pub fn overlaps(&self, other: &AABB) -> bool {
        self.min.x() <= other.max.x()
            && self.max.x() >= other.min.x()
            && self.min.y() <= other.max.y()
            && self.max.y() >= other.min.y()
    }

    pub fn contains_point(&self, point: &Vec2) -> bool {
        point.x() >= self.min.x()
            && point.x() <= self.max.x()
            && point.y() >= self.min.y()
            && point.y() <= self.max.y()
    }

    pub fn merge(&self, other: &AABB) -> AABB {
        AABB {
            min: Vec2::new(
                self.min.x().min(other.min.x()),
                self.min.y().min(other.min.y()),
            ),
            max: Vec2::new(
                self.max.x().max(other.max.x()),
                self.max.y().max(other.max.y()),
            ),
        }
    }

    pub fn surface_area(&self) -> f32 {
        let extent = self.max.subtract(&self.min);
        2.0 * (extent.x() + extent.y())
    }

    /// Fatten the AABB by a margin (used for dynamic AABB tree)
    pub fn fatten(&self, margin: f32) -> AABB {
        let margin_vec = Vec2::new(margin, margin);
        AABB {
            min: self.min.subtract(&margin_vec),
            max: self.max.add(&margin_vec),
        }
    }
}

/// Collider shape types
#[derive(Debug, Clone, PartialEq)]
pub enum ColliderShape {
    /// Circle with radius
    Circle { radius: f32 },
    /// Axis-aligned box with half extents
    Box { half_extents: Vec2 },
    /// Oriented box with half extents and local rotation
    OBB {
        half_extents: Vec2,
        local_rotation: f32,
    },
    /// Convex polygon with vertices in local space
    Polygon { vertices: Vec<Vec2> },
}

impl ColliderShape {
    /// Create a circle collider
    pub fn circle(radius: f32) -> Self {
        ColliderShape::Circle { radius }
    }

    /// Create a box collider (axis-aligned)
    pub fn box_shape(half_extents: Vec2) -> Self {
        ColliderShape::Box { half_extents }
    }

    /// Create an oriented box collider
    pub fn obb(half_extents: Vec2, local_rotation: f32) -> Self {
        ColliderShape::OBB {
            half_extents,
            local_rotation,
        }
    }

    /// Create a convex polygon collider
    pub fn polygon(vertices: Vec<Vec2>) -> Self {
        ColliderShape::Polygon { vertices }
    }

    /// Compute the AABB for this shape given a transform
    pub fn compute_aabb(&self, position: Vec2, rotation: f32, scale: Vec2) -> AABB {
        match self {
            ColliderShape::Circle { radius } => {
                let scaled_radius = *radius * scale.x().max(scale.y());
                AABB::from_center_size(position, Vec2::new(scaled_radius, scaled_radius))
            }
            ColliderShape::Box { half_extents } => {
                let scaled_extents = half_extents.multiply(&scale);
                if rotation.abs() < 0.001 {
                    // Axis-aligned optimization
                    AABB::from_center_size(position, scaled_extents)
                } else {
                    // Compute corners and find bounds
                    self.compute_rotated_aabb(position, rotation, scale)
                }
            }
            ColliderShape::OBB {
                half_extents,
                local_rotation,
            } => {
                let total_rotation = rotation + local_rotation;
                let scaled_extents = half_extents.multiply(&scale);
                self.compute_obb_aabb(position, total_rotation, scaled_extents)
            }
            ColliderShape::Polygon { vertices } => {
                self.compute_polygon_aabb(vertices, position, rotation, scale)
            }
        }
    }

    fn compute_rotated_aabb(&self, position: Vec2, rotation: f32, scale: Vec2) -> AABB {
        if let ColliderShape::Box { half_extents } = self {
            let scaled_extents = half_extents.multiply(&scale);
            self.compute_obb_aabb(position, rotation, scaled_extents)
        } else {
            // Fallback
            AABB::from_center_size(position, Vec2::new(1.0, 1.0))
        }
    }

    fn compute_obb_aabb(&self, position: Vec2, rotation: f32, half_extents: Vec2) -> AABB {
        let cos = rotation.cos();
        let sin = rotation.sin();

        // Get the four corners of the OBB
        let corners = [
            Vec2::new(-half_extents.x(), -half_extents.y()),
            Vec2::new(half_extents.x(), -half_extents.y()),
            Vec2::new(half_extents.x(), half_extents.y()),
            Vec2::new(-half_extents.x(), half_extents.y()),
        ];

        let mut min_x = f32::MAX;
        let mut max_x = f32::MIN;
        let mut min_y = f32::MAX;
        let mut max_y = f32::MIN;

        for corner in &corners {
            let rotated_x = corner.x() * cos - corner.y() * sin + position.x();
            let rotated_y = corner.x() * sin + corner.y() * cos + position.y();

            min_x = min_x.min(rotated_x);
            max_x = max_x.max(rotated_x);
            min_y = min_y.min(rotated_y);
            max_y = max_y.max(rotated_y);
        }

        AABB::new(Vec2::new(min_x, min_y), Vec2::new(max_x, max_y))
    }

    fn compute_polygon_aabb(
        &self,
        vertices: &[Vec2],
        position: Vec2,
        rotation: f32,
        scale: Vec2,
    ) -> AABB {
        if vertices.is_empty() {
            return AABB::from_center_size(position, Vec2::new(0.1, 0.1));
        }

        let cos = rotation.cos();
        let sin = rotation.sin();

        let mut min_x = f32::MAX;
        let mut max_x = f32::MIN;
        let mut min_y = f32::MAX;
        let mut max_y = f32::MIN;

        for vertex in vertices {
            let scaled = vertex.multiply(&scale);
            let rotated_x = scaled.x() * cos - scaled.y() * sin + position.x();
            let rotated_y = scaled.x() * sin + scaled.y() * cos + position.y();

            min_x = min_x.min(rotated_x);
            max_x = max_x.max(rotated_x);
            min_y = min_y.min(rotated_y);
            max_y = max_y.max(rotated_y);
        }

        AABB::new(Vec2::new(min_x, min_y), Vec2::new(max_x, max_y))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aabb_overlap() {
        let aabb1 = AABB::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0));
        let aabb2 = AABB::new(Vec2::new(0.5, 0.5), Vec2::new(1.5, 1.5));
        assert!(aabb1.overlaps(&aabb2));

        let aabb3 = AABB::new(Vec2::new(2.0, 2.0), Vec2::new(3.0, 3.0));
        assert!(!aabb1.overlaps(&aabb3));
    }

    #[test]
    fn test_circle_aabb() {
        let shape = ColliderShape::circle(1.0);
        let aabb = shape.compute_aabb(Vec2::new(0.0, 0.0), 0.0, Vec2::new(1.0, 1.0));
        assert_eq!(aabb.center(), Vec2::new(0.0, 0.0));
    }
}
