import pygame as pg
from pygame import Vector2, Rect
from .component import Component
from enum import Enum, auto
import math

class MouseButton(Enum):
    """Mouse button enumeration."""
    LEFT = auto()
    RIGHT = auto()
    MIDDLE = auto()

class MouseState:
    """Current mouse state including position, buttons, and drag info."""
    
    def __init__(self):
        self.position = Vector2(0, 0)  # Current mouse position
        self.world_position = Vector2(0, 0)  # Mouse position in world coordinates
        self.buttons = {
            MouseButton.LEFT: False,
            MouseButton.RIGHT: False,
            MouseButton.MIDDLE: False
        }
        self.wheel_delta = 0  # Mouse wheel delta
        self.dragging = False
        self.drag_start_pos = Vector2(0, 0)
        self.drag_current_pos = Vector2(0, 0)

class MouseInputSystem:
    """Centralized mouse input system managing all mouse state and events."""
    
    def __init__(self):
        self.current_state = MouseState()
        self.previous_state = MouseState()
        self.listeners = []  # Components that want mouse events
        self.drag_threshold = 5  # Minimum distance to start dragging
        
    def update(self, engine):
        """Update mouse state and process events."""
        # Store previous state for comparison
        self.previous_state.position = self.current_state.position.copy()
        self.previous_state.world_position = self.current_state.world_position.copy()
        self.previous_state.buttons = self.current_state.buttons.copy()
        self.previous_state.wheel_delta = self.current_state.wheel_delta
        
        # Update current state from pygame
        self.current_state.position = Vector2(pg.mouse.get_pos())
        self.current_state.world_position = engine.camera.screen_to_world(self.current_state.position)
        
        # Update button states from pygame
        mouse_buttons = pg.mouse.get_pressed()
        self.current_state.buttons[MouseButton.LEFT] = mouse_buttons[0]
        self.current_state.buttons[MouseButton.MIDDLE] = mouse_buttons[1]
        self.current_state.buttons[MouseButton.RIGHT] = mouse_buttons[2]
        
        # Reset wheel delta (will be set by engine if wheel event occurs)
        self.current_state.wheel_delta = 0
        
        # Update dragging state based on left button
        if self.is_button_pressed(MouseButton.LEFT):
            if not self.previous_state.buttons[MouseButton.LEFT]:
                # Button just pressed - start potential drag
                self.current_state.drag_start_pos = self.current_state.position.copy()
                self.current_state.drag_current_pos = self.current_state.position.copy()
                self.current_state.dragging = False
            else:
                # Button held - check if we're dragging
                self.current_state.drag_current_pos = self.current_state.position.copy()
                drag_distance = self.current_state.position.distance_to(self.current_state.drag_start_pos)
                if drag_distance > self.drag_threshold:
                    self.current_state.dragging = True
        else:
            # Button released - stop dragging
            self.current_state.dragging = False
        
        # Notify all registered listeners
        self._notify_listeners(engine)
    
    def add_listener(self, component):
        """Add a component to receive mouse events."""
        if component not in self.listeners:
            self.listeners.append(component)
    
    def remove_listener(self, component):
        """Remove a component from mouse event listeners."""
        if component in self.listeners:
            self.listeners.remove(component)
    
    def _notify_listeners(self, engine):
        """Notify all listeners of mouse events."""
        for listener in self.listeners:
            if listener.enabled:
                listener.on_mouse_update(self.current_state, self.previous_state, engine)
    
    def is_button_pressed(self, button):
        """Check if a mouse button is currently pressed."""
        return self.current_state.buttons[button]
    
    def is_button_just_pressed(self, button):
        """Check if a mouse button was just pressed this frame."""
        return (self.current_state.buttons[button] and 
                not self.previous_state.buttons[button])
    
    def is_button_just_released(self, button):
        """Check if a mouse button was just released this frame."""
        return (not self.current_state.buttons[button] and 
                self.previous_state.buttons[button])
    
    def get_position(self):
        """Get current mouse position in screen coordinates."""
        return self.current_state.position
    
    def get_world_position(self):
        """Get current mouse position in world coordinates."""
        return self.current_state.world_position
    
    def get_wheel_delta(self):
        """Get mouse wheel delta for this frame."""
        return self.current_state.wheel_delta
    
    def is_dragging(self):
        """Check if the mouse is currently dragging."""
        return self.current_state.dragging
    
    def get_drag_distance(self):
        """Get the distance the mouse has been dragged."""
        if self.current_state.dragging:
            return self.current_state.position.distance_to(self.current_state.drag_start_pos)
        return 0
    
    def get_drag_direction(self):
        """Get the direction of the drag as a normalized vector."""
        if self.current_state.dragging:
            direction = self.current_state.position - self.current_state.drag_start_pos
            if direction.length() > 0:
                return direction.normalize()
        return Vector2(0, 0)

