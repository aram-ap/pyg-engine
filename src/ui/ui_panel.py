"""
UIPanel - Container component for grouping UI elements
"""

import pygame as pg
from .ui_element import UIElement
from ..utilities.vector2 import Vector2
from ..utilities.color import Color
from .anchors import Anchor

class UIPanel(UIElement):
    """Container panel for grouping UI elements."""
    
    def __init__(self, size=Vector2(300, 200), background_color=None,
                 border_color=None, border_width=0, padding=10,
                 sprite=None, sprite_scale_mode="stretch",
                 anchor=Anchor.CENTER, offset=Vector2(0, 0),
                 visible=True, enabled=True, layer=0):
        """
        Initialize a panel.
        
        Args:
            size: Panel size (width, height)
            background_color: Background fill color (None for transparent)
            border_color: Border color (None for no border)
            border_width: Width of border line
            padding: Internal padding for child elements
            sprite: Path to background sprite image
            sprite_scale_mode: How to scale sprite ("stretch", "tile", "center")
            anchor: Anchor point for positioning
            offset: Offset from anchor point
            visible: Whether the panel is visible
            enabled: Whether the panel is enabled
            layer: Rendering layer
        """
        super().__init__(anchor, offset, size, visible, enabled, layer)
        
        self.background_color = background_color or Color(50, 50, 50, 200)
        self.border_color = border_color or Color(150, 150, 150)
        self.border_width = border_width
        self.padding = padding
        
        # Sprite support
        self.sprite = sprite
        self.sprite_scale_mode = sprite_scale_mode
        self._sprite_surface = None
        self._load_sprite()
    
    def _load_sprite(self):
        """Load and cache sprite surface."""
        if self.sprite:
            try:
                loaded = pg.image.load(self.sprite)
                # Use convert_alpha() for sprites with transparency
                if loaded.get_alpha() is not None or loaded.get_flags() & pg.SRCALPHA:
                    self._sprite_surface = loaded.convert_alpha()
                else:
                    self._sprite_surface = loaded.convert()
                
                # Scale based on mode
                if self.sprite_scale_mode == "stretch":
                    self._sprite_surface = pg.transform.scale(
                        self._sprite_surface, (int(self.size.x), int(self.size.y))
                    )
                # For "tile" and "center" modes, we'll handle it in render
            except pg.error as e:
                print(f"Error loading panel sprite '{self.sprite}': {e}")
    
    def render(self, screen, screen_height):
        """Render the panel."""
        if not self.visible:
            return
        
        # Convert engine coords to pygame coords
        screen_x = self._screen_pos.x
        screen_y = screen_height - self._screen_pos.y
        
        # Create panel rect
        panel_rect = pg.Rect(0, 0, int(self.size.x), int(self.size.y))
        panel_rect.center = (int(screen_x), int(screen_y))
        
        # Draw sprite or color background
        if self._sprite_surface:
            if self.sprite_scale_mode == "stretch":
                # Already scaled in _load_sprite
                screen.blit(self._sprite_surface, panel_rect)
            elif self.sprite_scale_mode == "center":
                # Center the sprite without scaling
                sprite_rect = self._sprite_surface.get_rect(center=panel_rect.center)
                screen.blit(self._sprite_surface, sprite_rect)
            elif self.sprite_scale_mode == "tile":
                # Tile the sprite across the panel
                sprite_width = self._sprite_surface.get_width()
                sprite_height = self._sprite_surface.get_height()
                for y in range(panel_rect.top, panel_rect.bottom, sprite_height):
                    for x in range(panel_rect.left, panel_rect.right, sprite_width):
                        screen.blit(self._sprite_surface, (x, y))
        elif self.background_color:
            # Draw color background
            if self.background_color.a < 255:
                panel_surface = pg.Surface((int(self.size.x), int(self.size.y)), pg.SRCALPHA)
                panel_surface.fill(self.background_color)
                screen.blit(panel_surface, panel_rect)
            else:
                pg.draw.rect(screen, self.background_color, panel_rect)
        
        # Draw border if specified
        if self.border_width > 0 and self.border_color:
            pg.draw.rect(screen, self.border_color, panel_rect, self.border_width)

