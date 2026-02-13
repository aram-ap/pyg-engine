// Separating Axis Theorem (SAT) collision detection

use super::shapes::ColliderShape;
use crate::types::vector::Vec2;

/// Collision manifold containing collision information
#[derive(Debug, Clone, PartialEq)]
pub struct CollisionManifold {
    pub penetration_depth: f32,
    pub normal: Vec2,            // Points from A to B
    pub contact_points: Vec<Vec2>, // Contact points in world space
}

impl CollisionManifold {
    pub fn new(penetration_depth: f32, normal: Vec2, contact_points: Vec<Vec2>) -> Self {
        Self {
            penetration_depth,
            normal,
            contact_points,
        }
    }
}

/// SAT collision detection
pub struct SAT;

impl SAT {
    /// Test collision between two shapes
    pub fn test_collision(
        shape_a: &ColliderShape,
        pos_a: Vec2,
        rot_a: f32,
        scale_a: Vec2,
        shape_b: &ColliderShape,
        pos_b: Vec2,
        rot_b: f32,
        scale_b: Vec2,
    ) -> Option<CollisionManifold> {
        match (shape_a, shape_b) {
            (ColliderShape::Circle { radius: r1 }, ColliderShape::Circle { radius: r2 }) => {
                Self::circle_vs_circle(pos_a, *r1 * scale_a.x().max(scale_a.y()), pos_b, *r2 * scale_b.x().max(scale_b.y()))
            }
            (ColliderShape::Circle { radius }, ColliderShape::Box { half_extents })
            | (ColliderShape::Box { half_extents }, ColliderShape::Circle { radius }) => {
                let (circle_pos, circle_radius, box_pos, box_extents, box_rot, box_scale) =
                    if matches!(shape_a, ColliderShape::Circle { .. }) {
                        (pos_a, *radius * scale_a.x().max(scale_a.y()), pos_b, *half_extents, rot_b, scale_b)
                    } else {
                        (pos_b, *radius * scale_b.x().max(scale_b.y()), pos_a, *half_extents, rot_a, scale_a)
                    };
                Self::circle_vs_box(circle_pos, circle_radius, box_pos, box_extents, box_rot, box_scale)
            }
            (ColliderShape::Box { half_extents: h1 }, ColliderShape::Box { half_extents: h2 }) => {
                // If both are axis-aligned, use AABB test
                if rot_a.abs() < 0.001 && rot_b.abs() < 0.001 {
                    Self::aabb_vs_aabb(pos_a, h1.multiply(&scale_a), pos_b, h2.multiply(&scale_b))
                } else {
                    Self::obb_vs_obb(pos_a, h1.multiply(&scale_a), rot_a, pos_b, h2.multiply(&scale_b), rot_b)
                }
            }
            (ColliderShape::OBB { half_extents: h1, local_rotation: lr1 },
             ColliderShape::OBB { half_extents: h2, local_rotation: lr2 }) => {
                Self::obb_vs_obb(
                    pos_a, h1.multiply(&scale_a), rot_a + lr1,
                    pos_b, h2.multiply(&scale_b), rot_b + lr2
                )
            }
            (ColliderShape::Box { half_extents }, ColliderShape::OBB { half_extents: h2, local_rotation })
            | (ColliderShape::OBB { half_extents: h2, local_rotation }, ColliderShape::Box { half_extents }) => {
                let (pos1, ext1, rot1, pos2, ext2, rot2) = if matches!(shape_a, ColliderShape::Box { .. }) {
                    (pos_a, half_extents.multiply(&scale_a), rot_a, pos_b, h2.multiply(&scale_b), rot_b + local_rotation)
                } else {
                    (pos_b, half_extents.multiply(&scale_b), rot_b, pos_a, h2.multiply(&scale_a), rot_a + local_rotation)
                };
                Self::obb_vs_obb(pos1, ext1, rot1, pos2, ext2, rot2)
            }
            _ => {
                // Unsupported shape combination (Polygon support would go here)
                None
            }
        }
    }

