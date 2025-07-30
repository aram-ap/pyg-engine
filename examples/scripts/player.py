"""
Player script for Pyg Engine examples
"""

import pygame as pg
import random
from pygame import Vector2, Color
from pyg_engine import Script, PymunkRigidBody, MouseHoverComponent, MouseClickComponent, MouseButton

class PlayerScript(Script):
    """A script that handles player input and movement."""
    
    def __init__(self, gameobject, speed=300, player_id=1, 
                 control_mode="velocity", movement_keys=None,
                 use_drag_control=True, drag_force_multiplier=1.0,
                 boundary_collision=False, **kwargs):
        super().__init__(gameobject)
        self.gameobject = gameobject
        
        self.speed = speed
        self.player_id = player_id
        self.control_mode = control_mode
        self.use_drag_control = use_drag_control
        self.drag_force_multiplier = drag_force_multiplier
        self.boundary_collision = boundary_collision
        
        # Drag state
        self.is_dragging = False
        self.drag_offset = Vector2(0, 0)
        self.original_color = self.gameobject.color
        
        # Pymunk constraint-based dragging
        self.mouse_joint = None
        self.mouse_body = None
        
        # Set up mouse components if drag control is enabled
        if self.use_drag_control:
            self._setup_mouse_components()
        
        # Default movement keys (WASD)
        self.movement_keys = movement_keys or {
            'up': [pg.K_w, pg.K_UP],
            'down': [pg.K_s, pg.K_DOWN],
            'left': [pg.K_a, pg.K_LEFT],
            'right': [pg.K_d, pg.K_RIGHT]
        }
        
        # Additional configuration
        for key, value in kwargs.items():
            setattr(self, key, value)
        
        # Get rigidbody component
        self.rigidbody = self.gameobject.get_component(PymunkRigidBody)
        
        print(f"Player script started on {self.gameobject.name} (ID: {self.player_id}) with speed: {self.speed}")
        print(f"Control mode: {self.control_mode}")
        if self.rigidbody:
            print(f"Rigidbody found: mass={self.rigidbody.mass}, kinematic={self.rigidbody.is_kinematic}")
        else:
            print("Warning: No PymunkRigidBody found on", self.gameobject.name)
    
    def update(self, engine):
        """Update player movement based on input."""
        # Handle manual drag system first
        if self.use_drag_control:
            self._handle_manual_drag(engine)
        
        # Only handle keyboard movement if not dragging
        if not self.is_dragging:
            keys = pg.key.get_pressed()
            
            # Calculate movement vector in world coordinates
            movement = Vector2(0, 0)
            
            # Keyboard input - PHYSICS coordinates (up = +Y, down = -Y, left = -X, right = +X)
            for direction, key_list in self.movement_keys.items():
                if any(keys[key] for key in key_list):
                    if direction == 'up':
                        movement.y += 1  # Physics up is positive Y (opposite of screen coordinates)
                    elif direction == 'down':
                        movement.y -= 1  # Physics down is negative Y (opposite of screen coordinates)
                    elif direction == 'left':
                        movement.x -= 1  # Physics left is negative X
                    elif direction == 'right':
                        movement.x += 1  # Physics right is positive X
            
            # Normalize movement vector for consistent speed
            if movement.length() > 0:
                movement = movement.normalize()
            
            # Apply movement based on control mode (WORLD COORDINATES ONLY)
            if self.rigidbody and movement.length() > 0:
                # Remove debug prints for cleaner output
                if self.control_mode == "force":
                    # Apply force-based movement - WORLD FORCE (not relative to rotation)
                    world_force = movement * self.speed
                    self.rigidbody.add_force(world_force)
                    
                elif self.control_mode == "velocity":
                    # Apply velocity-based movement - WORLD VELOCITY (not relative to rotation)
                    target_velocity = movement * self.speed
                    current_velocity = Vector2(self.rigidbody.velocity)
                    
                    # Smooth velocity transition for better feel
                    velocity_smoothing = getattr(self, 'velocity_smoothing', 0.8)
                    new_velocity = current_velocity.lerp(target_velocity, 1 - velocity_smoothing)
                    
                    # No artificial speed limits - let physics handle it naturally
                    self.rigidbody.velocity = new_velocity
    
    def set_speed(self, speed):
        """Set the player's movement speed."""
        self.speed = speed
    
    def set_movement_keys(self, direction, keys):
        """Set movement keys for a specific direction."""
        self.movement_keys[direction] = keys
    
    def _setup_mouse_components(self):
        """Set up mouse components for drag functionality."""
        # Add mouse components to the gameobject
        self.gameobject.add_component(MouseHoverComponent)
        self.gameobject.add_component(MouseClickComponent)
        
        # Get the components and set up callbacks
        hover_comp = self.gameobject.get_component(MouseHoverComponent)
        click_comp = self.gameobject.get_component(MouseClickComponent)
        
        if hover_comp:
            hover_comp.add_hover_callback(self._on_hover_event)
            
        if click_comp:
            # Only use click callback - drag system will be manual
            click_comp.add_click_callback(MouseButton.LEFT, self._on_left_click)
            
        # Additional drag state
        self.click_in_bounds = False
        self.mouse_down_start_pos = Vector2(0, 0)
        self.last_mouse_pos = Vector2(0, 0)
        self.drag_velocity = Vector2(0, 0)
    
    def _on_hover_event(self, event_type, mouse_pos, world_pos):
        """Handle hover events."""
        if event_type == 'enter' and not self.is_dragging:
            # Change color slightly when hovering
            self.gameobject.color = Color(min(255, self.original_color.r + 30), 
                                        min(255, self.original_color.g + 30), 
                                        min(255, self.original_color.b + 30))
        elif event_type == 'exit' and not self.is_dragging:
            self.gameobject.color = self.original_color

    def _on_left_click(self, button, mouse_pos, world_pos):
        """Handle left mouse click - prepare for dragging."""
        if not self.is_dragging:
            # Mark that a click occurred in bounds
            self.click_in_bounds = True
            self.mouse_down_start_pos = world_pos.copy()
            self.last_mouse_pos = world_pos.copy()
            
            # Calculate pivot point (where on the object you clicked)
            # This is the offset from the object's center to the click point
            self.pivot_offset = world_pos - self.gameobject.position
            
            # For dragging, we want the pivot point to follow the mouse
            # So the drag offset is the pivot offset
            self.drag_offset = self.pivot_offset
            
            print(f"Clicked on {self.gameobject.name} - mouse_pos: {mouse_pos}, world_pos: {world_pos}, object_pos: {self.gameobject.position}, pivot_offset: {self.pivot_offset}")
    
    def _handle_manual_drag(self, engine):
        """Handle drag system using Pymunk constraints (based on pymunk examples)."""
        # Get current mouse state
        mouse_pressed = pg.mouse.get_pressed()[0]  # Left button
        
        # Get world position from the engine's mouse input system
        if hasattr(engine, 'mouse_input') and engine.mouse_input:
            world_pos = engine.mouse_input.current_state.world_position
        else:
            # Fallback: manual conversion
            mouse_pos = Vector2(pg.mouse.get_pos())
            if hasattr(engine, 'camera'):
                world_pos = (mouse_pos / engine.camera.zoom) + engine.camera.position - Vector2(engine.getWindowSize().w, engine.getWindowSize().h) / (2 * engine.camera.zoom)
            else:
                world_pos = mouse_pos
        
        # Initialize mouse body if needed
        if self.mouse_body is None and self.rigidbody and self.rigidbody.body:
            import pymunk
            self.mouse_body = pymunk.Body(body_type=pymunk.Body.KINEMATIC)
        
        if mouse_pressed and self.click_in_bounds:
            if not self.is_dragging:
                # Start dragging with constraint-based system
                self.is_dragging = True
                self.gameobject.color = Color(255, 165, 0)  # Orange when dragging
                
                if self.rigidbody and self.rigidbody.body and self.mouse_body:
                    # Create a PivotJoint between mouse body and object body
                    # This is the key insight from the pymunk constraints example
                    import pymunk
                    
                    # Calculate the local point on the object where we clicked
                    if hasattr(self, 'pivot_offset') and self.pivot_offset.length() > 0:
                        local_point = (self.pivot_offset.x, self.pivot_offset.y)
                    else:
                        local_point = (0, 0)
                    
                    # Create the pivot joint (mouse_body -> object_body)
                    self.mouse_joint = pymunk.PivotJoint(
                        self.mouse_body,
                        self.rigidbody.body,
                        (0, 0),  # Anchor on mouse body (always at origin)
                        local_point  # Anchor on object body (where we clicked)
                    )
                    
                    # Configure the joint for tight, responsive dragging
                    self.mouse_joint.max_force = 200000  # Much stronger force for tighter control
                    self.mouse_joint.error_bias = (1 - 0.8) ** 60  # Much tighter correction (0.8 instead of 0.15)
                    
                    # Add damping to reduce swinging and make it stick to mouse more
                    if self.rigidbody.body:
                        # Apply heavy damping to current velocity
                        current_vel = Vector2(self.rigidbody.body.velocity.x, self.rigidbody.body.velocity.y)
                        damped_vel = current_vel * 0.3  # 70% damping for tighter control
                        self.rigidbody.body.velocity = (damped_vel.x, damped_vel.y)
                        
                        # Also damp angular velocity to reduce swinging
                        self.rigidbody.body.angular_velocity *= 0.3
                    
                    # Add the joint to the physics space
                    if hasattr(engine, 'physics_system') and hasattr(engine.physics_system, 'space'):
                        engine.physics_system.space.add(self.mouse_joint)
                    
                    print(f"{self.gameobject.name}: Started constraint-based dragging")
            
            # Continue dragging - update mouse body position with tighter control
            if self.is_dragging and self.mouse_body:
                self.mouse_body.position = (world_pos.x, world_pos.y)
                
                # Apply additional damping during drag to keep it tight to mouse
                if self.rigidbody and self.rigidbody.body:
                    # Damp velocity to reduce swinging and make it stick closer
                    current_vel = Vector2(self.rigidbody.body.velocity.x, self.rigidbody.body.velocity.y)
                    damped_vel = current_vel * 0.5  # 50% damping during drag
                    self.rigidbody.body.velocity = (damped_vel.x, damped_vel.y)
                    
                    # Also damp angular velocity to reduce rotation during drag
                    self.rigidbody.body.angular_velocity *= 0.5
                
                # Calculate drag velocity for throwing
                current_mouse_pos = world_pos
                if hasattr(self, 'last_mouse_pos') and self.last_mouse_pos != Vector2(0, 0):
                    mouse_delta = current_mouse_pos - self.last_mouse_pos
                    velocity_multiplier = 8.0
                    self.drag_velocity = mouse_delta * velocity_multiplier
                
                self.last_mouse_pos = current_mouse_pos
                
        elif not mouse_pressed and self.is_dragging:
            # Stop dragging
            self.is_dragging = False
            self.click_in_bounds = False
            self.gameobject.color = self.original_color
            
            # Remove the constraint joint
            if self.mouse_joint and hasattr(engine, 'physics_system') and hasattr(engine.physics_system, 'space'):
                engine.physics_system.space.remove(self.mouse_joint)
                self.mouse_joint = None
            
            # Apply throw velocity based on drag movement
            if self.rigidbody and hasattr(self, 'drag_velocity') and self.drag_velocity.length() > 2.0:
                throw_velocity = self.drag_velocity
                
                # Add randomness for natural feel
                random_factor = 0.9 + random.random() * 0.2
                throw_velocity *= random_factor
                
                self.rigidbody.body.velocity = (throw_velocity.x, throw_velocity.y)
                print(f"{self.gameobject.name}: Thrown with velocity {throw_velocity}")
            else:
                # Gentle drop
                if self.rigidbody:
                    current_vel = Vector2(self.rigidbody.body.velocity.x, self.rigidbody.body.velocity.y)
                    gentle_vel = current_vel * 0.3
                    self.rigidbody.body.velocity = (gentle_vel.x, gentle_vel.y)
                    print(f"{self.gameobject.name}: Dropped gently at {self.gameobject.position}")
            
            # Reset drag velocity
            self.drag_velocity = Vector2(0, 0)
            self.last_mouse_pos = Vector2(0, 0)
        
        elif not mouse_pressed:
            # Reset click state if mouse is released
            self.click_in_bounds = False 