"""
Advanced mouse interactions with physics example
"""

import pygame as pg
from pygame import Color, Vector2
from pyg_engine import Engine, GameObject, Size, BasicShape, MouseHoverComponent, MouseClickComponent, MouseWheelComponent, MouseButton, RigidBody, BoxCollider, CircleCollider

class DraggableObject(GameObject):
    """A draggable object that can be picked up and dropped."""
    
    def __init__(self, name, position, size, color, shape=BasicShape.Rectangle):
        super().__init__(name=name, position=position, size=size, color=color, basicShape=shape)
        
        # Add mouse input components
        self.add_component(MouseHoverComponent)
        self.add_component(MouseClickComponent)
        self.add_component(MouseWheelComponent)
        
        # Add physics components
        self.add_component(RigidBody, mass=1.0)
        if shape == BasicShape.Circle:
            self.add_component(CircleCollider, radius=size.x/2)
        else:
            self.add_component(BoxCollider, width=size.x, height=size.y)
        
        # State variables
        self.original_color = color
        self.hover_color = Color(255, 255, 0)  # Yellow when hovering
        self.drag_color = Color(255, 165, 0)    # Orange when dragging
        self.is_dragging = False
        self.drag_offset = Vector2(0, 0)
        self.scale = 1.0
        
        # Set up callbacks
        self._setup_callbacks()
    
    def _setup_callbacks(self):
        """Set up mouse event callbacks."""
        hover_comp = self.get_component(MouseHoverComponent)
        click_comp = self.get_component(MouseClickComponent)
        wheel_comp = self.get_component(MouseWheelComponent)
        
        # Hover callbacks
        hover_comp.add_hover_callback(self._on_hover_event)
        
        # Click callbacks
        click_comp.add_click_callback(MouseButton.LEFT, self._on_left_click)
        click_comp.add_click_callback(MouseButton.RIGHT, self._on_right_click)
        
        # Drag callbacks
        click_comp.add_drag_callback(self._on_drag_event)
        
        # Wheel callbacks
        wheel_comp.add_wheel_callback(self._on_wheel_event)
    
    def _on_hover_event(self, event_type, mouse_pos, world_pos):
        """Handle hover events."""
        if event_type == 'enter' and not self.is_dragging:
            self.color = self.hover_color
        elif event_type == 'exit' and not self.is_dragging:
            self.color = self.original_color
    
    def _on_left_click(self, button, mouse_pos, world_pos):
        """Handle left mouse click - prepare for dragging."""
        if not self.is_dragging:
            # Calculate drag offset (difference between mouse and object position)
            self.drag_offset = world_pos - self.position
            
            print(f"DEBUG: {self.name} click - mouse_pos: {mouse_pos}, world_pos: {world_pos}, object_pos: {self.position}, offset: {self.drag_offset}")
            
            # Don't start dragging yet - wait for actual drag movement
            # The drag system will handle starting the drag when movement is detected
    
    def _on_right_click(self, button, mouse_pos, world_pos):
        """Handle right mouse click - reset object."""
        self.is_dragging = False
        self.color = self.original_color
        
        # Make the object dynamic again
        rigidbody = self.get_component(RigidBody)
        if rigidbody:
            rigidbody.set_kinematic(False)
        
        print(f"{self.name}: Reset")
    
    def _on_drag_event(self, event_type, mouse_pos, world_pos, drag_vector):
        """Handle drag events."""
        if event_type == 'start':
            # Start dragging when movement is detected
            self.is_dragging = True
            self.color = self.drag_color
            
            # Make the object kinematic while dragging
            rigidbody = self.get_component(RigidBody)
            if rigidbody:
                print(f"DEBUG: {self.name} - Before kinematic: object_pos={self.position}, body_pos={rigidbody.body.position if rigidbody.body else 'None'}")
                rigidbody.set_kinematic(True)
                print(f"DEBUG: {self.name} - After kinematic: object_pos={self.position}, body_pos={rigidbody.body.position if rigidbody.body else 'None'}")
            
            print(f"{self.name}: Started dragging")
        
        elif event_type == 'drag' and self.is_dragging:
            # Move object to mouse position with offset
            target_pos = world_pos - self.drag_offset
            self.position = target_pos
            
            print(f"DEBUG: {self.name} drag - mouse_pos: {mouse_pos}, world_pos: {world_pos}, target_pos: {target_pos}")
            
            # Update rigidbody position
            rigidbody = self.get_component(RigidBody)
            if rigidbody and rigidbody.body:
                # Check for NaN values before setting position
                if not (target_pos.x != target_pos.x or target_pos.y != target_pos.y):
                    rigidbody.body.position = (target_pos.x, target_pos.y)
                else:
                    print(f"WARNING: {self.name} - NaN position detected, skipping update")
        
        elif event_type == 'end' and self.is_dragging:
            # Stop dragging
            self.is_dragging = False
            self.color = self.original_color
            
            # Make the object dynamic again
            rigidbody = self.get_component(RigidBody)
            if rigidbody:
                rigidbody.set_kinematic(False)
                # Ensure the body is at the correct position
                if rigidbody.body:
                    # Check for NaN values before setting position
                    if not (self.position.x != self.position.x or self.position.y != self.position.y):
                        rigidbody.body.position = (self.position.x, self.position.y)
                        # Reset velocity to prevent unwanted movement
                        rigidbody.body.velocity = (0, 0)
                        rigidbody.body.angular_velocity = 0
                    else:
                        print(f"WARNING: {self.name} - NaN position on drop, resetting to origin")
                        self.position = Vector2(0, 0)
                        rigidbody.body.position = (0, 0)
                        rigidbody.body.velocity = (0, 0)
                        rigidbody.body.angular_velocity = 0
            
            print(f"{self.name}: Dropped at {self.position}")
            print(f"DEBUG: {self.name} final - object_pos: {self.position}, body_pos: {rigidbody.body.position if rigidbody.body else 'None'}")
    
    def _on_wheel_event(self, delta, mouse_pos, world_pos):
        """Handle mouse wheel events - scale object."""
        if not self.is_dragging:
            # Scale the object with wheel
            scale_factor = 1 + delta * 0.1
            self.scale *= scale_factor
            self.scale = max(0.5, min(2.0, self.scale))  # Clamp between 0.5 and 2.0
            
            # Update size based on scale
            new_size = Vector2(self.size.x * scale_factor, self.size.y * scale_factor)
            self.size = new_size
            
            # Update collider size
            if self.basicShape == BasicShape.Circle:
                collider = self.get_component(CircleCollider)
                if collider:
                    collider.radius = new_size.x / 2
            else:
                collider = self.get_component(BoxCollider)
                if collider:
                    collider.width = new_size.x
                    collider.height = new_size.y

