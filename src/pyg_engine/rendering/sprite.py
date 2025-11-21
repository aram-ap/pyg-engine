'''
Sprite Component - sprite rendering system

Features:
    - Image loading from file paths with convert() optimization
    - Scaling and offset support
    - Rotation with proper image handling
    - Color tinting and alpha blending
    - Flip horizontal/vertical
    - Layer-based rendering
    - Culling for performance
    
Example usage:
    # Create sprite-based game object
    player = GameObject(name="Player", position=Vector2(100, 100))
    sprite = player.add_component(Sprite, image_path="assets/player.png", scale=Vector2(2, 2))
    
    # Change sprite at runtime
    sprite.set_image("assets/player_walk.png")
    sprite.flip_x = True
    sprite.tint = Color(255, 100, 100)
'''

import pygame
from pygame import Vector2 as PygVector2
from ..components.component import Component
from ..utilities.vector2 import Vector2
from ..utilities.color import Color


class Sprite(Component):
    """
    Sprite component for rendering images on GameObjects.
    Automatically handles image loading, scaling, rotation, and optimization.
    """
    
    def __init__(self, game_object, image_path=None, scale=None, 
                 offset=None, flip_x=False, flip_y=False,
                 tint=None, alpha=255, layer=0):
        """
        Initialize a Sprite component.
        
        Args:
            game_object: The GameObject this sprite is attached to
            image_path: Path to the image file
            scale: Scale multiplier (Vector2, tuple, or single float for uniform scaling)
            offset: Position offset from GameObject position (Vector2 or tuple)
            flip_x: Flip the sprite horizontally
            flip_y: Flip the sprite vertically
            tint: Color tint to apply (Color or None)
            alpha: Alpha transparency (0-255)
            layer: Rendering layer (higher = drawn on top)
        """
        super().__init__(game_object)
        
        # Image properties
        self.image_path = image_path
        self._original_image = None  # Store original for transformations
        self._current_image = None   # Current transformed image
        
        # Transform properties
        self.scale = self._convert_to_vector2(scale) if scale else Vector2(1, 1)
        self.offset = self._convert_to_vector2(offset) if offset else Vector2(0, 0)
        self.flip_x = flip_x
        self.flip_y = flip_y
        
        # Visual properties
        self.tint = tint if tint else None
        self.alpha = alpha
        self.layer = layer
        
        # Cached rect for rendering
        self._rect = None
        
        # Load image if provided
        if image_path:
            self.set_image(image_path)
    
    def _convert_to_vector2(self, value):
        """Convert various types to Vector2."""
        if isinstance(value, Vector2):
            return value
        elif isinstance(value, (tuple, list)):
            return Vector2(value[0], value[1])
        elif isinstance(value, (int, float)):
            return Vector2(value, value)
        return Vector2(1, 1)
    
    def set_image(self, image_path):
        """
        Load and set a new image with proper optimization.
        Uses convert() or convert_alpha() for better performance.
        """
        try:
            # Load the image
            loaded_image = pygame.image.load(image_path)
            
            # Optimize based on alpha channel
            if loaded_image.get_alpha() is not None or loaded_image.get_flags() & pygame.SRCALPHA:
                self._original_image = loaded_image.convert_alpha()
            else:
                self._original_image = loaded_image.convert()
            
            self.image_path = image_path
            self._apply_transformations()
            
        except pygame.error as e:
            print(f"Error loading sprite image '{image_path}': {e}")
            # Create a placeholder surface
            self._original_image = pygame.Surface((32, 32), pygame.SRCALPHA)
            self._original_image.fill((255, 0, 255))  # Magenta placeholder
            self._apply_transformations()
    
    def set_image_from_surface(self, surface):
        """
        Set sprite image from a pygame Surface (useful for animations).
        
        Args:
            surface: pygame.Surface object
        """
        if surface:
            # Optimize the surface
            if surface.get_alpha() is not None or surface.get_flags() & pygame.SRCALPHA:
                self._original_image = surface.convert_alpha()
            else:
                self._original_image = surface.convert()
            
            self._apply_transformations()
    
    def _apply_transformations(self):
        """Apply all transformations (scale, flip, rotation, tint, alpha) to the image."""
        if not self._original_image:
            return
        
        # Start with original image
        image = self._original_image.copy()
        
        # Apply scaling
        if self.scale.x != 1 or self.scale.y != 1:
            original_size = image.get_size()
            new_size = (
                int(original_size[0] * self.scale.x),
                int(original_size[1] * self.scale.y)
            )
            if new_size[0] > 0 and new_size[1] > 0:
                image = pygame.transform.scale(image, new_size)
        
        # Apply flipping
        if self.flip_x or self.flip_y:
            image = pygame.transform.flip(image, self.flip_x, self.flip_y)
        
        # Apply rotation (from GameObject rotation)
        if abs(self.game_object.rotation) > 0.1:
            image = pygame.transform.rotate(image, self.game_object.rotation)
        
        # Apply tint
        if self.tint:
            # Create a colored surface and blend it
            tint_surface = pygame.Surface(image.get_size(), pygame.SRCALPHA)
            tint_surface.fill((self.tint.r, self.tint.g, self.tint.b, 128))
            image.blit(tint_surface, (0, 0), special_flags=pygame.BLEND_RGBA_MULT)
        
        # Apply alpha
        if self.alpha < 255:
            image.set_alpha(self.alpha)
        
        self._current_image = image
        
        # Update rect
        self._rect = self._current_image.get_rect()
    
    def set_scale(self, scale):
        """Change the sprite scale."""
        self.scale = self._convert_to_vector2(scale)
        self._apply_transformations()
    
    def set_flip(self, flip_x=None, flip_y=None):
        """Change sprite flipping."""
        if flip_x is not None:
            self.flip_x = flip_x
        if flip_y is not None:
            self.flip_y = flip_y
        self._apply_transformations()
    
    def set_tint(self, color):
        """Apply a color tint to the sprite."""
        self.tint = color
        self._apply_transformations()
    
    def set_alpha(self, alpha):
        """Set sprite transparency (0-255)."""
        self.alpha = max(0, min(255, alpha))
        self._apply_transformations()
    
    def get_image(self):
        """Get the current transformed image surface."""
        return self._current_image
    
    def get_rect(self):
        """Get the current sprite rect."""
        return self._rect
    
    def get_size(self):
        """Get the current sprite size as Vector2."""
        if self._current_image:
            size = self._current_image.get_size()
            return Vector2(size[0], size[1])
        return Vector2(0, 0)
    
    def update(self, engine):
        """Update sprite (called every frame)."""
        # Reapply transformations if rotation changed
        if hasattr(self.game_object, 'rotation') and self._original_image:
            # Check if rotation changed significantly
            if not hasattr(self, '_last_rotation'):
                self._last_rotation = self.game_object.rotation
                self._apply_transformations()
            elif abs(self._last_rotation - self.game_object.rotation) > 0.5:
                self._last_rotation = self.game_object.rotation
                self._apply_transformations()
    
    def render(self, screen, camera=None):
        """
        Render the sprite to the screen.
        
        Args:
            screen: pygame display surface
            camera: Optional camera for world-to-screen conversion
        """
        if not self.enabled or not self._current_image:
            return
        
        # Calculate render position
        world_pos = self.game_object.position + self.offset
        
        if camera:
            # Convert world position to screen position using camera
            screen_pos = camera.world_to_screen(world_pos)
        else:
            # No camera, use world position directly
            screen_pos = world_pos
        
        # Update rect position
        self._rect.center = (int(screen_pos.x), int(screen_pos.y))
        
        # Draw the sprite
        screen.blit(self._current_image, self._rect)
    
    def on_destroy(self):
        """Clean up resources when sprite is destroyed."""
        self._original_image = None
        self._current_image = None
        self._rect = None


# Alias for backward compatibility
SpriteRenderer = Sprite
