---
layout: default
title: PyG Engine
---

<div class="hero-logo">
  <img src="{{ site.baseurl }}/images/1_lower-res.png" alt="PyG Engine Logo" style="max-width: 600px; height: auto; border-radius: 8px; border: 2px solid #30395C;">
</div>

# PyG Engine

A Python game engine built on Pygame and Pymunk for 2D physics, rendering, and game development. Built around scriptable game objects, rigidbody and collider system. Contains physics materials, update system, event system and mouse+keyboard input system

<div class="callout warning">
  <b>NOTE: This is in alpha development stage. Everything is under active development and large changes will likely be made.
  <i>Also,</i> its pronounced <i>pig </i>engine :)</b>
</div>

## Features

<div class="features-grid">
  <div class="feature-card">
    <h4>🎮 OOP Model</h4>
    <p>Simple game object implementation system</p>
  </div>
  
  <div class="feature-card">
    <h4>⚡ 2D Physics</h4>
    <p>Built-in physics via Pymunk</p>
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
    <p>Thread-safe event-driven communication with priority-based handling</p>
  </div>
  
  <div class="feature-card">
    <h4>📚 Documentation</h4>
    <p>Comprehensive CORE_SYSTEMS_GUIDE with examples and best practices</p>
  </div>
</div>

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

## 🚀 Installation

**Requires Python 3.7+**

### Dependencies:
- pygame >= 2.5.0
- pymunk >= 6.4.0

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
  <a href="docs/CORE_SYSTEMS_GUIDE.html" class="btn">📖 Get Started</a>
  <a href="/examples/" class="btn btn-secondary">🚀 View Examples</a>
</div> 