import pygame as pg
from pygame import Color, Vector2
from engine import Engine
from gameobject import BasicObject
from object_types import Size, BasicShape
from mouse_input import MouseHoverComponent, MouseClickComponent, MouseButton, get_mouse_position, get_mouse_world_position

class TestObject(BasicObject):
    """Simple test object to verify mouse input system."""
    
    def __init__(self, name, position, size, color):
        super().__init__(name=name, position=position, size=size, color=color)
        
        # Add mouse components
        self.add_component(MouseHoverComponent)
        self.add_component(MouseClickComponent)
        
        # Set up callbacks
        hover_comp = self.get_component(MouseHoverComponent)
        click_comp = self.get_component(MouseClickComponent)
        
        hover_comp.add_hover_callback(self._on_hover)
        click_comp.add_click_callback(MouseButton.LEFT, self._on_click)
        
        self.original_color = color
        self.hover_count = 0
        self.click_count = 0
    
    def _on_hover(self, event_type, mouse_pos, world_pos):
        if event_type == 'enter':
            self.hover_count += 1
            self.color = Color(255, 255, 0)  # Yellow
            print(f"{self.name}: Hover entered (count: {self.hover_count})")
        elif event_type == 'exit':
            self.color = self.original_color
            print(f"{self.name}: Hover exited")
    
    def _on_click(self, button, mouse_pos, world_pos):
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