import pygame as pg
from pygame import Vector2, Rect
import math
from .component import Component
from .material import PhysicsMaterial, Materials

class CollisionInfo:
    """Information about a collision between two objects."""

    def __init__(self, other_collider, contact_point, contact_normal, penetration_depth):
        self.other_collider = other_collider  # The other collider involved
        self.other_gameobject = other_collider.game_object  # Convenience reference
        self.contact_point = contact_point  # Where the collision occurred
        self.contact_normal = contact_normal  # Direction to separate objects
        self.penetration_depth = penetration_depth  # How much they're overlapping

    def __repr__(self):
        return f"CollisionInfo(other={self.other_gameobject.name}, depth={self.penetration_depth:.2f})"

class Collider(Component):
    """Base collider component for collision detection."""

    def __init__(self, game_object, is_trigger=False, material=None, collision_layer="Default"):
        super().__init__(game_object)

        # Collision settings
        self.is_trigger = is_trigger  # If True, detects collisions but doesn't stop movement
        self.material = material or Materials.DEFAULT
        self.collision_layer = collision_layer

        # Collision state tracking
        self._colliding_with = set()  # Objects currently colliding with
        self._collision_callbacks = {
            'enter': [],  # Called when collision starts
            'stay': [],   # Called while colliding
            'exit': []    # Called when collision ends
        }

        # Bounds (implemented by subclasses)
        self.bounds = None

        print(f"Collider created on {game_object.name} (trigger={is_trigger})")

    def start(self):
        """Initialize the collider."""
        self.update_bounds()
        print(f"Collider started on {self.game_object.name}")

    def update(self, engine):
        """Update collider bounds to match GameObject position."""
        if self.enabled:
            self.update_bounds()

    def update_bounds(self):
        """Update collision bounds based on GameObject position. Override in subclasses."""
        pass

    def check_collision(self, other_collider):
        """Check if this collider is colliding with another. Returns CollisionInfo or None."""
        if not (self.enabled and other_collider.enabled):
            return None
        if other_collider == self:
            return None

        # This will be implemented by subclasses for specific collision types
        return None

    # ================ Collision Event System ================

    def add_collision_callback(self, event_type, callback):
        """Add a callback for collision events. event_type: 'enter', 'stay', 'exit'."""
        if event_type in self._collision_callbacks:
            self._collision_callbacks[event_type].append(callback)

    def remove_collision_callback(self, event_type, callback):
        """Remove a collision callback."""
        if event_type in self._collision_callbacks and callback in self._collision_callbacks[event_type]:
            self._collision_callbacks[event_type].remove(callback)

    def _trigger_collision_event(self, event_type, collision_info):
        """Trigger collision callbacks."""
        for callback in self._collision_callbacks[event_type]:
            try:
                callback(collision_info)
            except Exception as e:
                print(f"Error in collision callback: {e}")

    def handle_collision(self, collision_info):
        """Handle a collision. Called by the physics system."""
        other_collider = collision_info.other_collider

        # Check if this is a new collision
        if other_collider not in self._colliding_with:
            self._colliding_with.add(other_collider)
            self._trigger_collision_event('enter', collision_info)
        else:
            self._trigger_collision_event('stay', collision_info)

    def end_collision(self, other_collider):
        """End a collision. Called by the physics system."""
        if other_collider in self._colliding_with:
            self._colliding_with.remove(other_collider)
            # Create a basic collision info for exit event
            exit_info = CollisionInfo(other_collider, Vector2(0, 0), Vector2(0, 0), 0)
            self._trigger_collision_event('exit', exit_info)