class MouseInputComponent(Component):
    """Base component for handling mouse input events."""
    
    def __init__(self, game_object):
        super().__init__(game_object)
        self.mouse_system = None  # Will be set by the engine
    
    def start(self):
        """Register with the mouse input system."""
        pass  # Will be done in update when engine is available
    
    def update(self, engine):
        """Update and register with mouse system if needed."""
        super().update(engine)
        
        # Register with mouse system on first update
        if self.mouse_system is None and hasattr(engine, 'mouse_input'):
            self.mouse_system = engine.mouse_input
            self.mouse_system.add_listener(self)
    
    def on_destroy(self):
        """Unregister from the mouse input system."""
        if self.mouse_system:
            self.mouse_system.remove_listener(self)
    
    def on_mouse_update(self, current_state, previous_state, engine):
        """Called every frame with mouse state updates. Override in subclasses."""
        pass
    
    def on_mouse_enter(self, mouse_pos, world_pos):
        """Called when mouse enters this object's bounds."""
        pass
    
    def on_mouse_exit(self, mouse_pos, world_pos):
        """Called when mouse exits this object's bounds."""
        pass
    
    def on_mouse_click(self, button, mouse_pos, world_pos):
        """Called when mouse clicks on this object."""
        pass
    
    def on_mouse_drag_start(self, mouse_pos, world_pos):
        """Called when mouse starts dragging on this object."""
        pass
    
    def on_mouse_drag(self, mouse_pos, world_pos, drag_distance, drag_direction):
        """Called while mouse is dragging on this object."""
        pass
    
    def on_mouse_drag_end(self, mouse_pos, world_pos):
        """Called when mouse stops dragging on this object."""
        pass
    
    def on_mouse_wheel(self, delta, mouse_pos, world_pos):
        """Called when mouse wheel is scrolled over this object."""
        pass

class MouseHoverComponent(MouseInputComponent):
    """Component that detects when mouse hovers over an object."""
    
    def __init__(self, game_object, hover_area=None):
        super().__init__(game_object)
        self.hover_area = hover_area
        self.is_hovering = False
        self.hover_callbacks = []
    
    def add_hover_callback(self, callback):
        """Add a callback function for hover events."""
        self.hover_callbacks.append(callback)
    
    def on_mouse_update(self, current_state, previous_state, engine):
        """Check for hover state changes."""
        # Determine the area to check for hover
        check_area = self.hover_area
        if check_area is None:
            # Use the game object's bounds
            obj = self.game_object
            check_area = Rect(
                obj.position.x - obj.size.x/2,
                obj.position.y - obj.size.y/2,
                obj.size.x,
                obj.size.y
            )
        
        # Convert mouse position to world coordinates for comparison
        mouse_world_pos = current_state.world_position
        
        # Check if mouse is within the hover area
        was_hovering = self.is_hovering
        self.is_hovering = check_area.collidepoint(mouse_world_pos.x, mouse_world_pos.y)
        
        # Handle hover state changes
        if self.is_hovering and not was_hovering:
            # Mouse just entered
            self.on_mouse_enter(current_state.position, mouse_world_pos)
            for callback in self.hover_callbacks:
                try:
                    callback(True, current_state.position, mouse_world_pos)
                except Exception as e:
                    print(f"Error in hover callback: {e}")
        
        elif not self.is_hovering and was_hovering:
            # Mouse just exited
            self.on_mouse_exit(current_state.position, mouse_world_pos)
            for callback in self.hover_callbacks:
                try:
                    callback(False, current_state.position, mouse_world_pos)
                except Exception as e:
                    print(f"Error in hover callback: {e}")

