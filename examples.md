---
layout: default
title: Examples
permalink: /examples/
---

# PyG Engine Examples

This page showcases various examples demonstrating the capabilities of the PyG Engine.

## Available Examples

### Basic Examples

- **basic_example.py** - Simple game setup with basic game objects
- **test.py** - Basic functionality test
- **main.py** - Main example demonstrating core features

### Input and Interaction

- **enhanced_mouse_example.py** - Advanced mouse input handling
- **mouse_test.py** - Mouse input testing
- **input_test.py** - Comprehensive input system testing

### Event System

- **enum_event_example.py** - Event system with enumerations
- **global_dictionary_test.py** - Global dictionary functionality

### Games and Demos

- **snake_game.py** - Complete Snake game implementation
- **runnable_demo.py** - Runnable system demonstration
- **visual_runnable_demo.py** - Visual demonstration of runnable system

### Performance and Analysis

- **performance_analysis.py** - Performance analysis tools
- **performance_test.py** - Performance testing utilities

### Scripts

- **scripts/player.py** - Player script example
- **scripts/snake_script.py** - Snake game script
- **scripts/test_script.py** - Test script example

## Running Examples

To run any example, navigate to the examples directory and execute:

```bash
cd examples
python basic_example.py
```

Or run from the project root:

```bash
python examples/basic_example.py
```

## Example Code Snippets

### Basic Game Setup

```python
from pyg_engine import Engine, GameObject, Size
from pygame import Color

# Create engine
engine = Engine(
    size=Size(w=800, h=600),
    backgroundColor=Color(0, 0, 0),
    windowName="Basic Example"
)

# Create game object
player = GameObject(
    name="Player",
    position=(400, 300),
    size=(50, 50),
    color=Color(255, 0, 0)
)

engine.addGameObject(player)
engine.start()
```

### Physics Example

```python
from pyg_engine import Engine, GameObject, RigidBody, BoxCollider
from pygame import Color

# Create engine with physics
engine = Engine(
    size=Size(w=800, h=600),
    backgroundColor=Color(0, 0, 0)
)

# Create physics object
physics_object = GameObject(
    name="PhysicsObject",
    position=(400, 100),
    size=(50, 50),
    color=Color(0, 255, 0)
)

# Add physics components
rigidbody = RigidBody(mass=1.0, gravity_scale=1.0)
collider = BoxCollider(width=50, height=50)

physics_object.addComponent(rigidbody)
physics_object.addComponent(collider)

engine.addGameObject(physics_object)
engine.start()
```

### Event System Example

```python
from pyg_engine import Engine, EventManager, Event
from pygame import Color

engine = Engine(size=Size(w=800, h=600))

# Create custom event
class CustomEvent(Event):
    def __init__(self, data):
        super().__init__("custom_event")
        self.data = data

# Register event handler
def handle_custom_event(event):
    print(f"Custom event received: {event.data}")

EventManager.register("custom_event", handle_custom_event)

# Trigger event
EventManager.trigger(CustomEvent("Hello from PyG Engine!"))

engine.start()
```

## Contributing Examples

If you create new examples, please:

1. Place them in the `examples/` directory
2. Include clear comments explaining the functionality
3. Follow the existing naming conventions
4. Test the example before submitting

For more detailed documentation, see the [Core Systems Guide](docs/CORE_SYSTEMS_GUIDE.html). 