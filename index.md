---
layout: default
title: PyG Engine
---

![Logo](images/1.png)

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

# Create a game object
player = GameObject(
    name="Player",
    position=(400, 300),
    size=(50, 50),
    color=Color(255, 0, 0)
)

# Add to engine
engine.addGameObject(player)

# Start the game loop
engine.start()
```

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

## Documentation

Check out our comprehensive [Core Systems Guide](docs/CORE_SYSTEMS_GUIDE.html) for detailed documentation on all features and systems.

## Examples

The project includes numerous examples demonstrating various features:

- Basic game setup and object management
- Physics and collision detection
- Input handling (mouse, keyboard)
- Event system usage
- Camera and rendering features
- Performance analysis tools

## Contributing

We welcome contributions! Please check out our [GitHub repository](https://github.com/your-username/LinearInterpolation) for more information.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details. 