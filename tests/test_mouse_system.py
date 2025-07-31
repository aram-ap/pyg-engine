import pygame as pg
from pygame import Color, Vector2
from engine import Engine
from gameobject import GameObject
from object_types import Size, BasicShape
from input import Input

class TestObject(GameObject):
    """Simple test object to verify modern mouse input system."""
    
    def __init__(self, name, position, size, color):
        super().__init__(name=name, position=position, size=size, color=color)
        
        self.original_color = color
        self.hover_count = 0
        self.click_count = 0
        self.is_hovering = False
    
    def update(self, engine):
        """Update method to handle mouse interactions using modern input system."""
        super().update(engine)
        
        # Get mouse position
        mouse_pos = engine.input.mouse.get_pos()
        world_pos = engine.camera.screen_to_world(mouse_pos)
        
        # Check if mouse is over this object
        obj_rect = pg.Rect(
            self.position.x - self.size.x/2,
            self.position.y - self.size.y/2,
            self.size.x,
            self.size.y
        )
        
        was_hovering = self.is_hovering
        self.is_hovering = obj_rect.collidepoint(world_pos.x, world_pos.y)
        
        # Handle hover events
        if self.is_hovering and not was_hovering:
            self.hover_count += 1
            self.color = Color(255, 255, 0)  # Yellow
            print(f"{self.name}: Hover entered (count: {self.hover_count})")
        elif not self.is_hovering and was_hovering:
            self.color = self.original_color
            print(f"{self.name}: Hover exited")
        
        # Handle click events
        if engine.input.get_event_state('mouse_button_down', 0) and self.is_hovering:
            self.click_count += 1
            print(f"{self.name}: Clicked (count: {self.click_count}) at {mouse_pos}")

def test_mouse_system():
    """Test the mouse input system with basic functionality."""
    print("Testing Mouse Input System...")
    
    # Create engine
    engine = Engine(
        size=Size(w=800, h=600),
        backgroundColor=Color(50, 50, 50),
        windowName="Mouse System Test",
        fpsCap=60
    )
    
    # Create test objects
    test_objects = [
        TestObject("Test1", Vector2(200, 200), Vector2(100, 100), Color(255, 0, 0)),
        TestObject("Test2", Vector2(400, 200), Vector2(100, 100), Color(0, 255, 0)),
        TestObject("Test3", Vector2(600, 200), Vector2(100, 100), Color(0, 0, 255)),
    ]
    
    # Add objects to engine
    for obj in test_objects:
        engine.addGameObject(obj)
    
    print("Test objects created. Try hovering and clicking on them.")
    print("Press ESC to pause/unpause, close window to exit.")
    
    # Start the engine
    engine.start()

if __name__ == "__main__":
    test_mouse_system() 