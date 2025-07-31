"""
Pyg Engine - A Python Game Engine

A comprehensive game engine built with Pygame and Pymunk for 2D physics,
rendering, and game development.
"""

__version__ = "1.0.0a4"
__author__ = "Pyg Engine Team"
__description__ = "A Python game engine with physics, rendering, and input systems"

# Core engine components
from .engine import Engine
from .engine import GlobalDictionary
from .gameobject import GameObject
from .camera import Camera
from .event_manager import EventManager
from .event import Event

# Runnable system
from .runnable import RunnableSystem, Priority, Runnable

# Physics system
from .physics_system import PhysicsSystem
from .rigidbody import RigidBody
from .collider import Collider
from .collider import Collider, BoxCollider, CircleCollider

# Input systems
from .input import Input

# Component system
from .component import Component
from .script import Script

# Utilities
from .object_types import Size, BasicShape, Tag
from .material import PhysicsMaterial, Materials

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
    'Collider',
    'BoxCollider',
    'CircleCollider',
    'Input',
    'Component',
    'Script',
    'Size',
    'BasicShape',
    'Tag',
    'PhysicsMaterial',
    'Materials'
]
