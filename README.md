# Pyg-Engine

![Pyg-Engine Logo](images/1.png)

A Python game engine with a high-performance C++ backend using SFML.

## What is Pyg-Engine?

Pyg-Engine is a hybrid game engine designed to bridge the gap between Python's ease of use and C++'s raw performance.
It allows developers to write gameplay logic, define scenes, and manage game objects entirely in Python, while the heavy lifting—rendering, physics integration, and input handling—is executed by a compiled C++ core.

The engine is built around a Component-Entity-System (ECS) inspired architecture, making it modular and easy to extend. Whether you are building a simple 2D arcade game or a complex simulation, Pyg-Engine provides the tools to do so efficiently.

> Note: While pyg-engine is designed with a C++ backend, I can't guarentee that it will be fast nor efficient. However, if you care more about usability, pyg-engine is for you.

### Core Components

*   **Engine**: The central hub that manages the game loop, time, and system initialization.
*   **GameObject**: The base entity class. GameObjects are containers for Components and have a Transform (position, rotation, scale).
*   **Components**: Modular blocks of logic attached to GameObjects.
    *   **Sprite**: Handles 2D rendering of textures.
    *   **RigidBody**: Manages physics properties (mass, velocity).
    *   **Collider**: Defines physical shapes (Box, Circle) for collision detection.
    *   **Script**: Base class for user-defined gameplay logic.
*   **Scene System**: Manages active GameObjects and transitions between different game states (e.g., Menu, Gameplay).
*   **Input Manager**: Exposes keyboard and mouse events from the C++ backend to Python scripts.
*   **Debugger**: An integrated runtime inspector (powered by ImGui) that allows real-time visualization and modification of the game state.

## Architectural Overview

Pyg-Engine combines the ease of use of Python with the performance of C++.
*   **C++ Binding Layer**: Exposes C++ engine core to Python.
*   **C++ Core Engine**: Handles performance-critical systems like rendering, input, events, and time.
*   **SFML Backend**: Provides window management, graphics (OpenGL), audio, and system events.

## Technology Stack

| Layer | Technology | Purpose |
|-------|-----------|---------|
| **Graphics** | SFML Graphics | 2D rendering, sprites, textures, shaders, OpenGL context |
| **Window** | SFML Window | Window management, Input events |
| **Audio** | SFML Audio | Sound effects, music |
| **UI** | ImGui | Debugger and Editor UI |
| **Logging** | spdlog | Fast C++ logging library |
| **Bindings** | pybind11 | C++ to Python interface |

## Key Features

*   **Hybrid Architecture**: Write game logic in Python, engine does the heavy lifting in C++.
*   **SFML Powered**: Robust cross-platform support for Windows, Linux, and macOS.
*   **OpenGL Support**: Built-in OpenGL context management via SFML.
*   **Physics**: Integrated 2D physics using pymunk.
*   **Component System**: Flexible ECS-like architecture for GameObjects.

## Installation

### System Dependencies

Before installing, you need to install system dependencies for SFML.

**Quick Install (Recommended):**

Download and run the dependency installer script:
```bash
curl -O https://raw.githubusercontent.com/aram-ap/pyg-engine/cpp/install_deps.py
python install_deps.py
```

Or install manually:

**Ubuntu/Debian:**
```bash
sudo apt-get update
sudo apt-get install -y \
    libxcursor-dev \
    libxrandr-dev \
    libxinerama-dev \
    libxi-dev \
    libudev-dev \
    libgl1-mesa-dev \
    libflac-dev \
    libogg-dev \
    libvorbis-dev \
    libopenal-dev \
    libfreetype6-dev \
    cmake
```

**macOS:**
```bash
brew install sfml cmake pybind11
```

**Windows:**
- Install Visual Studio with C++ support
- Install CMake from https://cmake.org/download/

### Install from PyPI -- ONCE THIS IS RELEASED TO MAIN

```bash
pip install pyg-engine
```

### Install from GitHub

```bash
# Install from the latest stable release
pip install git+https://github.com/aram-ap/pyg-engine.git

# Install from a specific branch
pip install git+https://github.com/aram-ap/pyg-engine.git@cpp
```

### Install from Source

```bash
git clone https://github.com/aram-ap/pyg-engine.git
cd pyg-engine
pip install .
```

## Build System

The project uses a hybrid build system:
*   **CMake**: Manages C++ compilation and dependencies (SFML, pybind11).
*   **setuptools**: Handles Python packaging and extension building.

## Example Usage

```python
import pyg

class TestObject(GameObject):
    def __init__(self, engine: Engine):
        # You can create a game object like this
        super().__init__("TestObject", engine)
        self.position = Vector2(100, 100)
        self.size = Vector2(50, 50)
        self.color = Color(255, 0, 0)

    def start(self):
        # This function is called when the game object is created
        pyg.log("Started TestObject")


    def update(self, deltatime: DeltaTime):
        # This function is called every frame
        pass

    def fixed_update(self, deltatime: DeltaTime):
        # This function is called at a fixed rate (i.e., 60 times a second)
        # Helpful if you wanted to create your own physics implementation
        pass

    def on_destroyed(self):
        # This function is called when the game object is destroyed
        pyg.log("Destroyed test object")


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

    # Create a game object
    test_object = TestObject(engine)
    scene.add_gameobject(test_object)

    # The scene contains all our game information, so we need to add this to
    # the engine. The scene object lets us easily swap out game scenes.
    # I.e., helpful for platformers with multiple levels and others
    engine.set_scene(scene)

    # Start the scene
    engine.begin()

if __name__ == "__main__":
    main()
```
