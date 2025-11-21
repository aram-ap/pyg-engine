"""
UICanvas - Manager for all UI elements
"""

import pygame as pg
from ..utilities.vector2 import Vector2

class UICanvas:
    """Canvas manager for UI rendering and input handling."""
    
    def __init__(self, engine):
        """
        Initialize the UI canvas.
        
        Args:
            engine: Reference to the game engine
        """
        self.engine = engine
        self.elements = []  # List of UI elements
        self.input_enabled = True
        self.name = getattr(engine, 'name', None)
        
        # Register with engine debug system if available
        if hasattr(engine, 'register_ui_canvas'):
            engine.register_ui_canvas(self)
        
    def add_element(self, element):
        """
        Add a UI element to the canvas.
        
        Args:
            element: UIElement to add
        """
        if element not in self.elements:
            self.elements.append(element)
            # Sort by layer (higher layer = rendered on top)
            self.elements.sort(key=lambda e: e.layer)
    
    def remove_element(self, element):
        """
        Remove a UI element from the canvas.
        
        Args:
            element: UIElement to remove
        """
        if element in self.elements:
            self.elements.remove(element)
    
    def clear(self):
        """Remove all UI elements."""
        self.elements.clear()

    def __del__(self):
        """Ensure canvas is unregistered."""
        if hasattr(self.engine, 'unregister_ui_canvas'):
            self.engine.unregister_ui_canvas(self)
    
    def get_elements_by_layer(self, layer):
        """Get all elements on a specific layer."""
        return [e for e in self.elements if e.layer == layer]
    
    def update(self):
        """Update all UI elements."""
        # Get screen size for position updates
        screen_size = self.engine.getWindowSize()
        screen_width = screen_size.w
        screen_height = screen_size.h
        
        # Update positions and logic for all elements
        for element in self.elements:
            if element.enabled:
                element.update_position(screen_width, screen_height)
                element.update(self.engine)
                
                # Update children
                for child in element.children:
                    if child.enabled:
                        child.update_position(screen_width, screen_height)
                        child.update(self.engine)
    
    def render(self, screen):
        """
        Render all UI elements to the screen.
        
        Args:
            screen: Pygame surface to render to
        """
        screen_height = self.engine.getWindowSize().h
        
        # Render elements in layer order (already sorted)
        for element in self.elements:
            if element.visible:
                element.render(screen, screen_height)
                
                # Render children
                for child in element.children:
                    if child.visible:
                        child.render(screen, screen_height)
    
    def handle_event(self, event):
        """
        Handle input events and route to UI elements.
        
        Args:
            event: Pygame event
        
        Returns:
            bool: True if event was handled by a UI element
        """
        if not self.input_enabled:
            return False
        
        # Handle events in reverse layer order (top to bottom)
        for element in reversed(self.elements):
            if element.enabled and element.visible:
                element.handle_event(event, self.engine)
                
                # Handle children
                for child in element.children:
                    if child.enabled and child.visible:
                        child.handle_event(event, self.engine)
                
                # Check if event was on this element (for blocking)
                if event.type in [pg.MOUSEBUTTONDOWN, pg.MOUSEBUTTONUP]:
                    mouse_pos = pg.mouse.get_pos()
                    if element.contains_point(mouse_pos[0], mouse_pos[1]):
                        return True  # Event handled, block propagation
        
        return False
    
    def find_element_at_position(self, x, y):
        """
        Find the topmost UI element at a screen position.
        
        Args:
            x: Screen X coordinate
            y: Screen Y coordinate
        
        Returns:
            UIElement or None
        """
        for element in reversed(self.elements):
            if element.visible and element.contains_point(x, y):
                return element
        return None
    
    def set_input_enabled(self, enabled):
        """Enable or disable input handling for all UI elements."""
        self.input_enabled = enabled
    
    def get_element_count(self):
        """Get the total number of elements in the canvas."""
        return len(self.elements)

