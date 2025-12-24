<!-- ![Logo](images/1_zoomed.png) -->
![Logo](images/1_lower-res.png)
# PyG Engine

A Python game engine built on Rust and WebGPU. 
Inspired by the Unity game engine's Monobehavior system with scriptable game objects, rigidbody and collider system.
Built-in physics materials, update system, event system and mouse+keyboard input system. Built-in window resizing.

> **NOTE:** This is in alpha development stage. Everything is under active development and large changes will likely be made.

## Features (SOON)

- **OOP Model**: Simple game object implementation system
- **2D Physics**: Built-in physics and rigidbody simulations
- **Input**: Mouse, keyboard, and joystick input handling
- **Components**: Modular component-based architecture
- **Scripts**: Dynamic script loading and execution
- **Camera**: Flexible camera with multiple scaling modes
- **Event System**: Thread-safe event-driven communication with priority-based handling
- **Documentation**: Comprehensive CORE_SYSTEMS_GUIDE with examples and best practices

## Installation

Requires Python 3.7+.

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

# Building and Testing

```bash
# Install for development
pip install -e .

# Build wheel for PyPI
python -m build --wheel

# Run tests
pytest tests/test_engine_rust.py -v
```

## Quick Start

```python
import pyg_engine as pyg

# Create the engine
engine = pyg.Engine()

# Log the version
engine.log(f"Welcome to pyg-engine!")

```

Examples:
- `basic_example.py` - Basic engine setup and object creation

## Documentation

See the `docs/` directory for detailed guides:

## Testing

Run the test suite (tests are located in the `tests/` directory):

```bash
pytest tests -v
```

## Development

To set up the development environment:

```bash
pip install -e .
```

## License

MIT License

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
