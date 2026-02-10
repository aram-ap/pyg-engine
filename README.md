![Logo](images/1_lower-res.png)

# PyG Engine

A high-performance Python game engine built on **Rust** and **WebGPU (wgpu)**.

PyG Engine combines the ease of use of Python with the raw performance and safety of Rust. It leverages `wgpu` for modern, hardware-accelerated rendering across all major platforms (Vulkan, DirectX 12, Metal, OpenGL).

> **NOTE:** This project is currently in **Alpha**. Features are under active development.

## 🚀 Key Features

*   **Modern Rendering**: Powered by **wgpu** for cross-platform, high-performance graphics.
*   **Rust Core**: The heavy lifting is done in Rust, ensuring speed and memory safety.
*   **Pythonic API**: Designed to feel natural for Python developers.
*   **Immediate Mode Drawing**: Easily draw lines, rectangles, circles, and pixels using pixel coordinates.
*   **Mesh System**: Render textured quads and game objects with a component-based architecture (using normalized coordinates).
*   **Thread Safety**: Unique `EngineHandle` system allows you to safely issue rendering commands from background Python threads.
*   **Robust Logging**: Integrated tracing-based logging system with file support and configurable levels.

## 📦 Installation

Requires **Python 3.7+**.

### From PyPI (Coming Soon)
```bash
pip install pyg-engine
```

### From Source
```bash
git clone https://github.com/yourusername/pyg-engine.git
cd pyg-engine
pip install -e .
```

## ⚡ Quick Start

### 1. Basic Window & Logging
```python
import pyg_engine as pyg

# Initialize the engine
engine = pyg.Engine(log_level="INFO")
engine.log_info("Welcome to PyG Engine!")

# Run a window (blocks until closed)
engine.run(title="My First Window", width=800, height=600)
```

### 2. Drawing Primitives (Pixel Coordinates)
```python
import pyg_engine as pyg

engine = pyg.Engine()

# Draw a cyan line
engine.draw_line(20, 20, 220, 80, pyg.Color.CYAN, thickness=2.0)

# Draw an orange rectangle outline
engine.draw_rectangle(60, 120, 180, 90, pyg.Color.ORANGE, filled=False, thickness=3.0)

# Draw text (built-in font by default)
engine.draw_text("Hello PyG", 32, 48, pyg.Color.WHITE, font_size=28.0)

# Start the application
engine.run(title="Direct Draw Demo", show_fps_in_title=True)
```

### 3. Using Game Objects & Meshes (Normalized Coordinates)
```python
import pyg_engine as pyg

engine = pyg.Engine()

# Create a Game Object
go = pyg.GameObject("Player")

# Add a Mesh Component
mesh = pyg.MeshComponent("PlayerSprite")
mesh.set_geometry_rectangle(1.0, 1.0) # 1.0 width/height in normalized units
mesh.set_fill_color(pyg.Color.RED)
# mesh.set_image_path("path/to/image.png") # Optional texture

go.set_mesh_component(mesh)

# Position is in Normalized Device Coordinates (NDC)
# (0,0) is center, (-1, -1) bottom-left, (1, 1) top-right
go.position = pyg.Vec2(0.0, 0.0)
go.scale = pyg.Vec2(0.5, 0.5)

engine.add_game_object(go)

engine.run(title="Game Object Demo")
```

## 🔧 Architecture & Roadmap

### Current Capabilities
- **Window Management**: Resizable windows, VSync control, Fullscreen support.
- **2D Rendering**:
    - **Primitives**: Immediate mode drawing using pixel coordinates.
    - **Text**: Built-in open-source font rendering with optional custom font files.
    - **Meshes**: Component-based rendering using normalized device coordinates.
    - **Layers**: Z-indexing and integer layering for draw order control.
- **Component System**: Basic `GameObject` with `TransformComponent` and `MeshComponent`.

### Planned Features (Roadmap)
- **Input System Exposure**: Exposing the internal Rust input manager (Keyboard, Mouse, Gamepad) to Python.
- **Physics Engine**: 2D rigid body physics and collision detection.
- **Scripting**: Enhanced script attachment to GameObjects.
- **Advanced Rendering**: Shaders, Particles, and Post-processing.
- **UI System**: Built-in UI components.

## 📂 Examples

Check the `examples/` directory for more complete demonstrations:

- `python_direct_draw_demo.py`: Shows how to draw basic shapes (pixels, lines, rects).
- `python_mesh_demo.py`: Demonstrates the GameObject and Mesh system.
- `python_threading_demo.py`: **Advanced**: Spawns a background thread that safely updates the UI using `engine.get_handle()`.
- `python_manual_loop.py`: Shows how to control the game loop manually (initialize -> poll -> update -> render).

## 🛠️ Development & Testing

To set up the development environment:

```bash
# Install in editable mode
pip install -e .

# Run tests
pytest tests/ -v
```

## 📄 License

MIT License
