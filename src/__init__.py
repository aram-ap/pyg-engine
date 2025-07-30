"""
Pyg Engine - A Python Game Engine

A comprehensive game engine built with Pygame and Pymunk for 2D physics,
rendering, and game development.
"""

__version__ = "1.0.0a2"
__author__ = "Pyg Engine Team"
__description__ = "A Python game engine with physics, rendering, and input systems"

# Core engine components
from .engine import Engine
from .gameobject import GameObject
from .camera import Camera

# Physics system
from .physics_system import PhysicsSystem
from .pymunk_physics_system import PymunkPhysicsSystem
from .rigidbody import RigidBody
from .pymunk_rigidbody import PymunkRigidBody
from .collider import Collider
from .pymunk_collider import PymunkCollider, PymunkBoxCollider, PymunkCircleCollider
from .collision_detector import CollisionDetectorScript

# Input systems
from .mouse_input import MouseInputSystem, MouseHoverComponent, MouseClickComponent, MouseWheelComponent, MouseButton

# Component system
from .component import Component
from .script import Script
from .scriptrunner import ScriptRunner

# Utilities
from .object_types import Size, BasicShape, Tag
from .material import PhysicsMaterial, Materials

# Main exports - these are the primary classes users will interact with
__all__ = [
    'Engine',
    'GameObject', 
    'Camera',
    'PhysicsSystem',
    'PymunkPhysicsSystem',
    'RigidBody',
    'PymunkRigidBody',
    'Collider',
    'PymunkCollider',
    'PymunkBoxCollider',
    'PymunkCircleCollider',
    'CollisionDetectorScript',
    'MouseInputSystem',
    'MouseHoverComponent',
    'MouseClickComponent',
    'MouseWheelComponent',
    'MouseButton',
    'Component',
    'Script',
    'ScriptRunner',
    'Size',
    'BasicShape',
    'Tag',
    'PhysicsMaterial',
    'Materials'
] 