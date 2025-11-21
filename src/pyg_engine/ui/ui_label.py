"""
UILabel - Text display component
"""

import pygame as pg
from .ui_element import UIElement
from ..utilities.vector2 import Vector2
from ..utilities.color import Color
from .anchors import Anchor

class UILabel(UIElement):
    """Text display UI element."""
    
    def __init__(self, text="", font_size=24, font_name=None, color=Color(255, 255, 255),
                 anchor=Anchor.CENTER, offset=Vector2(0, 0), bold=False, italic=False,
                 align="center", background_sprite=None, visible=True, enabled=True, layer=0):
        """
        Initialize a text label.
        
        Args:
            text: Text to display
            font_size: Size of the font
            font_name: Font family name (None for default)
            color: Text color
            anchor: Anchor point for positioning
            offset: Offset from anchor point
            bold: Whether text is bold
            italic: Whether text is italic
            align: Text alignment (left, center, right)
            background_sprite: Optional background sprite image path
            visible: Whether the element is visible
            enabled: Whether the element is enabled
            layer: Rendering layer
        """
        # Calculate size from text
        temp_font = pg.font.Font(font_name, font_size)
        text_surface = temp_font.render(text, True, (255, 255, 255))
        size = Vector2(text_surface.get_width(), text_surface.get_height())
        
        super().__init__(anchor, offset, size, visible, enabled, layer)
        
        self.text = text
        self.font_size = font_size
        self.font_name = font_name
        self.color = color
        self.bold = bold
        self.italic = italic
        self.align = align
        
        # Sprite support
        self.background_sprite = background_sprite
        self._sprite_surface = None
        self._load_sprite()
        
        # Cache font and rendered surface
        self._font = None
        self._text_surface = None
        self._rebuild_text()
    
    def _rebuild_text(self):
        """Rebuild the text surface when text or style changes."""
        self._font = pg.font.Font(self.font_name, self.font_size)
        if self.bold:
            self._font.set_bold(True)
        if self.italic:
            self._font.set_italic(True)
        
        self._text_surface = self._font.render(self.text, True, self.color)
        
        # Update size based on text
        self.size.x = self._text_surface.get_width()
        self.size.y = self._text_surface.get_height()
    
    def set_text(self, text):
        """Change the text content."""
        if self.text != text:
            self.text = text
            self._rebuild_text()
    
    def set_color(self, color):
        """Change the text color."""
        if self.color != color:
            self.color = color
            self._rebuild_text()
    
    def set_font_size(self, size):
        """Change the font size."""
        if self.font_size != size:
            self.font_size = size
            self._rebuild_text()
    
    def _load_sprite(self):
        """Load and cache background sprite."""
        if self.background_sprite:
            try:
                loaded = pg.image.load(self.background_sprite)
                # Use convert_alpha() for sprites with transparency
                if loaded.get_alpha() is not None or loaded.get_flags() & pg.SRCALPHA:
                    self._sprite_surface = loaded.convert_alpha()
                else:
                    self._sprite_surface = loaded.convert()
            except pg.error as e:
                print(f"Error loading label background sprite '{self.background_sprite}': {e}")
    
    def render(self, screen, screen_height):
        """Render the text label."""
        if not self.visible or not self._text_surface:
            return
        
        # Convert engine coords to pygame coords
        screen_x = self._screen_pos.x
        screen_y = screen_height - self._screen_pos.y
        
        # Adjust for alignment
        text_rect = self._text_surface.get_rect()
        if self.align == "center":
            text_rect.center = (int(screen_x), int(screen_y))
        elif self.align == "left":
            text_rect.midleft = (int(screen_x - self.size.x/2), int(screen_y))
        elif self.align == "right":
            text_rect.midright = (int(screen_x + self.size.x/2), int(screen_y))
        
        # Draw background sprite if set
        if self._sprite_surface:
            # Scale sprite to fit text size with some padding
            padding = 10
            sprite_scaled = pg.transform.scale(
                self._sprite_surface, 
                (int(self.size.x + padding * 2), int(self.size.y + padding * 2))
            )
            sprite_rect = sprite_scaled.get_rect(center=text_rect.center)
            screen.blit(sprite_scaled, sprite_rect)
        
        # Draw text
        screen.blit(self._text_surface, text_rect)

