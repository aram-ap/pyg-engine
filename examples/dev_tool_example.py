"""
Example demonstrating the Developer Debug Tool.

Run this example and the debug tool window will open alongside the game.
You can inspect GameObjects, edit properties, view stats, and serialize the scene.
"""

import sys
import os

# Add parent directory to path
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..'))

from src.core.engine import Engine
from src.core.gameobject import GameObject
from src.utilities.object_types import Size, Tag, BasicShape
from src.utilities.color import Color
from src.utilities.vector2 import Vector2
from src.physics.rigidbody import RigidBody
from src.physics.collider import CircleCollider
from tools.dev_tool import start_dev_tool


def main():
    # Create engine
    engine = Engine(
        size=Size(w=800, h=600),
        backgroundColor=Color(50, 50, 50),
        windowName="Dev Tool Example",
        fpsCap=60
    )
    
    # Create some test GameObjects
    ball1 = GameObject(
        name="Ball1",
        position=Vector2(100, 100),
        size=Vector2(30, 30),
        color=Color(255, 0, 0),
        tag=Tag.Player,
        basicShape=BasicShape.Circle
    )
    ball1.add_component(RigidBody, mass=1.0)
    ball1.add_component(CircleCollider, radius=15)
    engine.addGameObject(ball1)
    
    ball2 = GameObject(
        name="Ball2",
        position=Vector2(200, 150),
        size=Vector2(40, 40),
        color=Color(0, 255, 0),
        tag=Tag.Other,
        basicShape=BasicShape.Circle
    )
    ball2.add_component(RigidBody, mass=2.0)
    ball2.add_component(CircleCollider, radius=20)
    engine.addGameObject(ball2)
    
    box = GameObject(
        name="Box",
        position=Vector2(400, 300),
        size=Vector2(50, 50),
        color=Color(0, 0, 255),
        tag=Tag.Environment,
        basicShape=BasicShape.Rectangle
    )
    box.add_component(RigidBody, mass=0.0, is_kinematic=True)  # Static
    engine.addGameObject(box)
    
    # Start the developer tool
    dev_window = start_dev_tool(engine)
    if dev_window is None:
        print("Could not start dev tool. Make sure PyQt6 is installed:")
        print("  pip install PyQt6")
        print("Or install with dev-tools extra:")
        print("  pip install -e .[dev-tools]")
        return
    
    print("Developer tool started! The debug window should be visible.")
    print("You can:")
    print("  - View all GameObjects in the GameObjects tab")
    print("  - Inspect and edit properties in the Inspector tab")
    print("  - View engine stats in the Stats tab")
    print("  - Export/import scene data in the Serialization tab")
    print("  - Pause/step the game loop using the control panel")
    
    # Start the game loop (this will block)
    engine.start()


if __name__ == "__main__":
    main()

