from .script import Script
from .pymunk_collider import PymunkCollider
from .pymunk_rigidbody import PymunkRigidBody
import pygame as pg
from pygame import Color, Vector2

class CollisionDetectorScript(Script):
    """Script that changes the GameObject's color when collisions are detected."""

    def __init__(self, game_object, **kwargs):
        super().__init__(game_object, **kwargs)

        # Configuration - colors for different collision states
        self.original_color = self.get_config('original_color', Color(255, 0, 0))
        self.player_collision_color = self.get_config('player_collision_color', Color(255, 255, 0))  # Yellow
        self.floor_collision_color = self.get_config('floor_collision_color', Color(0, 255, 0))  # Green
        self.both_collision_color = self.get_config('both_collision_color', Color(255, 0, 255))  # Magenta

        # State tracking for collision detection
        self.colliding_with_player = False
        self.colliding_with_floor = False
        self.current_player_collisions = set()
        self.current_floor_collisions = set()

        # Add collision persistence for dynamic objects
        self.collision_persistence_time = 0.3  # How long to remember a collision (300ms)
        self.last_collision_times = {}  # Track when each collision started

    def start(self):
        print(f"CollisionDetector started on {self.game_object.name}")

        # Get collider and set up collision callbacks
        self.collider = self.get_component(PymunkCollider)
        if self.collider:
            self.collider.add_collision_callback('enter', self.on_collision_enter)
            self.collider.add_collision_callback('exit', self.on_collision_exit)
            print(f"Collision callbacks registered for {self.game_object.name}")
        else:
            print(f"Warning: No collider found on {self.game_object.name}")

        # Set initial color
        self.game_object.color = self.original_color

    def on_collision_enter(self, collision_info):
        """Called when a collision starts."""
        other_name = collision_info.other_gameobject.name
        print(f"{self.game_object.name} collision ENTER with {other_name}")

        # Record collision time for persistence
        self.last_collision_times[other_name] = 0.0  # Will be updated in update()

        # Determine what we collided with based on object name
        if "player" in other_name.lower():
            self.current_player_collisions.add(other_name)
            self.colliding_with_player = True
        elif "floor" in other_name.lower():
            self.current_floor_collisions.add(other_name)
            self.colliding_with_floor = True

        self.update_color()

    def on_collision_exit(self, collision_info):
        """Called when a collision ends."""
        other_name = collision_info.other_gameobject.name
        print(f"{self.game_object.name} collision EXIT with {other_name}")

        # For static objects (floor, walls), remove immediately
        # For dynamic objects (players), let persistence handle it
        if "floor" in other_name.lower() or "wall" in other_name.lower():
            # Static objects - remove immediately
            if "floor" in other_name.lower():
                self.current_floor_collisions.discard(other_name)
                self.colliding_with_floor = len(self.current_floor_collisions) > 0
            self.update_color()
        else:
            # Dynamic objects - let persistence handle it
            # Don't remove immediately, let the timer in update() handle it
            pass

    def update_color(self):
        """Update the GameObject's color based on current collisions."""
        if self.colliding_with_player and self.colliding_with_floor:
            # Colliding with both player and floor
            self.game_object.color = self.both_collision_color
            print(f"{self.game_object.name} color: BOTH (player + floor)")
        elif self.colliding_with_player:
            # Colliding with player only
            self.game_object.color = self.player_collision_color
            print(f"{self.game_object.name} color: PLAYER collision")
        elif self.colliding_with_floor:
            # Colliding with floor only
            self.game_object.color = self.floor_collision_color
            print(f"{self.game_object.name} color: FLOOR collision")
        else:
            # No collisions
            self.game_object.color = self.original_color
            print(f"{self.game_object.name} color: ORIGINAL (no collisions)")

    def update(self, engine):
        super().update(engine)

        # Update collision persistence timers
        dt = engine.dt()
        expired_collisions = []

        for other_name, collision_time in self.last_collision_times.items():
            self.last_collision_times[other_name] += dt

            # If collision has expired, remove it
            if self.last_collision_times[other_name] >= self.collision_persistence_time:
                expired_collisions.append(other_name)

        # Remove expired collisions (only for dynamic objects)
        for other_name in expired_collisions:
            del self.last_collision_times[other_name]

            # Remove from collision tracking (only for dynamic objects)
            if "player" in other_name.lower():
                self.current_player_collisions.discard(other_name)
                self.colliding_with_player = len(self.current_player_collisions) > 0
                # Update color after removing expired player collision
                self.update_color()

        # Optional: Add some movement controls for testing
        keys = pg.key.get_pressed()
        rb = self.get_component(PymunkRigidBody)

        if rb and self.game_object.name == "detector_player":
            # Proper Pymunk character controls using forces (not impulses with dt)
            move_force = 800  # Horizontal movement force
            max_speed = 250   # Maximum horizontal speed
            jump_impulse = 300  # Jump impulse (no dt scaling)

            # Get current velocity
            current_vel = rb.velocity

            # Lock rotation for stable controls (like Unity's Rigidbody constraints)
            # rb.set_rotation_lock(True)

            # Horizontal movement using forces (continuous) - ALWAYS in world coordinates
            if keys[pg.K_LEFT] or keys[pg.K_a]:
                if current_vel.x > -max_speed:  # Speed limiting
                    world_force = Vector2(-move_force, 0)
                    rb.add_force_at_point(world_force, self.game_object.position)
            elif keys[pg.K_RIGHT] or keys[pg.K_d]:
                if current_vel.x < max_speed:   # Speed limiting
                    world_force = Vector2(move_force, 0)
                    rb.add_force_at_point(world_force, self.game_object.position)
            # Let Pymunk's built-in friction handle stopping

            # Jumping using impulse (instantaneous, no dt scaling) - ALWAYS in world coordinates
            if keys[pg.K_UP] or keys[pg.K_w]:
                # Simple ground check - only jump if not moving up fast
                if current_vel.y < 50:  # Flipped condition for new coordinate system
                    world_impulse = Vector2(0, jump_impulse)  # Positive Y is up
                    rb.add_impulse_at_point(world_impulse, self.game_object.position)

