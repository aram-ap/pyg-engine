"""
UIButton - Interactive button component
"""

import pygame as pg
from enum import Enum, auto
from .ui_element import UIElement
from ..utilities.vector2 import Vector2
from ..utilities.color import Color
from .anchors import Anchor

class ButtonState(Enum):
    """Button state enumeration."""
    NORMAL = auto()
    HOVER = auto()
    PRESSED = auto()
    DISABLED = auto()

class UIButton(UIElement):
    """Interactive button UI element."""
    
    def __init__(self, text="Button", size=Vector2(150, 50), font_size=24,
                 anchor=Anchor.CENTER, offset=Vector2(0, 0),
                 onClick=None, onHover=None, onRelease=None,
                 normal_color=None, hover_color=None, pressed_color=None, disabled_color=None,
                 text_color=Color(255, 255, 255), border_width=2, border_color=None,
                 sprite=None, normal_sprite=None, hover_sprite=None, pressed_sprite=None, disabled_sprite=None,
                 visible=True, enabled=True, layer=0):
        """
        Initialize a button.
        
        Args:
            text: Button text
            size: Button size (width, height)
            font_size: Font size for text
            anchor: Anchor point for positioning
            offset: Offset from anchor point
            onClick: Callback when button is clicked
            onHover: Callback when mouse enters button
            onRelease: Callback when button is released
            normal_color: Background color in normal state
            hover_color: Background color when hovering
            pressed_color: Background color when pressed
            disabled_color: Background color when disabled
            text_color: Color of button text
            border_width: Width of button border (0 for no border)
            border_color: Color of button border
            sprite: Sprite path for all states (if state-specific sprites not provided)
            normal_sprite: Sprite path for normal state
            hover_sprite: Sprite path for hover state
            pressed_sprite: Sprite path for pressed state
            disabled_sprite: Sprite path for disabled state
            visible: Whether the button is visible
            enabled: Whether the button is enabled
            layer: Rendering layer
        """
        super().__init__(anchor, offset, size, visible, enabled, layer)
        
        self.text = text
        self.font_size = font_size
        self.text_color = text_color
        self.border_width = border_width
        self.border_color = border_color or Color(200, 200, 200)
        
        # Callbacks
        self.onClick = onClick
        self.onHover = onHover
        self.onRelease = onRelease
        
        # State colors
        self.normal_color = normal_color or Color(70, 70, 70)
        self.hover_color = hover_color or Color(100, 100, 100)
        self.pressed_color = pressed_color or Color(50, 50, 50)
        self.disabled_color = disabled_color or Color(40, 40, 40)
        
        # Current state
        self.state = ButtonState.NORMAL if enabled else ButtonState.DISABLED
        self._was_hovering = False
        self._was_pressed_last_frame = False
        
        # Font and text surface
        self._font = pg.font.Font(None, font_size)
        self._text_surface = self._font.render(text, True, text_color)
        
        # Sprite support
        self.sprite = sprite
        self.normal_sprite = normal_sprite or sprite
        self.hover_sprite = hover_sprite or sprite
        self.pressed_sprite = pressed_sprite or sprite
        self.disabled_sprite = disabled_sprite or sprite
        
        # Cached sprite surfaces
        self._sprite_surfaces = {}
        self._load_sprites()
    
    def _load_sprites(self):
        """Load and cache sprite surfaces."""
        sprite_paths = {
            ButtonState.NORMAL: self.normal_sprite,
            ButtonState.HOVER: self.hover_sprite,
            ButtonState.PRESSED: self.pressed_sprite,
            ButtonState.DISABLED: self.disabled_sprite
        }
        
        for state, path in sprite_paths.items():
            if path:
                try:
                    loaded = pg.image.load(path)
                    # Use convert_alpha() for sprites with transparency
                    if loaded.get_alpha() is not None or loaded.get_flags() & pg.SRCALPHA:
                        sprite_surface = loaded.convert_alpha()
                    else:
                        sprite_surface = loaded.convert()
                    
                    # Scale to button size
                    sprite_surface = pg.transform.scale(sprite_surface, (int(self.size.x), int(self.size.y)))
                    self._sprite_surfaces[state] = sprite_surface
                except pg.error as e:
                    print(f"Error loading button sprite '{path}': {e}")
    
    def _get_current_color(self):
        """Get the current background color based on state."""
        colors = {
            ButtonState.NORMAL: self.normal_color,
            ButtonState.HOVER: self.hover_color,
            ButtonState.PRESSED: self.pressed_color,
            ButtonState.DISABLED: self.disabled_color
        }
        return colors.get(self.state, self.normal_color)
    
    def _get_current_sprite(self):
        """Get the current sprite surface based on state."""
        return self._sprite_surfaces.get(self.state, None)
    
    def update(self, engine):
        """Update button state based on mouse interaction."""
        if not self.enabled or not self.visible:
            self.state = ButtonState.DISABLED if not self.enabled else ButtonState.NORMAL
            return
        
        # Get mouse position
        mouse_pos = engine.input.mouse.get_pos()
        is_hovering = self.contains_point(mouse_pos[0], mouse_pos[1])
        
        # Check for hover enter event
        if is_hovering and not self._was_hovering:
            if self.onHover:
                self.onHover()
        
        self._was_hovering = is_hovering
        
        # Update state based on mouse - handle click detection in update loop
        mouse_pressed = engine.input.mouse.get_button(0)
        mouse_just_released = engine.input.get_event_state('mouse_button_up', 0)
        
        if not is_hovering:
            self.state = ButtonState.NORMAL
            self._was_pressed_last_frame = False
        elif mouse_pressed:
            self.state = ButtonState.PRESSED
            self._was_pressed_last_frame = True
        elif not mouse_pressed and self._was_pressed_last_frame and is_hovering:
            # Mouse was released while hovering and button was pressed last frame
            self.state = ButtonState.HOVER
            self._was_pressed_last_frame = False
            if self.onClick:
                print(f"✅ Button '{self.text}' clicked!")
                self.onClick()
            if self.onRelease:
                self.onRelease()
        else:
            self.state = ButtonState.HOVER
            self._was_pressed_last_frame = False
    
    def handle_event(self, event, engine):
        """Handle mouse click events (currently unused - clicks handled in update())."""
        # Note: Click detection is now handled in update() method for better reliability
        pass
    
    def set_text(self, text):
        """Change the button text."""
        self.text = text
        self._text_surface = self._font.render(text, True, self.text_color)
    
    def render(self, screen, screen_height):
        """Render the button."""
        if not self.visible:
            return
        
        # Convert engine coords to pygame coords
        screen_x = self._screen_pos.x
        screen_y = screen_height - self._screen_pos.y
        
        # Create button rect
        button_rect = pg.Rect(0, 0, int(self.size.x), int(self.size.y))
        button_rect.center = (int(screen_x), int(screen_y))
        
        # Check if we have a sprite for current state
        sprite_surface = self._get_current_sprite()
        
        if sprite_surface:
            # Draw sprite
            screen.blit(sprite_surface, button_rect)
        else:
            # Draw color background
            bg_color = self._get_current_color()
            pg.draw.rect(screen, bg_color, button_rect)
        
        # Draw border if specified
        if self.border_width > 0:
            pg.draw.rect(screen, self.border_color, button_rect, self.border_width)
        
        # Draw text centered on button
        text_rect = self._text_surface.get_rect(center=button_rect.center)
        screen.blit(self._text_surface, text_rect)

