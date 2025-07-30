import pygame as pg
from pygame import Vector2
from .rigidbody import RigidBody
from .collider import Collider, BoxCollider, CircleCollider
from .material import PhysicsMaterial, Materials
import math

class PhysicsSystem:
    """Physics system with collision detection and resolution."""
    
    def __init__(self):
        self.collision_layers = {
            "Default": ["Default"],
            "Player": ["Player", "Environment"],
            "Environment": ["Player"],
            "NoCollision": []
        }
        self._active_collisions = {}
        self.enable_debug = False
        print("PhysicsSystem initialized (enhanced version)")

    def update(self, engine, game_objects):
        """Main physics update loop."""
        dt = engine.dt()
        if dt <= 0:
            return

        # Get all physics objects
        physics_objects = []  # List of (obj, rb, col) tuples

        for obj in game_objects:
            if not obj.enabled:
                continue
            rb = obj.get_component(RigidBody)
            col = obj.get_component(Collider)
            if rb and col:
                physics_objects.append((obj, rb, col))

        # STEP 1: Update velocities (gravity, forces, drag)
        for obj, rb, col in physics_objects:
            if rb.is_kinematic:
                continue

            # Apply gravity
            if rb.use_gravity:
                rb.velocity.y += rb.gravity.y * rb.gravity_scale * dt

            # Apply linear drag
            if rb.drag > 0:
                rb.velocity *= (1.0 - rb.drag * dt)

            # Apply angular drag
            if rb.angular_drag > 0:
                rb.angular_velocity *= (1.0 - rb.angular_drag * dt)

            # Apply accumulated forces
            for force in rb._forces:
                rb.velocity += (force / rb.mass) * dt
            rb._forces.clear()

            # Apply accumulated torques
            total_torque = sum(rb._torques)
            if total_torque != 0:
                rb.angular_velocity += (total_torque / rb.moment_of_inertia) * dt
            rb._torques.clear()

            # No artificial velocity limits - let physics handle it naturally

            # Clamp angular velocity
            max_angular_speed = 20  # radians per second
            rb.angular_velocity = max(-max_angular_speed, min(max_angular_speed, rb.angular_velocity))

        # STEP 2: Move objects and rotate them
        for obj, rb, col in physics_objects:
            if not rb.is_kinematic:
                obj.position += rb.velocity * dt
                obj.rotation += rb.angular_velocity * (180 / 3.14159) * dt  # Convert to degrees

        # STEP 3: Update all collider bounds
        for obj, rb, col in physics_objects:
            col.update_bounds()

        # STEP 4: Detect all collisions
        collisions = []
        for i, (obj_a, rb_a, col_a) in enumerate(physics_objects):
            for j, (obj_b, rb_b, col_b) in enumerate(physics_objects):
                if i >= j:  # Skip self and duplicates
                    continue

                if not self._should_collide(col_a, col_b):
                    continue

                collision = col_a.check_collision(col_b)
                if collision and collision.penetration_depth > 0:
                    collisions.append({
                        'obj_a': obj_a, 'rb_a': rb_a, 'col_a': col_a,
                        'obj_b': obj_b, 'rb_b': rb_b, 'col_b': col_b,
                        'info': collision
                    })

        # STEP 5: Resolve all collisions
        for collision_data in collisions:
            self._resolve_collision(collision_data, dt)

        # STEP 6: Handle collision events
        self._update_collision_events(collisions)

        # STEP 7: Enhanced damping and sleep logic
        for obj, rb, col in physics_objects:
            if rb.is_kinematic:
                continue

            speed = rb.velocity.length()
            angular_speed = abs(rb.angular_velocity)

            # Velocity-based damping: stronger at low speeds to prevent oscillation
            if speed < 20:
                rb.velocity *= (1 - 0.3 * dt)  # 30% damping per second at low speed
            if angular_speed < 1.0:
                rb.angular_velocity *= (1 - 0.5 * dt)  # 50% angular damping per second

            # Sleep if below thresholds and in contact
            sleep_threshold_linear = 2.0
            sleep_threshold_angular = 0.05
            if speed < sleep_threshold_linear and angular_speed < sleep_threshold_angular:
                is_resting = any(id(col) in pair_id for pair_id in self._active_collisions)
                if is_resting:
                    rb.velocity = Vector2(0, 0)
                    rb.angular_velocity = 0
                    if self.enable_debug:
                        print(f"Sleeping {obj.name}: fully rested")

    def _resolve_collision(self, collision_data, dt):
        """Resolve a collision with separation and impulse."""
        obj_a = collision_data['obj_a']
        obj_b = collision_data['obj_b']
        rb_a = collision_data['rb_a']
        rb_b = collision_data['rb_b']
        col_a = collision_data['col_a']
        col_b = collision_data['col_b']
        info = collision_data['info']

        # Skip if either is a trigger
        if col_a.is_trigger or col_b.is_trigger:
            return

        # Calculate masses (kinematic = infinite mass)
        mass_a = rb_a.mass if not rb_a.is_kinematic else float('inf')
        mass_b = rb_b.mass if not rb_b.is_kinematic else float('inf')

        if mass_a == float('inf') and mass_b == float('inf'):
            return

        # PART 1: Positional correction (separate objects)
        if info.penetration_depth > 0.1:  # Small threshold to avoid jitter
            percent = 0.8  # How much to correct
            slop = 0.01  # Allowable penetration
            correction = info.contact_normal * (max(info.penetration_depth - slop, 0) * percent)

            if mass_a == float('inf'):
                obj_b.position -= correction
            elif mass_b == float('inf'):
                obj_a.position += correction
            else:
                # Distribute based on mass
                total_mass = mass_a + mass_b
                obj_a.position += correction * (mass_b / total_mass)
                obj_b.position -= correction * (mass_a / total_mass)

        # PART 2: Velocity resolution with angular physics

        # Calculate contact points relative to centers of mass
        r_a = info.contact_point - obj_a.position
        r_b = info.contact_point - obj_b.position

        # Calculate velocities at contact points (including rotation)
        vel_a = rb_a.velocity if rb_a else Vector2(0, 0)
        vel_b = rb_b.velocity if rb_b else Vector2(0, 0)

        # Add tangential velocity from rotation
        if not rb_a.is_kinematic:
            # Tangential velocity = angular_velocity × radius (perpendicular to radius)
            tangential_a = Vector2(-r_a.y, r_a.x) * rb_a.angular_velocity
            vel_a = vel_a + tangential_a

        if not rb_b.is_kinematic:
            tangential_b = Vector2(-r_b.y, r_b.x) * rb_b.angular_velocity
            vel_b = vel_b + tangential_b

        # Calculate relative velocity at contact point
        relative_velocity = vel_a - vel_b

        # Velocity along collision normal
        velocity_along_normal = relative_velocity.dot(info.contact_normal)

        # Don't resolve if velocities are separating
        if velocity_along_normal > 0:
            return

        # Get combined material properties
        material = self._combine_materials(col_a.material, col_b.material)

        # Calculate impulse magnitude with angular effects
        e = material.bounce  # Restitution

        # Calculate effective mass including rotational inertia
        r_a_cross_n = r_a.x * info.contact_normal.y - r_a.y * info.contact_normal.x
        r_b_cross_n = r_b.x * info.contact_normal.y - r_b.y * info.contact_normal.x

        effective_mass_inv = 0
        if mass_a != float('inf'):
            effective_mass_inv += 1 / mass_a + (r_a_cross_n * r_a_cross_n) / rb_a.moment_of_inertia
        if mass_b != float('inf'):
            effective_mass_inv += 1 / mass_b + (r_b_cross_n * r_b_cross_n) / rb_b.moment_of_inertia

        if effective_mass_inv == 0:
            return

        impulse_magnitude = -(1 + e) * velocity_along_normal / effective_mass_inv

        # Apply linear impulse
        impulse = info.contact_normal * impulse_magnitude

        if not rb_a.is_kinematic:
            rb_a.velocity += impulse / mass_a
            # Apply angular impulse
            angular_impulse_a = r_a.x * impulse.y - r_a.y * impulse.x  # 2D cross product
            rb_a.angular_velocity += angular_impulse_a / rb_a.moment_of_inertia

        if not rb_b.is_kinematic:
            rb_b.velocity -= impulse / mass_b
            # Apply angular impulse
            angular_impulse_b = r_b.x * impulse.y - r_b.y * impulse.x  # 2D cross product
            rb_b.angular_velocity -= angular_impulse_b / rb_b.moment_of_inertia

        # PART 3: Friction (tangent impulse) with angular effects
        # Calculate tangent vector (perpendicular to normal)
        tangent = Vector2(-info.contact_normal.y, info.contact_normal.x)
        velocity_along_tangent = relative_velocity.dot(tangent)

        if abs(velocity_along_tangent) > 0.01:  # Only apply if sliding
            friction_magnitude = abs(impulse_magnitude) * material.friction

            # Clamp friction (can't reverse direction)
            if abs(friction_magnitude) > abs(velocity_along_tangent):
                friction_magnitude = abs(velocity_along_tangent)

            # Calculate friction impulse with angular effects
            friction_direction = -1 if velocity_along_tangent > 0 else 1

            # Effective mass for tangential direction
            r_a_cross_t = r_a.x * tangent.y - r_a.y * tangent.x
            r_b_cross_t = r_b.x * tangent.y - r_b.y * tangent.x

            tangent_mass_inv = 0
            if mass_a != float('inf'):
                tangent_mass_inv += 1 / mass_a + (r_a_cross_t * r_a_cross_t) / rb_a.moment_of_inertia
            if mass_b != float('inf'):
                tangent_mass_inv += 1 / mass_b + (r_b_cross_t * r_b_cross_t) / rb_b.moment_of_inertia

            if tangent_mass_inv > 0:
                friction_impulse_magnitude = friction_magnitude * friction_direction / tangent_mass_inv
                friction_impulse = tangent * friction_impulse_magnitude

                if not rb_a.is_kinematic:
                    rb_a.velocity += friction_impulse / mass_a
                    # Apply angular friction
                    angular_friction_a = r_a.x * friction_impulse.y - r_a.y * friction_impulse.x
                    rb_a.angular_velocity += angular_friction_a / rb_a.moment_of_inertia

                if not rb_b.is_kinematic:
                    rb_b.velocity -= friction_impulse / mass_b
                    # Apply angular friction
                    angular_friction_b = r_b.x * friction_impulse.y - r_b.y * friction_impulse.x
                    rb_b.angular_velocity -= angular_friction_b / rb_b.moment_of_inertia

        # PART 4: Apply gravity torque for stability
        if isinstance(col_a, BoxCollider) and abs(info.contact_normal.y) > 0.7:
            gravity_torque = rb_a.apply_gravity_torque(info.contact_point)
            scaled_torque = gravity_torque * dt * (1 - info.penetration_depth * 0.1)  # Scale down if deeply penetrating (resting)
            rb_a.add_torque(scaled_torque)

        if isinstance(col_b, BoxCollider) and abs(info.contact_normal.y) > 0.7:
            gravity_torque = rb_b.apply_gravity_torque(info.contact_point)
            scaled_torque = gravity_torque * dt * (1 - info.penetration_depth * 0.1)
            rb_b.add_torque(scaled_torque)

        if self.enable_debug:
            print(f"Resolved collision: {obj_a.name} vs {obj_b.name}, impulse={impulse_magnitude:.2f}, "
                  f"angular_a={rb_a.angular_velocity:.2f}, angular_b={rb_b.angular_velocity:.2f}")

    def _update_collision_events(self, current_collisions):
        """Track and trigger collision enter/stay/exit events."""
        current_pairs = set()

        # Process current collisions
        for collision in current_collisions:
            col_a = collision['col_a']
            col_b = collision['col_b']
            info = collision['info']

            # Create unique pair ID
            pair_id = tuple(sorted([id(col_a), id(col_b)]))
            current_pairs.add(pair_id)

            # Check if this is a new collision
            if pair_id not in self._active_collisions:
                # New collision - trigger enter event
                self._active_collisions[pair_id] = (col_a, col_b)
                if self.enable_debug:
                    print(f"Collision ENTER: {collision['obj_a'].name} vs {collision['obj_b'].name}")

            # Trigger collision events
            col_a.handle_collision(info)
            reverse_info = self._create_reverse_collision_info(info, col_a)
            col_b.handle_collision(reverse_info)

        # Check for ended collisions
        ended_pairs = set(self._active_collisions.keys()) - current_pairs
        for pair_id in ended_pairs:
            col_a, col_b = self._active_collisions[pair_id]
            col_a.end_collision(col_b)
            col_b.end_collision(col_a)
            del self._active_collisions[pair_id]
            if self.enable_debug:
                print(f"Collision EXIT")

    def _should_collide(self, collider_a, collider_b):
        """Check collision layers."""
        layer_a = collider_a.collision_layer
        layer_b = collider_b.collision_layer
        allowed = self.collision_layers.get(layer_a, [])
        return layer_b in allowed

    def _combine_materials(self, mat_a, mat_b):
        """Combine materials for physics properties."""
        bounce = max(mat_a.bounce, mat_b.bounce)
        friction = (mat_a.friction + mat_b.friction) / 2
        return PhysicsMaterial("Combined", bounce, friction)

    def _create_reverse_collision_info(self, original_info, original_collider):
        """Create reversed collision info."""
        from collider import CollisionInfo
        return CollisionInfo(
            other_collider=original_collider,
            contact_point=original_info.contact_point,
            contact_normal=-original_info.contact_normal,
            penetration_depth=original_info.penetration_depth
        )

