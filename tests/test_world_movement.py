import pygame as pg
from pygame import Color, Vector2
import sys
from object_types import Size, BasicShape
from gameobject import BasicObject
from engine import Engine
from scripts.player import PlayerScript
from rigidbody import RigidBody
from collider import CircleCollider

def main():
    print("Testing World-Axis Movement")

    # Create engine
    engine = Engine(fpsCap=60, windowName="World Movement Test", size=Size(800, 600))
    Engine.log_debug = True

    # Create a rotating player to test world-axis movement
    player = BasicObject(
        name="rotating_player",
        basicShape=BasicShape.Circle,
        color=Color(255, 0, 0),  # Red
        position=Vector2(400, 300),
        size=Vector2(40, 40)
    )

    # Add physics components
    player.add_component(RigidBody,
                        mass=1.0,
                        gravity_scale=0.0,  # No gravity for this test
                        use_gravity=False)
    player.add_component(CircleCollider,
                        radius=20,
                        material="Player")

    # Add player script with force-based controls
    player.add_script("scripts/player.py",
                     speed=200,
                     player_id=1,
                     control_mode="force",
                     move_force=500,
                     max_speed=150,
                     use_mouse_control=False)  # Disable mouse for this test

    # Create a simple rotation script to test movement during rotation
    class RotationTestScript:
        def __init__(self, game_object):
            self.game_object = game_object
            self.rotation_speed = 45  # degrees per second
            
        def update(self, engine):
            # Continuously rotate the player
            self.game_object.rotation += self.rotation_speed * engine.dt()
            
            # Also rotate the physics body if it exists
            rb = self.game_object.get_component(RigidBody)
            if rb and rb.body:
                rb.body.angle = -self.game_object.rotation * 3.14159 / 180  # Convert to radians

    # Add rotation script
    rotation_script = RotationTestScript(player)
    player.scripts.append(rotation_script)

    # Add player to engine
    engine.addGameObject(player)

    print("\n=== World Movement Test ===")
    print("The red circle will continuously rotate")
    print("Use WASD keys to move")
    print("Movement should be relative to the screen (world axes), not the circle's rotation")
    print("ESC: Pause/Unpause")

    # Start the engine
    engine.start()
    sys.exit()

if __name__ == "__main__":
    main() 