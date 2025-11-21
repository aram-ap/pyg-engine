# Pyg-Engine

![Pyg-Engine Logo](images/1.png)

A Python game engine with a high-performance C++ backend using SFML.

## What is Pyg-Engine?

Pyg-Engine is a hybrid game engine designed to bridge the gap between Python's ease of use and C++'s raw performance. It allows developers to write gameplay logic, define scenes, and manage game objects entirely in Python, while the heavy lifting—rendering, physics integration, and input handling—is executed by a compiled C++ core.

The engine is built around a Component-Entity-System (ECS) inspired architecture, making it modular and easy to extend. Whether you are building a simple 2D arcade game or a complex simulation, Pyg-Engine provides the tools to do so efficiently.

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
| **Physics** | pymunk | 2D physics simulation (wrapped) |
| **UI** | ImGui | Debugger and Editor UI |
| **Bindings** | pybind11 | C++ to Python interface |

## Key Features

*   **Hybrid Architecture**: Write game logic in Python, engine does the heavy lifting in C++.
*   **SFML Powered**: Robust cross-platform support for Windows, Linux, and macOS.
*   **OpenGL Support**: Built-in OpenGL context management via SFML.
*   **Physics**: Integrated 2D physics using pymunk.
*   **Component System**: Flexible ECS-like architecture for GameObjects.

## Installation

**Pypi**: 
```bash
pip install pyg-engine
```

You can install the engine via pip (requires CMake and a C++ compiler):

```bash
pip install .
```

## Build System

The project uses a hybrid build system:
*   **CMake**: Manages C++ compilation and dependencies (SFML, pybind11).
*   **setuptools**: Handles Python packaging and extension building.

## Example Usage

```python
import pyg

def main():
    engine = pyg.Engine()
    print(f"Pyg-Engine Version: {engine.version}")

if __name__ == "__main__":
    main()
```