class Wall(GameObject):
    """A static wall for physics boundaries."""
    
    def __init__(self, name, position, size, color=Color(100, 100, 100)):
        super().__init__(name=name, position=position, size=size, color=color)
        
        # Add static physics components
        self.add_component(RigidBody, mass=0.0, is_kinematic=True)  # Static
        self.add_component(BoxCollider, width=size.x, height=size.y)

class Floor(GameObject):
    """A static floor for physics boundaries."""
    
    def __init__(self, name, position, size, color=Color(139, 69, 19)):  # Brown
        super().__init__(name=name, position=position, size=size, color=color)
        
        # Add static physics components
        self.add_component(RigidBody, mass=0.0, is_kinematic=True)  # Static
        self.add_component(BoxCollider, width=size.x, height=size.y)

class CameraController(GameObject):
    """Enhanced camera controller with automatic view management."""
    
    def __init__(self, engine):
        super().__init__(name="CameraController", position=Vector2(0, 0), size=Vector2(0, 0), color=Color(0, 0, 0))
        
        # Add mouse input components
        self.add_component(MouseClickComponent)
        self.add_component(MouseWheelComponent)
        
        # Set up callbacks
        click_comp = self.get_component(MouseClickComponent)
        wheel_comp = self.get_component(MouseWheelComponent)
        
        click_comp.add_click_callback(MouseButton.MIDDLE, self._on_middle_click)
        click_comp.add_drag_callback(self._on_camera_drag)
        wheel_comp.add_wheel_callback(self._on_wheel_zoom)
        
        self.engine = engine
        self.drag_start_camera_pos = Vector2(0, 0)
        self.target_zoom = 1.0
        self.zoom_speed = 0.1
        
        # Camera bounds for keeping everything in view
        self.camera_bounds = Vector2(2000, 1500)  # World space bounds
    
    def _on_middle_click(self, button, mouse_pos, world_pos):
        """Handle middle mouse click for camera pan."""
        self.drag_start_camera_pos = self.engine.camera.position.copy()
    
    def _on_camera_drag(self, event_type, mouse_pos, world_pos, drag_vector):
        """Handle camera dragging."""
        if event_type == 'drag':
            # Pan camera in opposite direction of drag
            pan_speed = 0.5
            camera_delta = -drag_vector * pan_speed
            new_pos = self.engine.camera.position + camera_delta
            
            # Clamp camera position to bounds
            half_bounds = self.camera_bounds / 2
            new_pos.x = max(-half_bounds.x, min(half_bounds.x, new_pos.x))
            new_pos.y = max(-half_bounds.y, min(half_bounds.y, new_pos.y))
            
            self.engine.camera.position = new_pos
    
    def _on_wheel_zoom(self, delta, mouse_pos, world_pos):
        """Handle mouse wheel zoom."""
        zoom_factor = 1 + delta * self.zoom_speed
        self.target_zoom *= zoom_factor
        self.target_zoom = max(0.1, min(3.0, self.target_zoom))  # Clamp zoom
        
        # Smooth zoom
        current_zoom = self.engine.camera.zoom
        self.engine.camera.zoom = current_zoom + (self.target_zoom - current_zoom) * 0.1
    
    def update(self, engine):
        """Update camera to keep all objects in view."""
        super().update(engine)
        
        # Find all game objects
        all_objects = engine._Engine__gameobjects
        
        if not all_objects:
            return
        
        # Calculate bounds of all objects
        min_x = min_y = float('inf')
        max_x = max_y = float('-inf')
        
        for obj in all_objects:
            if obj and obj.enabled and hasattr(obj, 'position') and hasattr(obj, 'size'):
                # Calculate object bounds
                obj_min_x = obj.position.x - obj.size.x/2
                obj_max_x = obj.position.x + obj.size.x/2
                obj_min_y = obj.position.y - obj.size.y/2
                obj_max_y = obj.position.y + obj.size.y/2
                
                min_x = min(min_x, obj_min_x)
                max_x = max(max_x, obj_max_x)
                min_y = min(min_y, obj_min_y)
                max_y = max(max_y, obj_max_y)
        
        # Add padding
        padding = 100
        min_x -= padding
        max_x += padding
        min_y -= padding
        max_y += padding
        
        # Calculate center and size of all objects
        center_x = (min_x + max_x) / 2
        center_y = (min_y + max_y) / 2
        width = max_x - min_x
        height = max_y - min_y
        
        # Calculate optimal zoom to fit everything
        screen_width, screen_height = engine.getWindowSize().w, engine.getWindowSize().h
        zoom_x = screen_width / width if width > 0 else 1
        zoom_y = screen_height / height if height > 0 else 1
        optimal_zoom = min(zoom_x, zoom_y, 2.0)  # Don't zoom out too much
        
        # Smoothly adjust camera position and zoom
        target_pos = Vector2(center_x, center_y)
        current_pos = engine.camera.position
        
        # Smooth camera movement
        lerp_factor = 0.02  # Adjust for smoother/faster movement
        new_pos = current_pos + (target_pos - current_pos) * lerp_factor
        engine.camera.position = new_pos
        
        # Smooth zoom adjustment
        current_zoom = engine.camera.zoom
        zoom_lerp_factor = 0.05  # Adjust for smoother/faster zoom
        new_zoom = current_zoom + (optimal_zoom - current_zoom) * zoom_lerp_factor
        engine.camera.zoom = new_zoom