    fn circle_vs_circle(pos_a: Vec2, radius_a: f32, pos_b: Vec2, radius_b: f32) -> Option<CollisionManifold> {
        let delta = pos_b.subtract(&pos_a);
        let distance_sq = delta.dot(&delta);
        let radius_sum = radius_a + radius_b;

        if distance_sq >= radius_sum * radius_sum {
            return None;
        }

        let distance = distance_sq.sqrt();

        if distance < 0.0001 {
            // Circles are at the same position
            return Some(CollisionManifold {
                penetration_depth: radius_sum,
                normal: Vec2::new(1.0, 0.0),
                contact_points: vec![pos_a],
            });
        }

        let normal = delta.multiply_scalar(1.0 / distance);
        let penetration = radius_sum - distance;
        let contact_point = pos_a.add(&normal.multiply_scalar(radius_a - penetration * 0.5));

        Some(CollisionManifold {
            penetration_depth: penetration,
            normal,
            contact_points: vec![contact_point],
        })
    }

    fn aabb_vs_aabb(pos_a: Vec2, half_extents_a: Vec2, pos_b: Vec2, half_extents_b: Vec2) -> Option<CollisionManifold> {
        let delta = pos_b.subtract(&pos_a);
        let overlap_x = (half_extents_a.x() + half_extents_b.x()) - delta.x().abs();
        let overlap_y = (half_extents_a.y() + half_extents_b.y()) - delta.y().abs();

        if overlap_x <= 0.0 || overlap_y <= 0.0 {
            return None;
        }

        // Find the axis of minimum penetration
        let (penetration, normal) = if overlap_x < overlap_y {
            let normal_x = if delta.x() < 0.0 { -1.0 } else { 1.0 };
            (overlap_x, Vec2::new(normal_x, 0.0))
        } else {
            let normal_y = if delta.y() < 0.0 { -1.0 } else { 1.0 };
            (overlap_y, Vec2::new(0.0, normal_y))
        };

        // Compute contact point (center of overlap region)
        let contact_point = pos_a.add(&delta.multiply_scalar(0.5));

        Some(CollisionManifold {
            penetration_depth: penetration,
            normal,
            contact_points: vec![contact_point],
        })
    }

    fn circle_vs_box(
        circle_pos: Vec2,
        radius: f32,
        box_pos: Vec2,
        half_extents: Vec2,
        box_rot: f32,
        box_scale: Vec2,
    ) -> Option<CollisionManifold> {
        let scaled_extents = half_extents.multiply(&box_scale);

        // Transform circle to box local space
        let delta = circle_pos.subtract(&box_pos);
        let cos = box_rot.cos();
        let sin = box_rot.sin();
        let local_x = delta.x() * cos + delta.y() * sin;
        let local_y = -delta.x() * sin + delta.y() * cos;
        let local_circle = Vec2::new(local_x, local_y);

        // Find closest point on box to circle
        let clamped_x = local_circle.x().clamp(-scaled_extents.x(), scaled_extents.x());
        let clamped_y = local_circle.y().clamp(-scaled_extents.y(), scaled_extents.y());
        let closest_local = Vec2::new(clamped_x, clamped_y);

        // Check if circle contains the closest point
        let delta_to_closest = local_circle.subtract(&closest_local);
        let distance_sq = delta_to_closest.dot(&delta_to_closest);

        if distance_sq >= radius * radius {
            return None;
        }

        let distance = distance_sq.sqrt();

        let (normal_local, penetration) = if distance < 0.0001 {
            // Circle center is inside the box
            // Find the closest edge
            let dx = scaled_extents.x() - local_circle.x().abs();
            let dy = scaled_extents.y() - local_circle.y().abs();

            if dx < dy {
                let sign = if local_circle.x() > 0.0 { 1.0 } else { -1.0 };
                (Vec2::new(sign, 0.0), dx + radius)
            } else {
                let sign = if local_circle.y() > 0.0 { 1.0 } else { -1.0 };
                (Vec2::new(0.0, sign), dy + radius)
            }
        } else {
            let normal_local = delta_to_closest.multiply_scalar(1.0 / distance);
            (normal_local, radius - distance)
        };

        // Transform normal back to world space
        let normal = Vec2::new(
            normal_local.x() * cos - normal_local.y() * sin,
            normal_local.x() * sin + normal_local.y() * cos,
        );

        // Contact point
        let contact_point = circle_pos.subtract(&normal.multiply_scalar(radius - penetration * 0.5));

        Some(CollisionManifold {
            penetration_depth: penetration,
            normal,
            contact_points: vec![contact_point],
        })
    }

