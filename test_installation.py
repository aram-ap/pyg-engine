#!/usr/bin/env python3
"""
Test script to verify Pyg Engine installation and basic functionality
"""

import sys
import os

def test_imports():
    """Test that all core modules can be imported"""
    print("Testing imports...")
    
    try:
        from pyg_engine import (
            Engine, GameObject, Camera, 
            PhysicsSystem, RigidBody, Collider,
            Component, Script, Input,
            Size, BasicShape, Tag, PhysicsMaterial
        )
        print("✓ All core modules imported successfully")
        return True
    except ImportError as e:
        print(f"✗ Import error: {e}")
        return False

def test_basic_engine_creation():
    """Test that we can create a basic engine"""
    print("Testing engine creation...")
    
    try:
        from pyg_engine import Engine, Size
        from pygame import Color
        
        engine = Engine(
            size=Size(w=400, h=300),
            backgroundColor=Color(0, 0, 0),
            running=False
        )
        print("✓ Engine created successfully")
        return True
    except Exception as e:
        print(f"✗ Engine creation failed: {e}")
        return False

def test_gameobject_creation():
    """Test that we can create game objects"""
    print("Testing game object creation...")
    
    try:
        from pyg_engine import GameObject
        from pygame import Vector2, Color
        from pyg_engine import BasicShape, Tag
        
        obj = GameObject(
            name="TestObject",
            position=Vector2(100, 100),
            size=Vector2(50, 50),
            color=Color(255, 0, 0),
            tag=Tag.Other,
            basicShape=BasicShape.Rectangle
        )
        print("✓ Game object created successfully")
        return True
    except Exception as e:
        print(f"✗ Game object creation failed: {e}")
        return False

def main():
    """Main test function"""
    print("Pyg Engine Installation Test")
    print("=" * 40)
    
    tests = [
        test_imports,
        test_basic_engine_creation,
        test_gameobject_creation
    ]
    
    passed = 0
    total = len(tests)
    
    for test in tests:
        if test():
            passed += 1
        print()
    
    print("=" * 40)
    print(f"Tests passed: {passed}/{total}")
    
    if passed == total:
        print("✓ All tests passed! Pyg Engine is ready to use.")
        return 0
    else:
        print("✗ Some tests failed. Please check the installation.")
        return 1

if __name__ == "__main__":
    sys.exit(main()) 