def create_physics_environment(engine):
    """Create walls and floor for the physics environment."""
    walls = []
    
    # Floor
    floor = Floor("Floor", Vector2(0, 700), Vector2(2000, 100))
    engine.addGameObject(floor)
    walls.append(floor)
    
    # Left wall
    left_wall = Wall("LeftWall", Vector2(-900, 350), Vector2(100, 700))
    engine.addGameObject(left_wall)
    walls.append(left_wall)
    
    # Right wall
    right_wall = Wall("RightWall", Vector2(900, 350), Vector2(100, 700))
    engine.addGameObject(right_wall)
    walls.append(right_wall)
    
    # Top wall
    top_wall = Wall("TopWall", Vector2(0, -200), Vector2(2000, 100))
    engine.addGameObject(top_wall)
    walls.append(top_wall)
    
    return walls

def main():
    """Main function demonstrating enhanced mouse input system."""
    # Create engine
    engine = Engine(
        size=Size(w=1200, h=800),
        backgroundColor=Color(50, 50, 50),
        windowName="Enhanced Mouse Input Demo",
        fpsCap=60
    )
    
    # Create physics environment
    walls = create_physics_environment(engine)
    
    # Create draggable objects
    objects = [
        DraggableObject("Red Square", Vector2(-300, -100), Vector2(80, 80), Color(255, 0, 0)),
        DraggableObject("Green Circle", Vector2(-100, -100), Vector2(60, 60), Color(0, 255, 0), BasicShape.Circle),
        DraggableObject("Blue Rectangle", Vector2(100, -100), Vector2(100, 60), Color(0, 0, 255)),
        DraggableObject("Yellow Triangle", Vector2(300, -100), Vector2(80, 80), Color(255, 255, 0)),
        DraggableObject("Purple Square", Vector2(-200, 100), Vector2(80, 80), Color(255, 0, 255)),
        DraggableObject("Cyan Circle", Vector2(0, 100), Vector2(60, 60), Color(0, 255, 255), BasicShape.Circle),
        DraggableObject("Orange Rectangle", Vector2(200, 100), Vector2(120, 80), Color(255, 165, 0)),
        DraggableObject("Pink Circle", Vector2(400, 100), Vector2(70, 70), Color(255, 192, 203), BasicShape.Circle),
    ]
    
    # Add objects to engine
    for obj in objects:
        engine.addGameObject(obj)
    
    # Add camera controller
    camera_controller = CameraController(engine)
    engine.addGameObject(camera_controller)
    
    # Instructions
    print("Enhanced Mouse Input System Demo:")
    print("- Left click and drag to pick up objects")
    print("- Right click to reset object color")
    print("- Scroll wheel over objects to scale them")
    print("- Middle click and drag to pan camera")
    print("- Scroll wheel anywhere to zoom camera")
    print("- Camera automatically keeps everything in view")
    print("- Objects have physics and bounce off walls")
    print("- Press ESC to pause/unpause")
    
    # Start the engine
    engine.start()

if __name__ == "__main__":
    main() 