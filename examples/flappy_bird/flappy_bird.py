"""
Flappy Bird Game Example for Pyg Engine
A complete Flappy Bird game implementation using the pyg_engine system.
"""

import pygame as pg
from pyg_engine import (
    Engine, GameObject, Vector2, Size, BasicShape, Tag, Color, 
    start_dev_tool
)

def get_script_path(script_name):
    """Get the absolute path to a script in the examples/scripts directory."""
    import os
    scripts_dir = os.path.join(os.path.dirname(__file__), "scripts")
    return os.path.join(scripts_dir, script_name)

def main():
    """Main function for Flappy Bird game."""
    print("\n" + "=" * 50)
    print("=== Flappy Bird - Pyg Engine ===")
    print("=" * 50)
    print("Controls:")
    print("  SPACE or CLICK - Flap wings")
    print("  R - Restart (when game over)")
    print("  ESC - Quit")
    print("\nObjective: Fly through the pipes without hitting them!")
    print("=" * 50 + "\n")

    # Create engine with nice blue sky color
    # Note: Set Engine.log_debug = True for verbose output
    engine = Engine(
        size=Size(800, 600),
        backgroundColor=Color(135, 206, 235),  # Sky blue
        windowName="Flappy Bird - Pyg Engine",
        fpsCap=120
    )

    # Optional: Start dev tool if PyQt6 is installed
    try:
        start_dev_tool(engine)
    except RuntimeError as e:
        print(f"Note: {e}")
        print("Continuing without dev tool...\n")

    # Position camera to center the game area
    engine.camera.position = Vector2(400, 300)
    engine.camera.zoom = 1.0

    # Create game controller
    game_controller = GameObject(
        name="GameController",
        position=Vector2(400, 300),
        size=Vector2(1, 1),
        color=Color(0, 0, 0, 0),  # Invisible
        tag=Tag.Environment
    )

    # Add Flappy Bird controller script
    game_controller.add_script(
        get_script_path("flappy_bird_controller.py"), 
        "FlappyBirdController"
    )

    # Add to engine
    engine.addGameObject(game_controller)

    # Start the game
    engine.start()

if __name__ == "__main__":
    main()
