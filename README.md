# Pyg Engine

A Python game engine built with Pygame and Pymunk for 2D physics, rendering, and game development.
Inspired by the Unity game engine's Monobehavior system with scriptable game objects, rigidbody and collider system.
Built in physics materials, update system, and mouse+keyboard input system. Built-in window resizing.


> **NOTE:** This is in alpha development stage. Everything is under active development and large changes will likely be made.
> _Also,_ its pronounced _**pig engine**_ :)

## Features

- **OOP Model**: Easy and simple game object implementation system. Create players, environment, enemies, etc. with ease
- **2D Physics**: Built-in physics via Pymunk
- **Input**: Mouse and keyboard input handling
- **Components**: Modular component-based architecture
- **Scripts**: Dynamic script loading and execution
- **Camera**: Flexible camera with multiple scaling modes
- **Collision**: Advanced collision detection and response

## Installation

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
python examples/mouse_example.py
python examples/enhanced_mouse_example.py
python examples/main_physics_test.py
python examples/simple_drag_test.py
```

Available examples:
- `basic_example.py` - Basic engine setup and object creation
- `main.py` - Complete physics demo with collision detection, mouse and keyboard input, multiple game objects, and physics materials.
- `enhanced_mouse_example.py` - Advanced mouse interactions with physics
- `mouse_example.py` - Mouse input handling and interaction

## Documentation

See the `docs/` directory for detailed guides:

- `README.md` - General documentation
- `PHYSICS_CONTROLS_GUIDE.md` - Physics system guide
- `MOUSE_INPUT_GUIDE.md` - Input system guide
- `SPRITE_SYSTEM_UPGRADE.md` - Sprite system documentation

## Testing

Run the test suite:

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
##### In Development:
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