class MouseClickComponent(MouseInputComponent):
    """Component that handles mouse clicks and drags on an object."""
    
    def __init__(self, game_object, click_area=None):
        super().__init__(game_object)
        self.click_area = click_area
        self.click_callbacks = {}  # button -> list of callbacks
        self.drag_callbacks = []
        self.drag_started = False
        self.drag_start_pos = Vector2(0, 0)
    
    def add_click_callback(self, button, callback):
        """Add a callback function for click events on a specific button."""
        if button not in self.click_callbacks:
            self.click_callbacks[button] = []
        self.click_callbacks[button].append(callback)
    
    def add_drag_callback(self, callback):
        """Add a callback function for drag events."""
        self.drag_callbacks.append(callback)
    
    def on_mouse_update(self, current_state, previous_state, engine):
        """Check for clicks and drags."""
        # Determine the area to check for interactions
        check_area = self.click_area
        if check_area is None:
            # Use the game object's bounds
            obj = self.game_object
            check_area = Rect(
                obj.position.x - obj.size.x/2,
                obj.position.y - obj.size.y/2,
                obj.size.x,
                obj.size.y
            )
        
        # Convert mouse position to world coordinates
        mouse_world_pos = current_state.world_position
        
        # Check if mouse is within the click area
        mouse_in_area = check_area.collidepoint(mouse_world_pos.x, mouse_world_pos.y)
        
        # Handle button clicks
        for button in MouseButton:
            if self.mouse_system.is_button_just_pressed(button) and mouse_in_area:
                # Click occurred
                self.on_mouse_click(button, current_state.position, mouse_world_pos)
                if button in self.click_callbacks:
                    for callback in self.click_callbacks[button]:
                        try:
                            callback(button, current_state.position, mouse_world_pos)
                        except Exception as e:
                            print(f"Error in click callback: {e}")
                
                # Start potential drag
                self.drag_started = True
                self.drag_start_pos = current_state.position
        
        # Handle drag events
        if self.drag_started:
            if self.mouse_system.is_button_pressed(MouseButton.LEFT):
                if self.mouse_system.is_dragging():
                    # Drag is happening
                    drag_distance = self.mouse_system.get_drag_distance()
                    drag_direction = self.mouse_system.get_drag_direction()
                    
                    self.on_mouse_drag(current_state.position, mouse_world_pos, 
                                     drag_distance, drag_direction)
                    
                    for callback in self.drag_callbacks:
                        try:
                            callback(current_state.position, mouse_world_pos, 
                                   drag_distance, drag_direction)
                        except Exception as e:
                            print(f"Error in drag callback: {e}")
                else:
                    # Drag started
                    self.on_mouse_drag_start(current_state.position, mouse_world_pos)
            else:
                # Button released - end drag
                if self.drag_started:
                    self.on_mouse_drag_end(current_state.position, mouse_world_pos)
                    self.drag_started = False

class MouseWheelComponent(MouseInputComponent):
    """Component that handles mouse wheel events on an object."""
    
    def __init__(self, game_object, wheel_area=None):
        super().__init__(game_object)
        self.wheel_area = wheel_area
        self.wheel_callbacks = []
    
    def add_wheel_callback(self, callback):
        """Add a callback function for wheel events."""
        self.wheel_callbacks.append(callback)
    
    def on_mouse_update(self, current_state, previous_state, engine):
        """Check for wheel events."""
        # Determine the area to check for wheel events
        check_area = self.wheel_area
        if check_area is None:
            # Use the game object's bounds
            obj = self.game_object
            check_area = Rect(
                obj.position.x - obj.size.x/2,
                obj.position.y - obj.size.y/2,
                obj.size.x,
                obj.size.y
            )
        
        # Convert mouse position to world coordinates
        mouse_world_pos = current_state.world_position
        
        # Check if mouse is within the wheel area and wheel was scrolled
        mouse_in_area = check_area.collidepoint(mouse_world_pos.x, mouse_world_pos.y)
        wheel_delta = current_state.wheel_delta
        
        if mouse_in_area and wheel_delta != 0:
            # Wheel event occurred
            self.on_mouse_wheel(wheel_delta, current_state.position, mouse_world_pos)
            
            for callback in self.wheel_callbacks:
                try:
                    callback(wheel_delta, current_state.position, mouse_world_pos)
                except Exception as e:
                    print(f"Error in wheel callback: {e}")

# Utility functions for easy access to mouse state
def get_mouse_position(engine):
    """Get current mouse position in screen coordinates."""
    return engine.mouse_input.get_position()

def get_mouse_world_position(engine):
    """Get current mouse position in world coordinates."""
    return engine.mouse_input.get_world_position()

def is_mouse_button_pressed(engine, button):
    """Check if a mouse button is currently pressed."""
    return engine.mouse_input.is_button_pressed(button)

def is_mouse_button_just_pressed(engine, button):
    """Check if a mouse button was just pressed this frame."""
    return engine.mouse_input.is_button_just_pressed(button)

def is_mouse_button_just_released(engine, button):
    """Check if a mouse button was just released this frame."""
    return engine.mouse_input.is_button_just_released(button) 