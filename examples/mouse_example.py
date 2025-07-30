"""
Mouse input handling and interaction example
"""

import pygame as pg
from pygame import Color, Vector2
from pyg_engine import Engine, GameObject, Size, BasicShape, MouseHoverComponent, MouseClickComponent, MouseWheelComponent, MouseButton, PymunkRigidBody, PymunkBoxCollider

class InteractiveObject(GameObject):
    """A game object that responds to mouse input."""
    
    def __init__(self, name, position, size, color, shape=BasicShape.Rectangle):
        super().__init__(name=name, position=position, size=size, color=color, basicShape=shape)
        
        # Add mouse input components
        self.add_component(MouseHoverComponent)
        self.add_component(MouseClickComponent)
        self.add_component(MouseWheelComponent)
        
        # Add physics components for dragging
        self.add_component(PymunkRigidBody, mass=1.0)
        self.add_component(PymunkBoxCollider, width=size.x, height=size.y)
        
        # State variables
        self.original_color = color
        self.hover_color = Color(255, 255, 0)  # Yellow when hovering
        self.click_color = Color(255, 0, 0)     # Red when clicked
        self.is_clicked = False
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
        if event_type == 'enter':
            print(f"{self.name}: Mouse entered at {mouse_pos}")
            self.color = self.hover_color
        elif event_type == 'exit':
            print(f"{self.name}: Mouse exited at {mouse_pos}")
            if not self.is_clicked:
                self.color = self.original_color
    
    def _on_left_click(self, button, mouse_pos, world_pos):
        """Handle left mouse click."""
        print(f"{self.name}: Left clicked at {mouse_pos}")
        self.is_clicked = True
        self.color = self.click_color
    
    def _on_right_click(self, button, mouse_pos, world_pos):
        """Handle right mouse click."""
        print(f"{self.name}: Right clicked at {mouse_pos}")
        # Reset color on right click
        self.is_clicked = False
        self.color = self.original_color
    
    def _on_drag_event(self, event_type, mouse_pos, world_pos, drag_vector):
        """Handle drag events."""
        if event_type == 'start':
            print(f"{self.name}: Started dragging at {mouse_pos}")
        elif event_type == 'drag':
            print(f"{self.name}: Dragging by {drag_vector}")
            # Move the object with the drag
            rigidbody = self.get_component(PymunkRigidBody)
            if rigidbody:
                # Apply force in drag direction
                force = drag_vector * 1000
                rigidbody.apply_force(force)
        elif event_type == 'end':
            print(f"{self.name}: Stopped dragging at {mouse_pos}")
    
    def _on_wheel_event(self, delta, mouse_pos, world_pos):
        """Handle mouse wheel events."""
        print(f"{self.name}: Wheel scrolled {delta} at {mouse_pos}")
        # Scale the object with wheel
        self.scale += delta * 0.1
        self.scale = max(0.5, min(2.0, self.scale))  # Clamp between 0.5 and 2.0
        
        # Update size based on scale
        self.size = Vector2(self.size.x * (1 + delta * 0.1), 
                           self.size.y * (1 + delta * 0.1))
        
        # Update collider size
        collider = self.get_component(PymunkBoxCollider)
        if collider:
            collider.width = self.size.x
            collider.height = self.size.y

class CameraController(GameObject):
    """A simple camera controller that responds to mouse input."""
    
    def __init__(self, engine):
        super().__init__(name="CameraController", position=Vector2(0, 0), size=Vector2(0, 0), color=Color(0, 0, 0))
        
        # Add mouse input components
        self.add_component(MouseClickComponent)
        
        # Set up callbacks
        click_comp = self.get_component(MouseClickComponent)
        click_comp.add_click_callback(MouseButton.MIDDLE, self._on_middle_click)
        click_comp.add_drag_callback(self._on_camera_drag)
        
        self.engine = engine
        self.drag_start_camera_pos = Vector2(0, 0)
    
    def _on_middle_click(self, button, mouse_pos, world_pos):
        """Handle middle mouse click for camera pan."""
        print("Camera: Middle clicked - starting camera pan")
        self.drag_start_camera_pos = self.engine.camera.position.copy()
    
    def _on_camera_drag(self, event_type, mouse_pos, world_pos, drag_vector):
        """Handle camera dragging."""
        if event_type == 'start':
            print("Camera: Started panning")
        elif event_type == 'drag':
            # Pan camera in opposite direction of drag
            pan_speed = 0.5
            camera_delta = -drag_vector * pan_speed
            self.engine.camera.position += camera_delta
            print(f"Camera: Panning by {camera_delta}")
        elif event_type == 'end':
            print("Camera: Stopped panning")

def main():
    """Main function demonstrating the mouse input system."""
    # Create engine
    engine = Engine(
        size=Size(w=1200, h=800),
        backgroundColor=Color(50, 50, 50),
        windowName="Mouse Input Example",
        fpsCap=60
    )
    
    # Create interactive objects
    objects = [
        InteractiveObject("Red Square", Vector2(200, 200), Vector2(80, 80), Color(255, 0, 0)),
        InteractiveObject("Green Circle", Vector2(400, 200), Vector2(60, 60), Color(0, 255, 0), BasicShape.Circle),
        InteractiveObject("Blue Rectangle", Vector2(600, 200), Vector2(100, 60), Color(0, 0, 255)),
        InteractiveObject("Yellow Triangle", Vector2(200, 400), Vector2(80, 80), Color(255, 255, 0)),
        InteractiveObject("Purple Square", Vector2(400, 400), Vector2(80, 80), Color(255, 0, 255)),
        InteractiveObject("Cyan Circle", Vector2(600, 400), Vector2(60, 60), Color(0, 255, 255), BasicShape.Circle),
    ]
    
    # Add objects to engine
    for obj in objects:
        engine.addGameObject(obj)
    
    # Add camera controller
    camera_controller = CameraController(engine)
    engine.addGameObject(camera_controller)
    
    # Instructions
    print("Mouse Input System Demo:")
    print("- Hover over objects to see them turn yellow")
    print("- Left click to make objects red")
    print("- Right click to reset object color")
    print("- Drag objects to move them with physics")
    print("- Scroll wheel over objects to scale them")
    print("- Middle click and drag to pan camera")
    print("- Press ESC to pause/unpause")
    
    # Start the engine
    engine.start()

if __name__ == "__main__":
    main() 