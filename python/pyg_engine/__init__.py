"""
<img src="https://github.com/aram-ap/pyg-engine/blob/main/images/1_zoomed.png?raw=true" width="500">

[pyg_engine](https://www.github.com/aram-ap/pyg-engine) - A Python game engine with Rust-powered native performance.
"""

from pyg_engine.engine import DrawCommand, Engine, EngineHandle, Input, UpdateContext, UIManager

try:
    from pyg_engine.pyg_engine_native import (
        Vec2,
        Vec3,
        Color,
        Time,
        GameObject,
        MeshComponent,
        TransformComponent,
        ButtonComponent,
        PanelComponent,
        LabelComponent,
        CameraAspectMode,
        MouseButton,
        Keys,
        PhysicsLayers,
        ColliderShape,
        Collider,
        version as _version_func,
    )
    # Expose version as a module-level attribute (from native binary)
    version = _version_func()  # type: ignore
except ImportError:
    # If native module isn't built yet, provide a helpful error message
    Vec2 = None  # type: ignore
    Vec3 = None  # type: ignore
    Color = None  # type: ignore
    Time = None  # type: ignore
    GameObject = None  # type: ignore
    MeshComponent = None  # type: ignore
    TransformComponent = None  # type: ignore
    ButtonComponent = None  # type: ignore
    PanelComponent = None  # type: ignore
    LabelComponent = None  # type: ignore
    CameraAspectMode = None  # type: ignore
    MouseButton = None  # type: ignore
    Keys = None  # type: ignore
    PhysicsLayers = None  # type: ignore
    ColliderShape = None  # type: ignore
    Collider = None  # type: ignore
    version = None  # type: ignore

# Auto-generated version from git tags via setuptools-scm
try:
    from pyg_engine._version import version as __version__
except ImportError:
    __version__ = "unknown"

__author__ = "Aram Aprahamian"
__description__ = "A Python game engine with Rust-powered native performance"

# Import UI wrappers
from pyg_engine.ui import Button, Panel, Label

__all__ = [
    "Engine",
    "EngineHandle",
    "DrawCommand",
    "Input",
    "UpdateContext",
    "Vec2",
    "Vec3",
    "Color",
    "Time",
    "GameObject",
    "UIManager",
    "MeshComponent",
    "TransformComponent",
    "ButtonComponent",
    "PanelComponent",
    "LabelComponent",
    "Button",
    "Panel",
    "Label",
    "CameraAspectMode",
    "MouseButton",
    "Keys",
    "PhysicsLayers",
    "ColliderShape",
    "Collider",
    "version",
]
