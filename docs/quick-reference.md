---
layout: docs
title: Quick Reference
permalink: /docs/quick-reference.html
---

# PyG Engine Quick Reference

A quick reference guide for the most commonly used features and functions in PyG Engine.

## 🚀 Getting Started

### Basic Engine Setup
```python
from pyg_engine import Engine, GameObject, Size
from pygame import Color

# Create engine
engine = Engine(
    size=Size(w=800, h=600),
    backgroundColor=Color(0, 0, 0),
    windowName="My Game"
)

# Create game object
player = GameObject(
    name="Player",
    position=(400, 300),
    size=(50, 50),
    color=Color(255, 0, 0)
)

engine.addGameObject(player)
engine.start()
```

### Adding Components
```python
from pyg_engine import RigidBody, BoxCollider

# Add physics components
rigidbody = RigidBody(mass=1.0, gravity_scale=1.0)
collider = BoxCollider(width=50, height=50)

player.add_component(rigidbody)
player.add_component(collider)
```

## 🎮 Core Classes

### Engine
- `Engine(size, backgroundColor, windowName)` - Create game engine
- `engine.start()` - Start game loop
- `engine.stop()` - Stop game loop
- `engine.addGameObject(obj)` - Add object to engine
- `engine.getGameObjects()` - Get all objects

### GameObject
- `GameObject(name, position, size, color)` - Create game object
- `obj.add_component(component)` - Add component
- `obj.get_component(ComponentClass)` - Get component
- `obj.destroy()` - Destroy object

### Components
- `RigidBody(mass, gravity_scale)` - Physics body
- `BoxCollider(width, height)` - Rectangular collider
- `CircleCollider(radius)` - Circular collider
- `Camera()` - Viewport camera

## ⚡ Physics System

### RigidBody
```python
rigidbody = RigidBody(
    mass=1.0,           # Object mass
    gravity_scale=1.0,  # Gravity multiplier
    friction=0.5,       # Surface friction
    bounce=0.3          # Bounce factor
)
```

### Colliders
```python
# Box collider
box_collider = BoxCollider(width=50, height=50)

# Circle collider
circle_collider = CircleCollider(radius=25)

# Add to game object
obj.add_component(box_collider)
```

### Physics Materials
```python
from pyg_engine import Materials

# Predefined materials
rigidbody.material = Materials.BOUNCY    # High bounce
rigidbody.material = Materials.ICE       # Low friction
rigidbody.material = Materials.METAL     # Moderate bounce/friction
```

## 🖱️ Input System

### Keyboard Input
```python
from pyg_engine import Input

# Check key states
if Input.get('SPACE'):
    print("Space is pressed")

if Input.get('W'):
    player.position.y -= 5
```

### Mouse Input
```python
# Mouse position
mouse_pos = Input.get('MOUSE_POS')

# Mouse buttons
if Input.get('MOUSE_LEFT'):
    print("Left mouse button pressed")

# Mouse wheel
wheel_delta = Input.get('MOUSE_WHEEL')
```

### Custom Input
```python
# Define custom input
Input.Keybind('JUMP', ['SPACE', 'W'])
Input.Axis('HORIZONTAL', ['A', 'D'], ['LEFT', 'RIGHT'])

# Use custom input
if Input.get('JUMP'):
    player.jump()

horizontal = Input.get_axis('HORIZONTAL')
player.position.x += horizontal * 5
```

## 📡 Event System

### Creating Events
```python
from pyg_engine import Event, EventManager

# Create custom event
class PlayerDeathEvent(Event):
    def __init__(self, player_name):
        super().__init__("player_death")
        self.player_name = player_name

# Trigger event
EventManager.dispatch("player_death", PlayerDeathEvent("Player1"))
```

### Listening to Events
```python
def on_player_death(event):
    print(f"Player {event.player_name} died!")

# Subscribe to event
EventManager.subscribe("player_death", on_player_death)
```

## 📷 Camera System

### Camera Setup
```python
from pyg_engine import Camera

# Create camera
camera = Camera()
engine.add_component(camera)

# Follow target
camera.follow(player, offset=(0, -100))

# Manual positioning
camera.set_position(400, 300)
```

### Camera Functions
```python
# Convert coordinates
world_pos = camera.screen_to_world((400, 300))
screen_pos = camera.world_to_screen((400, 300))

# Get visible area
visible_rect = camera.get_visible_rect()
```

## 🧩 Component System

### Creating Custom Components
```python
from pyg_engine import Component

class PlayerController(Component):
    def start(self):
        # Called once when component is added
        self.speed = 5.0
    
    def update(self, engine):
        # Called every frame
        if Input.get('W'):
            self.gameobject.position.y -= self.speed
        if Input.get('S'):
            self.gameobject.position.y += self.speed

# Add to game object
player.add_component(PlayerController)
```

### Component Lifecycle
```python
class MyComponent(Component):
    def start(self):
        # Called once when component is added
        pass
    
    def update(self, engine):
        # Called every frame
        pass
    
    def on_destroy(self):
        # Called when component is destroyed
        pass
```

## 📜 Script System

### Loading Scripts
```python
# Load script from file
player.add_script("scripts/player_controller.py", "PlayerController")

# Configure script
player.configure_script(PlayerController, speed=10.0, jump_force=15.0)
```

### Script Structure
```python
# scripts/player_controller.py
from pyg_engine import Script

class PlayerController(Script):
    def __init__(self, speed=5.0, jump_force=10.0):
        super().__init__()
        self.speed = speed
        self.jump_force = jump_force
    
    def update(self, engine):
        # Update logic here
        pass
```

## 🔧 Performance Tips

### Optimization
```python
# Use object pooling for frequently created objects
# Enable debug mode for development
engine.set_debug_mode(True)

# Get performance stats
stats = engine.get_runnable_stats()
print(f"Active runnables: {stats['active_count']}")
```

### Best Practices
- Use components for reusable functionality
- Leverage the event system for loose coupling
- Use appropriate physics materials
- Clean up objects when destroyed
- Use the camera system for large worlds

## 📚 Common Patterns

### Player Movement
```python
class PlayerMovement(Component):
    def update(self, engine):
        # Get input
        horizontal = Input.get_axis('HORIZONTAL')
        vertical = Input.get_axis('VERTICAL')
        
        # Apply movement
        self.gameobject.position.x += horizontal * self.speed * engine.dt()
        self.gameobject.position.y += vertical * self.speed * engine.dt()
```

### Collision Detection
```python
class CollisionHandler(Component):
    def start(self):
        self.collider = self.gameobject.get_component(BoxCollider)
        self.collider.add_collision_callback('on_collision', self.on_collision)
    
    def on_collision(self, collision_info):
        print(f"Collided with {collision_info.other_gameobject.name}")
```

### Camera Following
```python
class CameraFollow(Component):
    def start(self):
        self.target = None
        self.offset = (0, -100)
    
    def set_target(self, gameobject):
        self.target = gameobject
        camera = self.gameobject.get_component(Camera)
        camera.follow(self.target, self.offset)
```

---

<div class="callout info">
  <strong>Need more details?</strong> Check out the full <a href="CORE_SYSTEMS_GUIDE.html" class="btn">📖 Core Systems Guide</a> for comprehensive documentation.
</div> 