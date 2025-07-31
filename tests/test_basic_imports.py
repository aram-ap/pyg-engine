"""
Basic import tests for Pyg Engine
"""

import pytest

def test_engine_import():
    """Test that the Engine class can be imported"""
    from pyg_engine import Engine
    assert Engine is not None

def test_gameobject_import():
    """Test that the GameObject class can be imported"""
    from pyg_engine import GameObject
    assert GameObject is not None

def test_camera_import():
    """Test that the Camera class can be imported"""
    from pyg_engine import Camera
    assert Camera is not None

def test_physics_system_import():
    """Test that physics systems can be imported"""
    from pyg_engine import PhysicsSystem
    assert PhysicsSystem is not None

def test_rigidbody_import():
    """Test that rigidbody classes can be imported"""
    from pyg_engine import RigidBody
    assert RigidBody is not None

def test_collider_import():
    """Test that collider classes can be imported"""
    from pyg_engine import Collider, Collider
    assert Collider is not None
    assert Collider is not None

def test_mouse_input_import():
    """Test that mouse input system can be imported"""
    from pyg_engine import MouseInputSystem
    assert MouseInputSystem is not None

def test_component_system_import():
    """Test that component system can be imported"""
    from pyg_engine import Component, Script, ScriptRunner
    assert Component is not None
    assert Script is not None
    assert ScriptRunner is not None

def test_utilities_import():
    """Test that utility classes can be imported"""
    from pyg_engine import Size, BasicShape, Tag, Material
    assert Size is not None
    assert BasicShape is not None
    assert Tag is not None
    assert Material is not None

def test_package_version():
    """Test that the package has a version"""
    from pyg_engine import __version__
    assert __version__ is not None
    assert isinstance(__version__, str)

def test_package_description():
    """Test that the package has a description"""
    from pyg_engine import __description__
    assert __description__ is not None
    assert isinstance(__description__, str) 