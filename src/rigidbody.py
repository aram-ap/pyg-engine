import pygame as pg
from pygame import Vector2
from .component import Component

class RigidBody(Component):
    """Physics component with angular motion support."""

    def __init__(self, game_object, mass=1.0, gravity_scale=1.0, drag=0.0,
                 use_gravity=True, is_kinematic=False, angular_drag=0.1):
        super().__init__(game_object)

        # Linear physics
        self.mass = max(0.1, mass)
        self.velocity = Vector2(0, 0)
        self.acceleration = Vector2(0, 0)
        self.gravity_scale = gravity_scale
        self.drag = max(0.0, drag)
        self.use_gravity = use_gravity
        self.is_kinematic = is_kinematic
        self.gravity = Vector2(0, 980)

        # Angular physics
        self.angular_velocity = 0.0  # radians per second
        self.angular_acceleration = 0.0  # radians per second²
        self.angular_drag = angular_drag  # rotational air resistance
        self._moment_of_inertia = None  # Will be calculated when needed
        self._cached_collider_info = None  # Cache to detect changes
        self._shape_type = None  # Track if this is a box or circle

        # Runtime state
        self._forces = []
        self._torques = []  # List of torques to apply this frame
        self.previous_position = Vector2(0, 0)  # For Verlet integration

        print(f"RigidBody created with mass={self.mass}, gravity_scale={self.gravity_scale}")

    @property
    def moment_of_inertia(self):
        """Lazy calculation of moment of inertia."""
        try:
            from .collider import Collider, BoxCollider, CircleCollider
        except ImportError:
            return self._fallback_moment_of_inertia()

        collider = self.game_object.get_component(Collider)

        # Check if we need to recalculate
        current_info = None
        shape_type = None
        if collider:
            if hasattr(collider, 'width') and hasattr(collider, 'height'):
                current_info = (collider.width, collider.height, self.mass)
                shape_type = 'box'
            elif hasattr(collider, 'radius'):
                current_info = (collider.radius, self.mass)
                shape_type = 'circle'

        if (self._moment_of_inertia is None or
            self._cached_collider_info != current_info or
            self._shape_type != shape_type):

            self._cached_collider_info = current_info
            self._shape_type = shape_type
            self._moment_of_inertia = self._calculate_moment_of_inertia()

            # Set appropriate angular drag based on shape
            if shape_type == 'box':
                self.angular_drag = max(self.angular_drag, 0.15)  # Boxes need more drag

        return self._moment_of_inertia

    def _calculate_moment_of_inertia(self):
        """Calculate moment of inertia based on shape and mass."""
        try:
            from .collider import Collider, BoxCollider, CircleCollider
        except ImportError:
            return self._fallback_moment_of_inertia()

        collider = self.game_object.get_component(Collider)

        if collider:
            if hasattr(collider, 'width') and hasattr(collider, 'height'):
                # Box collider - moment of inertia for rectangle about center
                width = collider.width
                height = collider.height

                # For a rectangular box: I = (mass/12) * (width² + height²)
                # This is the correct formula for a solid rectangle rotating about its center
                box_inertia = self.mass * (width * width + height * height) / 12.0

                # Ensure minimum moment of inertia to prevent division by zero
                return max(box_inertia, 0.1)

            elif hasattr(collider, 'radius'):
                # Circle collider - moment of inertia for solid disk
                radius = collider.radius

                # For a solid disk: I = (1/2) * mass * radius²
                circle_inertia = 0.5 * self.mass * radius * radius

                # Ensure minimum moment of inertia
                return max(circle_inertia, 0.1)

        return self._fallback_moment_of_inertia()

    def apply_gravity_torque(self, contact_point):
        """Apply rotational torque from gravity if center of mass is offset from contact."""
        if not self.use_gravity:
            return 0.0

        # Vector from center to contact point
        r = contact_point - self.game_object.position

        # Gravity force vector (downward)
        gravity_force = Vector2(0, self.mass * self.gravity.y * self.gravity_scale)

        # Torque = r x F (2D cross product)
        torque = r.x * gravity_force.y - r.y * gravity_force.x

        # Adjust sign for coordinate system (y-down)
        return -torque  # Flip to match clockwise positive rotation

    def _fallback_moment_of_inertia(self):
        """Fallback calculation when collider is not available."""
        size = max(self.game_object.size.x, self.game_object.size.y)
        if size <= 0:
            size = 32

        # Assume square shape for fallback
        return max(self.mass * (size * size) / 6.0, 0.1)

    def start(self):
        print(f"RigidBody started on {self.game_object.name}")
        self.previous_position = self.game_object.position.copy()

        # Force recalculation of moment of inertia
        self._moment_of_inertia = None
        self._cached_collider_info = None
        self._shape_type = None

        # Trigger calculation to set up shape-specific properties
        _ = self.moment_of_inertia

        # Log the calculated values for debugging
        print(f"  Shape: {self._shape_type}, Moment of Inertia: {self._moment_of_inertia:.2f}, Angular Drag: {self.angular_drag}")

    def update(self, engine):
        """RigidBody update - physics simulation is handled by PhysicsSystem."""
        pass

    # ================ Force/Torque Application Methods ================

    def add_force(self, force):
        """Add a force to be applied this frame."""
        if isinstance(force, (list, tuple)):
            force = Vector2(force)
        self._forces.append(force)

    def add_torque(self, torque):
        """Add a torque (rotational force) to be applied this frame."""
        self._torques.append(torque)

    def add_force_at_point(self, force, point):
        """Add a force at a specific world point (creates both force and torque)."""
        if isinstance(force, (list, tuple)):
            force = Vector2(force)
        if isinstance(point, (list, tuple)):
            point = Vector2(point)

        # Add the force
        self.add_force(force)

        # Calculate torque: torque = cross_product(r, F) where r is from center of mass to point
        r = point - self.game_object.position
        torque = r.x * force.y - r.y * force.x  # 2D cross product
        self.add_torque(torque)

    def add_impulse(self, impulse):
        """Apply an immediate change to velocity."""
        if isinstance(impulse, (list, tuple)):
            impulse = Vector2(impulse)
        self.velocity += impulse

    def add_angular_impulse(self, angular_impulse):
        """Apply an immediate change to angular velocity."""
        self.angular_velocity += angular_impulse / self.moment_of_inertia

    def set_velocity(self, velocity):
        """Directly set the velocity."""
        if isinstance(velocity, (list, tuple)):
            velocity = Vector2(velocity)
        self.velocity = velocity

    def add_velocity(self, velocity):
        """Add to the current velocity."""
        if isinstance(velocity, (list, tuple)):
            velocity = Vector2(velocity)
        self.velocity += velocity

    # ================ Utility Methods ================

    def get_speed(self):
        """Get the current linear speed."""
        return self.velocity.length()

    def get_angular_speed(self):
        """Get the current angular speed (absolute value)."""
        return abs(self.angular_velocity)

    def get_kinetic_energy(self):
        """Get total kinetic energy (linear + rotational)."""
        linear_ke = 0.5 * self.mass * (self.velocity.length_squared())
        rotational_ke = 0.5 * self.moment_of_inertia * (self.angular_velocity * self.angular_velocity)
        return linear_ke + rotational_ke

    def stop(self):
        """Stop all movement."""
        self.velocity = Vector2(0, 0)
        self.acceleration = Vector2(0, 0)
        self.angular_velocity = 0.0
        self.angular_acceleration = 0.0
        self._forces.clear()
        self._torques.clear()

    def freeze_rotation(self):
        """Stop rotational movement."""
        self.angular_velocity = 0.0
        self.angular_acceleration = 0.0
        self._torques.clear()

    # ================ Configuration Methods ================

    def set_mass(self, mass):
        """Change the mass at runtime."""
        self.mass = max(0.1, mass)
        self._moment_of_inertia = None  # Force recalculation
        print(f"RigidBody mass changed to {self.mass}")

    def set_gravity_scale(self, scale):
        """Change how much gravity affects this object."""
        self.gravity_scale = scale
        print(f"RigidBody gravity scale changed to {self.gravity_scale}")

    def set_kinematic(self, is_kinematic):
        """Enable/disable kinematic mode."""
        self.is_kinematic = is_kinematic
        if is_kinematic:
            self.stop()
        print(f"RigidBody kinematic mode: {self.is_kinematic}")

    # ================ Debug Methods ================

    def __repr__(self):
        return (f"RigidBody(mass={self.mass}, velocity={self.velocity}, "
                f"angular_velocity={self.angular_velocity:.2f}, "
                f"moment_of_inertia={self.moment_of_inertia:.2f}, "
                f"shape={self._shape_type}, kinematic={self.is_kinematic})")

