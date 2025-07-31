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
    print("=== Comprehensive Rotation Test ===")

    # Create engine
    engine = Engine(fpsCap=60, windowName="Comprehensive Rotation Test", size=Size(1200, 600))
    Engine.log_debug = True

    # Configure collision layers
    engine.physics_system.collision_layers = {
        "Player": ["Player", "Environment"],
        "Environment": ["Player"],
        "NoCollision": []
    }

    print("Creating test objects and floor...")

    # ================ ROLLING BALL (RED) ================
    rolling_ball = GameObject(
        name="rolling_ball",
        basicShape=BasicShape.Circle,
        color=Color(255, 100, 100),  # Red
        position=Vector2(200, 100),
        size=Vector2(50, 50),
        tag=Tag.Player
    )

    # Natural physics - can roll due to friction
    rolling_ball.add_component(RigidBody,
                              mass=1.0,
                              gravity_scale=1.0,
                              drag=0.05,
                              use_gravity=True,
                              lock_rotation=False)  # Allow natural rolling

    rolling_ball.add_component(CircleCollider,
                              radius=25,
                              material=Materials.METAL,  # High friction for rolling
                              collision_layer="Player")

    # ================ ROTATING RECTANGLE (GREEN) ================
    rotating_rect = GameObject(
        name="rotating_rect",
        basicShape=BasicShape.Rectangle,
        color=Color(100, 255, 100),  # Green
        position=Vector2(400, 100),
        size=Vector2(60, 40),
        tag=Tag.Player
    )

    rotating_rect.add_component(RigidBody,
                               mass=1.0,
                               gravity_scale=1.0,
                               drag=0.05,
                               use_gravity=True,
                               lock_rotation=False)  # Allow natural rotation

    rotating_rect.add_component(BoxCollider,
                               width=60,
                               height=40,
                               material=Materials.METAL,
                               collision_layer="Player")

    # ================ TALL RECTANGLE (BLUE) ================
    tall_rect = GameObject(
        name="tall_rect",
        basicShape=BasicShape.Rectangle,
        color=Color(100, 100, 255),  # Blue
        position=Vector2(600, 100),
        size=Vector2(40, 80),
        tag=Tag.Player
    )

    tall_rect.add_component(RigidBody,
                           mass=1.0,
                           gravity_scale=1.0,
                           drag=0.05,
                           use_gravity=True,
                           lock_rotation=False)  # Allow natural rotation

    tall_rect.add_component(BoxCollider,
                           width=40,
                           height=80,
                           material=Materials.METAL,
                           collision_layer="Player")

    # ================ LOCKED BALL (YELLOW) ================
    locked_ball = GameObject(
        name="locked_ball",
        basicShape=BasicShape.Circle,
        color=Color(255, 255, 100),  # Yellow
        position=Vector2(800, 100),
        size=Vector2(50, 50),
        tag=Tag.Player
    )

    locked_ball.add_component(RigidBody,
                             mass=1.0,
                             gravity_scale=1.0,
                             drag=0.05,
                             use_gravity=True,
                             lock_rotation=True)  # Lock rotation

    locked_ball.add_component(CircleCollider,
                             radius=25,
                             material=Materials.METAL,
                             collision_layer="Player")

    # ================ LOCKED RECTANGLE (MAGENTA) ================
    locked_rect = GameObject(
        name="locked_rect",
        basicShape=BasicShape.Rectangle,
        color=Color(255, 100, 255),  # Magenta
        position=Vector2(1000, 100),
        size=Vector2(60, 40),
        tag=Tag.Player
    )

    locked_rect.add_component(RigidBody,
                             mass=1.0,
                             gravity_scale=1.0,
                             drag=0.05,
                             use_gravity=True,
                             lock_rotation=True)

    locked_rect.add_component(BoxCollider,
                             width=60,
                             height=40,
                             material=Materials.METAL,
                             collision_layer="Player")

    # Add input handling script
    class ComprehensiveInputScript:
        def __init__(self, game_object):
            self.game_object = game_object
            self.push_force = 600
            
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

    # Add input script to rolling ball
    input_script = ComprehensiveInputScript(rolling_ball)
    rolling_ball.scripts.append(input_script)

    # ================ FLOOR ================
    floor = GameObject(
        name="test_floor",
        basicShape=BasicShape.Rectangle,
        color=Color(139, 69, 19),  # Brown
        position=Vector2(600, 550),
        size=Vector2(1200, 100),
        tag=Tag.Environment
    )

    floor.add_component(RigidBody,
                       mass=100.0,
                       is_kinematic=True,
                       use_gravity=False)

    floor.add_component(BoxCollider,
                       width=1200,
                       height=100,
                       material=Materials.WOOD,
                       collision_layer="Environment")

    # ================ ADD OBJECTS TO ENGINE ================
    engine.addGameObject(rolling_ball)
    engine.addGameObject(rotating_rect)
    engine.addGameObject(tall_rect)
    engine.addGameObject(locked_ball)
    engine.addGameObject(locked_rect)
    engine.addGameObject(floor)

    # Set camera to show all objects
    engine.camera.set_position(600, 300)
    engine.camera.zoom = 0.7

    # ================ INSTRUCTIONS ================
    print("\n" + "="*60)
    print("COMPREHENSIVE ROTATION TEST:")
    print("="*60)
    print("🔴 RED BALL: Natural rolling (use A/D to push)")
    print("🟢 GREEN RECTANGLE: Natural rotation")
    print("🔵 BLUE RECTANGLE: Tall rectangle with natural rotation")
    print("🟡 YELLOW BALL: Locked rotation (no rolling)")
    print("🟣 MAGENTA RECTANGLE: Locked rotation (no rotation)")
    print("🟤 BROWN FLOOR: Static platform")
    print()
    print("CONTROLS:")
    print("- A/LEFT: Push red ball left")
    print("- D/RIGHT: Push red ball right")
    print("- Watch all objects' rotation behavior")
    print()
    print("EXPECTED BEHAVIOR:")
    print("- Push LEFT: Objects should rotate COUNTER-CLOCKWISE")
    print("- Push RIGHT: Objects should rotate CLOCKWISE")
    print("- Yellow ball and magenta rectangle should slide without rotating")
    print("- All other objects should rotate naturally when falling")
    print("="*60)

    # Start the physics simulation
    engine.start()
    sys.exit()

if __name__ == "__main__":
    main() 