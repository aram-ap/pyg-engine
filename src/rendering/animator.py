'''
Animator Component - Frame-based animation system

Features:
    - Frame-based animation with timing control
    - Multiple animation states (idle, walk, jump, etc.)
    - Loop and one-shot animation modes
    - Animation events and callbacks
    - Smooth transitions between animations
    - Integrates with Sprite component
    
Example usage:
    # Create animated sprite
    player = GameObject(name="Player", position=Vector2(100, 100))
    sprite = player.add_component(Sprite)
    animator = player.add_component(Animator)
    
    # Load animation frames
    idle_frames = load_animation_frames(["idle1.png", "idle2.png", "idle3.png"])
    walk_frames = load_animation_frames(["walk1.png", "walk2.png", "walk3.png", "walk4.png"])
    
    # Add animations
    animator.add_animation("idle", idle_frames, frame_duration=0.2, loop=True)
    animator.add_animation("walk", walk_frames, frame_duration=0.1, loop=True)
    
    # Play animation
    animator.play("idle")
'''

import pygame
from ..components.component import Component
from .sprite import Sprite


class AnimationState:
    """Represents a single animation with its frames and properties."""
    
    def __init__(self, name, frames, frame_duration=0.1, loop=True, on_complete=None):
        """
        Initialize an animation state.
        
        Args:
            name: Name identifier for this animation
            frames: List of pygame.Surface frames
            frame_duration: Duration of each frame in seconds
            loop: Whether the animation loops
            on_complete: Callback function when animation completes (for non-looping)
        """
        self.name = name
        self.frames = frames if frames else []
        self.frame_duration = frame_duration
        self.loop = loop
        self.on_complete = on_complete
        
        # Playback state
        self.current_frame = 0
        self.time_accumulator = 0.0
        self.finished = False
    
    def reset(self):
        """Reset the animation to the beginning."""
        self.current_frame = 0
        self.time_accumulator = 0.0
        self.finished = False
    
    def get_current_frame(self):
        """Get the current frame surface."""
        if not self.frames:
            return None
        return self.frames[self.current_frame]
    
    def update(self, delta_time):
        """
        Update the animation timing and advance frames.
        
        Args:
            delta_time: Time elapsed since last update in seconds
        """
        if self.finished or not self.frames:
            return
        
        self.time_accumulator += delta_time
        
        # Check if we need to advance to the next frame
        while self.time_accumulator >= self.frame_duration:
            self.time_accumulator -= self.frame_duration
            self.current_frame += 1
            
            # Handle end of animation
            if self.current_frame >= len(self.frames):
                if self.loop:
                    self.current_frame = 0
                else:
                    self.current_frame = len(self.frames) - 1
                    self.finished = True
                    if self.on_complete:
                        self.on_complete()
                    break


class Animator(Component):
    """
    Animator component for managing sprite animations.
    Works with the Sprite component to display animated frames.
    """
    
    def __init__(self, game_object, default_animation=None):
        """
        Initialize the Animator.
        
        Args:
            game_object: The GameObject this animator is attached to
            default_animation: Name of the default animation to play
        """
        super().__init__(game_object)
        
        # Animation library
        self.animations = {}  # name -> AnimationState
        
        # Playback state
        self.current_animation = None
        self.current_animation_name = None
        self.default_animation = default_animation
        
        # Component references
        self._sprite = None
        
        # Speed control
        self.speed = 1.0  # Animation speed multiplier
        self.paused = False
    
    def start(self):
        """Initialize the animator and get sprite reference."""
        # Get the Sprite component
        self._sprite = self.game_object.get_component(Sprite)
        if not self._sprite:
            print(f"Warning: Animator on '{self.game_object.name}' has no Sprite component")
        
        # Play default animation if set
        if self.default_animation and self.default_animation in self.animations:
            self.play(self.default_animation)
    
    def add_animation(self, name, frames, frame_duration=0.1, loop=True, on_complete=None):
        """
        Add a new animation to the animator.
        
        Args:
            name: Unique name for this animation
            frames: List of pygame.Surface frames
            frame_duration: Duration of each frame in seconds
            loop: Whether the animation loops
            on_complete: Callback when animation completes
            
        Returns:
            The created AnimationState
        """
        animation = AnimationState(name, frames, frame_duration, loop, on_complete)
        self.animations[name] = animation
        return animation
    
    def play(self, animation_name, restart=True):
        """
        Play an animation by name.
        
        Args:
            animation_name: Name of the animation to play
            restart: If True, restart the animation even if it's already playing
            
        Returns:
            True if animation started, False if animation not found
        """
        if animation_name not in self.animations:
            print(f"Warning: Animation '{animation_name}' not found on '{self.game_object.name}'")
            return False
        
        # Check if already playing this animation
        if self.current_animation_name == animation_name and not restart:
            return True
        
        # Switch to new animation
        self.current_animation = self.animations[animation_name]
        self.current_animation_name = animation_name
        
        if restart:
            self.current_animation.reset()
        
        # Update sprite with first frame
        self._update_sprite_frame()
        
        return True
    
    def stop(self):
        """Stop the current animation."""
        self.current_animation = None
        self.current_animation_name = None
    
    def pause(self):
        """Pause the current animation."""
        self.paused = True
    
    def resume(self):
        """Resume the paused animation."""
        self.paused = False
    
    def set_speed(self, speed):
        """
        Set the animation playback speed.
        
        Args:
            speed: Speed multiplier (1.0 = normal, 2.0 = double speed, 0.5 = half speed)
        """
        self.speed = max(0.0, speed)
    
    def is_playing(self, animation_name=None):
        """
        Check if an animation is currently playing.
        
        Args:
            animation_name: Specific animation name to check (None checks if any animation is playing)
            
        Returns:
            True if the animation is playing
        """
        if animation_name:
            return self.current_animation_name == animation_name and not self.paused
        return self.current_animation is not None and not self.paused
    
    def is_finished(self):
        """Check if the current animation has finished (for non-looping animations)."""
        return self.current_animation and self.current_animation.finished
    
    def get_current_animation_name(self):
        """Get the name of the currently playing animation."""
        return self.current_animation_name
    
    def has_animation(self, animation_name):
        """Check if an animation exists."""
        return animation_name in self.animations
    
    def remove_animation(self, animation_name):
        """Remove an animation from the animator."""
        if animation_name in self.animations:
            # Stop if this is the current animation
            if self.current_animation_name == animation_name:
                self.stop()
            del self.animations[animation_name]
            return True
        return False
    
    def _update_sprite_frame(self):
        """Update the sprite component with the current animation frame."""
        if not self._sprite or not self.current_animation:
            return
        
        frame = self.current_animation.get_current_frame()
        if frame:
            self._sprite.set_image_from_surface(frame)
    
    def update(self, engine):
        """Update the animation (called every frame)."""
        if not self.enabled or self.paused or not self.current_animation:
            return
        
        # Get delta time from engine (automatically scaled by time_scale)
        delta_time = engine.dt()
        
        # Apply speed multiplier
        delta_time *= self.speed
        
        # Update animation
        self.current_animation.update(delta_time)
        
        # Update sprite with new frame
        self._update_sprite_frame()
    
    def on_destroy(self):
        """Clean up when the animator is destroyed."""
        self.animations.clear()
        self.current_animation = None
        self._sprite = None