    fn obb_vs_obb(
        pos_a: Vec2,
        half_extents_a: Vec2,
        rot_a: f32,
        pos_b: Vec2,
        half_extents_b: Vec2,
        rot_b: f32,
    ) -> Option<CollisionManifold> {
        let cos_a = rot_a.cos();
        let sin_a = rot_a.sin();
        let cos_b = rot_b.cos();
        let sin_b = rot_b.sin();

        // Axis vectors for both boxes
        let axes = [
            Vec2::new(cos_a, sin_a),        // A's X axis
            Vec2::new(-sin_a, cos_a),       // A's Y axis
            Vec2::new(cos_b, sin_b),        // B's X axis
            Vec2::new(-sin_b, cos_b),       // B's Y axis
        ];

        let mut min_overlap = f32::MAX;
        let mut min_axis = Vec2::new(1.0, 0.0);

        // Get corners of both boxes
        let corners_a = Self::get_box_corners(pos_a, half_extents_a, rot_a);
        let corners_b = Self::get_box_corners(pos_b, half_extents_b, rot_b);

        // Test all axes
        for axis in &axes {
            let (min_a, max_a) = Self::project_vertices(&corners_a, axis);
            let (min_b, max_b) = Self::project_vertices(&corners_b, axis);

            let overlap = (max_a.min(max_b) - min_a.max(min_b)).max(0.0);

            if overlap <= 0.0 {
                return None; // Separating axis found
            }

            if overlap < min_overlap {
                min_overlap = overlap;
                min_axis = *axis;
            }
        }

        // Ensure normal points from A to B
        let delta = pos_b.subtract(&pos_a);
        if delta.dot(&min_axis) < 0.0 {
            min_axis = min_axis.multiply_scalar(-1.0);
        }

        // Simple contact point (midpoint between centers)
        let contact_point = pos_a.add(&pos_b).multiply_scalar(0.5);

        Some(CollisionManifold {
            penetration_depth: min_overlap,
            normal: min_axis,
            contact_points: vec![contact_point],
        })
    }

    fn get_box_corners(pos: Vec2, half_extents: Vec2, rotation: f32) -> [Vec2; 4] {
        let cos = rotation.cos();
        let sin = rotation.sin();

        let local_corners = [
            Vec2::new(-half_extents.x(), -half_extents.y()),
            Vec2::new(half_extents.x(), -half_extents.y()),
            Vec2::new(half_extents.x(), half_extents.y()),
            Vec2::new(-half_extents.x(), half_extents.y()),
        ];

        local_corners.map(|corner| {
            let rotated_x = corner.x() * cos - corner.y() * sin;
            let rotated_y = corner.x() * sin + corner.y() * cos;
            Vec2::new(rotated_x + pos.x(), rotated_y + pos.y())
        })
    }

    fn project_vertices(vertices: &[Vec2], axis: &Vec2) -> (f32, f32) {
        let mut min = f32::MAX;
        let mut max = f32::MIN;

        for vertex in vertices {
            let projection = vertex.dot(axis);
            min = min.min(projection);
            max = max.max(projection);
        }

        (min, max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_collision() {
        let pos_a = Vec2::new(0.0, 0.0);
        let pos_b = Vec2::new(1.0, 0.0);

        let manifold = SAT::circle_vs_circle(pos_a, 0.75, pos_b, 0.75);
        assert!(manifold.is_some());

        let manifold = SAT::circle_vs_circle(pos_a, 0.4, pos_b, 0.4);
        assert!(manifold.is_none());
    }

    #[test]
    fn test_aabb_collision() {
        let pos_a = Vec2::new(0.0, 0.0);
        let pos_b = Vec2::new(0.5, 0.0);
        let half_extents = Vec2::new(0.5, 0.5);

        let manifold = SAT::aabb_vs_aabb(pos_a, half_extents, pos_b, half_extents);
        assert!(manifold.is_some());
    }
}
