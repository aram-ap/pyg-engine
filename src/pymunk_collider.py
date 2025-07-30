import pygame as pg
from pygame import Vector2, Rect
import pymunk
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

class PymunkCollider(Component):
    """Base collider component using Pymunk for collision detection."""

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

        # Pymunk shape will be set by the physics system
        self.shape = None
        
        # For pygame compatibility
        self.bounds = Rect(0, 0, 32, 32)  # Default bounds

        print(f"PymunkCollider created on {game_object.name} (trigger={is_trigger})")

    def start(self):
        """Initialize the collider."""
        self.update_bounds()
        print(f"PymunkCollider started on {self.game_object.name}")

    def update(self, engine):
        """Update collider bounds to match GameObject position."""
        if self.enabled:
            self.update_bounds()

    def update_bounds(self):
        """Update collision bounds based on GameObject position."""
        if self.shape:
            # Get bounding box from pymunk shape
            bb = self.shape.bb
            self.bounds = Rect(bb.left, bb.top, bb.right - bb.left, bb.bottom - bb.top)
        else:
            # Fallback to GameObject position
            pos = self.game_object.position
            size = self.game_object.size
            width = max(size.x, 32)
            height = max(size.y, 32)
            self.bounds = Rect(pos.x - width//2, pos.y - height//2, width, height)

    def check_collision(self, other_collider):
        """Check if this collider is colliding with another. Returns CollisionInfo or None."""
        if not (self.enabled and other_collider.enabled):
            return None
        if other_collider == self:
            return None

        # In Pymunk, collision detection is handled by the space
        # This method is kept for API compatibility but actual detection
        # will be done by the PymunkPhysicsSystem
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

class PymunkBoxCollider(PymunkCollider):
    """Rectangle-based collider using Pymunk for realistic physics."""

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
        self.bounds = Rect(0, 0, width, height)  # Initial bounds

        print(f"PymunkBoxCollider created: {width}x{height}")

    def update_bounds(self):
        """Update the collision rectangle based on GameObject position."""
        center_x = self.game_object.position.x + self.offset.x
        center_y = self.game_object.position.y + self.offset.y

        if self.shape:
            # Get accurate bounds from pymunk shape
            bb = self.shape.bb
            self.bounds = Rect(bb.left, bb.top, bb.right - bb.left, bb.bottom - bb.top)
        else:
            # Fallback to simple bounds
            self.bounds.centerx = int(center_x)
            self.bounds.centery = int(center_y)

    def get_world_corners(self):
        """Returns the four world-space corners of the box."""
        if self.shape and hasattr(self.shape, 'get_vertices'):
            # Get vertices from pymunk shape
            vertices = []
            for v in self.shape.get_vertices():
                # Transform local vertices to world coordinates
                world_v = v.rotated(self.shape.body.angle) + self.shape.body.position
                vertices.append(Vector2(world_v.x, world_v.y))
            return vertices
        else:
            # Fallback to calculating corners manually
            center = Vector2(self.bounds.centerx, self.bounds.centery)
            half_w = self.width / 2
            half_h = self.height / 2
            return [
                Vector2(center.x - half_w, center.y - half_h),
                Vector2(center.x + half_w, center.y - half_h),
                Vector2(center.x + half_w, center.y + half_h),
                Vector2(center.x - half_w, center.y + half_h),
            ]

class PymunkCircleCollider(PymunkCollider):
    """Circle-based collider using Pymunk for realistic physics."""

    def __init__(self, game_object, radius=None, offset=Vector2(0, 0), **kwargs):
        super().__init__(game_object, **kwargs)

        if radius is None:
            radius = max(game_object.size.x, game_object.size.y) / 2 if game_object.size.x > 0 else 16
        self.radius = radius
        self.offset = offset
        self.center_x, self.center_y = 0, 0
        print(f"PymunkCircleCollider created: radius={radius}")

    def update_bounds(self):
        """Update the collision circle based on GameObject position."""
        self.center_x = self.game_object.position.x + self.offset.x
        self.center_y = self.game_object.position.y + self.offset.y
        
        if self.shape:
            # Get accurate bounds from pymunk shape
            bb = self.shape.bb
            self.bounds = Rect(bb.left, bb.top, bb.right - bb.left, bb.bottom - bb.top)
        else:
            # Fallback bounds calculation
            self.bounds = Rect(
                self.center_x - self.radius, self.center_y - self.radius,
                self.radius * 2, self.radius * 2
            )

# Legacy aliases for backward compatibility
BoxCollider = PymunkBoxCollider
CircleCollider = PymunkCircleCollider
Collider = PymunkCollider 