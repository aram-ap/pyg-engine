import pygame as pg
from pygame import Rect
import pymunk
from enum import Enum, auto
from ..components.component import Component
from ..utilities.vector2 import Vector2
from .material import PhysicsMaterial, Materials

class CollisionEvent(Enum):
    """Collision event types for consistent event handling."""
    ENTER = auto()      # Collision started
    STAY = auto()       # Collision continues
    EXIT = auto()       # Collision ended
    TRIGGER_ENTER = auto()  # Trigger collision started
    TRIGGER_STAY = auto()   # Trigger collision continues
    TRIGGER_EXIT = auto()   # Trigger collision ended
    OVERLAP_START = auto()  # Objects started overlapping
    OVERLAP_CONTINUE = auto()  # Objects continue overlapping
    OVERLAP_END = auto()   # Objects stopped overlapping
    PENETRATION = auto()  # Deep penetration detected
    SEPARATION = auto()   # Objects separated after penetration
    CONTACT_POINT = auto()  # New contact point detected
    CONTACT_LOST = auto()   # Contact point lost
    COLLISION_IGNORED = auto()  # Collision was ignored
    COLLISION_FILTERED = auto()  # Collision was filtered out

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
    """Base collider component using Pymunk for collision detection."""

    def __init__(self, game_object, is_trigger=False, material=None, collision_layer="Default"):
        super().__init__(game_object)

        # Collision settings
        self.is_trigger = is_trigger  # If True, detects collisions but doesn't stop movement
        self.material = material or Materials.DEFAULT
        self.collision_layer = collision_layer

        # Collision state tracking
        self._colliding_with = set()  # Objects currently colliding with
        self._overlapping_with = set()  # Objects currently overlapping (for triggers)
        self._contact_points = {}  # Track contact points per object
        self._collision_callbacks = {
            event_type: [] for event_type in CollisionEvent
        }

        # Pymunk shape will be set by the physics system
        self.shape = None
        
        # For pygame compatibility
        self.bounds = Rect(0, 0, 32, 32)  # Default bounds

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
        # will be done by the PhysicsSystem
        return None

    # ================ Enhanced Collision Event System ================

    def add_collision_callback(self, event_type, callback):
        """Add a callback for collision events using CollisionEvent enum."""
        if isinstance(event_type, str):
            # Legacy support for string-based event types
            event_mapping = {
                'enter': CollisionEvent.ENTER,
                'stay': CollisionEvent.STAY,
                'exit': CollisionEvent.EXIT,
                'trigger_enter': CollisionEvent.TRIGGER_ENTER,
                'trigger_stay': CollisionEvent.TRIGGER_STAY,
                'trigger_exit': CollisionEvent.TRIGGER_EXIT
            }
            event_type = event_mapping.get(event_type, CollisionEvent.ENTER)
        
        if event_type in self._collision_callbacks:
            self._collision_callbacks[event_type].append(callback)

    def remove_collision_callback(self, event_type, callback):
        """Remove a collision callback."""
        if isinstance(event_type, str):
            # Legacy support for string-based event types
            event_mapping = {
                'enter': CollisionEvent.ENTER,
                'stay': CollisionEvent.STAY,
                'exit': CollisionEvent.EXIT,
                'trigger_enter': CollisionEvent.TRIGGER_ENTER,
                'trigger_stay': CollisionEvent.TRIGGER_STAY,
                'trigger_exit': CollisionEvent.TRIGGER_EXIT
            }
            event_type = event_mapping.get(event_type, CollisionEvent.ENTER)
        
        if event_type in self._collision_callbacks and callback in self._collision_callbacks[event_type]:
            self._collision_callbacks[event_type].remove(callback)

    def _trigger_collision_event(self, event_type, collision_info):
        """Trigger collision callbacks for a specific event type."""
        for callback in self._collision_callbacks[event_type]:
            try:
                callback(collision_info)
            except Exception as e:
                print(f"Error in collision callback for {event_type.name}: {e}")

    def handle_collision(self, collision_info):
        """Handle a collision. Called by the physics system."""
        other_collider = collision_info.other_collider

        if self.is_trigger:
            # Handle trigger collisions
            if other_collider not in self._overlapping_with:
                self._overlapping_with.add(other_collider)
                self._trigger_collision_event(CollisionEvent.TRIGGER_ENTER, collision_info)
                self._trigger_collision_event(CollisionEvent.OVERLAP_START, collision_info)
            else:
                self._trigger_collision_event(CollisionEvent.TRIGGER_STAY, collision_info)
                self._trigger_collision_event(CollisionEvent.OVERLAP_CONTINUE, collision_info)
        else:
            # Handle physical collisions
            if other_collider not in self._colliding_with:
                self._colliding_with.add(other_collider)
                self._trigger_collision_event(CollisionEvent.ENTER, collision_info)
                
                # Check for deep penetration
                if collision_info.penetration_depth > 10.0:  # Threshold for deep penetration
                    self._trigger_collision_event(CollisionEvent.PENETRATION, collision_info)
            else:
                self._trigger_collision_event(CollisionEvent.STAY, collision_info)

        # Track contact points
        if other_collider not in self._contact_points:
            self._contact_points[other_collider] = []
        self._contact_points[other_collider].append(collision_info.contact_point)
        self._trigger_collision_event(CollisionEvent.CONTACT_POINT, collision_info)

    def end_collision(self, other_collider):
        """End a collision. Called by the physics system."""
        if self.is_trigger:
            if other_collider in self._overlapping_with:
                self._overlapping_with.remove(other_collider)
                # Create a basic collision info for exit event
                exit_info = CollisionInfo(other_collider, Vector2(0, 0), Vector2(0, 0), 0)
                self._trigger_collision_event(CollisionEvent.TRIGGER_EXIT, exit_info)
                self._trigger_collision_event(CollisionEvent.OVERLAP_END, exit_info)
        else:
            if other_collider in self._colliding_with:
                self._colliding_with.remove(other_collider)
                # Create a basic collision info for exit event
                exit_info = CollisionInfo(other_collider, Vector2(0, 0), Vector2(0, 0), 0)
                self._trigger_collision_event(CollisionEvent.EXIT, exit_info)
                self._trigger_collision_event(CollisionEvent.SEPARATION, exit_info)

        # Clear contact points
        if other_collider in self._contact_points:
            self._contact_points.pop(other_collider)
            self._trigger_collision_event(CollisionEvent.CONTACT_LOST, exit_info)

    def ignore_collision(self, other_collider, reason="filtered"):
        """Handle ignored collisions (for collision filtering)."""
        exit_info = CollisionInfo(other_collider, Vector2(0, 0), Vector2(0, 0), 0)
        exit_info.ignore_reason = reason
        self._trigger_collision_event(CollisionEvent.COLLISION_IGNORED, exit_info)

    def filter_collision(self, other_collider, filter_reason="layer_mismatch"):
        """Handle filtered collisions (for collision layer filtering)."""
        exit_info = CollisionInfo(other_collider, Vector2(0, 0), Vector2(0, 0), 0)
        exit_info.filter_reason = filter_reason
        self._trigger_collision_event(CollisionEvent.COLLISION_FILTERED, exit_info)

    def get_colliding_objects(self):
        """Get all objects currently colliding with this collider."""
        return list(self._colliding_with)

    def get_overlapping_objects(self):
        """Get all objects currently overlapping with this trigger collider."""
        return list(self._overlapping_with)

    def get_contact_points(self, other_collider=None):
        """Get contact points for a specific collider or all contact points."""
        if other_collider:
            return self._contact_points.get(other_collider, [])
        return self._contact_points

class BoxCollider(Collider):
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

        print(f"BoxCollider created: {width}x{height}")

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

class CircleCollider(Collider):
    """Circle-based collider using Pymunk for realistic physics."""

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
BoxCollider = BoxCollider
CircleCollider = CircleCollider
Collider = Collider 