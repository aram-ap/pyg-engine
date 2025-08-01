---
layout: docs
title: Core Systems Guide
permalink: /docs/CORE_SYSTEMS_GUIDE.html
---

# PyG Engine Core Systems Guide

A comprehensive guide to the core systems of the PyG Engine: Engine architecture, Global Dictionary, Runnable System, and component-based game development.

## Table of Contents

1. [Classes](#classes)
   - [BasicShape](#basicshape)
   - [BoxCollider](#boxcollider)
   - [Camera](#camera)
   - [CircleCollider](#circlecollider)
   - [Collider](#collider)
   - [CollisionInfo](#collisioninfo)
   - [Component](#component)
   - [Engine](#engine)
   - [Event](#event)
   - [EventManager](#eventmanager)
   - [GameObject](#gameobject)
   - [GlobalDictionary](#globaldictionary)
   - [Input](#input)
   - [Input.Keybind](#inputkeybind)
   - [Input.Axis](#inputaxis)
   - [Input.InputEvent](#inputinputevent)
   - [Input.Mouse](#inputmouse)
   - [Materials](#materials)
   - [MouseWheelComponent](#mousewheelcomponent)
   - [PhysicsMaterial](#physicsmaterial)
   - [PhysicsSystem](#physicssystem)
   - [Priority](#priority)
   - [RigidBody](#rigidbody)
   - [Runnable](#runnable)
   - [RunnableSystem](#runnablesystem)
   - [Script](#script)
   - [Size](#size)
   - [Tag](#tag)
2. [Usage](#usage)
   - [Basic Setup](#basic-setup)
   - [Game Object Management](#game-object-management)
   - [Physics System](#physics-system)
   - [Input Handling](#input-handling)
   - [Camera System](#camera-system)
   - [Component System](#component-system)
   - [Script System](#script-system)
   - [Performance Optimization](#performance-optimization)
3. [Integration Examples](#integration-examples)
4. [Best Practices](#best-practices)

---

## Classes

<div class="class-doc">
  <h3>BasicShape</h3>
  <div class="class-purpose">
    <strong>Purpose</strong>: Enumeration of basic geometric shapes for game objects.
  </div>
  
  <div class="class-functions">
    <h4>Values</h4>
    <ul class="function-list">
      <li><code>Rectangle</code> - Rectangular shape</li>
      <li><code>Circle</code> - Circular shape</li>
    </ul>
  </div>
</div>

<div class="class-doc">
  <h3>BoxCollider</h3>
  <div class="class-purpose">
    <strong>Purpose</strong>: Rectangular collision detection with width and height bounds.
  </div>
  
  <div class="class-functions">
    <h4>Functions</h4>
    <ul class="function-list">
      <li><code>check_collision(other_collider)</code> - Detect collision with other collider</li>
      <li><code>get_world_corners()</code> - Get world coordinates of box corners</li>
      <li><code>update_bounds()</code> - Update collision bounds</li>
    </ul>
  </div>
</div>

<div class="class-doc">
  <h3>Camera</h3>
  <div class="class-purpose">
    <strong>Purpose</strong>: Viewport management and target following with zoom and scaling support.
  </div>
  
  <div class="class-functions">
    <h4>Functions</h4>
    <ul class="function-list">
      <li><code>follow(game_object, offset)</code> - Set GameObject to follow with optional offset</li>
      <li><code>get_visible_rect()</code> - Get visible world area</li>
      <li><code>resize(new_width, new_height)</code> - Handle window resize</li>
      <li><code>screen_to_world(screen_pos)</code> - Convert screen to world coordinates</li>
      <li><code>set_position(x, y)</code> - Manually set camera position</li>
      <li><code>update(dt)</code> - Update camera position and bounds</li>
      <li><code>world_to_screen(world_pos)</code> - Convert world to screen coordinates</li>
    </ul>
  </div>
</div>

<div class="class-doc">
  <h3>CircleCollider</h3>
  <div class="class-purpose">
    <strong>Purpose</strong>: Circular collision detection with radius-based bounds.
  </div>
  
  <div class="class-functions">
    <h4>Functions</h4>
    <ul class="function-list">
      <li><code>check_collision(other_collider)</code> - Detect collision with other collider</li>
      <li><code>update_bounds()</code> - Update collision bounds</li>
    </ul>
  </div>
</div>

<div class="class-doc">
  <h3>Collider</h3>
  <div class="class-purpose">
    <strong>Purpose</strong>: Base class for collision detection components.
  </div>
  
  <div class="class-functions">
    <h4>Functions</h4>
    <ul class="function-list">
      <li><code>add_collision_callback(event_type, callback)</code> - Add collision event handler</li>
      <li><code>check_collision(other_collider)</code> - Detect collision with other collider</li>
      <li><code>handle_collision(collision_info)</code> - Process collision event</li>
      <li><code>remove_collision_callback(event_type, callback)</code> - Remove collision handler</li>
      <li><code>update_bounds()</code> - Update collision bounds</li>
    </ul>
  </div>
</div>

<div class="class-doc">
  <h3>CollisionInfo</h3>
  <div class="class-purpose">
    <strong>Purpose</strong>: Information about a collision between two objects.
  </div>
  
  <div class="class-functions">
    <h4>Properties</h4>
    <ul class="function-list">
      <li><code>other_collider</code> - The other collider involved</li>
      <li><code>other_gameobject</code> - Reference to the other GameObject</li>
      <li><code>contact_point</code> - Where the collision occurred</li>
      <li><code>contact_normal</code> - Direction to separate objects</li>
      <li><code>penetration_depth</code> - How much objects are overlapping</li>
    </ul>
  </div>
</div>

<div class="class-doc">
  <h3>Component</h3>
  <div class="class-purpose">
    <strong>Purpose</strong>: Base class for all components that can be attached to GameObjects.
  </div>
  
  <div class="class-functions">
    <h4>Functions</h4>
    <ul class="function-list">
      <li><code>on_destroy()</code> - Called when component is destroyed</li>
      <li><code>start()</code> - Called once when component is first added</li>
      <li><code>update(engine)</code> - Called every frame if component is enabled</li>
    </ul>
  </div>
</div>

<div class="class-doc">
  <h3>Engine</h3>
  <div class="class-purpose">
    <strong>Purpose</strong>: Core game engine managing game loop, rendering, and system coordination.
  </div>
  
  <div class="class-functions">
    <h4>Functions</h4>
    <ul class="function-list">
      <li><code>add_runnable(func, event_type, priority, max_runs, key, error_handler)</code> - Add function to execution queue</li>
      <li><code>addGameObject(gameobj)</code> - Add GameObject to engine</li>
      <li><code>add_error_handler(handler)</code> - Set global error handler</li>
      <li><code>clear_runnable_queue(event_type, key)</code> - Clear specific runnable queue</li>
      <li><code>dt()</code> - Get delta time</li>
      <li><code>get_runnable_stats()</code> - Get execution statistics</li>
      <li><code>getGameObjects()</code> - Get all active GameObjects</li>
      <li><code>removeGameObject(gameobj)</code> - Remove GameObject from engine</li>
      <li><code>running()</code> - Check if engine is active</li>
      <li><code>set_debug_mode(enabled)</code> - Enable debug logging</li>
      <li><code>setRunning(running)</code> - Set engine running state</li>
      <li><code>start()</code> - Start game loop</li>
      <li><code>stop()</code> - Stop game loop</li>
    </ul>
  </div>
</div>

### Event
**Purpose**: Immutable data class representing an event in the game engine.

**Properties**:
- `type` - Event type identifier (e.g., 'collision', 'input', 'spawn')
- `data` - Optional payload containing event-specific data
- `timestamp` - Creation time for event ordering

### EventManager
**Purpose**: Manages event subscription, dispatch, and processing with thread-safe operations.

**Functions**:
- `subscribe(event_type, listener, priority)` - Subscribe a listener to an event type with optional priority
- `unsubscribe(event_type, listener)` - Remove a listener from an event type subscription
- `dispatch(event_type, data, immediate)` - Dispatch an event to all subscribed listeners
- `process_queue()` - Process all queued events in FIFO order

### GameObject
**Purpose**: Game object with component and script attachment capabilities.

**Functions**:
- `add_component(component_class, **kwargs)` - Add component to GameObject
- `add_script(script_path, script_class_name, **kwargs)` - Add script from file
- `add_script_configs(script_configs)` - Add multiple scripts
- `configure_script(script_class_type, **kwargs)` - Configure existing script
- `destroy()` - Destroy GameObject and cleanup
- `get_all_components()` - Get all attached components
- `get_all_scripts()` - Get all attached scripts
- `get_component(component_class)` - Get specific component
- `get_script(script_class_type)` - Get specific script
- `has_component(component_class)` - Check if component exists
- `kill(engine)` - Remove from engine
- `list_components()` - List all components
- `list_scripts()` - List all scripts
- `remove_component(component_class)` - Remove component
- `update_color(new_color)` - Update sprite color
- `update_position(new_position)` - Update sprite position
- `update_rotation(new_rotation)` - Update sprite rotation
- `update_size(new_size)` - Update sprite size

### GlobalDictionary
**Purpose**: Thread-safe global variable system with categorization and caching.

**Functions**:
- `clear_category(category)` - Remove all variables in category
- `get(key, default, category)` - Retrieve variable value
- `get_all(category)` - Get all variables in category
- `has(key, category)` - Check if variable exists
- `remove(key, category)` - Remove specific variable
- `set(key, value, category)` - Store variable value

### Materials
**Purpose**: Collection of predefined physics materials.

**Values**:
- `DEFAULT` - Standard material with moderate friction
- `BOUNCY` - High bounce, low friction material
- `ICE` - Low friction, low bounce material
- `RUBBER` - High bounce, high friction material
- `METAL` - Moderate bounce and friction material
- `WOOD` - Low bounce, high friction material

### Input
**Purpose**: Unified input system managing keyboard, mouse, and joystick input with axis support.

**Functions**:
- `get(input)` - Get input state (bool for keys/buttons, float for axes)
- `get_axis(axis)` - Get axis value (-1.0 to 1.0)
- `get_raw_axis(axis)` - Get raw axis value without clamping
- `get_event_state(event_type, key)` - Get event-based input state
- `process_event(event)` - Process pygame events
- `update()` - Update input states
- `try_joystick_init()` - Initialize joystick support

**Properties**:
- `mouse` - Mouse interface object
- `alias` - Input binding dictionary
- `event_states` - Event-based input states

### Input.Keybind
**Purpose**: Enumeration of keyboard keys and mouse buttons.

**Values**:
- Letter keys: `A-Z`
- Number keys: `K_1-K_0`
- Function keys: `F1-F12`
- Special keys: `SPACE`, `ENTER`, `ESCAPE`, `TAB`, etc.
- Arrow keys: `K_UP`, `K_DOWN`, `K_LEFT`, `K_RIGHT`
- Modifier keys: `M_SHIFT_L/R`, `M_CTRL_L/R`, `M_ALT_L/R`
- Mouse buttons: `MOUSE_LEFT`, `MOUSE_RIGHT`, `MOUSE_MIDDLE`

### Input.Axis
**Purpose**: Enumeration of input axes for movement and actions.

**Values**:
- Movement: `HORIZONTAL`, `VERTICAL`
- Mouse: `MOUSE_X`, `MOUSE_Y`, `MOUSE_REL_X`, `MOUSE_REL_Y`, `MOUSE_SCROLL`
- Gameplay: `JUMP`, `CROUCH`, `SPRINT`, `WALK`
- Combat: `FIRE`, `FIRE2`, `FIRE3`, `RELOAD`, `INTERACT`
- Camera: `LOOK_X`, `LOOK_Y`, `ZOOM_IN`, `ZOOM_OUT`
- Menu: `MENU_UP`, `MENU_DOWN`, `MENU_LEFT`, `MENU_RIGHT`
- Vehicle: `ACCELERATE`, `BRAKE`, `STEER_LEFT`, `STEER_RIGHT`
- Joystick: `JOYSTICK_LEFT_X/Y`, `JOYSTICK_RIGHT_X/Y`, `JOYSTICK_A/B/X/Y`

### Input.InputEvent
**Purpose**: Enumeration of input event types for event-based input.

**Values**:
- Mouse events: `MOUSE_SCROLL_UP/DOWN`, `MOUSE_BUTTON_DOWN/UP`, `MOUSE_MOTION`
- Keyboard events: `KEY_DOWN/UP`, `KEY_REPEAT`, `KEY_HOLD`
- Joystick events: `JOYSTICK_BUTTON_DOWN/UP`, `JOYSTICK_AXIS_MOTION`
- Window events: `WINDOW_FOCUS_GAINED/LOST`, `WINDOW_RESIZED`

### Input.Mouse
**Purpose**: Clean mouse interface providing position, movement, and button access.

**Functions**:
- `get_pos()` - Get current mouse position (x, y)
- `get_rel()` - Get relative mouse movement since last frame
- `get_scroll()` - Get scroll wheel movement
- `get_button(button)` - Get mouse button state (0=Left, 1=Middle, 2=Right)
- `get_button_down(button)` - Check if button just pressed
- `get_button_up(button)` - Check if button just released
- `set_pos(pos)` - Set mouse position
- `set_visible(visible)` - Set cursor visibility
- `get_visible()` - Get cursor visibility

### PhysicsMaterial
**Purpose**: Defines physical properties for collision behavior.

**Functions**:
- `__init__(name, bounce, friction, friction_combine)` - Create physics material

**Properties**:
- `name` - Material name
- `bounce` - Restitution coefficient (0.0 = no bounce, 1.0 = perfect bounce)
- `friction` - Surface friction (0.0 = ice, 1.0 = rubber)
- `friction_combine` - Friction combination method

### PhysicsSystem
**Purpose**: Physics system with collision detection and resolution using Pymunk.

**Functions**:
- `add_object(game_object)` - Add GameObject to physics simulation
- `get_gravity()` - Get current gravity vector
- `remove_object(game_object)` - Remove GameObject from physics simulation
- `set_gravity(gravity_x, gravity_y)` - Set gravity vector
- `update(engine, game_objects)` - Main physics update loop

### Priority
**Purpose**: Priority levels for runnable execution order.

**Values**:
- `LOW` - Execute last (UI, cleanup)
- `NORMAL` - Standard execution (gameplay logic)
- `HIGH` - Execute early (physics, AI)
- `CRITICAL` - Execute first (input handling, core systems)

### RigidBody
**Purpose**: Physics component with linear and angular motion support using Pymunk.

**Functions**:
- `add_angular_impulse(angular_impulse)` - Apply angular impulse
- `add_force(force, point)` - Apply linear force
- `add_force_at_point(force, point)` - Apply force at specific point
- `add_impulse(impulse, point)` - Apply linear impulse
- `add_impulse_at_point(impulse, point)` - Apply impulse at specific point
- `add_torque(torque)` - Apply rotational torque
- `add_velocity(velocity)` - Add to current velocity
- `freeze_rotation()` - Disable rotation
- `get_angular_speed()` - Get angular velocity magnitude
- `get_kinetic_energy()` - Calculate kinetic energy
- `get_speed()` - Get linear velocity magnitude
- `set_gravity_scale(scale)` - Set gravity influence
- `set_kinematic(is_kinematic)` - Set kinematic mode
- `set_mass(mass)` - Set object mass
- `set_rotation_lock(lock_rotation)` - Set rotation lock
- `set_velocity(velocity)` - Set linear velocity
- `stop()` - Stop all motion
- `unfreeze_rotation()` - Enable rotation

### Runnable
**Purpose**: A runnable function with priority, execution limits, and error handling.

**Functions**:
- `execute(engine)` - Execute the runnable with exception handling
- `_handle_error(error, engine)` - Handle execution errors
- `_remove_from_engine(engine)` - Remove from engine queues

### RunnableSystem
**Purpose**: Event-driven function execution system with priority queues.

**Functions**:
- `add_error_handler(handler)` - Set global error handler
- `add_runnable(func, event_type, priority, max_runs, key, error_handler)` - Add function to queue
- `clear_queue(event_type, key)` - Clear specific queue
- `execute_runnables(event_type, key, engine)` - Execute queued functions
- `get_queue_stats()` - Get queue statistics
- `set_debug_mode(enabled)` - Enable debug mode

### Script
**Purpose**: Base class for game scripts that can be attached to GameObjects.

**Functions**:
- `get_component(component_class)` - Get component from GameObject
- `get_config(key, default)` - Get configuration value
- `get_collider()` - Get collider component
- `get_rigidbody()` - Get rigidbody component
- `has_component(component_class)` - Check if component exists
- `on_destroy()` - Handle destruction
- `require_component(component_class)` - Get required component
- `set_config(key, value)` - Set configuration value
- `start(engine)` - Initialize script
- `update(engine)` - Update script logic

### Size
**Purpose**: Width and height dimensions.

**Functions**:
- `__str__()` - String representation

**Properties**:
- `w` - Width dimension
- `h` - Height dimension

### Tag
**Purpose**: Object tags for categorization and collision filtering.

**Values**:
- `Player` - Player objects
- `Environment` - Environment objects
- `Other` - Other objects

---

## Usage

### Basic Setup

```python
from src.engine import Engine, Size, Color
from pygame import Vector2

# Initialize engine
engine = Engine(
    size=Size(w=800, h=600),
    backgroundColor=Color(0, 0, 0),
    windowName="My Game",
    fpsCap=60
)

# Start the game loop
engine.start()
```

**Explanation**: This creates a game engine with an 800x600 window, black background, and 60 FPS cap. The `start()` method begins the main game loop.

### Game Object Management

```python
from src.gameobject import GameObject
from pygame import Vector2, Color

# Create game object
player = GameObject(
    name="player",
    position=Vector2(100, 100),
    size=Vector2(32, 32),
    color=Color(255, 0, 0)
)

# Add to engine
engine.addGameObject(player)

# Remove from engine
engine.removeGameObject(player)

# Get all objects
objects = engine.getGameObjects()
```

**Explanation**: Creates a red 32x32 player object at position (100, 100). The engine manages all GameObjects and provides methods to add, remove, and retrieve them.

### Physics System

```python
from pyg_engine import RigidBody, BoxCollider, CircleCollider

# Add physics components
player.add_component(RigidBody, mass=1.0, gravity_scale=1.0)
player.add_component(BoxCollider, width=32, height=32)

# Create physics object with circle collider
ball = GameObject(name="ball", position=Vector2(200, 50))
ball.add_component(RigidBody, mass=0.5)
ball.add_component(CircleCollider, radius=16)

# Set physics properties
rigidbody = player.get_component(RigidBody)
rigidbody.add_force(Vector2(100, 0))  # Apply force
rigidbody.set_velocity(Vector2(50, 0))  # Set velocity
```

**Explanation**: Adds physics components to objects. RigidBody handles motion and forces, while Colliders define collision shapes. The physics system automatically handles gravity, collision detection, and motion.

### Input Handling

The PyG Engine provides a comprehensive input system that unifies keyboard, mouse, and joystick input through a clean, event-driven interface. The system supports both direct input access and axis-based input for complex game controls.

#### Basic Input System Usage

```python
from pyg_engine import Engine, Input, Priority

engine = Engine()
input = engine.input

# Direct key checking
def handle_movement(engine):
    # Check individual keys
    if input.get(Input.Keybind.W):
        print("W key pressed")
    if input.get(Input.Keybind.SPACE):
        print("Space pressed")
    
    # Check mouse buttons
    if input.get(Input.Keybind.MOUSE_LEFT):
        print("Left mouse button pressed")
    if input.get(Input.Keybind.MOUSE_RIGHT):
        print("Right mouse button pressed")

engine.add_runnable(handle_movement, 'update', Priority.NORMAL)
```

#### Axis-Based Input

The input system provides axis-based input that can combine multiple input sources:

```python
def handle_axis_input(engine):
    # Get axis values (-1.0 to 1.0)
    horizontal = input.get_axis(Input.Axis.HORIZONTAL)  # A/D keys or left stick
    vertical = input.get_axis(Input.Axis.VERTICAL)      # W/S keys or left stick
    
    # Apply movement based on axis values
    if horizontal != 0 or vertical != 0:
        movement = Vector2(horizontal, vertical)
        player.position += movement * 200 * engine.dt()
    
    # Check action axes
    if input.get_axis(Input.Axis.JUMP) > 0:
        print("Jump action triggered")
    
    if input.get_axis(Input.Axis.FIRE) > 0:
        print("Fire action triggered")

engine.add_runnable(handle_axis_input, 'update', Priority.NORMAL)
```

#### Mouse Input System

The engine provides a dedicated mouse interface with comprehensive functionality:

```python
def handle_mouse_input(engine):
    mouse = input.mouse
    
    # Get mouse position and movement
    pos = mouse.get_pos()           # Current position (x, y)
    rel = mouse.get_rel()           # Relative movement since last frame
    scroll = mouse.get_scroll()     # Scroll wheel movement
    
    # Check button states
    left_pressed = mouse.get_button(0)    # Left button
    middle_pressed = mouse.get_button(1)  # Middle button
    right_pressed = mouse.get_button(2)   # Right button
    
    # Check button events (one-frame events)
    left_down = mouse.get_button_down(0)  # Just pressed
    left_up = mouse.get_button_up(0)      # Just released
    
    # Handle mouse activity
    if rel != (0, 0):
        print(f"Mouse moved: {rel}")
    
    if scroll != (0, 0):
        print(f"Scroll: {scroll}")
    
    if left_down:
        print("Left button pressed")
    if left_up:
        print("Left button released")

engine.add_runnable(handle_mouse_input, 'update', Priority.NORMAL)
```

#### Event-Based Input

The input system provides event-based input for precise timing:

```python
def handle_input_events(engine):
    # Check for specific events
    if input.get_event_state('key_down', Input.Keybind.SPACE.value):
        print("Space key just pressed")
    
    if input.get_event_state('mouse_button_down', 0):
        print("Left mouse button just pressed")
    
    if input.get_event_state('mouse_scroll_up'):
        print("Mouse wheel scrolled up")
    
    if input.get_event_state('mouse_scroll_down'):
        print("Mouse wheel scrolled down")

engine.add_runnable(handle_input_events, 'update', Priority.NORMAL)
```

#### Joystick Support

The input system automatically detects and maps joystick input:

```python
def handle_joystick_input(engine):
    # Joystick axes (returns -1.0 to 1.0)
    left_stick_x = input.get_axis(Input.Axis.JOYSTICK_LEFT_X)
    left_stick_y = input.get_axis(Input.Axis.JOYSTICK_LEFT_Y)
    right_stick_x = input.get_axis(Input.Axis.JOYSTICK_RIGHT_X)
    right_stick_y = input.get_axis(Input.Axis.JOYSTICK_RIGHT_Y)
    
    # Joystick triggers (returns 0.0 to 1.0)
    left_trigger = input.get_axis(Input.Axis.JOYSTICK_LEFT_TRIGGER)
    right_trigger = input.get_axis(Input.Axis.JOYSTICK_RIGHT_TRIGGER)
    
    # Joystick buttons (returns bool)
    a_button = input.get(Input.Axis.JOYSTICK_A)
    b_button = input.get(Input.Axis.JOYSTICK_B)
    x_button = input.get(Input.Axis.JOYSTICK_X)
    y_button = input.get(Input.Axis.JOYSTICK_Y)
    
    # Apply joystick input to movement
    if left_stick_x != 0 or left_stick_y != 0:
        movement = Vector2(left_stick_x, left_stick_y)
        player.position += movement * 200 * engine.dt()
    
    # Apply right stick to camera
    if right_stick_x != 0 or right_stick_y != 0:
        camera_movement = Vector2(right_stick_x, right_stick_y)
        engine.camera.position += camera_movement * 100 * engine.dt()

engine.add_runnable(handle_joystick_input, 'update', Priority.NORMAL)
```


#### Advanced Input Features

**Input Aliases and String-Based Access:**
```python
def handle_string_input(engine):
    # Use string aliases for input
    if input.get("left"):
        print("Left arrow or A key pressed")
    if input.get("space"):
        print("Space key pressed")
    if input.get("escape"):
        print("Escape key pressed")

engine.add_runnable(handle_string_input, 'update', Priority.NORMAL)
```

**Raw Axis Values:**
```python
def handle_raw_axis(engine):
    # Get raw axis values without clamping
    raw_horizontal = input.get_raw_axis(Input.Axis.HORIZONTAL)
    raw_vertical = input.get_raw_axis(Input.Axis.VERTICAL)
    
    # Apply custom scaling or processing
    if raw_horizontal != 0:
        print(f"Raw horizontal: {raw_horizontal}")

engine.add_runnable(handle_raw_axis, 'update', Priority.NORMAL)
```

**Input System Features**

**Key Features:**
- **Unified Interface**: Single system for keyboard, mouse, and joystick input
- **Axis Support**: Combine multiple inputs into logical axes (e.g., WASD + arrow keys)
- **Event-Based**: Precise timing for button presses and releases
- **Component System**: Object-specific mouse interactions
- **Joystick Support**: Automatic joystick detection and mapping
- **Input Aliases**: String-based input for easy configuration
- **Deadzone Support**: Automatic joystick deadzone handling
- **Input Binding**: Flexible axis and key binding system

**Available Input Types:**
- **Keyboard**: All standard keys, function keys, modifiers
- **Mouse**: Position, movement, buttons, scroll wheel
- **Joystick**: Buttons, axes, triggers, d-pad
- **Axes**: Movement, combat, camera, menu, vehicle controls

**Event Types:**
- `KEY_DOWN` / `KEY_UP`: Keyboard button events
- `MOUSE_BUTTON_DOWN` / `MOUSE_BUTTON_UP`: Mouse button events
- `MOUSE_SCROLL_UP` / `MOUSE_SCROLL_DOWN`: Scroll wheel events
- `MOUSE_MOTION`: Mouse movement events
- `JOYSTICK_*`: Joystick events

**Best Practices:**
- Use axis-based input for movement and continuous actions
- Use event-based input for precise timing (clicks, key presses)
- Use mouse components for object-specific interactions
- Cache frequently accessed input values in local variables
- Handle input in high-priority runnables for responsive controls
- Use joystick deadzones to prevent drift
- Combine keyboard and joystick input for better accessibility

### Event System

The Event System provides a robust, thread-safe communication mechanism between different components of the game engine. It supports priority-based event handling, automatic memory management, and both immediate and queued event processing.

#### Core Components

**Event Class**
```python
from pyg_engine import Event

# Create events with type and optional data
collision_event = Event(type="collision", data={"damage": 10, "source": "enemy"})
input_event = Event(type="key_press", data={"key": "SPACE"})
spawn_event = Event(type="spawn", data={"object_type": "enemy", "position": Vector2(100, 100)})
```

**EventManager**
```python
from pyg_engine import EventManager, Priority

# Access the engine's event manager
event_manager = engine.event_manager

# Subscribe to events with priority
def handle_collision(event):
    print(f"Collision detected: {event.data}")
    # Handle collision logic

def handle_spawn(event):
    print(f"Spawning: {event.data}")
    # Handle spawn logic

# Subscribe with different priorities
event_manager.subscribe("collision", handle_collision, Priority.HIGH)
event_manager.subscribe("spawn", handle_spawn, Priority.NORMAL)

# Unsubscribe when no longer needed
event_manager.unsubscribe("collision", handle_collision)
```

#### Event Dispatch and Processing

**Immediate Event Processing**
```python
def handle_immediate_events(engine):
    # Process events immediately (synchronous)
    event_manager.dispatch("player_moved", {
        "position": player.position,
        "velocity": player.velocity
    }, immediate=True)

engine.add_runnable(handle_immediate_events, 'update', Priority.NORMAL)
```

**Queued Event Processing**
```python
def handle_queued_events(engine):
    # Queue events for later processing (asynchronous)
    event_manager.dispatch("enemy_spawned", {
        "enemy_type": "goblin",
        "position": Vector2(200, 200)
    }, immediate=False)
    
    # Process all queued events
    event_manager.process_queue()

engine.add_runnable(handle_queued_events, 'update', Priority.NORMAL)
```

#### Priority-Based Event Handling

```python
def setup_event_priorities(engine):
    # Critical events (input, collision detection)
    def handle_input_event(event):
        print("Processing input event")
    
    # High priority events (physics, AI)
    def handle_physics_event(event):
        print("Processing physics event")
    
    # Normal priority events (gameplay logic)
    def handle_gameplay_event(event):
        print("Processing gameplay event")
    
    # Low priority events (UI, cleanup)
    def handle_ui_event(event):
        print("Processing UI event")
    
    # Subscribe with different priorities
    event_manager.subscribe("input", handle_input_event, Priority.CRITICAL)
    event_manager.subscribe("physics", handle_physics_event, Priority.HIGH)
    event_manager.subscribe("gameplay", handle_gameplay_event, Priority.NORMAL)
    event_manager.subscribe("ui", handle_ui_event, Priority.LOW)

engine.add_runnable(setup_event_priorities, 'start', Priority.CRITICAL, max_runs=1)
```

#### Component Integration

```python
from pyg_engine import Component

class EventComponent(Component):
    def __init__(self, game_object):
        super().__init__(game_object)
        self.event_manager = game_object.engine.event_manager
        
    def start(self):
        # Subscribe to events when component starts
        self.event_manager.subscribe("collision", self.handle_collision)
        self.event_manager.subscribe("damage", self.handle_damage)
    
    def handle_collision(self, event):
        if event.data.get("target") == self.game_object:
            print(f"{self.game_object.name} was hit!")
    
    def handle_damage(self, event):
        if event.data.get("target") == self.game_object:
            damage = event.data.get("amount", 0)
            print(f"{self.game_object.name} took {damage} damage!")
    
    def on_destroy(self):
        # Unsubscribe when component is destroyed
        self.event_manager.unsubscribe("collision", self.handle_collision)
        self.event_manager.unsubscribe("damage", self.handle_damage)

# Add event component to game object
player.add_component(EventComponent)
```

#### Script Integration

```python
# player_script.py
class PlayerScript(Script):
    def start(self, engine):
        # Subscribe to events in script
        engine.event_manager.subscribe("powerup_collected", self.handle_powerup)
        engine.event_manager.subscribe("level_complete", self.handle_level_complete)
    
    def handle_powerup(self, event):
        powerup_type = event.data.get("type")
        if powerup_type == "speed":
            self.speed *= 1.5
        elif powerup_type == "health":
            self.health += 25
    
    def handle_level_complete(self, event):
        print("Level completed!")
        # Handle level completion logic
    
    def on_destroy(self):
        # Unsubscribe from events
        self.engine.event_manager.unsubscribe("powerup_collected", self.handle_powerup)
        self.engine.event_manager.unsubscribe("level_complete", self.handle_level_complete)

# Add script with event handling
player.add_script("player_script.py", "PlayerScript")
```

#### Advanced Event Patterns

**Event Chaining**
```python
def setup_event_chain(engine):
    def handle_player_damage(event):
        # Player took damage
        damage = event.data.get("amount", 0)
        print(f"Player took {damage} damage")
        
        # Chain to UI update event
        engine.event_manager.dispatch("ui_update", {
            "health": player.health,
            "damage_taken": damage
        })
    
    def handle_ui_update(event):
        # Update UI based on event data
        health = event.data.get("health")
        damage = event.data.get("damage_taken")
        print(f"UI updated: Health={health}, Damage={damage}")
    
    event_manager.subscribe("player_damage", handle_player_damage)
    event_manager.subscribe("ui_update", handle_ui_update)

engine.add_runnable(setup_event_chain, 'start', Priority.CRITICAL, max_runs=1)
```

**Event Filtering**
```python
def handle_filtered_events(engine):
    def handle_enemy_event(event):
        # Only handle events for specific enemy types
        enemy_type = event.data.get("enemy_type")
        if enemy_type in ["goblin", "orc"]:
            print(f"Handling {enemy_type} event")
    
    event_manager.subscribe("enemy_spawned", handle_enemy_event)
    event_manager.subscribe("enemy_died", handle_enemy_event)

engine.add_runnable(handle_filtered_events, 'start', Priority.CRITICAL, max_runs=1)
```

**Event Broadcasting**
```python
def broadcast_system_events(engine):
    # Broadcast system-wide events
    def broadcast_game_state():
        engine.event_manager.dispatch("game_state_changed", {
            "score": engine.globals.get("score", 0),
            "level": engine.globals.get("level", 1),
            "time": engine.globals.get("game_time", 0)
        })
    
    # Broadcast every second
    engine.add_runnable(broadcast_game_state, 'update', Priority.LOW)

engine.add_runnable(broadcast_system_events, 'start', Priority.CRITICAL, max_runs=1)
```

#### Event System Features

**Key Features:**
- **Thread-Safe**: All operations are protected by locks for concurrent access
- **Memory Management**: Automatic cleanup of dead references using weak references
- **Priority Support**: Four priority levels for event processing order
- **Flexible Processing**: Both immediate and queued event processing
- **Component Integration**: Easy integration with components and scripts
- **Event Chaining**: Events can trigger other events for complex workflows
- **Automatic Cleanup**: Dead references are automatically removed

**Priority Levels:**
- `CRITICAL`: Execute first (input handling, core systems)
- `HIGH`: Execute early (physics, AI, collision detection)
- `NORMAL`: Standard execution (gameplay logic, state updates)
- `LOW`: Execute last (UI updates, cleanup, logging)

**Event Types:**
- **System Events**: Engine state changes, initialization, cleanup
- **Input Events**: Key presses, mouse actions, joystick input
- **Physics Events**: Collisions, triggers, physics state changes
- **Gameplay Events**: Player actions, enemy behavior, level progression
- **UI Events**: Interface updates, menu navigation, HUD changes

**Best Practices:**
- Use appropriate priorities for event processing order
- Unsubscribe from events when components/scripts are destroyed
- Use immediate processing for critical events (input, collision)
- Use queued processing for non-critical events (UI updates, logging)
- Chain events carefully to avoid infinite loops
- Filter events based on data content when needed
- Use weak references for automatic memory management
- Handle event processing errors gracefully
- Monitor event queue size for performance optimization

### Camera System

```python
# Follow target
engine.camera.follow(player, offset=Vector2(0, -50))

# Manual camera control
def camera_controls(engine):
    keys = pg.key.get_pressed()
    speed = 100 * engine.dt()
    
    if keys[pg.K_UP]:
        engine.camera.position.y += speed
    if keys[pg.K_DOWN]:
        engine.camera.position.y -= speed

engine.add_runnable(camera_controls, 'update', Priority.HIGH)

# Zoom control
def handle_zoom(engine):
    wheel_delta = engine.mouse_input.get_wheel_delta()
    if wheel_delta != 0:
        engine.camera.zoom += wheel_delta * 0.1
        engine.camera.zoom = max(0.1, min(5.0, engine.camera.zoom))

engine.add_runnable(handle_zoom, 'update', Priority.HIGH)
```

**Explanation**: The camera can follow a target with an optional offset, or be controlled manually with arrow keys. Mouse wheel controls zoom level, clamped between 0.1x and 5x magnification.

### Component System

```python
from component import Component

class CustomComponent(Component):
    def __init__(self, game_object, custom_param=10):
        super().__init__(game_object)
        self.custom_param = custom_param
    
    def start(self):
        print(f"Custom component started on {self.game_object.name}")
    
    def update(self, engine):
        # Update logic here
        pass

# Add custom component
player.add_component(CustomComponent, custom_param=20)

# Get component
custom_comp = player.get_component(CustomComponent)
```

**Explanation**: Components are reusable pieces of functionality that can be attached to GameObjects. They have lifecycle methods (`start()`, `update()`, `on_destroy()`) and can access their parent GameObject.

### Script System

The Script system provides a powerful way to create reusable game logic that can be attached to GameObjects. Scripts are similar to components but offer additional features:

- **Configuration System**: Scripts can accept parameters during creation and runtime
- **Component Access**: Built-in methods to easily access and require components
- **Lifecycle Management**: Automatic initialization and cleanup
- **File-based**: Scripts are loaded from external Python files

#### Creating and Adding Scripts

```python
# Create script file: player_script.py
class PlayerScript(Script):
    def __init__(self, game_object, speed=200, health=100):
        super().__init__(game_object, speed=speed, health=health)
        self.speed = speed
        self.health = health
    
    def start(self, engine):
        """Called once when the script is first initialized."""
        print(f"PlayerScript started on {self.game_object.name}")
        print(f"Speed: {self.speed}, Health: {self.health}")
        
        # Access components in start method
        rigidbody = self.get_rigidbody()
        if rigidbody:
            rigidbody.set_velocity(Vector2(0, 0))
    
    def update(self, engine):
        """Called every frame in the game loop."""
        keys = pg.key.get_pressed()
        movement = Vector2(0, 0)
        
        if keys[pg.K_a]: movement.x -= 1
        if keys[pg.K_d]: movement.x += 1
        if keys[pg.K_w]: movement.y -= 1
        if keys[pg.K_s]: movement.y += 1
        
        if movement.length() > 0:
            movement = movement.normalize() * self.speed * engine.dt()
            self.game_object.position += movement

# Add script to GameObject with parameters
player.add_script("player_script.py", "PlayerScript", speed=300, health=150)

# Get and configure script at runtime
player_script = player.get_script(PlayerScript)
if player_script:
    player_script.set_config("speed", 400)
    player_script.speed = 400  # Direct property access also works
```

**Explanation**: The `start(engine)` method is called once when the script is first initialized, allowing for one-time setup like component access and initial configuration. The `update(engine)` method is called every frame for continuous logic. Scripts can accept parameters during creation and be reconfigured at runtime using `set_config()` or direct property access.

**Key Differences from Components**:
- Scripts are loaded from external files, making them more modular
- Built-in configuration system with `get_config()` and `set_config()`
- Convenience methods like `get_rigidbody()` and `get_collider()`
- Automatic lifecycle management with proper initialization
- `start(engine)` method for one-time initialization
- Parameter passing during script creation and runtime configuration

### Performance Optimization

```python
# Use global dictionary for frequently accessed data
engine.globals.set("player_health", 100, "player")
engine.globals.set("game_state", "playing", "game")

# Cache frequently used values
health = engine.globals.get("player_health", category="player")

# Use appropriate runnable priorities
engine.add_runnable(input_handler, 'key_press', Priority.CRITICAL)
engine.add_runnable(physics_update, 'physics_update', Priority.HIGH)
engine.add_runnable(gameplay_logic, 'update', Priority.NORMAL)
engine.add_runnable(ui_render, 'render', Priority.LOW)

# Monitor performance
def performance_monitor(engine):
    stats = engine.get_runnable_stats()
    if stats['total_runnables'] > 100:
        print("Warning: Too many active runnables")

engine.add_runnable(performance_monitor, 'update', Priority.LOW)
```

**Explanation**: The global dictionary provides thread-safe storage for game state. Caching frequently accessed values reduces lookup overhead. Runnable priorities ensure critical systems (input, physics) execute before less important ones (UI rendering).

---

## Integration Examples

### Physics Demo with Mouse Spawning

```python
from pyg_engine import Engine, Priority, GameObject, RigidBody, BoxCollider, CircleCollider, Input
from pygame import Vector2, Color

engine = Engine(windowName="Physics Demo")

# Spawn objects at mouse click
def spawn_at_mouse(engine):
    # Check for left mouse button press using event system
    if engine.input.get_event_state('mouse_button_down', 0):
        # Get mouse position in screen coordinates
        mouse_pos = engine.input.mouse.get_pos()
        # Convert to world coordinates using camera
        world_pos = engine.camera.screen_to_world(mouse_pos)
        
        # Random shape
        import random
        if random.random() > 0.5:
            obj = GameObject(name="box", position=world_pos, size=Vector2(30, 30))
            obj.add_component(RigidBody, mass=1.0)
            obj.add_component(BoxCollider, width=30, height=30)
        else:
            obj = GameObject(name="circle", position=world_pos, size=Vector2(30, 30))
            obj.add_component(RigidBody, mass=1.0)
            obj.add_component(CircleCollider, radius=15)
        
        engine.addGameObject(obj)

engine.add_runnable(spawn_at_mouse, 'update', Priority.NORMAL)

# Toggle gravity
def toggle_gravity(engine):
    if engine.input.get(Input.Keybind.G):
        gravity_enabled = engine.globals.get('gravity_enabled', True, 'physics')
        engine.globals.set('gravity_enabled', not gravity_enabled, 'physics')
        
        if gravity_enabled:
            engine.physics_system.set_gravity(0, 0)
        else:
            engine.physics_system.set_gravity(0, -500)

engine.add_runnable(toggle_gravity, 'update', Priority.NORMAL)

engine.start()
```

**Explanation**: This demo creates physics objects at mouse click positions using the modern input system. Randomly spawns either boxes or circles with physics components. The 'G' key toggles gravity on/off, demonstrating real-time physics system control.

### Camera Follow System

```python
def camera_follow_system(engine):
    # Find player object
    for obj in engine.getGameObjects():
        if obj.name == "player":
            engine.camera.follow(obj, offset=Vector2(0, -100))
            break

engine.add_runnable(camera_follow_system, 'start', Priority.CRITICAL, max_runs=1)
```

**Explanation**: This system automatically finds the player object and sets the camera to follow it with a vertical offset. The `max_runs=1` ensures it only runs once at startup.

### Mouse Interaction System

```python
# Add mouse interaction to objects using the input system
def add_mouse_interaction(engine):
    for obj in engine.getGameObjects():
        if obj.name.startswith("interactive"):
            # Use the input system to detect mouse interactions
            mouse_pos = engine.input.mouse.get_pos()
            world_pos = engine.camera.screen_to_world(mouse_pos)
            
            # Check if mouse is over the object
            obj_rect = pygame.Rect(
                obj.position.x - obj.size.x/2,
                obj.position.y - obj.size.y/2,
                obj.size.x,
                obj.size.y
            )
            
            if obj_rect.collidepoint(world_pos.x, world_pos.y):
                # Mouse is over the object
                if engine.input.get_event_state('mouse_button_down', 0):
                    print(f"Clicked on {obj.name}")

engine.add_runnable(add_mouse_interaction, 'update', Priority.NORMAL)
```

**Explanation**: This system uses the modern input system to detect mouse interactions with objects. It checks if the mouse is over interactive objects and handles click events.

---

## Best Practices

### Global Dictionary
- Use categories to organize variables logically
- Cache frequently accessed values in local variables
- Clean up unused variables to prevent memory leaks

### Runnable System
- Choose appropriate priorities for execution order
- Use execution limits for temporary effects
- Handle errors gracefully with custom error handlers

### Engine Integration
- Initialize systems in start runnables
- Use delta time for smooth movement
- Clean up resources regularly

### Performance Optimization
- Minimize global dictionary access in loops
- Use appropriate runnable frequencies
- Monitor performance with statistics

---

## Testing

Run the Global Dictionary test:
```bash
cd pyg_engine
PYTHONPATH=. python examples/global_dictionary_test.py
```

Run the Runnable System demo:
```bash
cd pyg_engine
PYTHONPATH=. python examples/visual_runnable_demo.py
```

Run the Mouse Input test:
```bash
cd pyg_engine
PYTHONPATH=. python examples/mouse_test.py
```

---

This guide provides the foundation for building robust, performant games with the PyG Engine. The systems are designed to work together seamlessly while maintaining flexibility and performance. 