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

def main():
    print("=== Rotation System Test ===")

    # Create engine
    engine = Engine(fpsCap=60, windowName="Rotation System Test", size=Size(1000, 600))
    Engine.log_debug = True

    # Configure collision layers
    engine.physics_system.collision_layers = {
        "Player": ["Player", "Environment"],
        "Environment": ["Player"],
        "NoCollision": []
    }

    print("Creating test objects...")

    # ================ ROLLING BALL (Natural Physics) ================
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

    # ================ LOCKED BALL (No Rotation) ================
    locked_ball = GameObject(
        name="locked_ball",
        basicShape=BasicShape.Circle,
        color=Color(100, 255, 100),  # Green
        position=Vector2(400, 100),
        size=Vector2(50, 50),
        tag=Tag.Player
    )

    # Locked rotation - won't roll
    locked_ball.add_component(RigidBody,
                             mass=1.0,
                             gravity_scale=1.0,
                             drag=0.05,
                             use_gravity=True,
                             lock_rotation=True)  # Prevent rolling

    locked_ball.add_component(CircleCollider,
                             radius=25,
                             material=Materials.METAL,
                             collision_layer="Player")

    # ================ RECTANGLE (Natural Physics) ================
    rolling_rect = GameObject(
        name="rolling_rect",
        basicShape=BasicShape.Rectangle,
        color=Color(100, 100, 255),  # Blue
        position=Vector2(600, 100),
        size=Vector2(60, 40),
        tag=Tag.Player
    )

    # Natural physics - can rotate
    rolling_rect.add_component(RigidBody,
                              mass=1.0,
                              gravity_scale=1.0,
                              drag=0.05,
                              use_gravity=True,
                              lock_rotation=False)  # Allow natural rotation

    rolling_rect.add_component(BoxCollider,
                              width=60,
                              height=40,
                              material=Materials.METAL,
                              collision_layer="Player")

    # ================ LOCKED RECTANGLE (No Rotation) ================
    locked_rect = GameObject(
        name="locked_rect",
        basicShape=BasicShape.Rectangle,
        color=Color(255, 255, 100),  # Yellow
        position=Vector2(800, 100),
        size=Vector2(60, 40),
        tag=Tag.Player
    )

    # Locked rotation - won't rotate
    locked_rect.add_component(RigidBody,
                             mass=1.0,
                             gravity_scale=1.0,
                             drag=0.05,
                             use_gravity=True,
                             lock_rotation=True)  # Prevent rotation

    locked_rect.add_component(BoxCollider,
                             width=60,
                             height=40,
                             material=Materials.METAL,
                             collision_layer="Player")

    # ================ FLOOR ================
    floor = GameObject(
        name="test_floor",
        basicShape=BasicShape.Rectangle,
        color=Color(139, 69, 19),  # Brown
        position=Vector2(500, 550),
        size=Vector2(1000, 100),
        tag=Tag.Environment
    )

    floor.add_component(RigidBody,
                       mass=100.0,
                       is_kinematic=True,
                       use_gravity=False)

    floor.add_component(BoxCollider,
                       width=1000,
                       height=100,
                       material=Materials.WOOD,
                       collision_layer="Environment")

    # ================ ADD OBJECTS TO ENGINE ================
    engine.addGameObject(rolling_ball)
    engine.addGameObject(locked_ball)
    engine.addGameObject(rolling_rect)
    engine.addGameObject(locked_rect)
    engine.addGameObject(floor)

    # Set camera to show all objects
    engine.camera.set_position(500, 300)
    engine.camera.zoom = 0.8

    # ================ INSTRUCTIONS ================
    print("\n" + "="*60)
    print("ROTATION SYSTEM TEST:")
    print("="*60)
    print("🔴 RED CIRCLE: Natural rolling physics (lock_rotation=False)")
    print("🟢 GREEN CIRCLE: Locked rotation (lock_rotation=True)")
    print("🔵 BLUE RECTANGLE: Natural rotation physics (lock_rotation=False)")
    print("🟡 YELLOW RECTANGLE: Locked rotation (lock_rotation=True)")
    print("🟤 BROWN FLOOR: Static platform")
    print()
    print("OBSERVATIONS:")
    print("- Red ball should roll naturally when pushed")
    print("- Green ball should slide without rolling")
    print("- Blue rectangle should rotate when pushed")
    print("- Yellow rectangle should slide without rotating")
    print("="*60)

    # Start the physics simulation
    engine.start()
    sys.exit()

if __name__ == "__main__":
    main() 