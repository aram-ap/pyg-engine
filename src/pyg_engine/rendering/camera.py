import pygame as pg
from ..utilities.vector2 import Vector2

class Camera:
    """Camera system for viewport management and target following."""

    def __init__(self, width, height):
        self.width = width
        self.height = height
        self.position = Vector2(0, 0)  # Camera center in world space
        self.target = None  # GameObject to follow
        self.zoom = 1.0

        # Camera settings
        self.follow_speed = 0.1  # How quickly camera follows target (0-1)
        self.offset = Vector2(0, 0)  # Offset from target
        self.bounds = None  # Optional world bounds Rect

        # Scaling mode for window resize
        self.scale_mode = "fit"  # "fit", "fill", "stretch", "fixed"
        self.base_width = width
        self.base_height = height

    def update(self, dt):
        """Update camera position to follow target."""
        if self.target and self.target.enabled:
            target_pos = self.target.position + self.offset

            # Smooth follow
            if self.follow_speed >= 1.0:
                self.position = target_pos
            else:
                diff = target_pos - self.position
                self.position += diff * self.follow_speed

        # Keep camera within bounds if set
        if self.bounds:
            half_w = self.width / (2 * self.zoom)
            half_h = self.height / (2 * self.zoom)

            self.position.x = max(self.bounds.left + half_w,
                                 min(self.position.x, self.bounds.right - half_w))
            self.position.y = max(self.bounds.top + half_h,
                                 min(self.position.y, self.bounds.bottom - half_h))

    def follow(self, game_object, offset=Vector2(0, 0)):
        """Set a GameObject for the camera to follow."""
        self.target = game_object
        self.offset = offset

    def set_position(self, x, y):
        """Manually set camera position."""
        self.position = Vector2(x, y)
        self.target = None  # Stop following

    def resize(self, new_width, new_height):
        """Handle window resize and adjust zoom accordingly."""
        self.width = new_width
        self.height = new_height

        # Adjust zoom based on scale mode
        if self.scale_mode == "fit":
            # Scale to fit content in view
            scale_x = new_width / self.base_width
            scale_y = new_height / self.base_height
            self.zoom = min(scale_x, scale_y)
        elif self.scale_mode == "fill":
            # Scale to fill entire view
            scale_x = new_width / self.base_width
            scale_y = new_height / self.base_height
            self.zoom = max(scale_x, scale_y)
        elif self.scale_mode == "stretch":
            # Don't maintain aspect ratio
            self.zoom = 1.0  # Handled in transform
        # "fixed" mode keeps zoom unchanged

    def world_to_screen(self, world_pos):
        """Convert world coordinates to screen coordinates."""
        if isinstance(world_pos, (list, tuple)):
            world_pos = Vector2(world_pos[0], world_pos[1])

        # Check for NaN values
        if (isinstance(world_pos.x, float) and (world_pos.x != world_pos.x or world_pos.x == float('inf') or world_pos.x == float('-inf'))) or \
           (isinstance(world_pos.y, float) and (world_pos.y != world_pos.y or world_pos.y == float('inf') or world_pos.y == float('-inf'))):
            return Vector2(0, 0)

        # Apply camera transformation (flip Y for Pygame coordinate system)
        screen_x = (world_pos.x - self.position.x) * self.zoom + self.width / 2
        screen_y = (self.position.y - world_pos.y) * self.zoom + self.height / 2  # Flip Y axis

        return Vector2(screen_x, screen_y)

    def screen_to_world(self, screen_pos):
        """Convert screen coordinates to world coordinates."""
        if isinstance(screen_pos, (list, tuple)):
            screen_pos = Vector2(screen_pos[0], screen_pos[1])

        # Check for NaN values
        if (isinstance(screen_pos.x, float) and (screen_pos.x != screen_pos.x or screen_pos.x == float('inf') or screen_pos.x == float('-inf'))) or \
           (isinstance(screen_pos.y, float) and (screen_pos.y != screen_pos.y or screen_pos.y == float('inf') or screen_pos.y == float('-inf'))):
            return Vector2(0, 0)

        world_x = (screen_pos.x - self.width / 2) / self.zoom + self.position.x
        world_y = self.position.y - (screen_pos.y - self.height / 2) / self.zoom  # Flip Y axis

        return Vector2(world_x, world_y)

    def get_visible_rect(self):
        """Get the world-space rectangle that's currently visible."""
        half_w = self.width / (2 * self.zoom)
        half_h = self.height / (2 * self.zoom)

        return pg.Rect(
            self.position.x - half_w,
            self.position.y - half_h,
            half_w * 2,
            half_h * 2
        )

