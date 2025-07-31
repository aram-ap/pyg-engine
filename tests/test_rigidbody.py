from script import Script
from rigidbody import RigidBody
import pygame as pg
from pygame import Vector2
from input import Input

class RigidBodyTestScript(Script):
    def start(self):
        # Get or add a rigidbody to this GameObject
        self.rb = self.get_component(RigidBody)
        if not self.rb:
            print(f"No RigidBody found on {self.game_object.name}")
            return

        print(f"RigidBody test script started on {self.game_object.name}")
        print(f"Initial mass: {self.rb.mass}")
        print("Use SPACE to jump, WASD to apply forces")

    def update(self, engine):
        super().update(engine)

        if not self.rb:
            return

        # Handle input using the new input system
        input = engine.input

        # Movement forces (like thrusters)
        force_strength = 2000  # Newtons

        if input.get(Input.Keybind.A) or input.get(Input.Keybind.K_LEFT):
            self.rb.add_force(Vector2(-force_strength, 0))
        if input.get(Input.Keybind.D) or input.get(Input.Keybind.K_RIGHT):
            self.rb.add_force(Vector2(force_strength, 0))
        if input.get(Input.Keybind.W) or input.get(Input.Keybind.K_UP):
            self.rb.add_force(Vector2(0, -force_strength))
        if input.get(Input.Keybind.S) or input.get(Input.Keybind.K_DOWN):
            self.rb.add_force(Vector2(0, force_strength))

        # Jump (impulse)
        if input.get(Input.Keybind.K_SPACE):
            # Only jump if not moving up too fast already
            if self.rb.velocity.y > -200:
                self.rb.add_impulse(Vector2(0, -300))  # Upward impulse

        # Debug output (every 60 frames)
        if hasattr(self, 'debug_counter'):
            self.debug_counter += 1
        else:
            self.debug_counter = 0

        if self.debug_counter % 60 == 0:  # Every second at 60 FPS
            print(f"{self.game_object.name}: vel={self.rb.velocity}, pos={self.game_object.position}")

