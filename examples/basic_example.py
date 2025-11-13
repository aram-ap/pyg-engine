"""
Basic example demonstrating the core functionality of Pyg Engine
"""

from pyg_engine import Engine, GameObject, Size
from pygame import Color, Vector2
from pyg_engine.object_types import BasicShape, Tag

def main():
    """Main function demonstrating basic engine usage"""
    
    # Create the engine
    engine = Engine(
        size=Size(w=800, h=600),
        backgroundColor=Color(50, 50, 50),  # Dark gray background
        windowName="Pyg Engine - Basic Example",
        running=False  # Don't start automatically
    )
    
    # Create a red square player
    player = GameObject(
        name="Player",
        position=Vector2(400, 300),
        size=Vector2(50, 50),
        color=Color(255, 0, 0),  # Red
        tag=Tag.Player,
        basicShape=BasicShape.Rectangle
    )
    
    # Create a blue circle obstacle
    obstacle = GameObject(
        name="Obstacle",
        position=Vector2(200, 200),
        size=Vector2(40, 40),
        color=Color(0, 0, 255),  # Blue
        tag=Tag.Environment,
        basicShape=BasicShape.Circle
    )
    
    # Create a green rectangle platform
    platform = GameObject(
        name="Platform",
        position=Vector2(600, 400),
        size=Vector2(100, 20),
        color=Color(0, 255, 0),  # Green
        tag=Tag.Environment,
        basicShape=BasicShape.Rectangle
    )
    
    # Add all game objects to the engine
    engine.addGameObject(player)
    engine.addGameObject(obstacle)
    engine.addGameObject(platform)
    
    print("Pyg Engine Basic Example")
    print("Controls:")
    print("  - ESC: Quit")
    print("  - P: Pause/Unpause")
    print("  - Mouse: Move objects")
    print("  - Close window to exit")
    
    # Start the engine
    engine.start()

if __name__ == "__main__":
    main() 