"""
Bird Script for Flappy Bird
Controls the bird's physics and input with sprite animation and sound
"""

import pygame as pg
from pyg_engine import Script, Input, Vector2, Animator, audio_manager
from pyg_engine.rendering import load_animation_frames
import os

class BirdScript(Script):
    """Script for controlling the bird in Flappy Bird."""

    def __init__(self, game_object, controller=None):
        super().__init__(game_object)
        
        # Reference to game controller
        self.controller = controller
        
        # Physics properties
        self.velocity = Vector2(0, 0)
        self.gravity = -800  # Pixels per second squared (negative pulls down in engine coords)
        self.flap_strength = 350  # Positive for upward movement in engine coords
        
        # Rotation for visual effect
        self.max_rotation = 25  # Degrees
        self.rotation_speed = 3.0
        
        # State
        self.is_alive = True
        self.can_flap = True
        
        # Input tracking for button release flapping
        self.was_button_pressed = False
        
        # Animation
        self.animator = None

    def start(self, engine):
        """Called when the script starts."""
        super().start(engine)
        
        # Get animator component
        self.animator = self.game_object.get_component(Animator)
        if self.animator and self.animator.has_animation("fly"):
            self.animator.play("fly")

    def update(self, engine):
        """Update bird physics and input."""
        if not self.controller:
            return
        
        dt = engine.dt()
        
        # Only update physics when playing
        if self.controller.game_state == "playing":
            # Handle input for flapping - only flap once per button press
            button_pressed = (engine.input.get(Input.Keybind.K_SPACE) or \
                            engine.input.mouse.get_button(0))
            
            # Flap on initial button press only (not while holding)
            if button_pressed and not self.was_button_pressed and self.can_flap:
                self._flap()
                self.can_flap = False  # Prevent multiple flaps until button is released
            
            # Allow flapping again once button is released
            if not button_pressed and self.was_button_pressed:
                self.can_flap = True
            
            # Update button state for next frame
            self.was_button_pressed = button_pressed
            
            # Apply gravity
            self.velocity.y += self.gravity * dt
            
            # Clamp falling speed
            if self.velocity.y < -600:
                self.velocity.y = -600
            
            # Update position
            self.game_object.position.y += self.velocity.y * dt
            
            # Update rotation based on velocity (visual effect)
            target_rotation = 0
            if self.velocity.y > 0:
                # Flying up - tilt up
                target_rotation = -self.max_rotation
            elif self.velocity.y < -200:
                # Falling fast - tilt down
                target_rotation = self.max_rotation * 2
            
            # Smoothly interpolate rotation
            current_rotation = self.game_object.rotation
            self.game_object.rotation += (target_rotation - current_rotation) * self.rotation_speed * dt
            
            # Clamp rotation
            if self.game_object.rotation > 90:
                self.game_object.rotation = 90
            if self.game_object.rotation < -self.max_rotation:
                self.game_object.rotation = -self.max_rotation
            
            # Reset animation speed after flap
            if self.animator and self.animator.speed > 1.0:
                self.animator.set_speed(max(1.0, self.animator.speed - 2.0 * dt))
        
        elif self.controller.game_state == "ready":
            # Idle floating animation
            import math
            time = pg.time.get_ticks() / 1000.0
            offset = math.sin(time * 3) * 10
            # Get screen height dynamically for centered idle position
            screen_height = engine.getWindowSize().h if engine else 600
            base_y = screen_height * 0.5
            self.game_object.position.y = base_y + offset
            self.velocity = Vector2(0, 0)
            self.game_object.rotation = 0

    def _flap(self):
        """Make the bird flap its wings."""
        self.velocity.y = self.flap_strength
        self.can_flap = True
        
        # Play flap sound
        audio_manager.play_sound("wing", volume=0.6)
        
        # Speed up animation briefly for visual feedback
        if self.animator:
            self.animator.set_speed(1.5)

    def reset(self):
        """Reset bird to initial state."""
        self.velocity = Vector2(0, 0)
        self.game_object.rotation = 0
        self.is_alive = True
        self.can_flap = True
        self.was_button_pressed = False
