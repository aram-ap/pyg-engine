---
layout: default
title: PyG Engine
---

<div class="hero-logo">
  <img src="{{ site.baseurl }}/images/1_lower-res.png" alt="PyG Engine Logo" style="max-width: 600px; height: auto; border-radius: 8px; border: 2px solid #30395C;">
</div>

# PyG Engine

PyG-Engine is a Python game engine built on SFML C++, made for quick prototyping and game development. It is built around scriptable game objects, rigidbody and collider system.
Contains a built-in physics system, update system, events, live serialization, opengl rendering, and more.


<div class="callout warning">
  <b>NOTE: This is in alpha development stage. Everything is under active development and large changes will likely be made.
</div>

## Features

<div class="features-grid">
  <div class="feature-card">
    <h4>🎮 OOP Model</h4>
    <p>Simple game object implementation system</p>
  </div>

  <div class="feature-card">
    <h4>⚡ 2D Physics</h4>
    <p>Built-in physics</p>
  </div>

  <div class="feature-card">
    <h4>🖱️ Input</h4>
    <p>Mouse, keyboard, and joystick input handling</p>
  </div>

  <div class="feature-card">
    <h4>🧩 Components</h4>
    <p>Modular component-based architecture</p>
  </div>

  <div class="feature-card">
    <h4>📜 Scripts</h4>
    <p>Dynamic script loading and execution</p>
  </div>

  <div class="feature-card">
    <h4>📷 Camera</h4>
    <p>Flexible camera with multiple scaling modes</p>
  </div>

  <div class="feature-card">
    <h4>📡 Event System</h4>
    <p>Thread-safe event-driven communication with priority-based handling (WIP)</p>
  </div>

  <div class="feature-card">
    <h4>📚 Documentation</h4>
    <p>Comprehensive guide with examples and best practices (WIP)</p>
  </div>
</div>

## Quick Start

```python
import pyg

def main():
    # Create the engine
    engine = pyg.Engine()

    # Logging through the engine (optional)
    pyg.log(f"Pyg-Engine Version: {engine.version}")

    # Set the window size
    engine.window.size = (700, 700)

    # Disable the resize function
    # (disabled by default but if you wanted to change it, here it is)
    engine.window.resize = False

    # Set FPS cap, default is uncapped (any value < 0)
    engine.fps = 60

    # Create our scene, this is where all game objects will exist
    scene = Scene()

    # The scene object has a camera object we can change to fit our needs
    # - CameraType.Screenspace (default) indicates that the window serves as
    # - the canvas for everything. (0,0) will always be at the center of the
    # - worldspace
    scene.camera.type = CameraType.Screenspace

    # Screenspace.Center (default) specifies that our (0,0) point is at the
    # - center of the screen. This can be changed to Screenspace.TopLeft,
    # - TopCenter, TopRight, Left, Right, BottomLeft, BottomCenter, BottomRight
    # - depending on your needs. The Screenspace location type is used only for
    # - the Screenspace camera and/or canvas
    scene.camera.screenspace_anchor = Screenspace.Center

    # - CameraType.Worldspace would act as if the camera was a game object-
    # - containing its own location (that can be changed), rotation, size, etc.
    # - This would be more helpful with things like 2D platformers
    # - scene.camera.type = CameraType.Worldspace

    scene.camera.background = Color("#101010")

    # The scene contains all our game information, so we need to add this to
    # the engine. The scene object lets us easily swap out game scenes.
    # I.e., helpful for platformers with multiple levels and others
    engine.set_scene(scene)

    # Start the scene
    engine.begin()

if __name__ == "__main__":
    main()
```

## 🚀 Installation

**Requires Python 3.7+**

### Dependencies:

- SFML >= 2.5.0
- Pybind11 >= 2.10.0

### Install via pip:

```bash
pip install pyg-engine
```

### Or install from source:

```bash
git clone <repository-url>
cd pyg-engine
pip install -e .
```

## Documentation

<div class="callout info">
  Check out our comprehensive <a href="docs/CORE_SYSTEMS_GUIDE.html" class="btn">📖 Core Systems Guide</a> for detailed documentation on all features and systems.
</div>

## Examples

<div class="callout">
  The project includes numerous examples demonstrating various features:

  <ul>
    <li>🎮 Basic game setup and object management</li>
    <li>⚡ Physics and collision detection</li>
    <li>🖱️ Input handling (mouse, keyboard)</li>
    <li>📡 Event system usage</li>
    <li>📷 Camera and rendering features</li>
    <li>📊 Performance analysis tools</li>
  </ul>

  <a href="/examples/" class="btn btn-secondary">🚀 View All Examples</a>
</div>

## Contributing

<div class="callout">
  We welcome contributions! Please check out the <a href="https://github.com/aram-ap/pyg-engine" class="btn">🐙 GitHub Repository</a> for more information.
</div>

## License

<div class="callout info">
  This project is licensed under the MIT License - see the <a href="LICENSE" class="btn btn-secondary">📄 LICENSE</a> file for details.
</div>

---

<div style="text-align: center; margin-top: 40px; padding: 20px; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); border-radius: 12px; color: white;">
  <h3>🎮 Ready to build amazing games?</h3>
  <p>Start with PyG Engine today and bring your game ideas to life!</p>
  <!-- <a href="docs/CORE_SYSTEMS_GUIDE.html" class="btn">📖 Get Started</a> -->
  <a href="/examples/" class="btn btn-secondary">🚀 View Examples</a>
</div>
