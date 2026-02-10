<!-- ![Logo](images/1_zoomed.png) -->
![Logo](images/1_lower-res.png)
# PyG Engine

A Python game engine built on Rust and WebGPU. 
Automatic Hardware Acceleration on Vulkan, DX12, OpenGL, Metal through the WGPU API.
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

## Python Rendering Examples

After installation (`pip install -e .`), try:

- `python examples/python_direct_draw_demo.py`
- `python examples/python_mesh_demo.py`

Direct draw from Python:

```python
import pyg_engine as pyg

engine = pyg.Engine()
engine.draw_line(20, 20, 220, 80, pyg.Color.CYAN, thickness=2.0)
engine.draw_rectangle(60, 120, 180, 90, pyg.Color.ORANGE, filled=False, thickness=3.0)
engine.run(title="Python Direct Draw", show_fps_in_title=True, redraw_on_change_only=False)
```

GameObject + Mesh from Python:

```python
import pyg_engine as pyg

engine = pyg.Engine()
go = pyg.GameObject("Quad")
mesh = pyg.MeshComponent("QuadMesh")
mesh.set_geometry_rectangle(1.0, 1.0)
mesh.set_fill_color(pyg.Color.WHITE)
mesh.set_image_path("images/1.png")
go.set_mesh_component(mesh)
engine.add_game_object(go)
engine.run(title="Python Mesh")
```

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
