# Mouse Input System Guide

The mouse input system provides a comprehensive way to handle mouse interactions in your game engine. It includes support for hover detection, clicking, dragging, and mouse wheel events.

## Features

- **Mouse Position Tracking**: Get mouse position in both screen and world coordinates
- **Button State Management**: Track mouse button presses, releases, and held states
- **Hover Detection**: Detect when mouse enters or exits object bounds
- **Click Detection**: Handle clicks on specific objects
- **Drag Detection**: Detect and handle mouse dragging with customizable thresholds
- **Mouse Wheel Support**: Handle scroll wheel events
- **Camera Integration**: Works seamlessly with the camera system for world coordinates
- **Component-Based**: Easy to add to any GameObject using components

## Quick Start

### Basic Usage

```python
from mouse_input import MouseHoverComponent, MouseClickComponent, MouseButton

# Add mouse components to a GameObject
game_object.add_component(MouseHoverComponent(game_object))
game_object.add_component(MouseClickComponent(game_object))

# Set up callbacks
hover_comp = game_object.get_component(MouseHoverComponent)
click_comp = game_object.get_component(MouseClickComponent)

hover_comp.add_hover_callback(lambda event, pos, world_pos: print(f"Hover {event}"))
click_comp.add_click_callback(MouseButton.LEFT, lambda btn, pos, world_pos: print("Left clicked!"))
```

### Complete Example

```python
import pygame as pg
from pygame import Color, Vector2
from engine import Engine
from gameobject import BasicObject
from object_types import Size, BasicShape
from mouse_input import MouseHoverComponent, MouseClickComponent, MouseButton

class InteractiveObject(BasicObject):
    def __init__(self, name, position, size, color):
        super().__init__(name, position, size, color)
        
        # Add mouse components
        self.add_component(MouseHoverComponent(self))
        self.add_component(MouseClickComponent(self))
        
        # Set up callbacks
        hover_comp = self.get_component(MouseHoverComponent)
        click_comp = self.get_component(MouseClickComponent)
        
        hover_comp.add_hover_callback(self._on_hover)
        click_comp.add_click_callback(MouseButton.LEFT, self._on_click)
        
        self.original_color = color
    
    def _on_hover(self, event_type, mouse_pos, world_pos):
        if event_type == 'enter':
            self.color = Color(255, 255, 0)  # Yellow
        elif event_type == 'exit':
            self.color = self.original_color
    
    def _on_click(self, button, mouse_pos, world_pos):
        print(f"{self.name} was clicked at {mouse_pos}")

# Create engine and objects
engine = Engine(size=Size(w=800, h=600))
obj = InteractiveObject("Test", Vector2(400, 300), Vector2(100, 100), Color(255, 0, 0))
engine.addGameObject(obj)
engine.start()
```

## Components

### MouseHoverComponent

Detects when the mouse enters or exits an object's bounds.

```python
from mouse_input import MouseHoverComponent

# Add to GameObject
hover_comp = MouseHoverComponent(game_object)
game_object.add_component(hover_comp)

# Add callback
hover_comp.add_hover_callback(lambda event, pos, world_pos: print(f"Hover {event}"))

# Custom hover area (optional)
custom_area = pg.Rect(0, 0, 200, 200)
hover_comp = MouseHoverComponent(game_object, hover_area=custom_area)
```

**Methods:**
- `add_hover_callback(callback)`: Add a callback for hover events

### MouseClickComponent

Handles mouse clicks and drags on objects.

```python
from mouse_input import MouseClickComponent, MouseButton

# Add to GameObject
click_comp = MouseClickComponent(game_object)
game_object.add_component(click_comp)

# Add click callbacks
click_comp.add_click_callback(MouseButton.LEFT, lambda btn, pos, world_pos: print("Left click"))
click_comp.add_click_callback(MouseButton.RIGHT, lambda btn, pos, world_pos: print("Right click"))

# Add drag callbacks
click_comp.add_drag_callback(lambda pos, world_pos, distance, direction: print(f"Dragging {distance}"))

# Custom click area (optional)
custom_area = pg.Rect(0, 0, 200, 200)
click_comp = MouseClickComponent(game_object, click_area=custom_area)
```

**Methods:**
- `add_click_callback(button, callback)`: Add a callback for click events
- `add_drag_callback(callback)`: Add a callback for drag events

### MouseWheelComponent

Handles mouse wheel events over objects.

```python
from mouse_input import MouseWheelComponent

# Add to GameObject
wheel_comp = MouseWheelComponent(game_object)
game_object.add_component(wheel_comp)

# Add wheel callback
wheel_comp.add_wheel_callback(lambda delta, pos, world_pos: print(f"Wheel {delta}"))

# Custom wheel area (optional)
custom_area = pg.Rect(0, 0, 200, 200)
wheel_comp = MouseWheelComponent(game_object, wheel_area=custom_area)
```

**Methods:**
- `add_wheel_callback(callback)`: Add a callback for wheel events

## Mouse Button Enum

