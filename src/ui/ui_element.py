"""
Base UI Element class for all UI components
"""

import pygame as pg
from ..utilities.vector2 import Vector2
from ..utilities.color import Color
from .anchors import Anchor, get_anchor_position

class UIElement:
    """Base class for all UI elements."""
    
    def __init__(self, anchor=Anchor.CENTER, offset=Vector2(0, 0), 
                 size=Vector2(100, 50), visible=True, enabled=True, layer=0):
        """
        Initialize a UI element.
        
        Args:
            anchor: Anchor point for positioning
            offset: Offset from anchor point
            size: Size of the element (width, height)
            visible: Whether the element is visible
            enabled: Whether the element is enabled
            layer: Rendering layer (higher = on top)
        """
        self.anchor = anchor
        self.offset = offset
        self.size = size
        self.visible = visible
        self.enabled = enabled
        self.layer = layer
        
        # Calculated screen position
        self._screen_pos = Vector2(0, 0)
        self._rect = pg.Rect(0, 0, int(size.x), int(size.y))
        
        # Parent-child system
        self.parent = None
        self.children = []
        
    def update_position(self, screen_width, screen_height):
        """Update screen position based on anchor and offset."""
        anchor_x, anchor_y = get_anchor_position(self.anchor, screen_width, screen_height)
        self._screen_pos.x = anchor_x + self.offset.x
        self._screen_pos.y = anchor_y + self.offset.y
        
        # Update rect for collision detection
        self._rect.center = (int(self._screen_pos.x), int(screen_height - self._screen_pos.y))
        
    def get_rect(self):
        """Get the bounding rectangle for this element."""
        return self._rect
    
    def contains_point(self, x, y):
        """Check if a point is inside this element."""
        return self._rect.collidepoint(x, y)
    
    def add_child(self, child):
        """Add a child element."""
        if child not in self.children:
            child.parent = self
            self.children.append(child)
    
    def remove_child(self, child):
        """Remove a child element."""
        if child in self.children:
            child.parent = None
            self.children.remove(child)
    
    def update(self, engine):
        """Update logic - override in subclasses."""
        pass
    
    def render(self, screen, screen_height):
        """Render the element - override in subclasses."""
        pass
    
    def handle_event(self, event, engine):
        """Handle input events - override in subclasses."""
        pass
