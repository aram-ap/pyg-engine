"""
Pyg Engine - A Python Game Engine

A comprehensive game engine built with Pygame and Pymunk for 2D physics,
rendering, and game development.
"""

from typing import TYPE_CHECKING

__version__ = "1.0.0a6"
__author__ = "Aram Aprahamian"
__description__ = "A Python game engine with physics, rendering, and input systems"

# Core engine components
from .core.engine import Engine
from .core.engine import GlobalDictionary
from .core.gameobject import GameObject
from .rendering.camera import Camera
from .events.event_manager import EventManager
from .events.event import Event

# Runnable system
from .core.runnable import RunnableSystem, Priority, Runnable

# Physics system
from .physics.physics_system import PhysicsSystem
from .physics.rigidbody import RigidBody
from .physics.collider import Collider, BoxCollider, CircleCollider

# Input systems
from .input.input import Input

# Component system
from .components.component import Component
from .components.script import Script

# Rendering system
from .rendering.sprite import Sprite, SpriteRenderer
from .rendering.animator import Animator, AnimationState
from .rendering.sprite_sheet import SpriteSheet, load_animation_frames

# Audio system
from .audio.audio_manager import AudioManager, audio_manager
from .audio.sound import Sound, SoundOneShot

# Utilities
from .utilities.color import Color, Colors
from .utilities.object_types import Size, BasicShape, Tag
from .physics.material import PhysicsMaterial, Materials
from .utilities.vector2 import Vector2

# Developer tools (optional PyQt dependency)
if TYPE_CHECKING:
    # For type checkers, provide a stub signature
    def start_dev_tool(engine: 'Engine', *, window_title: str = "Pyg Engine Dev Tool") -> None:
        """Launch the developer debug tool for real-time engine inspection."""
        ...
else:
    # At runtime, try to import the actual implementation
    _dev_tool_import_error = None
    try:
        import sys
        import os
        # Add parent directory to path for tools import
        # From src/pyg_engine/__init__.py, go up to project root, then to tools/
        _tools_path = os.path.join(os.path.dirname(__file__), '..', '..', 'tools')
        if _tools_path not in sys.path:
            sys.path.insert(0, _tools_path)
        from dev_tool import start_dev_tool
    except Exception as e:  # pragma: no cover - optional dependency (PyQt)
        _dev_tool_import_error = e
        
        def start_dev_tool(*_, **__):
            """Fallback when dev tool dependencies are missing."""
            raise RuntimeError(
                "Developer tool is unavailable. Install PyQt6 (e.g. pip install PyQt6 "
                "or pip install -e .[dev-tools]) to enable start_dev_tool()."
            ) from _dev_tool_import_error

# Main exports - these are the primary classes users will interact with
__all__ = [
    'Engine',
    'GlobalDictionary',
    'GameObject',
    'Camera',
    'Event',
    'EventManager',
    'RunnableSystem',
    'Priority',
    'Runnable',
    'PhysicsSystem',
    'RigidBody',
    'Collider',
    'BoxCollider',
    'CircleCollider',
    'Input',
    'Component',
    'Script',
    'Sprite',
    'SpriteRenderer',
    'Animator',
    'AnimationState',
    'SpriteSheet',
    'load_animation_frames',
    'AudioManager',
    'audio_manager',
    'Sound',
    'SoundOneShot',
    'Size',
    'BasicShape',
    'Tag',
    'PhysicsMaterial',
    'Materials',
    'Color',
    'Colors',
    'Vector2',
    'start_dev_tool'
]
