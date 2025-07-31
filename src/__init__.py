"""
Pyg Engine - A Python Game Engine

A comprehensive game engine built with Pygame and Pymunk for 2D physics,
rendering, and game development.
"""

__version__ = "1.0.0a4"
__author__ = "Pyg Engine Team"
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

# Utilities
from .utilities.object_types import Size, BasicShape, Tag
from .physics.material import PhysicsMaterial, Materials

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
    'Size',
    'BasicShape',
    'Tag',
    'PhysicsMaterial',
    'Materials'
]
