import pygame as pg
from pygame import Color, Vector2
import sys
import os
sys.path.append(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from object_types import Size, BasicShape, Tag
from gameobject import GameObject
from engine import Engine
from rigidbody import RigidBody
from collider import BoxCollider, CircleCollider
from material import Materials
from input import Input

def main():
    print("=== Rotation Direction Test ===")

    # Create engine
    engine = Engine(fpsCap=60, windowName="Rotation Direction Test", size=Size(800, 600))
    Engine.log_debug = True

    # Configure collision layers
    engine.physics_system.collision_layers = {
        "Player": ["Player", "Environment"],
        "Environment": ["Player"],
        "NoCollision": []
    }

    print("Creating test ball and floor...")

    # ================ TEST BALL ================
    test_ball = GameObject(
        name="test_ball",
        basicShape=BasicShape.Circle,
        color=Color(255, 100, 100),  # Red
        position=Vector2(400, 100),
        size=Vector2(50, 50),
        tag=Tag.Player
    )

    # Natural physics - can roll due to friction
    test_ball.add_component(RigidBody,
                           mass=1.0,
                           gravity_scale=1.0,
                           drag=0.05,
                           use_gravity=True,
                           lock_rotation=False)  # Allow natural rolling

    test_ball.add_component(CircleCollider,
                           radius=25,
                           material=Materials.METAL,  # High friction for rolling
                           collision_layer="Player")

    # Add simple input handling script
    class SimpleInputScript:
        def __init__(self, game_object):
            self.game_object = game_object
            self.push_force = 1000
            
        def update(self, engine):
            input = engine.input
            rb = self.game_object.get_component(RigidBody)
            
            if rb:
                # Push left/right to test rotation direction
                if input.get(Input.Keybind.K_LEFT) or input.get(Input.Keybind.A):
                    world_force = Vector2(-self.push_force, 0)
                    rb.add_force_at_point(world_force, self.game_object.position)
                elif input.get(Input.Keybind.K_RIGHT) or input.get(Input.Keybind.D):
                    world_force = Vector2(self.push_force, 0)
                    rb.add_force_at_point(world_force, self.game_object.position)

    # Add input script
    input_script = SimpleInputScript(test_ball)
    test_ball.scripts.append(input_script)

    # ================ FLOOR ================
    floor = GameObject(
        name="test_floor",
        basicShape=BasicShape.Rectangle,
        color=Color(139, 69, 19),  # Brown
        position=Vector2(400, 550),
        size=Vector2(800, 100),
        tag=Tag.Environment
    )

    floor.add_component(RigidBody,
                       mass=100.0,
                       is_kinematic=True,
                       use_gravity=False)

    floor.add_component(BoxCollider,
                       width=800,
                       height=100,
                       material=Materials.WOOD,
                       collision_layer="Environment")

    # ================ ADD OBJECTS TO ENGINE ================
    engine.addGameObject(test_ball)
    engine.addGameObject(floor)

    # Set camera to show the ball clearly
    engine.camera.set_position(400, 300)
    engine.camera.zoom = 1.0

    # ================ INSTRUCTIONS ================
    print("\n" + "="*60)
    print("ROTATION DIRECTION TEST:")
    print("="*60)
    print("🔴 RED BALL: Natural rolling physics")
    print("🟤 BROWN FLOOR: Static platform")
    print()
    print("CONTROLS:")
    print("- A/LEFT: Push ball left")
    print("- D/RIGHT: Push ball right")
    print("- Watch the ball's rotation direction")
    print()
    print("EXPECTED BEHAVIOR:")
    print("- Push LEFT: Ball should roll COUNTER-CLOCKWISE")
    print("- Push RIGHT: Ball should roll CLOCKWISE")
    print("="*60)

    # Start the physics simulation
    engine.start()
    sys.exit()

if __name__ == "__main__":
    main() 