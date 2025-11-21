"""
Snake Game Example for Pyg Engine
A complete Snake game implementation using the pyg_engine system.
"""

import pygame as pg
import random
import math
import pyg_engine
from pyg_engine import (
    Engine, GameObject, Vector2, Size, BasicShape, Tag,
    RigidBody, BoxCollider, Materials, Color
)

def get_script_path(script_name):
    """Get the absolute path to a script in the examples/scripts directory."""
    import os
    scripts_dir = os.path.join(os.path.dirname(__file__), "scripts")
    return os.path.join(scripts_dir, script_name)

def main():
    """Main function for Snake game."""
    print("=== Snake Game ===")
    print("Controls: Arrow keys or WASD to move, ESC to pause, R to restart")
    print("Objective: Eat the red food to grow and increase your score!")
    print("Don't hit the walls or yourself!")
    print("=" * 50)

    # Create engine
    engine = Engine(
        size=Size(800, 600),
        backgroundColor=Color(20, 20, 20),  # Dark background
        windowName="Snake Game - Pyg Engine",
        fpsCap=0
    )

    # Position camera to center the game area
    # Game area is 40x30 cells, each cell is 20px, so total area is 800x600
    # Center the camera on the game area
    engine.camera.position = Vector2(400, 300)
    engine.camera.zoom = 1.0  # Set zoom to 1 for proper scaling

    # Create game controller
    game_controller = GameObject(
        name="GameController",
        position=Vector2(0, 0),
        size=Vector2(1, 1),
        color=Color(0, 0, 0, 0),  # Invisible
        tag=Tag.Environment
    )

    # Add snake script using the proper scripting system
    game_controller.add_script(get_script_path("snake_script.py"), "SnakeScript",
                              grid_size=20,
                              game_speed=10)

    # Add to engine
    engine.addGameObject(game_controller)

    # Start the game
    engine.start()

if __name__ == "__main__":
    main()
