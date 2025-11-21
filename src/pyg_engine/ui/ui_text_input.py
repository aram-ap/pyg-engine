"""
UITextInput - Editable text input field component
"""

import pygame as pg
from .ui_element import UIElement
from ..utilities.vector2 import Vector2
from ..utilities.color import Color
from .anchors import Anchor

class UITextInput(UIElement):
    """Editable text input UI element."""
    
    def __init__(self, placeholder="Enter text...", text="", max_length=20,
                 size=Vector2(300, 50), font_size=24,
                 anchor=Anchor.CENTER, offset=Vector2(0, 0),
                 onSubmit=None, onChange=None,
                 background_color=None, active_color=None,
                 text_color=Color(255, 255, 255), placeholder_color=None,
                 border_width=2, border_color=None, active_border_color=None,
                 cursor_blink_speed=0.5,
                 visible=True, enabled=True, layer=0):
        """
        Initialize a text input field.
        
        Args:
            placeholder: Placeholder text when empty
            text: Initial text value
            max_length: Maximum number of characters
            size: Input field size (width, height)
            font_size: Font size for text
            anchor: Anchor point for positioning
            offset: Offset from anchor point
            onSubmit: Callback when Enter is pressed (receives text as argument)
            onChange: Callback when text changes (receives text as argument)
            background_color: Background color when inactive
            active_color: Background color when active/focused
            text_color: Color of input text
            placeholder_color: Color of placeholder text
            border_width: Width of border
            border_color: Color of border when inactive
            active_border_color: Color of border when active
            cursor_blink_speed: Speed of cursor blinking in seconds
            visible: Whether the field is visible
            enabled: Whether the field is enabled
            layer: Rendering layer
        """
        super().__init__(anchor, offset, size, visible, enabled, layer)
        
        self.text = text
        self.placeholder = placeholder
        self.max_length = max_length
        self.font_size = font_size
        
        # Callbacks
        self.onSubmit = onSubmit
        self.onChange = onChange
        
        # Colors
        self.background_color = background_color or Color(40, 40, 40)
        self.active_color = active_color or Color(60, 60, 60)
        self.text_color = text_color
        self.placeholder_color = placeholder_color or Color(150, 150, 150)
        self.border_color = border_color or Color(100, 100, 100)
        self.active_border_color = active_border_color or Color(200, 200, 255)
        self.border_width = border_width
        
        # State
        self.is_focused = False
        self.cursor_visible = True
        self.cursor_timer = 0
        self.cursor_blink_speed = cursor_blink_speed
        self.cursor_position = len(text)  # Cursor at end of text
        
        # Font
        self._font = pg.font.Font(None, font_size)
        
        # Rendered surfaces cache
        self._text_surface = None
        self._placeholder_surface = None
        self._rebuild_surfaces()
    
    def _rebuild_surfaces(self):
        """Rebuild text surfaces when text changes."""
        if self.text:
            self._text_surface = self._font.render(self.text, True, self.text_color)
        else:
            self._text_surface = None
        
        self._placeholder_surface = self._font.render(self.placeholder, True, self.placeholder_color)
    
    def set_text(self, text):
        """Set the text value."""
        if len(text) <= self.max_length:
            old_text = self.text
            self.text = text
            self.cursor_position = min(self.cursor_position, len(text))
            self._rebuild_surfaces()
            
            if old_text != text and self.onChange:
                self.onChange(self.text)
    
    def get_text(self):
        """Get the current text value."""
        return self.text
    
    def clear(self):
        """Clear the text input."""
        self.set_text("")
        self.cursor_position = 0
    
    def focus(self):
        """Give focus to this text input."""
        self.is_focused = True
        self.cursor_visible = True
        self.cursor_timer = 0
    
    def unfocus(self):
        """Remove focus from this text input."""
        self.is_focused = False
        self.cursor_visible = False
    
    def update(self, engine):
        """Update text input state."""
        if not self.enabled or not self.visible:
            return
        
        # Update cursor blinking
        if self.is_focused:
            self.cursor_timer += engine.dt()
            if self.cursor_timer >= self.cursor_blink_speed:
                self.cursor_visible = not self.cursor_visible
                self.cursor_timer = 0
        
        # Check for clicks to focus/unfocus
        if engine.input.get_event_state('mouse_button_down', 0):
            mouse_pos = engine.input.mouse.get_pos()
            if self.contains_point(mouse_pos[0], mouse_pos[1]):
                if not self.is_focused:
                    self.focus()
            else:
                if self.is_focused:
                    self.unfocus()
    
    def handle_event(self, event, engine):
        """Handle keyboard and text input events."""
        if not self.enabled or not self.is_focused:
            return
        
        # Handle text input events (from pg.TEXTINPUT)
        if event.type == pg.TEXTINPUT:
            # Add character at cursor position
            if len(self.text) < self.max_length:
                self.text = self.text[:self.cursor_position] + event.text + self.text[self.cursor_position:]
                self.cursor_position += len(event.text)
                self._rebuild_surfaces()
                if self.onChange:
                    self.onChange(self.text)
                return
        
        if event.type == pg.KEYDOWN:
            if event.key == pg.K_RETURN or event.key == pg.K_KP_ENTER:
                # Submit
                if self.onSubmit:
                    self.onSubmit(self.text)
                self.unfocus()
            
            elif event.key == pg.K_BACKSPACE:
                # Delete character before cursor
                if self.cursor_position > 0:
                    self.text = self.text[:self.cursor_position-1] + self.text[self.cursor_position:]
                    self.cursor_position -= 1
                    self._rebuild_surfaces()
                    if self.onChange:
                        self.onChange(self.text)
            
            elif event.key == pg.K_DELETE:
                # Delete character after cursor
                if self.cursor_position < len(self.text):
                    self.text = self.text[:self.cursor_position] + self.text[self.cursor_position+1:]
                    self._rebuild_surfaces()
                    if self.onChange:
                        self.onChange(self.text)
            
            elif event.key == pg.K_LEFT:
                # Move cursor left
                self.cursor_position = max(0, self.cursor_position - 1)
                self.cursor_visible = True
                self.cursor_timer = 0
            
            elif event.key == pg.K_RIGHT:
                # Move cursor right
                self.cursor_position = min(len(self.text), self.cursor_position + 1)
                self.cursor_visible = True
                self.cursor_timer = 0
            
            elif event.key == pg.K_HOME:
                # Move cursor to start
                self.cursor_position = 0
                self.cursor_visible = True
                self.cursor_timer = 0
            
            elif event.key == pg.K_END:
                # Move cursor to end
                self.cursor_position = len(self.text)
                self.cursor_visible = True
                self.cursor_timer = 0
            
            elif event.key == pg.K_ESCAPE:
                # Unfocus
                self.unfocus()
    
    def render(self, screen, screen_height):
        """Render the text input field."""
        if not self.visible:
            return
        
        # Convert engine coords to pygame coords
        screen_x = self._screen_pos.x
        screen_y = screen_height - self._screen_pos.y
        
        # Create input rect
        input_rect = pg.Rect(0, 0, int(self.size.x), int(self.size.y))
        input_rect.center = (int(screen_x), int(screen_y))
        
        # Draw background
        bg_color = self.active_color if self.is_focused else self.background_color
        pg.draw.rect(screen, bg_color, input_rect)
        
        # Draw border
        border_color = self.active_border_color if self.is_focused else self.border_color
        if self.border_width > 0:
            pg.draw.rect(screen, border_color, input_rect, self.border_width)
        
        # Draw text or placeholder
        text_padding = 10
        if self.text:
            # Draw actual text
            if self._text_surface:
                text_rect = self._text_surface.get_rect()
                text_rect.midleft = (input_rect.left + text_padding, input_rect.centery)
                
                # Clip text if it's too long
                clip_rect = input_rect.copy()
                clip_rect.left += text_padding
                clip_rect.width -= text_padding * 2
                screen.set_clip(clip_rect)
                screen.blit(self._text_surface, text_rect)
                screen.set_clip(None)
        else:
            # Draw placeholder
            if self._placeholder_surface:
                placeholder_rect = self._placeholder_surface.get_rect()
                placeholder_rect.midleft = (input_rect.left + text_padding, input_rect.centery)
                screen.blit(self._placeholder_surface, placeholder_rect)
        
        # Draw cursor if focused
        if self.is_focused and self.cursor_visible:
            # Calculate cursor x position
            cursor_text = self.text[:self.cursor_position]
            if cursor_text:
                cursor_surface = self._font.render(cursor_text, True, self.text_color)
                cursor_x = input_rect.left + text_padding + cursor_surface.get_width()
            else:
                cursor_x = input_rect.left + text_padding
            
            # Draw cursor line
            cursor_height = self.font_size
            cursor_y_top = input_rect.centery - cursor_height // 2
            cursor_y_bottom = input_rect.centery + cursor_height // 2
            pg.draw.line(screen, self.text_color, 
                        (cursor_x, cursor_y_top), 
                        (cursor_x, cursor_y_bottom), 2)

