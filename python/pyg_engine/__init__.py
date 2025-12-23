"""
pyg_engine - A Python game engine with Rust-powered native performance.
"""

from pyg_engine.engine import Engine

try:
    from pyg_engine.pyg_engine_native import (
        Vec2,
        Vec3,
        Color,
        Time,
        GameObject,
        TransformComponent,
    )
except ImportError:
    # If native module isn't built yet, provide a helpful error message
    Vec2 = None  # type: ignore
    Vec3 = None  # type: ignore
    Color = None  # type: ignore
    Time = None  # type: ignore
    GameObject = None  # type: ignore
    TransformComponent = None  # type: ignore

__version__ = "1.2.0"
__author__ = "Aram Aprahamian"
__description__ = "A Python game engine with Rust-powered native performance"

__all__ = [
    "Engine",
    "Vec2",
    "Vec3",
    "Color",
    "Time",
    "GameObject",
    "TransformComponent",
]
