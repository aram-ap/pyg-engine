<!-- ![Logo](images/1_zoomed.png) -->
![Logo](images/1_lower-res.png)
# PyG Engine

A Python game engine built on Pygame and Pymunk for 2D physics, rendering, and game development.
Inspired by the Unity game engine's Monobehavior system with scriptable game objects, rigidbody and collider system.
Built-in physics materials, update system, event system and mouse+keyboard input system. Built-in window resizing.

> **NOTE:** This is in alpha development stage. Everything is under active development and large changes will likely be made.
> _Also,_ its pronounced _**pig engine**_ :)

## Features

- **OOP Model**: Simple game object implementation system
- **2D Physics**: Built-in physics via Pymunk
- **Input**: Mouse, keyboard, and joystick input handling
- **Components**: Modular component-based architecture
- **Scripts**: Dynamic script loading and execution
- **Camera**: Flexible camera with multiple scaling modes
- **Event System**: Thread-safe event-driven communication with priority-based handling
- **Documentation**: Comprehensive CORE_SYSTEMS_GUIDE with examples and best practices

## Installation

Requires Python 3.7+.

Dependencies:
- pygame >= 2.5.0
- pymunk >= 6.4.0

Install via pip:

```bash
pip install pyg-engine
```

Or install from source:

```bash
git clone <repository-url>
cd pyg-engine
pip install -e .
```

## Quick Start

```python
from pyg_engine import Engine, GameObject, Size
from pygame import Color, Vector2

# Create the engine
engine = Engine(
    size=Size(w=800, h=600),
    backgroundColor=Color(0, 0, 0),
    windowName="My Game"
)

# Create a game object (supports both tuple and Vector2 formats)
player = GameObject(
    name="Player",
    position=(400, 300),  # Can use tuple
    size=(50, 50),        # Can use tuple
    color=Color(255, 0, 0)
)

# Alternative using Vector2 objects:
# player = GameObject(
#     name="Player",
#     position=Vector2(400, 300),
#     size=Vector2(50, 50),
#     color=Color(255, 0, 0)
# )

# Add to engine
engine.addGameObject(player)

# Start the game loop
engine.start()
```

## Examples

Run examples directly:

```bash
# List all available examples
python examples/__init__.py

# Run a specific example
python examples/basic_example.py
python examples/test.py
python examples/enhanced_mouse_example.py
python examples/enum_event_example.py
python examples/main.py
python examples/snake_game.py
python examples/performance_analysis.py
python examples/performance_test.py
python examples/visual_runnable_demo.py
```

Available examples:
- `basic_example.py` - Basic engine setup and object creation
- `test.py` - Basic gameobject creation, components, and scripts
- `main.py` - Complete physics demo with collision detection, input, multiple game objects, and physics materials
- `enhanced_mouse_example.py` - Advanced mouse interactions with physics
- `enum_event_example.py` - Enum-based event handling example
- `snake_game.py` - Simple snake game demo
- `performance_analysis.py` - Performance profiling and analysis
- `performance_test.py` - Performance test scenarios
- `visual_runnable_demo.py` - Visual demo with runnables
- `mouse_test.py` - Mouse input testing and validation
- `input_test.py` - Input system testing and validation
- `using_no_display.py` - Headless engine operation without display
- `global_dictionary_test.py` - Global dictionary system testing
- `runnable_demo.py` - Runnable system demonstration and testing

## Documentation

See the `docs/` directory for detailed guides:
- `CORE_SYSTEMS_GUIDE.md` - Comprehensive guide to engine architecture, systems, and usage

## Testing

Run the test suite (tests are located in the `tests/` directory):

```bash
cd pyg_engine
python -m pytest tests/
```

## Development

To set up the development environment:

```bash
pip install -e .
```

## TODO
##### Known Bugs/QOL Issues:
QOL:
- Components should be separate objects that can be passed into the add_component() function
- Discrepancies in sizing. Box colliders should accept Size() or Tuples.
- Needs better debug logs + distinguish between errors and console outputs
- Need a texture system

BUGS:
- Theres definitely some, but idk yet. Need to search more.

##### In Development:
- Sprite rendering and physics
- State machine
- Animation system
- Sprite colliders
- Audio system
- Coroutines and async services
- More basic shapes

##### Planned:
- Debug interface
- File storage system
- 2D lighting system

## License

MIT License

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
