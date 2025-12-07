# Pyg-Engine Architecture - SFML + C++ Backend with Python Bindings

## Table of Contents
1. [Architectural Overview](#architectural-overview)
2. [Technology Stack](#technology-stack)
3. [Project Structure](#project-structure)
4. [C++ Core Layer (SFML)](#c-core-layer-sfml)
5. [Python Binding Layer](#python-binding-layer)
6. [Python API Layer](#python-api-layer)
7. [Build System](#build-system)
8. [Development Priority for Flappy Bird](#development-priority-for-flappy-bird)
9. [Example Implementation](#example-implementation)

---

## Architectural Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    PYTHON GAME API                          │
│         (High-level, user-facing GameObject/Component)      │
│                      pyg/__init__.py                        │
└─────────────────────────┬───────────────────────────────────┘
                          │
┌─────────────────────────▼───────────────────────────────────┐
│                  PYTHON WRAPPER LAYER                       │
│            (GameObject, Component, Scene, etc.)             │
│                   pyg/core/*.py                             │
└─────────────────────────┬───────────────────────────────────┘
                          │
                          │ pybind11/Cython bindings
                          │
┌─────────────────────────▼───────────────────────────────────┐
│                   C++ BINDING LAYER                         │
│              (Expose C++ to Python)                         │
│                  src/bindings/*.cpp                         │
└─────────────────────────┬───────────────────────────────────┘
                          │
┌─────────────────────────▼───────────────────────────────────┐
│                    C++ CORE ENGINE                          │
│         (Performance-critical systems)                      │
│                   src/core/*.cpp                            │
│                                                             │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐   │
│  │Renderer  │  │Physics   │  │Input     │  │Audio     │   │
│  │(SFML)    │  │(pymunk)  │  │(SFML)    │  │(SFML)    │   │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘   │
└─────────────────────────┬───────────────────────────────────┘
                          │
┌─────────────────────────▼───────────────────────────────────┐
│                        SFML                                 │
│  System | Window | Graphics | Audio | Network              │
└─────────────────────────────────────────────────────────────┘
```

---

## Technology Stack

### Core Technologies

| Layer | Technology | Purpose |
|-------|-----------|---------|
| **Graphics** | SFML Graphics | 2D rendering, sprites, textures, shaders |
| **Window** | SFML Window | Window management, OpenGL context |
| **Audio** | SFML Audio | Sound effects, music, spatial audio |
| **Input** | SFML Window Events | Keyboard, mouse, joystick |
| **Physics** | pymunk (Python) | 2D physics simulation |
| **UI** | ImGui + ImGui-SFML | Editor and debug UI |
| **Bindings** | pybind11 | C++ ↔ Python interface |
| **Build** | CMake | Cross-platform C++ build |
| **Package** | setuptools + CMake | Python package with C++ extension |

### Why SFML?

✅ **Pros:**
- Clean, intuitive C++ API
- Built specifically for games and multimedia
- Excellent 2D support (sprites, textures, shaders)
- Built-in audio system
- Good performance
- Cross-platform (Windows, Linux, macOS)
- Active community

✅ **Better than SDL3 for:**
- Object-oriented design (vs SDL's C API)
- Built-in sprite batching
- Easier text rendering
- Better shader support

---

## Project Structure

```
pyg-engine/
│
├── src/                                  # C++ source code
│   ├── core/                            # Core C++ engine
│   │   ├── Engine.h / Engine.cpp
│   │   ├── Time.h / Time.cpp
│   │   └── Math.h / Math.cpp
│   │
│   ├── rendering/                       # SFML rendering wrapper
│   │   ├── Renderer.h / Renderer.cpp
│   │   ├── Texture.h / Texture.cpp
│   │   ├── Sprite.h / Sprite.cpp
│   │   ├── Camera.h / Camera.cpp
│   │   └── RenderWindow.h / RenderWindow.cpp
│   │
│   ├── input/                           # SFML input wrapper
│   │   ├── InputManager.h / InputManager.cpp
│   │   ├── Keyboard.h / Keyboard.cpp
│   │   └── Mouse.h / Mouse.cpp
│   │
│   ├── audio/                           # SFML audio wrapper
│   │   ├── AudioManager.h / AudioManager.cpp
│   │   ├── Sound.h / Sound.cpp
│   │   └── Music.h / Music.cpp
│   │
│   ├── resources/                       # Resource management
│   │   ├── ResourceManager.h / ResourceManager.cpp
│   │   └── TextureCache.h / TextureCache.cpp
│   │
│   └── bindings/                        # pybind11 bindings
│       ├── core_bindings.cpp
│       ├── rendering_bindings.cpp
│       ├── input_bindings.cpp
│       └── audio_bindings.cpp
│
├── pyg/                                 # Python wrapper/API
│   ├── __init__.py                     # Public API exports
│   ├── _native.so / _native.pyd        # Compiled C++ module
│   │
│   ├── core/                           # Python GameObject system
│   │   ├── engine.py
│   │   ├── game_object.py
│   │   ├── component.py
│   │   ├── transform.py
│   │   └── time.py
│   │
│   ├── rendering/                      # Python rendering wrappers
│   │   ├── sprite.py                  # Component wrapping C++ Sprite
│   │   ├── texture.py
│   │   └── camera.py
│   │
│   ├── physics/                        # Python physics (pymunk)
│   │   ├── physics_world.py
│   │   ├── rigidbody.py
│   │   └── collider.py
│   │
│   ├── scene/
│   │   ├── scene.py
│   │   └── scene_manager.py
│   │
│   ├── utils/
│   │   ├── events.py
│   │   ├── coroutine.py
│   │   └── math.py                    # Python Vector2 (wraps C++)
│   │
│   └── input/
│       └── input_manager.py           # Python wrapper for C++ input
│
├── include/                            # Public C++ headers
│   └── pyg/
│       ├── Engine.h
│       ├── Rendering.h
│       └── Input.h
│
├── external/                           # Third-party libraries
│   ├── SFML/
│   ├── pybind11/
│   ├── imgui/
│   └── imgui-sfml/
│
├── examples/                           # Example games
│   └── flappy_bird/
│       ├── main.py
│       ├── bird.py
│       └── pipe.py
│
├── tests/
│   ├── cpp/                           # C++ unit tests
│   └── python/                        # Python tests
│
├── CMakeLists.txt                     # C++ build configuration
├── setup.py                           # Python package build
├── pyproject.toml                     # Python project metadata
└── README.md
```

---

## C++ Core Layer (SFML)

### 1. Rendering System (C++)

```cpp
// src/rendering/Renderer.h
#pragma once
#include <SFML/Graphics.hpp>
#include <memory>

namespace pyg {

class Renderer {
public:
    Renderer(const std::string& title, unsigned int width, unsigned int height);
    ~Renderer();

    // Window management
    void clear(const sf::Color& color = sf::Color::Black);
    void present();
    bool isOpen() const;
    void close();

    // Drawing methods
    void drawSprite(const sf::Sprite& sprite);
    void drawTexture(const sf::Texture& texture, const sf::Vector2f& position,
                     float rotation = 0.0f, const sf::Vector2f& scale = {1.0f, 1.0f});

    // Camera/View
    void setView(const sf::View& view);
    sf::View getView() const;

    // Get native SFML window
    sf::RenderWindow& getWindow() { return m_window; }

private:
    sf::RenderWindow m_window;
};

} // namespace pyg
```

```cpp
// src/rendering/Renderer.cpp
#include "Renderer.h"

namespace pyg {

Renderer::Renderer(const std::string& title, unsigned int width, unsigned int height)
    : m_window(sf::VideoMode(width, height), title) {
    m_window.setVerticalSyncEnabled(true);
}

Renderer::~Renderer() {
    if (m_window.isOpen()) {
        m_window.close();
    }
}

void Renderer::clear(const sf::Color& color) {
    m_window.clear(color);
}

void Renderer::present() {
    m_window.display();
}

bool Renderer::isOpen() const {
    return m_window.isOpen();
}

void Renderer::close() {
    m_window.close();
}

void Renderer::drawSprite(const sf::Sprite& sprite) {
    m_window.draw(sprite);
}

void Renderer::drawTexture(const sf::Texture& texture, const sf::Vector2f& position,
                          float rotation, const sf::Vector2f& scale) {
    sf::Sprite sprite(texture);
    sprite.setPosition(position);
    sprite.setRotation(rotation);
    sprite.setScale(scale);
    m_window.draw(sprite);
}

void Renderer::setView(const sf::View& view) {
    m_window.setView(view);
}

sf::View Renderer::getView() const {
    return m_window.getView();
}

} // namespace pyg
```

### 2. Texture Management (C++)

```cpp
// src/rendering/Texture.h
#pragma once
#include <SFML/Graphics.hpp>
#include <string>
#include <memory>

namespace pyg {

class Texture {
public:
    Texture() = default;
    explicit Texture(const std::string& filepath);

    bool loadFromFile(const std::string& filepath);

    unsigned int getWidth() const { return m_texture.getSize().x; }
    unsigned int getHeight() const { return m_texture.getSize().y; }

    const sf::Texture& getSFMLTexture() const { return m_texture; }
    sf::Texture& getSFMLTexture() { return m_texture; }

private:
    sf::Texture m_texture;
    std::string m_filepath;
};

} // namespace pyg
```

### 3. Input System (C++)

```cpp
// src/input/InputManager.h
#pragma once
#include <SFML/Window.hpp>
#include <unordered_map>
#include <unordered_set>

namespace pyg {

class InputManager {
public:
    InputManager() = default;

    void processEvents(sf::RenderWindow& window);

    // Keyboard
    bool isKeyPressed(sf::Keyboard::Key key) const;
    bool isKeyDown(sf::Keyboard::Key key) const;  // Pressed this frame
    bool isKeyUp(sf::Keyboard::Key key) const;    // Released this frame

    // Mouse
    bool isMouseButtonPressed(sf::Mouse::Button button) const;
    bool isMouseButtonDown(sf::Mouse::Button button) const;
    bool isMouseButtonUp(sf::Mouse::Button button) const;
    sf::Vector2i getMousePosition() const;

    // Window
    bool isQuitRequested() const { return m_quitRequested; }

    void update();  // Call at end of frame to clear "this frame" states

private:
    std::unordered_set<sf::Keyboard::Key> m_keysPressed;
    std::unordered_set<sf::Keyboard::Key> m_keysDown;
    std::unordered_set<sf::Keyboard::Key> m_keysUp;

    std::unordered_set<sf::Mouse::Button> m_mousePressed;
    std::unordered_set<sf::Mouse::Button> m_mouseDown;
    std::unordered_set<sf::Mouse::Button> m_mouseUp;

    sf::Vector2i m_mousePosition;
    bool m_quitRequested = false;
};

} // namespace pyg
```

### 4. Time System (C++)

```cpp
// src/core/Time.h
#pragma once
#include <SFML/System.hpp>

namespace pyg {

class Time {
public:
    Time();

    void tick();

    float getDeltaTime() const { return m_deltaTime; }
    float getElapsedTime() const { return m_clock.getElapsedTime().asSeconds(); }

    void setTimeScale(float scale) { m_timeScale = scale; }
    float getTimeScale() const { return m_timeScale; }

private:
    sf::Clock m_clock;
    sf::Time m_lastTime;
    float m_deltaTime;
    float m_timeScale;
};

} // namespace pyg
```

### 5. Audio System (C++)

```cpp
// src/audio/AudioManager.h
#pragma once
#include <SFML/Audio.hpp>
#include <unordered_map>
#include <memory>
#include <string>

namespace pyg {

class AudioManager {
public:
    AudioManager() = default;

    // Sound effects (short, loaded into memory)
    void loadSound(const std::string& name, const std::string& filepath);
    void playSound(const std::string& name, float volume = 100.0f);
    void stopSound(const std::string& name);

    // Music (streamed from disk)
    void loadMusic(const std::string& name, const std::string& filepath);
    void playMusic(const std::string& name, bool loop = true, float volume = 100.0f);
    void stopMusic();
    void pauseMusic();

    // Global volume
    void setMasterVolume(float volume);

private:
    std::unordered_map<std::string, sf::SoundBuffer> m_soundBuffers;
    std::unordered_map<std::string, std::unique_ptr<sf::Sound>> m_sounds;
    std::unique_ptr<sf::Music> m_currentMusic;

    float m_masterVolume = 100.0f;
};

} // namespace pyg
```

---

## Python Binding Layer

### Using pybind11

```cpp
// src/bindings/rendering_bindings.cpp
#include <pybind11/pybind11.h>
#include <pybind11/stl.h>
#include "rendering/Renderer.h"
#include "rendering/Texture.h"

namespace py = pybind11;

void bind_rendering(py::module& m) {
    // Renderer
    py::class_<pyg::Renderer>(m, "Renderer")
        .def(py::init<const std::string&, unsigned int, unsigned int>(),
             py::arg("title"), py::arg("width"), py::arg("height"))
        .def("clear", &pyg::Renderer::clear, py::arg("color") = sf::Color::Black)
        .def("present", &pyg::Renderer::present)
        .def("is_open", &pyg::Renderer::isOpen)
        .def("close", &pyg::Renderer::close)
        .def("draw_sprite", &pyg::Renderer::drawSprite);

    // Texture
    py::class_<pyg::Texture>(m, "Texture")
        .def(py::init<>())
        .def(py::init<const std::string&>())
        .def("load_from_file", &pyg::Texture::loadFromFile)
        .def("get_width", &pyg::Texture::getWidth)
        .def("get_height", &pyg::Texture::getHeight);

    // SFML types
    py::class_<sf::Color>(m, "Color")
        .def(py::init<uint8_t, uint8_t, uint8_t, uint8_t>(),
             py::arg("r"), py::arg("g"), py::arg("b"), py::arg("a") = 255)
        .def_readwrite("r", &sf::Color::r)
        .def_readwrite("g", &sf::Color::g)
        .def_readwrite("b", &sf::Color::b)
        .def_readwrite("a", &sf::Color::a);

    py::class_<sf::Vector2f>(m, "Vector2f")
        .def(py::init<float, float>())
        .def_readwrite("x", &sf::Vector2f::x)
        .def_readwrite("y", &sf::Vector2f::y);

    py::class_<sf::Sprite>(m, "Sprite")
        .def(py::init<>())
        .def(py::init<const sf::Texture&>())
        .def("set_position", [](sf::Sprite& sprite, float x, float y) {
            sprite.setPosition(x, y);
        })
        .def("set_rotation", &sf::Sprite::setRotation)
        .def("set_scale", [](sf::Sprite& sprite, float x, float y) {
            sprite.setScale(x, y);
        })
        .def("get_position", &sf::Sprite::getPosition)
        .def("set_texture", [](sf::Sprite& sprite, pyg::Texture& tex) {
            sprite.setTexture(tex.getSFMLTexture());
        });
}
```

```cpp
// src/bindings/input_bindings.cpp
#include <pybind11/pybind11.h>
#include "input/InputManager.h"

namespace py = pybind11;

void bind_input(py::module& m) {
    py::class_<pyg::InputManager>(m, "InputManager")
        .def(py::init<>())
        .def("is_key_pressed", &pyg::InputManager::isKeyPressed)
        .def("is_key_down", &pyg::InputManager::isKeyDown)
        .def("is_key_up", &pyg::InputManager::isKeyUp)
        .def("is_mouse_button_pressed", &pyg::InputManager::isMouseButtonPressed)
        .def("is_mouse_button_down", &pyg::InputManager::isMouseButtonDown)
        .def("get_mouse_position", &pyg::InputManager::getMousePosition)
        .def("is_quit_requested", &pyg::InputManager::isQuitRequested)
        .def("update", &pyg::InputManager::update);

    // KeyCode enum
    py::enum_<sf::Keyboard::Key>(m, "KeyCode")
        .value("A", sf::Keyboard::A)
        .value("B", sf::Keyboard::B)
        .value("C", sf::Keyboard::C)
        .value("D", sf::Keyboard::D)
        .value("W", sf::Keyboard::W)
        .value("S", sf::Keyboard::S)
        .value("SPACE", sf::Keyboard::Space)
        .value("ESCAPE", sf::Keyboard::Escape)
        .value("ENTER", sf::Keyboard::Enter)
        .value("LEFT", sf::Keyboard::Left)
        .value("RIGHT", sf::Keyboard::Right)
        .value("UP", sf::Keyboard::Up)
        .value("DOWN", sf::Keyboard::Down);

    py::enum_<sf::Mouse::Button>(m, "MouseButton")
        .value("LEFT", sf::Mouse::Left)
        .value("RIGHT", sf::Mouse::Right)
        .value("MIDDLE", sf::Mouse::Middle);
}
```

```cpp
// src/bindings/module.cpp
#include <pybind11/pybind11.h>

namespace py = pybind11;

void bind_rendering(py::module& m);
void bind_input(py::module& m);
void bind_audio(py::module& m);
void bind_core(py::module& m);

PYBIND11_MODULE(_native, m) {
    m.doc() = "Pyg-Engine C++ native module (SFML backend)";

    bind_core(m);
    bind_rendering(m);
    bind_input(m);
    bind_audio(m);
}
```

---

## Python API Layer

### High-Level Python GameObject System

```python
# pyg/core/component.py
from abc import ABC
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from .game_object import GameObject

class Component(ABC):
    """Base component class"""

    def __init__(self, game_object: 'GameObject'):
        self.game_object = game_object
        self.transform = game_object.transform
        self.enabled = True
        self._started = False

    def start(self):
        """Called before first update"""
        pass

    def update(self, delta_time: float):
        """Called every frame"""
        pass

    def on_destroy(self):
        """Called when component is destroyed"""
        pass

    def get_component(self, component_type):
        """Get component of type on same GameObject"""
        return self.game_object.get_component(component_type)
```

```python
# pyg/rendering/sprite.py
from pyg.core.component import Component
from pyg import _native  # C++ module
from typing import Optional

class Sprite(Component):
    """Sprite component - wraps C++ sf::Sprite"""

    def __init__(self, game_object):
        super().__init__(game_object)
        self._native_sprite = _native.Sprite()
        self.texture: Optional[_native.Texture] = None
        self.color = _native.Color(255, 255, 255, 255)
        self.visible = True

    def set_texture(self, texture):
        """Set the texture for this sprite"""
        self.texture = texture
        self._native_sprite.set_texture(texture)

    def render(self, renderer):
        """Render the sprite (called by engine)"""
        if not self.visible or not self.texture:
            return

        # Update sprite position from transform
        pos = self.transform.position
        self._native_sprite.set_position(pos.x, pos.y)
        self._native_sprite.set_rotation(self.transform.rotation)

        # Draw using C++ renderer
        renderer.draw_sprite(self._native_sprite)
```

```python
# pyg/core/engine.py
from pyg import _native
from .time import Time
from ..input.input_manager import InputManager
from ..scene.scene_manager import SceneManager

class Engine:
    """Main engine class"""

    _instance = None

    def __new__(cls):
        if cls._instance is None:
            cls._instance = super().__new__(cls)
        return cls._instance

    def __init__(self):
        if hasattr(self, '_initialized'):
            return
        self._initialized = True

        self.running = False
        self.target_fps = 60

        # C++ systems
        self.renderer: Optional[_native.Renderer] = None
        self._native_input: Optional[_native.InputManager] = None

        # Python systems
        self.time = Time()
        self.input_manager = InputManager()
        self.scene_manager = SceneManager()

    def initialize(self, title: str = "Pyg-Engine", width: int = 800, height: int = 600):
        """Initialize engine and create window"""
        # Create C++ renderer (SFML window)
        self.renderer = _native.Renderer(title, width, height)
        self._native_input = _native.InputManager()

        print(f"Engine initialized: {title} ({width}x{height})")
        print(f"Using SFML backend")

    def run(self):
        """Main game loop"""
        self.running = True

        while self.running and self.renderer.is_open():
            # Tick time
            self.time.tick()
            delta_time = self.time.delta_time

            # Process input (C++)
            # Note: Need to pass SFML window reference
            # self._native_input.process_events(self.renderer.get_window())

            # Check for quit
            if self._native_input.is_quit_requested():
                self.running = False
                break

            # Update input state
            self.input_manager.update()

            # Update scene
            if self.scene_manager.current_scene:
                self.scene_manager.current_scene._update(delta_time)

            # Render
            self.renderer.clear(_native.Color(100, 149, 237))  # Cornflower blue

            if self.scene_manager.current_scene:
                self.scene_manager.current_scene.render(self.renderer)

            self.renderer.present()

            # Update input for next frame
            self._native_input.update()

    def quit(self):
        """Shutdown engine"""
        self.running = False
        if self.renderer:
            self.renderer.close()
```

---

## Build System

### CMakeLists.txt

```cmake
cmake_minimum_required(VERSION 3.15)
project(pyg_engine CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Find SFML
find_package(SFML 2.5 COMPONENTS system window graphics audio REQUIRED)

# Find Python
find_package(Python COMPONENTS Interpreter Development REQUIRED)

# Add pybind11
add_subdirectory(external/pybind11)

# Source files
set(CORE_SOURCES
    src/core/Time.cpp
    src/core/Math.cpp
)

set(RENDERING_SOURCES
    src/rendering/Renderer.cpp
    src/rendering/Texture.cpp
)

set(INPUT_SOURCES
    src/input/InputManager.cpp
)

set(AUDIO_SOURCES
    src/audio/AudioManager.cpp
)

set(BINDING_SOURCES
    src/bindings/module.cpp
    src/bindings/core_bindings.cpp
    src/bindings/rendering_bindings.cpp
    src/bindings/input_bindings.cpp
    src/bindings/audio_bindings.cpp
)

# Create Python module
pybind11_add_module(_native
    ${CORE_SOURCES}
    ${RENDERING_SOURCES}
    ${INPUT_SOURCES}
    ${AUDIO_SOURCES}
    ${BINDING_SOURCES}
)

# Link SFML
target_link_libraries(_native PRIVATE
    sfml-system
    sfml-window
    sfml-graphics
    sfml-audio
)

# Include directories
target_include_directories(_native PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src
    ${CMAKE_CURRENT_SOURCE_DIR}/include
)

# Install
install(TARGETS _native DESTINATION pyg)
```

### setup.py

```python
# setup.py
from setuptools import setup, Extension
from setuptools.command.build_ext import build_ext
import sys
import os
import subprocess

class CMakeExtension(Extension):
    def __init__(self, name, sourcedir=''):
        Extension.__init__(self, name, sources=[])
        self.sourcedir = os.path.abspath(sourcedir)

class CMakeBuild(build_ext):
    def run(self):
        for ext in self.extensions:
            self.build_extension(ext)

    def build_extension(self, ext):
        extdir = os.path.abspath(os.path.dirname(self.get_ext_fullpath(ext.name)))

        cmake_args = [
            f'-DCMAKE_LIBRARY_OUTPUT_DIRECTORY={extdir}',
            f'-DPYTHON_EXECUTABLE={sys.executable}',
        ]

        build_args = ['--config', 'Release']

        if not os.path.exists(self.build_temp):
            os.makedirs(self.build_temp)

        subprocess.check_call(['cmake', ext.sourcedir] + cmake_args, cwd=self.build_temp)
        subprocess.check_call(['cmake', '--build', '.'] + build_args, cwd=self.build_temp)

setup(
    name='pyg-engine',
    version='0.1.0',
    author='Your Name',
    description='A Python game engine with C++/SFML backend',
    long_description='',
    packages=['pyg', 'pyg.core', 'pyg.rendering', 'pyg.physics', 'pyg.scene'],
    ext_modules=[CMakeExtension('pyg._native')],
    cmdclass={'build_ext': CMakeBuild},
    install_requires=[
        'pymunk>=6.0.0',
    ],
    python_requires='>=3.8',
    zip_safe=False,
)
```

### pyproject.toml

```toml
[build-system]
requires = ["setuptools>=61.0", "wheel", "cmake>=3.15", "pybind11>=2.10.0"]
build-backend = "setuptools.build_meta"

[project]
name = "pyg-engine"
version = "0.1.0"
description = "A Python game engine with C++/SFML backend"
requires-python = ">=3.8"
dependencies = [
    "pymunk>=6.0.0",
]

[project.optional-dependencies]
dev = [
    "pytest>=7.0",
    "black>=22.0",
    "mypy>=0.950",
]
```

---

## Development Priority for Flappy Bird

### Phase 1: C++ Core (Week 1-2)

```
1. ✓ CMake setup
2. ✓ SFML integration
3. ✓ C++ Time class
4. ✓ C++ Renderer (SFML wrapper)
5. ✓ C++ Texture loading
6. ✓ C++ InputManager
7. ✓ pybind11 bindings for above
```

**Files to create:**
```
src/core/Time.h/.cpp
src/rendering/Renderer.h/.cpp
src/rendering/Texture.h/.cpp
src/input/InputManager.h/.cpp
src/bindings/module.cpp
src/bindings/rendering_bindings.cpp
src/bindings/input_bindings.cpp
CMakeLists.txt
```

### Phase 2: Python Wrapper Layer (Week 2-3)

```
8. ✓ Python Engine class
9. ✓ Python GameObject
10. ✓ Python Component
11. ✓ Python Transform
12. ✓ Python Sprite (wraps C++ sprite)
13. ✓ Python Scene system
```

**Files to create:**
```
pyg/core/engine.py
pyg/core/game_object.py
pyg/core/component.py
pyg/core/transform.py
pyg/rendering/sprite.py
pyg/scene/scene.py
pyg/scene/scene_manager.py
```

### Phase 3: Physics (Python - Week 3)

```
14. ✓ pymunk integration (pure Python, no C++)
15. ✓ RigidBody component
16. ✓ Collider components
17. ✓ Physics callbacks
```

### Phase 4: Flappy Bird Game (Week 4)

```
18. ✓ Bird component
19. ✓ Pipe spawner
20. ✓ Game manager
21. ✓ Score system
```

---

## Example Implementation

### Flappy Bird Main (Python)

```python
# examples/flappy_bird/main.py
import pyg
from pyg import Engine, GameObject, Scene, Vector2
from bird import Bird
from pipe_spawner import PipeSpawner

def main():
    # Initialize engine
    engine = Engine()
    engine.initialize("Flappy Bird", 400, 600)

    # Create scene
    scene = Scene("Game")

    # Create bird
    bird = GameObject("Bird")
    bird.transform.position = Vector2(100, 300)

    # Add sprite
    sprite = bird.add_component(pyg.Sprite)
    texture = engine.resource_manager.load(pyg.Texture, "assets/bird.png")
    sprite.set_texture(texture)

    # Add physics
    rb = bird.add_component(pyg.RigidBody)
    rb.use_gravity = True

    collider = bird.add_component(pyg.BoxCollider)
    collider.size = Vector2(34, 24)

    # Add game logic
    bird.add_component(Bird)

    scene.add(bird)

    # Create pipe spawner
    spawner = GameObject("PipeSpawner")
    spawner.add_component(PipeSpawner)
    scene.add(spawner)

    # Load scene and run
    engine.scene_manager.load_scene(scene)
    engine.run()

if __name__ == "__main__":
    main()
```

### Bird Component (Python)

```python
# examples/flappy_bird/bird.py
import pyg
from pyg import Component, Vector2, KeyCode

class Bird(Component):
    def __init__(self, game_object):
        super().__init__(game_object)
        self.jump_force = 300.0
        self.rigidbody = None
        self.alive = True

    def start(self):
        self.rigidbody = self.get_component(pyg.RigidBody)

    def update(self, delta_time):
        if not self.alive:
            return

        # Jump on spacebar
        if pyg.Input.get_key_down(KeyCode.SPACE):
            self.rigidbody.velocity = Vector2(0, self.jump_force)

        # Rotate based on velocity
        if self.rigidbody:
            rotation = -self.rigidbody.velocity.y * 0.05
            self.transform.rotation = max(-90, min(45, rotation))

    def on_collision_enter(self, collision):
        if collision.other.game_object.tag in ["Pipe", "Ground"]:
            self.die()

    def die(self):
        self.alive = False
        print("Game Over!")
```

---

## Summary

### Technology Stack Decision

| System | Technology | Layer |
|--------|-----------|-------|
| **Rendering** | SFML Graphics | C++ |
| **Window** | SFML Window | C++ |
| **Audio** | SFML Audio | C++ |
| **Input** | SFML Events | C++ |
| **Physics** | pymunk | Python |
| **Game Logic** | GameObject/Component | Python |
| **Bindings** | pybind11 | C++/Python |

### Advantages of this Architecture

✅ **Performance**: C++ handles rendering, input, audio (performance-critical)
✅ **Ease of Use**: Python for game logic, components, gameplay
✅ **SFML Benefits**: Clean API, good 2D support, cross-platform
✅ **Flexibility**: Easy to add features to either layer
✅ **Distribution**: Single Python package with compiled C++ extension

### Build Process

```bash
# Development build
python setup.py build_ext --inplace

# Install locally
pip install -e .

# Build wheel for distribution
python setup.py bdist_wheel
```

### Next Steps

1. Set up CMake project with SFML
2. Create C++ rendering and input wrappers
3. Add pybind11 bindings
4. Build Python GameObject/Component system
5. Test with simple example
6. Build Flappy Bird clone
7. Add audio, polish, documentation

This hybrid approach gives you the best of both worlds: C++ performance with Python productivity!


