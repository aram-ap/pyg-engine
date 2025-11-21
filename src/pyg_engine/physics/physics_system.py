import pygame as pg
from pygame import Vector2
import pymunk
from .rigidbody import RigidBody
from .collider import Collider, BoxCollider, CircleCollider, CollisionInfo
from .material import PhysicsMaterial, Materials

class PhysicsSystem:
    """Physics system using Pymunk for realistic 2D physics simulation."""

    def __init__(self, engine):  # NEW: Accept engine reference for event dispatching
        self.engine = engine  # Store for dispatches

        # Create pymunk space with physics settings
        self.space = pymunk.Space()
        self.space.gravity = (0, -500)  # Standard physics: negative Y is downward
        self.space.damping = 0.98  # More damping to reduce oscillations
        self.space.iterations = 10  # Reduced for better performance while maintaining stability

        # Collision layers for filtering
        self.collision_layers = {
            "Default": ["Default"],
            "Player": ["Player", "Environment"],
            "Environment": ["Player"],
            "NoCollision": []
        }

        # Collision tracking and mapping
        self._active_collisions = {}
        self._collider_map = {}  # Map pymunk shapes to our colliders
        self.enable_debug = False

        # Pause state
        self.paused = False
        self.space._prev_gravity = self.space.gravity
        self.space._prev_damping = self.space.damping

        # Physics settings for fixed timestep
        self.iterations = 10  # Reduced for better performance while maintaining stability
        self.dt_fixed = 1.0/120.0  # Fixed timestep for stability
        self.accumulator = 0.0  # For fixed timestep accumulation

        print("PhysicsSystem initialized")

    def update(self, engine, game_objects):
        """Main physics update with fixed timestep."""
        dt = engine.dt()
        if dt <= 0:
            return

        # Accumulate time for fixed timestep
        self.accumulator += dt

        # Process physics in fixed timesteps for stability
        while self.accumulator >= self.dt_fixed:
            self._physics_step(engine, game_objects, self.dt_fixed)
            self.accumulator -= self.dt_fixed

        # Update GameObject positions/rotations from physics bodies
        self._sync_gameobjects_from_physics(game_objects)

    def _physics_step(self, engine, game_objects, dt):
        """Perform one physics step."""
        if self.paused:
            return

        # Step 1: Ensure all physics objects are properly set up
        self._setup_physics_objects(game_objects)

        # Step 2: Apply forces and handle user input
        self._apply_forces_and_input(game_objects, dt)

        # Step 3: Step the physics simulation
        self.space.step(dt)

        # Step 4: Handle collision events (passing engine for dispatches)
        self._handle_collision_events(engine)

    def _setup_physics_objects(self, game_objects):
        """Ensure all physics objects have proper pymunk bodies and shapes."""
        for obj in game_objects:
            if not obj.enabled:
                continue

            rb = obj.get_component(RigidBody)
            collider = obj.get_component(Collider)

            if rb and collider and not rb.body:
                self._create_physics_body(obj, rb, collider)

    def _create_physics_body(self, game_object, rigidbody, collider):
        """Create pymunk body and shape for a game object."""

        # Create body based on kinematic state
        if rigidbody.is_kinematic:
            body = pymunk.Body(body_type=pymunk.Body.KINEMATIC)
        else:
            # Calculate moment of inertia based on shape for dynamic bodies
            if isinstance(collider, BoxCollider):
                moment = pymunk.moment_for_box(rigidbody.mass, (collider.width, collider.height))
            elif isinstance(collider, CircleCollider):
                # Use proper moment calculation for a solid circle (inner radius = 0)
                moment = pymunk.moment_for_circle(rigidbody.mass, 0, collider.radius, (0, 0))
            else:
                moment = pymunk.moment_for_box(rigidbody.mass, (32, 32))  # Default

            body = pymunk.Body(rigidbody.mass, moment)

            # Store the original moment for rotation freezing/unfreezing
            rigidbody._original_moment = moment

        # Set initial position and rotation
        body.position = (game_object.position.x, game_object.position.y)
        body.angle = game_object.rotation * 3.14159 / 180  # Convert to radians, Y-flip handled in rendering

        # Create shape based on collider type
        if isinstance(collider, CircleCollider):
            # Create circle shape - always centered to prevent rocking
            offset = (0, 0)  # Force center to prevent center-of-mass issues
            shape = pymunk.Circle(body, collider.radius, offset)
        elif isinstance(collider, BoxCollider):
            # Create box shape
            half_width = collider.width / 2
            half_height = collider.height / 2
            vertices = [
                (-half_width, -half_height),
                (half_width, -half_height),
                (half_width, half_height),
                (-half_width, half_height)
            ]
            # Apply offset to vertices instead of using offset parameter
            if collider.offset.x != 0 or collider.offset.y != 0:
                offset_vertices = []
                for vertex in vertices:
                    offset_vertices.append((vertex[0] + collider.offset.x, vertex[1] + collider.offset.y))
                vertices = offset_vertices
            shape = pymunk.Poly(body, vertices)
        else:
            # Default box shape
            shape = pymunk.Poly.create_box(body, (32, 32))

        # Set material properties
        material = collider.material
        if isinstance(material, str):
            # Convert string material to PhysicsMaterial object
            from material import Materials
            if material == "Player":
                material = Materials.RUBBER  # Good for player movement
            elif material == "Environment":
                material = Materials.WOOD  # Good for platforms
            else:
                material = Materials.DEFAULT  # Fallback

        shape.friction = material.friction
        shape.elasticity = material.bounce

        # Set collision filters based on collision layers
        shape.collision_type = self._get_collision_type(collider.collision_layer)

        # Set sensor (trigger) property
        shape.sensor = collider.is_trigger

        # Store references
        rigidbody.body = body
        rigidbody.shape = shape
        collider.shape = shape

        # CRITICAL FIX: Store original moment of inertia for kinematic transitions
        if not rigidbody.is_kinematic:
            rigidbody._original_moment = body.moment

        # Map shape to collider for collision callbacks
        self._collider_map[shape] = collider

        # Add to space
        if not rigidbody.is_kinematic:
            self.space.add(body, shape)
        else:
            self.space.add(body, shape)

        # Apply initial velocity if any
        if hasattr(rigidbody, '_pygame_velocity'):
            body.velocity = (rigidbody._pygame_velocity.x, rigidbody._pygame_velocity.y)
        if hasattr(rigidbody, '_pygame_angular_velocity'):
            body.angular_velocity = rigidbody._pygame_angular_velocity

        print(f"Created pymunk body for {game_object.name}: mass={rigidbody.mass}, "
              f"moment={body.moment:.2f}, kinematic={rigidbody.is_kinematic}")

    def _get_collision_type(self, layer_name):
        """Convert collision layer name to collision type number."""
        layer_map = {
            "Default": 1,
            "Player": 2,
            "Environment": 3,
            "NoCollision": 4
        }
        return layer_map.get(layer_name, 1)

    def _apply_forces_and_input(self, game_objects, dt):
        """Apply gravity scaling and other forces."""
        for obj in game_objects:
            if not obj.enabled:
                continue

            rb = obj.get_component(RigidBody)
            if rb and rb.body and not rb.is_kinematic:
                # Apply gravity scaling
                if rb.use_gravity and rb.gravity_scale != 1.0:
                    # Cancel default gravity and apply scaled version
                    gravity_force = (0, self.space.gravity[1] * rb.mass * (rb.gravity_scale - 1.0))
                    rb.body.apply_force_at_local_point(gravity_force, (0, 0))

                # Apply drag (air resistance) - let Pymunk handle this more naturally
                if rb.drag > 0:
                    velocity = rb.body.velocity
                    drag_force = (-velocity.x * rb.drag * rb.mass,
                                 -velocity.y * rb.drag * rb.mass)
                    rb.body.apply_force_at_local_point(drag_force, (0, 0))

                # Apply angular drag
                if rb.angular_drag > 0:
                    angular_drag_torque = -rb.body.angular_velocity * rb.angular_drag * rb.body.moment
                    rb.body.torque += angular_drag_torque

    def _sync_gameobjects_from_physics(self, game_objects):
        """Update GameObject positions/rotations from pymunk bodies."""
        for obj in game_objects:
            if not obj.enabled:
                continue

            rb = obj.get_component(RigidBody)
            if rb and rb.body:
                # Don't override position for kinematic bodies that might be manually controlled
                # Only update position for dynamic bodies, or kinematic bodies that aren't being dragged
                is_being_dragged = hasattr(obj, 'is_dragging') and obj.is_dragging

                if not rb.is_kinematic or not is_being_dragged:
                    # Update GameObject position and rotation from physics
                    # Check for NaN values before updating
                    body_x, body_y = rb.body.position.x, rb.body.position.y
                    if not (body_x != body_x or body_y != body_y):  # NaN check
                        obj.position.x = body_x
                        obj.position.y = body_y
                        # Check if this is a circle - circles need inverted rotation for correct rolling
                        from .collider import CircleCollider
                        is_circle = obj.get_component(CircleCollider) is not None

                        if is_circle:
                            obj.rotation = -rb.body.angle * 180 / 3.14159  # Invert for correct rolling
                        else:
                            obj.rotation = rb.body.angle * 180 / 3.14159  # Direct conversion for rectangles
                    else:
                        print(f"WARNING: NaN position detected for {obj.name}, resetting to origin")
                        obj.position.x = 0
                        obj.position.y = 0
                        rb.body.position = (0, 0)
                        rb.body.velocity = (0, 0)

                # Update rigidbody component
                rb.update(None)

    def _handle_collision_events(self, engine):  # UPDATED: Accept engine for dispatching
        """Handle collision enter/stay/exit events using bounding box collision detection."""
        # print(f"DEBUG: Checking collisions for {len(list(self._collider_map.values()))} colliders")


        current_collisions = set()

        # Get all colliders
        colliders = list(self._collider_map.values())

        # Check collisions between all pairs using bounding boxes
        for i, collider_a in enumerate(colliders):
            for j, collider_b in enumerate(colliders):
                if i >= j:  # Skip self and duplicates
                    continue

                if not self._should_collide(collider_a, collider_b):
                    continue

                # Update bounds first
                collider_a.update_bounds()
                collider_b.update_bounds()

                # Simple bounding box collision check
                if collider_a.bounds.colliderect(collider_b.bounds):
                    # print(f"DEBUG: Collision detected between {collider_a.game_object.name} and {collider_b.game_object.name}")

                    pair_id = tuple(sorted([id(collider_a), id(collider_b)]))
                    current_collisions.add(pair_id)

                    # Create approximate collision info
                    center_a = Vector2(collider_a.bounds.centerx, collider_a.bounds.centery)
                    center_b = Vector2(collider_b.bounds.centerx, collider_b.bounds.centery)

                    # Calculate normal from a to b
                    if center_a.distance_to(center_b) > 0:
                        normal = (center_b - center_a).normalize()
                    else:
                        normal = Vector2(1, 0)  # Default normal

                    # Contact point is between the two centers
                    contact_point = (center_a + center_b) / 2

                    # Approximate penetration depth
                    overlap_x = min(collider_a.bounds.right, collider_b.bounds.right) - max(collider_a.bounds.left, collider_b.bounds.left)
                    overlap_y = min(collider_a.bounds.bottom, collider_b.bounds.bottom) - max(collider_a.bounds.top, collider_b.bounds.top)
                    penetration = min(overlap_x, overlap_y)

                    # Check if this is a new collision
                    if pair_id not in self._active_collisions:
                        self._active_collisions[pair_id] = (collider_a, collider_b)

                        # Trigger enter events
                        info_a = CollisionInfo(collider_b, contact_point, normal, penetration)
                        info_b = CollisionInfo(collider_a, contact_point, -normal, penetration)

                        collider_a.handle_collision(info_a)
                        collider_b.handle_collision(info_b)

                        # Dispatch collision_enter event
                        engine.dispatch_event(
                            "collision_enter",
                            data={
                                "collider_a": collider_a,
                                "collider_b": collider_b,
                                "gameobject_a": collider_a.game_object,
                                "gameobject_b": collider_b.game_object,
                                "info_a": info_a,
                                "info_b": info_b
                            }
                        )

                        if self.enable_debug:
                            print(f"Collision ENTER: {collider_a.game_object.name} vs {collider_b.game_object.name}")
                    else:
                        # Trigger stay events
                        info_a = CollisionInfo(collider_b, contact_point, normal, penetration)
                        info_b = CollisionInfo(collider_a, contact_point, -normal, penetration)

                        collider_a.handle_collision(info_a)
                        collider_b.handle_collision(info_b)

                        # NEW: Dispatch collision_stay event (optional; comment out if too frequent)
                        engine.dispatch_event(
                            "collision_stay",
                            data={
                                "collider_a": collider_a,
                                "collider_b": collider_b,
                                "gameobject_a": collider_a.game_object,
                                "gameobject_b": collider_b.game_object,
                                "info_a": info_a,
                                "info_b": info_b
                            }
                        )

        # Check for ended collisions
        ended_pairs = set(self._active_collisions.keys()) - current_collisions
        for pair_id in ended_pairs:
            collider_a, collider_b = self._active_collisions[pair_id]
            collider_a.end_collision(collider_b)
            collider_b.end_collision(collider_a)
            del self._active_collisions[pair_id]

            # NEW: Dispatch collision_exit event
            engine.dispatch_event(
                "collision_exit",
                data={
                    "collider_a": collider_a,
                    "collider_b": collider_b,
                    "gameobject_a": collider_a.game_object,
                    "gameobject_b": collider_b.game_object
                }
            )

            if self.enable_debug:
                print(f"Collision EXIT: {collider_a.game_object.name} vs {collider_b.game_object.name}")

    def _should_collide(self, collider_a, collider_b):
        """Check if two colliders should collide based on collision layers."""
        layer_a = collider_a.collision_layer
        layer_b = collider_b.collision_layer
        allowed = self.collision_layers.get(layer_a, [])
        return layer_b in allowed

    def _combine_materials(self, mat_a, mat_b):
        """Combine materials for physics properties."""
        bounce = max(mat_a.bounce, mat_b.bounce)
        friction = (mat_a.friction + mat_b.friction) / 2
        return PhysicsMaterial("Combined", bounce, friction)

    def add_object(self, game_object):
        """Add a game object to the physics simulation."""
        rb = game_object.get_component(RigidBody)
        collider = game_object.get_component(Collider)

        if rb and collider:
            self._create_physics_body(game_object, rb, collider)

    def remove_object(self, game_object):
        """Remove a game object from the physics simulation."""
        rb = game_object.get_component(RigidBody)
        collider = game_object.get_component(Collider)

        if rb and rb.body:
            if rb.shape in self._collider_map:
                del self._collider_map[rb.shape]

            try:
                if rb.body in self.space.bodies:
                    self.space.remove(rb.body)
                if rb.shape in self.space.shapes:
                    self.space.remove(rb.shape)
            except:
                pass  # Object might already be removed

            rb.body = None
            rb.shape = None

        if collider:
            collider.shape = None

    def set_gravity(self, gravity_x, gravity_y):
        """Set the global gravity."""
        self.space.gravity = (gravity_x, gravity_y)
        print(f"Gravity set to ({gravity_x}, {gravity_y})")

    def get_gravity(self):
        """Get the current global gravity."""
        return Vector2(self.space.gravity[0], self.space.gravity[1])

    def pause(self):
        """Pause the physics simulation."""
        self.paused = True
        self.space._prev_gravity = self.space.gravity
        self.space._prev_damping = self.space.damping
        self.space.gravity = (0, 0)
        self.space.damping = 1.0

        # NEW: Dispatch physics_paused event
        self.engine.dispatch_event("physics_paused")

    def unpause(self):
        """Unpause the physics simulation."""
        self.paused = False
        self.space.gravity = self.space._prev_gravity
        self.space.damping = self.space._prev_damping

        # NEW: Dispatch physics_resumed event
        self.engine.dispatch_event("physics_resumed")

    def toggle_pause(self):
        """Toggle the pause state of the physics simulation."""
        if self.paused:
            self.unpause()
        else:
            self.pause()

    def set_damping(self, damping):
        """Set the damping of the physics simulation."""
        self.space.damping = damping
        self.space._prev_damping = damping

    def __del__(self):
        """Clean up the physics space."""
        if hasattr(self, 'space'):
            # Clear all bodies and shapes
            for body in list(self.space.bodies):
                self.space.remove(body)
            for shape in list(self.space.shapes):
                self.space.remove(shape)
            print("PhysicsSystem cleaned up")