class BoxCollider(Collider):
    """Rectangle-based collider with rotation support."""

    def __init__(self, game_object, width=None, height=None, offset=Vector2(0, 0), **kwargs):
        super().__init__(game_object, **kwargs)

        # Use GameObject size if width/height not specified
        if width is None:
            width = game_object.size.x if game_object.size.x > 0 else 32
        if height is None:
            height = game_object.size.y if game_object.size.y > 0 else 32

        self.width = width
        self.height = height
        self.offset = offset  # Offset from GameObject center
        self.bounds = Rect(0, 0, width, height)  # AABB for broad-phase

        # Rotation support
        self.corners = []  # World-space corners of rotated rectangle
        self.edges = []    # World-space edges (for SAT)
        self.normals = []  # Edge normals (for SAT)
        self.is_rotated = False  # Optimization flag

        print(f"BoxCollider created: {width}x{height}")

    def update_bounds(self):
        """Update the collision rectangle based on GameObject position and rotation."""
        center_x = self.game_object.position.x + self.offset.x
        center_y = self.game_object.position.y + self.offset.y
        rotation = self.game_object.rotation

        # Check if object is rotated (with small tolerance)
        self.is_rotated = abs(rotation) > 0.1

        if not self.is_rotated:
            # Simple AABB update for non-rotated objects (faster)
            self.bounds.centerx = int(center_x)
            self.bounds.centery = int(center_y)
            self.corners = []
            self.edges = []
            self.normals = []
        else:
            # Calculate rotated corners
            half_w = self.width / 2
            half_h = self.height / 2

            # Match clockwise screen rotation with counter-clockwise math rotation
            angle_rad = -math.radians(rotation)
            cos_a = math.cos(angle_rad)
            sin_a = math.sin(angle_rad)

            # Local corners (relative to center)
            local_corners = [
                Vector2(-half_w, -half_h),
                Vector2(half_w, -half_h),
                Vector2(half_w, half_h),
                Vector2(-half_w, half_h)
            ]

            # Rotate and translate corners to world space
            self.corners = []
            min_x, max_x = float('inf'), float('-inf')
            min_y, max_y = float('inf'), float('-inf')

            for corner in local_corners:
                rotated_x = corner.x * cos_a - corner.y * sin_a
                rotated_y = corner.x * sin_a + corner.y * cos_a
                world_x = center_x + rotated_x
                world_y = center_y + rotated_y
                self.corners.append(Vector2(world_x, world_y))

                min_x = min(min_x, world_x)
                max_x = max(max_x, world_x)
                min_y = min(min_y, world_y)
                max_y = max(max_y, world_y)

            # Update AABB bounds (encompasses the rotated rectangle)
            self.bounds = Rect(min_x, min_y, max_x - min_x, max_y - min_y)

            # Calculate edges and normals for SAT
            self.edges = []
            self.normals = []
            for i in range(4):
                edge = self.corners[(i + 1) % 4] - self.corners[i]
                self.edges.append(edge)
                normal = Vector2(-edge.y, edge.x).normalize()
                self.normals.append(normal)

    def check_collision(self, other_collider):
        """Check collision with another collider."""
        result = super().check_collision(other_collider)
        if result is not None: return result

        if isinstance(other_collider, BoxCollider):
            return self._check_box_vs_box(other_collider)
        elif isinstance(other_collider, CircleCollider):
            return self._check_box_vs_circle(other_collider)

        return None

    def get_world_corners(self):
        """Returns the four world-space corners of the box, even if not rotated."""
        if self.is_rotated and self.corners:
            return self.corners

        center = self.bounds.center
        half_w = self.width / 2
        half_h = self.height / 2
        return [
            Vector2(center[0] - half_w, center[1] - half_h),
            Vector2(center[0] + half_w, center[1] - half_h),
            Vector2(center[0] + half_w, center[1] + half_h),
            Vector2(center[0] - half_w, center[1] + half_h),
        ]

    def _find_best_contact_point(self, box_a_corners, box_b_corners):
        """Finds the vertex of one box that is deepest inside the other."""
        deepest_point = None
        max_penetration = -float('inf')

        for v in box_a_corners:
            # Check if vertex v of box A is inside box B
            # This is a simplification; a full check is more complex.
            # We find the vertex that is 'most' inside the other shape.
            # A full implementation would involve projecting onto normals.
            pass  # Placeholder for a more complex geometric calculation

        # For now, we will find the two closest vertices between the boxes
        min_dist_sq = float('inf')
        closest_pair = (None, None)
        for v1 in box_a_corners:
            for v2 in box_b_corners:
                dist_sq = (v1 - v2).length_squared()
                if dist_sq < min_dist_sq:
                    min_dist_sq = dist_sq
                    closest_pair = (v1, v2)

        if closest_pair[0] and closest_pair[1]:
            # The contact point is the midpoint of the two closest vertices
            return (closest_pair[0] + closest_pair[1]) / 2.0

        return None

    def _check_box_vs_box(self, other):
        """Box vs Box collision detection with rotation support using SAT."""
        if not self.bounds.colliderect(other.bounds):
            return None

        if not self.is_rotated and not other.is_rotated:
            return self._check_aabb_vs_aabb(other)

        return self._check_sat_box_vs_box(other)

    def _check_sat_box_vs_box(self, other):
        """SAT-based collision detection for rotated boxes."""
        axes = []
        axes.extend(self.normals[:2] if self.is_rotated else [Vector2(1, 0), Vector2(0, 1)])
        axes.extend(other.normals[:2] if other.is_rotated else [Vector2(1, 0), Vector2(0, 1)])

        min_overlap = float('inf')
        min_axis = None

        for axis in axes:
            if axis.length_squared() == 0: continue

            min1, max1 = self._project_on_axis(self.get_world_corners(), axis)
            min2, max2 = self._project_on_axis(other.get_world_corners(), axis)

            if max1 < min2 or max2 < min1:
                return None

            overlap = min(max1, max2) - max(min1, min2)
            if overlap < min_overlap:
                min_overlap = overlap
                min_axis = axis

        if min_axis is None: return None

        # Ensure the collision normal points from other to self
        center_vec = self.game_object.position - other.game_object.position
        if center_vec.dot(min_axis) < 0:
            min_axis = -min_axis

        # Use the improved contact point calculation
        contact_point = self._find_best_contact_point(self.get_world_corners(), other.get_world_corners())

        # Fallback if the new method fails
        if contact_point is None:
             contact_point = self.game_object.position - min_axis * (self.width / 2)

        return CollisionInfo(other, contact_point, min_axis, min_overlap)

    def _project_on_axis(self, corners, axis):
        """Helper for SAT: projects corners onto an axis and returns min/max."""
        min_proj, max_proj = float('inf'), float('-inf')
        for corner in corners:
            proj = corner.dot(axis)
            min_proj = min(min_proj, proj)
            max_proj = max(max_proj, proj)
        return min_proj, max_proj

    def _check_aabb_vs_aabb(self, other):
        """Original AABB vs AABB collision detection."""
        overlap_x = min(self.bounds.right, other.bounds.right) - max(self.bounds.left, other.bounds.left)
        overlap_y = min(self.bounds.bottom, other.bounds.bottom) - max(self.bounds.top, other.bounds.top)

        if overlap_x < overlap_y:
            penetration = overlap_x
            if self.bounds.centerx < other.bounds.centerx:
                normal = Vector2(-1, 0)
                contact_point = Vector2(self.bounds.right, self.bounds.centery)
            else:
                normal = Vector2(1, 0)
                contact_point = Vector2(self.bounds.left, self.bounds.centery)
        else:
            penetration = overlap_y
            if self.bounds.centery < other.bounds.centery:
                normal = Vector2(0, -1)
                contact_point = Vector2(self.bounds.centerx, self.bounds.bottom)
            else:
                normal = Vector2(0, 1)
                contact_point = Vector2(self.bounds.centerx, self.bounds.top)

        return CollisionInfo(other, contact_point, normal, penetration)

    def _check_box_vs_circle(self, circle_collider):
        """Box vs Circle collision detection with proper rotation support."""
        circle_center = Vector2(circle_collider.center_x, circle_collider.center_y)

        if not self.is_rotated:
            # Efficient method for non-rotated boxes
            closest_point = Vector2(
                max(self.bounds.left, min(circle_center.x, self.bounds.right)),
                max(self.bounds.top, min(circle_center.y, self.bounds.bottom))
            )
            distance_vec = circle_center - closest_point
            distance = distance_vec.length()

            if distance > circle_collider.radius: return None

            penetration = circle_collider.radius - distance
            if distance == 0:
                # Fallback for when circle center is inside box
                return CollisionInfo(circle_collider, circle_center, -distance_vec.normalize(), penetration)
            return CollisionInfo(circle_collider, closest_point, -distance_vec.normalize(), penetration)

        else:
            # Proper rotated box vs circle collision
            box_center = self.game_object.position
            local_circle_pos = circle_center - box_center

            angle_rad = math.radians(self.game_object.rotation)
            cos_a, sin_a = math.cos(angle_rad), math.sin(angle_rad)

            # Rotate circle center into box's local space
            local_x = local_circle_pos.x * cos_a + local_circle_pos.y * sin_a
            local_y = -local_circle_pos.x * sin_a + local_circle_pos.y * cos_a

            half_w, half_h = self.width / 2, self.height / 2

            # Find closest point on the AABB in local space
            closest_local = Vector2(
                max(-half_w, min(local_x, half_w)),
                max(-half_h, min(local_y, half_h))
            )

            dist_vec_local = Vector2(local_x, local_y) - closest_local
            if dist_vec_local.length_squared() > circle_collider.radius**2:
                return None

            # Transform closest point back to world space for contact point
            closest_world = box_center + Vector2(
                closest_local.x * cos_a - closest_local.y * sin_a,
                closest_local.x * sin_a + closest_local.y * cos_a
            )

            normal = (circle_center - closest_world).normalize()
            penetration = circle_collider.radius - (circle_center - closest_world).length()

            return CollisionInfo(circle_collider, closest_world, -normal, penetration)