```python
from mouse_input import MouseButton

# Available buttons
MouseButton.LEFT    # Left mouse button
MouseButton.RIGHT   # Right mouse button
MouseButton.MIDDLE  # Middle mouse button (scroll wheel click)
```

## Callback Functions

### Hover Callbacks
```python
def hover_callback(is_entering, mouse_pos, world_pos):
    """
    Args:
        is_entering (bool): True if mouse is entering, False if exiting
        mouse_pos (Vector2): Mouse position in screen coordinates
        world_pos (Vector2): Mouse position in world coordinates
    """
    if is_entering:
        print("Mouse entered object")
    else:
        print("Mouse exited object")
```

### Click Callbacks
```python
def click_callback(button, mouse_pos, world_pos):
    """
    Args:
        button (MouseButton): Which button was clicked
        mouse_pos (Vector2): Mouse position in screen coordinates
        world_pos (Vector2): Mouse position in world coordinates
    """
    print(f"Clicked {button} at {mouse_pos}")
```

### Drag Callbacks
```python
def drag_callback(mouse_pos, world_pos, drag_distance, drag_direction):
    """
    Args:
        mouse_pos (Vector2): Current mouse position in screen coordinates
        world_pos (Vector2): Current mouse position in world coordinates
        drag_distance (float): Distance dragged from start point
        drag_direction (Vector2): Normalized direction of drag
    """
    print(f"Dragging {drag_distance} units in direction {drag_direction}")
```

### Wheel Callbacks
```python
def wheel_callback(delta, mouse_pos, world_pos):
    """
    Args:
        delta (int): Wheel scroll amount (positive = up, negative = down)
        mouse_pos (Vector2): Mouse position in screen coordinates
        world_pos (Vector2): Mouse position in world coordinates
    """
    print(f"Wheel scrolled {delta}")
```

## Advanced Usage

### Custom Areas

You can specify custom areas for mouse detection instead of using the object's bounds:

```python
# Custom rectangular area
custom_area = pg.Rect(100, 100, 200, 150)
hover_comp = MouseHoverComponent(game_object, hover_area=custom_area)
```

### Multiple Components

You can add multiple mouse components to the same object:

```python
# Add both hover and click components
game_object.add_component(MouseHoverComponent(game_object))
game_object.add_component(MouseClickComponent(game_object))
game_object.add_component(MouseWheelComponent(game_object))
```

### Dynamic Callbacks

You can add and remove callbacks dynamically:

```python
hover_comp = game_object.get_component(MouseHoverComponent)

# Add callback
def my_callback(is_entering, mouse_pos, world_pos):
    print("Hover event")

hover_comp.add_hover_callback(my_callback)

# Remove callback (if needed)
# Note: The system doesn't currently support callback removal
# You can modify the callback to do nothing instead
```

## Coordinate Systems

### Screen Coordinates
- Origin at top-left corner of window
- X increases right, Y increases down
- Used for UI elements and screen-space calculations

### World Coordinates
- Origin at world center (or camera position)
- Affected by camera position and zoom
- Used for game logic and physics

### Converting Between Systems

The mouse system automatically handles coordinate conversion:

```python
# Get mouse position in screen coordinates
screen_pos = engine.mouse_input.get_position()

# Get mouse position in world coordinates
world_pos = engine.mouse_input.get_world_position()

# Convert manually if needed
world_pos = engine.camera.screen_to_world(screen_pos)
screen_pos = engine.camera.world_to_screen(world_pos)
```

## Performance Considerations

### Efficient Callbacks
Keep callback functions lightweight:

```python
# ✅ GOOD: Lightweight callback
def hover_callback(is_entering, mouse_pos, world_pos):
    self.highlighted = is_entering

# ❌ BAD: Heavy callback
def hover_callback(is_entering, mouse_pos, world_pos):
    # Don't do heavy processing in callbacks
    self.complex_calculation()
    self.database_query()
    self.network_request()
```

### Component Management
Remove components when not needed:

```python
# Remove component when object is destroyed
def on_destroy(self):
    if hasattr(self, 'hover_comp'):
        self.hover_comp = None
```

## Troubleshooting

### Common Issues

1. **Mouse events not firing**: Check if component is enabled and object is active
2. **Wrong coordinates**: Verify you're using the right coordinate system
3. **Drag not working**: Check drag threshold and button state
4. **Performance issues**: Keep callbacks lightweight

### Debug Tips

```python
# Print mouse state for debugging
print(f"Mouse position: {engine.mouse_input.get_position()}")
print(f"Mouse buttons: {engine.mouse_input.current_state.buttons}")
print(f"Is dragging: {engine.mouse_input.is_dragging()}")
```

## Best Practices

1. **Use appropriate components**: Choose the right component for your needs
2. **Keep callbacks simple**: Don't do heavy processing in callbacks
3. **Handle errors**: Wrap callback logic in try-catch blocks
4. **Test thoroughly**: Test mouse interactions with different scenarios
5. **Consider accessibility**: Provide keyboard alternatives for mouse-only features 