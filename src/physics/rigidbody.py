import pygame as pg
import pymunk
from ..components.component import Component
from ..utilities.vector2 import Vector2
class RigidBody(Component):
    """Physics component using Pymunk for realistic physics simulation."""

    def __init__(self, game_object, mass=1.0, gravity_scale=1.0, drag=0.0,
                 use_gravity=True, is_kinematic=False, angular_drag=0.1, lock_rotation=False):
        super().__init__(game_object)

        # Physics properties
        self.mass = max(0.5, mass)  # Increased minimum mass for better stability
        self.gravity_scale = gravity_scale
        self.drag = max(0.0, drag)
        self.angular_drag = angular_drag
        self.use_gravity = use_gravity
        self.is_kinematic = is_kinematic

        # Rotation constraints (like Unity's Rigidbody constraints)
        self.lock_rotation = lock_rotation

        # Pymunk body and shape will be set by the physics system
        self.body = None
        self.shape = None

        # For maintaining pygame compatibility
        self._pygame_velocity = Vector2(0, 0)
        self._pygame_angular_velocity = 0.0

        self.paused:bool = False
        self._prev_velocity = (0,0)
        self._prev_angular_velocity = 0
        self._prev_gravity = 0

        print(f"RigidBody created with mass={self.mass}, gravity_scale={self.gravity_scale}, lock_rotation={self.lock_rotation}")

    def start(self):
        """Initialize the rigidbody. The actual pymunk body will be created by the physics system."""
        print(f"RigidBody started on {self.game_object.name}")

    def update(self, engine):
        """Update pygame position/rotation from pymunk body."""
        if self.body and not self.is_kinematic:
            # Update GameObject position from pymunk body
            self.game_object.position.x = self.body.position.x
            self.game_object.position.y = self.body.position.y

            # Update GameObject rotation from pymunk body (convert radians to degrees)
            # Check if this is a circle - circles need inverted rotation for correct rolling direction
            from .collider import CircleCollider
            is_circle = self.game_object.get_component(CircleCollider) is not None

            if is_circle:
                self.game_object.rotation = -self.body.angle * 180 / 3.14159  # Invert for correct rolling
            else:
                self.game_object.rotation = self.body.angle * 180 / 3.14159  # Direct conversion for rectangles

            # Update velocity properties for compatibility
            self._pygame_velocity.x = self.body.velocity.x
            self._pygame_velocity.y = self.body.velocity.y
            self._pygame_angular_velocity = self.body.angular_velocity  # Correct coordinate system conversion



            # Apply rotation lock if enabled
            if self.lock_rotation:
                self.body.angular_velocity = 0.0
                # Keep the body at a fixed angle (0 degrees)
                self.body.angle = 0.0

    # ================ Properties for API compatibility ================

    @property
    def velocity(self):
        """Get velocity as pygame Vector2."""
        if self.body:
            return Vector2(self.body.velocity.x, self.body.velocity.y)
        return self._pygame_velocity

    @velocity.setter
    def velocity(self, value):
        """Set velocity from pygame Vector2."""
        if isinstance(value, (list, tuple)):
            value = Vector2(value[0], value[1])
        self._pygame_velocity = value
        if self.body:
            self.body.velocity = (value.x, value.y)

    @property
    def angular_velocity(self):
        """Get angular velocity in radians per second."""
        if self.body:
            return self.body.angular_velocity  # Correct coordinate system conversion
        return self._pygame_angular_velocity

    @angular_velocity.setter
    def angular_velocity(self, value):
        """Set angular velocity in radians per second."""
        self._pygame_angular_velocity = value
        if self.body:
            self.body.angular_velocity = value  # Correct coordinate system conversion

    @property
    def moment_of_inertia(self):
        """Get moment of inertia from pymunk body."""
        if self.body:
            return self.body.moment
        return 1.0  # Default value


    # ================ Force/Impulse Application Methods ================

    def add_force(self, force, point=None):
        """Add a force to be applied this frame."""
        if not self.body or self.is_kinematic:
            return

        if isinstance(force, (list, tuple)):
            force = Vector2(force[0], force[1])

        if point:
            if isinstance(point, (list, tuple)):
                point = Vector2(point[0], point[1])
            self.body.apply_force_at_world_point((force.x, force.y), (point.x, point.y))
        else:
            self.body.apply_force_at_local_point((force.x, force.y), (0, 0))

    def add_torque(self, torque):
        """Add a torque (rotational force) to be applied this frame."""
        if not self.body or self.is_kinematic:
            return
        self.body.apply_force_at_local_point((0, 0), (0, 0))  # Reset any previous force
        self.body.torque += torque

    def add_force_at_point(self, force, point):
        """Add a force at a specific world point."""
        if not self.body or self.is_kinematic:
            return

        if isinstance(force, (list, tuple)):
            force = Vector2(force[0], force[1])
        if isinstance(point, (list, tuple)):
            point = Vector2(point[0], point[1])

        self.body.apply_force_at_world_point((force.x, force.y), (point.x, point.y))

    def add_impulse(self, impulse, point=None):
        """Apply an immediate change to velocity."""
        if not self.body or self.is_kinematic:
            return

        if isinstance(impulse, (list, tuple)):
            impulse = Vector2(impulse[0], impulse[1])

        if point:
            if isinstance(point, (list, tuple)):
                point = Vector2(point[0], point[1])
            self.body.apply_impulse_at_world_point((impulse.x, impulse.y), (point.x, point.y))
        else:
            self.body.apply_impulse_at_local_point((impulse.x, impulse.y), (0, 0))

    def add_impulse_at_point(self, impulse, point):
        """Apply an immediate change to velocity at a specific world point."""
        if not self.body or self.is_kinematic:
            return

        if isinstance(impulse, (list, tuple)):
            impulse = Vector2(impulse[0], impulse[1])
        if isinstance(point, (list, tuple)):
            point = Vector2(point[0], point[1])

        self.body.apply_impulse_at_world_point((impulse.x, impulse.y), (point.x, point.y))

    def add_angular_impulse(self, angular_impulse):
        """Apply an immediate change to angular velocity."""
        if not self.body or self.is_kinematic:
            return
        # Apply angular impulse by applying impulse at offset point
        self.body.angular_velocity += angular_impulse

    def set_velocity(self, velocity):
        """Directly set the velocity."""
        if isinstance(velocity, (list, tuple)):
            velocity = Vector2(velocity[0], velocity[1])
        self.velocity = velocity

    def add_velocity(self, velocity):
        """Add to the current velocity."""
        if isinstance(velocity, (list, tuple)):
            velocity = Vector2(velocity[0], velocity[1])
        current_vel = self.velocity
        self.velocity = current_vel + velocity

    # ================ Utility Methods ================

    def get_speed(self):
        """Get the current linear speed."""
        return self.velocity.magnitude

    def get_angular_speed(self):
        """Get the current angular speed (absolute value)."""
        return abs(self.angular_velocity)

    def get_kinetic_energy(self):
        """Get total kinetic energy (linear + rotational)."""
        if self.body:
            return self.body.kinetic_energy
        return 0.0

    def stop(self):
        """Stop all movement."""
        self.velocity = Vector2(0, 0)
        self.angular_velocity = 0.0

    def freeze_rotation(self):
        """Stop rotational movement and prevent future rotation."""
        self.lock_rotation = True
        self.angular_velocity = 0.0
        if self.body:
            # Set the body to not rotate
            self.body.angular_velocity = 0.0

    def unfreeze_rotation(self):
        """Restore normal rotation behavior."""
        self.lock_rotation = False

    def set_rotation_lock(self, lock_rotation):
        """Set whether rotation is locked (like Unity's Rigidbody constraints)."""
        self.lock_rotation = lock_rotation
        if self.body and lock_rotation:
            self.body.angular_velocity = 0.0

    def pause(self):
        if(self.paused is True):
            return
        self.paused = True
        self._prev_angular_velocity = self.angular_velocity
        self._prev_velocity = self.velocity
        self._prev_gravity = self.gravity_scale
        self.gravity_scale = 0
        self.stop()
        self.set_kinematic(False)

    def unpause(self):
        if(self.paused is False):
            return
        self.paused = False
        self.angular_velocity = self._prev_angular_velocity
        self.velocity = self._prev_velocity
        self.gravity_scale = self._prev_gravity
        self.set_kinematic(True)



    def set_pause(self, value):
        if self.paused is value:
            return

        if value:
            self.pause()
        else:
            self.unpause()

    def toggle_pause(self):
        if(self.paused):
            self.unpause()
        else:
            self.pause()



    # ================ Configuration Methods ================

    def set_mass(self, mass):
        """Change the mass at runtime."""
        self.mass = max(0.5, mass)  # Increased minimum mass for better stability
        if self.body:
            self.body.mass = self.mass
        print(f"RigidBody mass changed to {self.mass}")

    def set_gravity_scale(self, scale):
        """Change how much gravity affects this object."""
        self.gravity_scale = scale
        print(f"RigidBody gravity scale changed to {self.gravity_scale}")

    def set_kinematic(self, is_kinematic):
        """Enable/disable kinematic mode."""
        self.is_kinematic = is_kinematic
        if self.body:
            if is_kinematic:
                # Convert to kinematic body
                self.body.body_type = pymunk.Body.KINEMATIC
                self.stop()
            else:
                # Convert to dynamic body
                self.body.body_type = pymunk.Body.DYNAMIC

                # Kinematic bodies have infinite mass, so we need to restore the original values
                self.body.mass = self.mass

                # Recalculate moment of inertia based on the shape
                if hasattr(self, '_original_moment'):
                    self.body.moment = self._original_moment
                else:
                    # If we don't have the original moment, calculate a reasonable one
                    # This is a fallback - ideally we should store the original moment
                    self.body.moment = pymunk.moment_for_box(self.mass, (50, 50))  # Default safe moment
                    print(f"WARNING: Using default moment for {self.game_object.name}")


    # ================ Debug Methods ================

    def __repr__(self):
        return (f"RigidBody(mass={self.mass}, velocity={self.velocity}, "
                f"angular_velocity={self.angular_velocity:.2f}, "
                f"kinematic={self.is_kinematic})")