class CircleCollider(Collider):
    """Circle-based collider (rotation doesn't affect circles)."""

    def __init__(self, game_object, radius=None, offset=Vector2(0, 0), **kwargs):
        super().__init__(game_object, **kwargs)

        if radius is None:
            radius = max(game_object.size.x, game_object.size.y) / 2 if game_object.size.x > 0 else 16
        self.radius = radius
        self.offset = offset
        self.center_x, self.center_y = 0, 0
        print(f"CircleCollider created: radius={radius}")

    def update_bounds(self):
        """Update the collision circle based on GameObject position."""
        self.center_x = self.game_object.position.x + self.offset.x
        self.center_y = self.game_object.position.y + self.offset.y
        self.bounds = Rect(
            self.center_x - self.radius, self.center_y - self.radius,
            self.radius * 2, self.radius * 2
        )

    def check_collision(self, other_collider):
        """Check collision with another collider."""
        result = super().check_collision(other_collider)
        if result is not None: return result

        if isinstance(other_collider, CircleCollider):
            return self._check_circle_vs_circle(other_collider)
        elif isinstance(other_collider, BoxCollider):
            collision = other_collider._check_box_vs_circle(self)
            if collision:
                return CollisionInfo(
                    other_collider,
                    collision.contact_point,
                    -collision.contact_normal,
                    collision.penetration_depth
                )
            return None
        return None

    def _check_circle_vs_circle(self, other):
        """Circle vs Circle collision detection."""
        center1 = Vector2(self.center_x, self.center_y)
        center2 = Vector2(other.center_x, other.center_y)
        distance_vec = center2 - center1
        distance = distance_vec.length()
        min_distance = self.radius + other.radius

        if distance <= min_distance:
            normal = distance_vec.normalize() if distance > 0 else Vector2(1, 0)
            penetration = min_distance - distance
            contact_point = center1 + normal * self.radius
            return CollisionInfo(other, contact_point, -normal, penetration)
        return None

