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
    print("=== Rotation Fix Test ===")

    # Create engine
    engine = Engine(fpsCap=60, windowName="Rotation Fix Test", size=Size(800, 600))
    Engine.log_debug = True

    # Configure collision layers
    engine.physics_system.collision_layers = {
        "Player": ["Player", "Environment"],
        "Environment": ["Player"],
        "NoCollision": []
    }

    print("Creating test player and floor...")

    # ================ TEST PLAYER ================
    test_player = GameObject(
        name="test_player",
        basicShape=BasicShape.Circle,
        color=Color(255, 100, 100),  # Red
        position=Vector2(400, 100),
        size=Vector2(50, 50),
        tag=Tag.Player
    )

    # Add physics components with rotation prevention
    test_player.add_component(RigidBody,
                             mass=1.0,
                             gravity_scale=1.0,
                             drag=0.05,
                             use_gravity=True)

    test_player.add_component(CircleCollider,
                             radius=25,
                             material=Materials.METAL,  # High friction to prevent sliding
                             collision_layer="Player")

    # Add simple input handling script
    class SimpleInputScript:
        def __init__(self, game_object):
            self.game_object = game_object
            self.move_force = 800
            self.max_speed = 250
            self.jump_impulse = 300
            
        def update(self, engine):
            input = engine.input
            rb = self.game_object.get_component(RigidBody)
            
            if rb:
                current_vel = rb.velocity
                
                # Horizontal movement - ALWAYS in world coordinates
                if input.get(Input.Keybind.K_LEFT) or input.get(Input.Keybind.A):
                    if current_vel.x > -self.max_speed:
                        world_force = Vector2(-self.move_force, 0)
                        rb.add_force_at_point(world_force, self.game_object.position)
                elif input.get(Input.Keybind.K_RIGHT) or input.get(Input.Keybind.D):
                    if current_vel.x < self.max_speed:
                        world_force = Vector2(self.move_force, 0)
                        rb.add_force_at_point(world_force, self.game_object.position)
                
                # Jumping
                if input.get(Input.Keybind.K_UP) or input.get(Input.Keybind.W):
                    if current_vel.y > -50:  # Simple ground check
                        world_impulse = Vector2(0, -self.jump_impulse)
                        rb.add_impulse_at_point(world_impulse, self.game_object.position)

    # Add input script
    input_script = SimpleInputScript(test_player)
    test_player.scripts.append(input_script)

    # ================ FLOOR ================
    floor = GameObject(
        name="test_floor",
        basicShape=BasicShape.Rectangle,
        color=Color(139, 69, 19),  # Brown
        position=Vector2(400, 550),  # Bottom center
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
                       material=Materials.WOOD,  # Medium friction
                       collision_layer="Environment")

    # ================ ADD OBJECTS TO ENGINE ================
    engine.addGameObject(test_player)
    engine.addGameObject(floor)

    # Make camera follow the test player
    engine.camera.follow(test_player, offset=Vector2(0, -50))
    engine.camera.follow_speed = 0.1

    # Set camera bounds
    world_bounds = pg.Rect(0, 0, 1000, 800)
    engine.camera.bounds = world_bounds

    # ================ INSTRUCTIONS ================
    print("\n" + "="*60)
    print("ROTATION FIX TEST:")
    print("="*60)
    print("🔴 RED CIRCLE: Test player")
    print("🟤 BROWN RECTANGLE: Floor")
    print()
    print("CONTROLS:")
    print("- A/D or LEFT/RIGHT: Move horizontally")
    print("- W or UP: Jump")
    print("- Watch for rotation when moving on the floor")
    print("- The player should NOT rotate when moving horizontally")
    print("="*60)

    # Start the physics simulation
    engine.start()
    sys.exit()

if __name__ == "__main__":
    main() 