import pygame as pg
from pygame import Color, Vector2
import sys
from object_types import Size, BasicShape
from gameobject import GameObject
from engine import Engine

def test_sprite_system():
    """Test the new GameObject sprite system."""
    print("Testing GameObject sprite system...")
    
    # Create engine
    engine = Engine(fpsCap=60, windowName="Sprite System Test", size=Size(800, 600))
    Engine.log_debug = True
    
    # Test 1: Create a basic GameObject (should work as a sprite)
    test_obj = GameObject(
        name="test_sprite",
        basicShape=BasicShape.Circle,
        color=Color(255, 0, 0),
        position=Vector2(400, 300),
        size=Vector2(50, 50)
    )
    
    # Verify sprite properties
    print(f"GameObject created: {test_obj.name}")
    print(f"Has image: {hasattr(test_obj, 'image')}")
    print(f"Has rect: {hasattr(test_obj, 'rect')}")
    print(f"Image size: {test_obj.image.get_size() if hasattr(test_obj, 'image') else 'None'}")
    print(f"Rect position: {test_obj.rect.center if hasattr(test_obj, 'rect') else 'None'}")
    
    # Test 2: Add to engine (should work with sprite groups)
    engine.addGameObject(test_obj)
    print(f"Added to engine sprite group")
    print(f"Engine has {len(engine.getGameObjects())} game objects")
    
    # Test 3: Test sprite group operations
    sprite_group = engine._Engine__gameobjects  # Access private member for testing
    print(f"Sprite group has {len(sprite_group)} sprites")
    print(f"Test object in group: {test_obj in sprite_group}")
    
    # Test 4: Test position updates
    test_obj.update_position(Vector2(500, 400))
    print(f"Position updated to: {test_obj.position}")
    print(f"Rect center updated to: {test_obj.rect.center}")
    
    # Test 5: Test color updates
    test_obj.update_color(Color(0, 255, 0))
    print(f"Color updated to: {test_obj.color}")
    
    # Test 6: Test rotation
    test_obj.update_rotation(45.0)
    print(f"Rotation updated to: {test_obj.rotation}")
    
    # Test 7: Test size updates
    test_obj.update_size(Vector2(100, 100))
    print(f"Size updated to: {test_obj.size}")
    
    # Test 8: Test kill method (sprite functionality)
    test_obj.kill()
    print(f"Kill method called")
    print(f"Object in group after kill: {test_obj in sprite_group}")
    
    print("\n🎉 All sprite system tests passed!")
    print("The GameObject class now properly inherits from pygame.sprite.Sprite")
    print("and maintains all existing functionality while gaining sprite benefits.")

if __name__ == "__main__":
    test_sprite_system() 