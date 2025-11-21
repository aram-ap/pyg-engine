# Pyg-Engine Structure Breakdown

### Main rendering system: SDL3
- Video (GPU acceleration)
- Input Events
- Audio
- Threads
- Time
- File I/O

### Pyg-Engine:
- Rendering / Graphics
    – Draws 2‑D, handles shaders, lighting, shadows, and post‑processing.
- Physics Simulation (Mostly pymunk, but will need to be wrapped. Shouldn't need to import pymunk to use this game engine)
    – rigid‑body dynamics
        - gravity
        - material properties
        - springs
    - collision detection
    - soft‑body physics
- Audio System
    – Manages sound playback, spatial audio, mixing, effects (reverb, doppler), and supports various audio formats.
- Scripting / Gameplay Logic
    – Embeds a scripting language Python so designers can implement game rules without recompiling the engine core.
- Input Management
    – Abstracts keyboards, mice, gamepads, touch, and motion controllers into a unified event system.
- Animation System
    – Handles skeletal animation, blend trees, morph targets, inverse kinematics, and procedural animation
- User Interface (UI) System (ImGui)
    – Provides tools for creating HUDs, menus, and in‑game UI using widgets, layout engines, and skinning
- Scene / World Management
    – object organization into scenes or levels, supports hierarchical transforms, culling, and streaming of large worlds
- Asset Pipeline & Management
    – Imports, converts, and caches textures, models, audio, and other resources; maybe include compression and runtime loading
- Networking
    – low‑level socket handling, replication, client‑server synchronization, lag compensation, and matchmaking support

- Memory & Resource Management (Later on)
- Tooling and editor
- Profiling and Debugging


# Pyg-Engine Architecture Documentation

## Table of Contents
1. [High-Level System Architecture](#high-level-system-architecture)
2. [Component Lifecycle Flow](#component-lifecycle-flow)
3. [GameObject Hierarchy System](#gameobject-hierarchy-system)
4. [Event System Architecture](#event-system-architecture)
5. [Coroutine Execution Flow](#coroutine-execution-flow)
6. [Physics System Integration](#physics-system-integration)
7. [Complete Directory Structure](#complete-directory-structure)
8. [Data Flow: Engine Update Cycle](#data-flow-engine-update-cycle)
9. [Component Communication Patterns](#component-communication-patterns)
10. [Memory Management & Object Lifecycle](#memory-management--object-lifecycle)
11. [Rendering Pipeline](#rendering-pipeline)

---

## High-Level System Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                         PYG-ENGINE PUBLIC API                       │
│    (GameObject, Component, Events, Coroutines, Scene Management)    │
└─────────────────────────────────────────────────────────────────────┘
                                    │
                    ┌───────────────┴───────────────┐
                    │                               │
         ┌──────────▼──────────┐         ┌─────────▼─────────┐
         │   ENGINE CORE       │         │   SCENE MANAGER   │
         │   (Singleton)       │◄────────┤   (Multi-Scene)   │
         └──────────┬──────────┘         └─────────┬─────────┘
                    │                               │
        ┌───────────┼───────────┬──────────────────┼────────────┐
        │           │           │                  │            │
┌───────▼────┐ ┌───▼────┐ ┌────▼────┐      ┌─────▼──────┐ ┌──▼──────┐
│ Coroutine  │ │ Event  │ │  Time   │      │   Scene    │ │Resource │
│  Manager   │ │  Bus   │ │ Manager │      │GameObject  │ │ Manager │
└────────────┘ └────────┘ └─────────┘      │ Hierarchy  │ └─────────┘
                                            └─────┬──────┘
                                                  │
                    ┌─────────────────────────────┼─────────────────────┐
                    │                             │                     │
            ┌───────▼────────┐          ┌────────▼────────┐    ┌──────▼──────┐
            │   RENDERING    │          │    PHYSICS      │    │    AUDIO    │
            │   SYSTEM       │          │    SYSTEM       │    │   SYSTEM    │
            │   (SDL3)       │          │   (pymunk)      │    │   (SDL3)    │
            └────────────────┘          └─────────────────┘    └─────────────┘
                    │                            │                     │
            ┌───────┼────────┐          ┌────────┼────────┐    ┌──────┼──────┐
            │       │        │          │        │        │    │      │      │
         ┌──▼──┐ ┌─▼──┐ ┌───▼───┐   ┌──▼───┐ ┌─▼──┐ ┌───▼──┐ ┌▼──┐ ┌─▼──┐
         │Sprite│ │Light│ │Shader│   │Rigid │ │Coll│ │Rayca│ │Mix│ │Spat│
         │      │ │     │ │      │   │Body  │ │ider│ │st   │ │   │ │ial │
         └──────┘ └─────┘ └──────┘   └──────┘ └────┘ └─────┘ └───┘ └────┘
                    │
            ┌───────▼────────┐
            │   UI SYSTEM    │
            │    (ImGui)     │
            └────────────────┘
                    │
    ┌───────────────┴────────────────┐
    │                                │
┌───▼───────┐              ┌────────▼─────────┐
│  Input    │              │   Animation      │
│  Manager  │              │    System        │
└───────────┘              └──────────────────┘
```

---

## Component Lifecycle Flow

```
GameObject Creation
        │
        ▼
┌───────────────────────────────────────────────┐
│  GameObject.__init__()                        │
│  - Create Transform                           │
│  - Initialize hierarchy                       │
│  - Add to GameObject._all_objects             │
└───────────────┬───────────────────────────────┘
                │
                ▼
┌───────────────────────────────────────────────┐
│  GameObject.add_component(ComponentType)      │
│  - Instantiate component                      │
│  - Store in _components dict                  │
└───────────────┬───────────────────────────────┘
                │
                ▼
        ┌───────────────┐
        │ AWAKE()       │  ◄── Called immediately
        └───────┬───────┘      (initialization)
                │
                ▼
        ┌───────────────┐
        │ ON_ENABLE()   │  ◄── If active & enabled
        └───────┬───────┘
                │
                ▼
        ┌───────────────┐
        │ START()       │  ◄── Before first Update
        └───────┬───────┘      (called once)
                │
    ┌───────────┴───────────┐
    │                       │
    ▼                       ▼
┌──────────────┐    ┌──────────────┐
│ FIXED_UPDATE │    │   UPDATE     │  ◄── Every frame
│ (50 FPS)     │    │ (delta_time) │
└──────┬───────┘    └──────┬───────┘
       │                   │
       │                   ▼
       │           ┌──────────────┐
       │           │ LATE_UPDATE  │  ◄── After all Updates
       │           └──────┬───────┘
       │                  │
       └──────────┬───────┘
                  │
    ┌─────────────┴─────────────┐
    │                           │
    ▼                           ▼
┌──────────────┐        ┌──────────────┐
│ ON_DISABLE() │        │ ON_DESTROY() │
└──────────────┘        └──────────────┘
```

### Lifecycle Methods Description

| Method | When Called | Purpose |
|--------|-------------|---------|
| `awake()` | Immediately when component is added | Initialize references, set up internal state |
| `on_enable()` | When component/GameObject becomes active | Subscribe to events, enable features |
| `start()` | Before first frame (after all awake() calls) | Access other components, finalize setup |
| `fixed_update()` | Fixed time intervals (50 FPS default) | Physics calculations, fixed-rate logic |
| `update()` | Every frame | Main gameplay logic, input handling |
| `late_update()` | After all update() calls | Camera following, procedural animation |
| `on_disable()` | When component/GameObject becomes inactive | Unsubscribe events, cleanup |
| `on_destroy()` | When component/GameObject is destroyed | Final cleanup, save state |

---

## GameObject Hierarchy System

```
                    ┌─────────────────┐
                    │ GameObject Root │
                    │  (parent: None) │
                    └────────┬────────┘
                             │
                   ┌─────────┴─────────┐
                   │    Transform      │
                   │  - local_position │
                   │  - local_rotation │
                   │  - local_scale    │
                   └─────────┬─────────┘
                             │
        ┏━━━━━━━━━━━━━━━━━━━━┻━━━━━━━━━━━━━━━━━━━━┓
        ▼                                          ▼
┌───────────────┐                        ┌───────────────┐
│ GameObject    │                        │ GameObject    │
│ "Player"      │                        │ "Enemy"       │
└───────┬───────┘                        └───────────────┘
        │
        │ parent/child relationship
        │
    ┌───┴───┬────────┬────────┐
    ▼       ▼        ▼        ▼
┌────────┐┌──────┐┌───────┐┌──────┐
│"Weapon"││"Head"││"Body" ││"Legs"│
└────────┘└──────┘└───────┘└──────┘
    │
    └──► Child GameObjects
         - Inherit active state
         - Transform relative to parent
         - Recursive search methods
```

### Hierarchy API Methods

```python
# Parent/Child Management
player.parent = root_object              # Set parent
weapon.parent = player                   # weapon becomes child of player
player.parent = None                     # Remove from parent

# Accessing Children
child_count = player.get_child_count()   # Number of children
first_child = player.get_child(0)        # Get by index
all_children = player.children           # Get all children (copy)

# Finding Objects
weapon = player.find_child("Weapon")     # Find immediate child
weapon = player.find_child_recursive("Weapon")  # Search recursively
enemies = player.get_children_with_tag("Enemy") # Filter by tag

# Hierarchy Queries
root = player.get_root()                 # Get root GameObject
is_child = weapon.is_child_of(player)    # Check relationship
player.detach_children()                 # Remove all children

# Active State Propagation
player.set_active(False)                 # Deactivate player and all children
is_active = weapon.active_in_hierarchy   # Check if active considering parents
```

---

## Event System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                       EVENT SYSTEM                          │
└─────────────────────────────────────────────────────────────┘
                             │
            ┌────────────────┴────────────────┐
            │                                 │
    ┌───────▼────────┐              ┌────────▼─────────┐
    │  Local Events  │              │  Global EventBus │
    │  (Per Object)  │              │   (Singleton)    │
    └───────┬────────┘              └────────┬─────────┘
            │                                 │
    ┌───────┴────────┐              ┌────────┴─────────┐
    │                │              │                  │
    ▼                ▼              ▼                  ▼
┌────────┐    ┌──────────┐   ┌──────────┐      ┌──────────┐
│on_death│    │on_hit    │   │"player_  │      │"level_   │
│.invoke()    │.invoke() │   │spawned" │      │complete" │
└────┬───┘    └────┬─────┘   └────┬─────┘      └────┬─────┘
     │             │              │                  │
     ▼             ▼              ▼                  ▼
┌─────────────────────────────────────────────────────────┐
│              LISTENER CALLBACKS                         │
│  - event += lambda: callback()                          │
│  - event -= callback (unsubscribe)                      │
│  - EventBus.subscribe("event_name", callback)           │
└─────────────────────────────────────────────────────────┘
```

### Event System Usage

#### Local Component Events

```python
from pyg import Component, Event

class Player(Component):
    def __init__(self, game_object):
        super().__init__(game_object)
        # Create events
        self.on_death = Event()
        self.on_damage_taken = Event()
        self.on_level_up = Event()

    def take_damage(self, amount):
        self.health -= amount
        # Trigger event with data
        self.on_damage_taken.invoke(amount, self.health)

        if self.health <= 0:
            self.on_death.invoke(self)

# Subscribing to events
player_comp = player.get_component(Player)

# Method 1: Using += operator
player_comp.on_death += self.respawn_player
player_comp.on_damage_taken += lambda amt, hp: print(f"Took {amt} damage!")

# Method 2: Using add_listener
player_comp.on_death.add_listener(self.game_over)

# Unsubscribing
player_comp.on_death -= self.respawn_player
```

#### Global EventBus

```python
from pyg import EventBus

# Get the global event bus (singleton)
event_bus = EventBus()

# Subscribe anywhere in your game
event_bus.subscribe("enemy_spawned", self.update_enemy_counter)
event_bus.subscribe("level_complete", self.show_victory_screen)
event_bus.subscribe("game_paused", self.pause_audio)

# Publish from anywhere
event_bus.publish("enemy_spawned", enemy_type="zombie", position=Vector2(100, 50))
event_bus.publish("level_complete", score=1500, time=120.5)

# Clean up
event_bus.unsubscribe("enemy_spawned", self.update_enemy_counter)
event_bus.clear_event("level_complete")  # Remove all listeners for this event
```

---

## Coroutine Execution Flow

```
Component.start_coroutine(generator_function())
            │
            ▼
┌───────────────────────────────────────┐
│  Coroutine Object Created             │
│  - Wraps Python generator             │
│  - Registered in CoroutineManager     │
└───────────────┬───────────────────────┘
                │
    ┌───────────┴──────────┐
    │   Engine.run() loop  │
    └───────────┬──────────┘
                │
                ▼
┌───────────────────────────────────────┐
│  CoroutineManager.update()            │
│  (called every frame)                 │
└───────────────┬───────────────────────┘
                │
                ▼
        ┌───────────────┐
        │ Coroutine.    │
        │ update()      │
        └───────┬───────┘
                │
    ┌───────────┴───────────┐
    │                       │
    ▼                       ▼
┌─────────────┐     ┌──────────────┐
│ Yield       │     │ Continue     │
│ Instruction │     │ Immediately  │
└──────┬──────┘     └──────────────┘
       │
       ▼
┌─────────────────────────────────────┐
│  YieldInstruction.is_done()?        │
├─────────────────────────────────────┤
│  WaitForSeconds(2.0)                │
│  WaitUntil(lambda: health <= 0)     │
│  WaitWhile(lambda: is_moving)       │
│  WaitForFixedUpdate()               │
│  Nested Coroutine                   │
└──────┬──────────────────────────────┘
       │
       ▼ (when done)
┌─────────────────┐
│ next(generator) │  ─► Continue coroutine
└─────────────────┘
       │
       ▼ (StopIteration)
┌─────────────────┐
│ Coroutine Done  │  ─► Remove from manager
└─────────────────┘
```

### Coroutine Examples

```python
from pyg import Component, Vector2
from pyg.utils.coroutine import WaitForSeconds, WaitUntil, WaitWhile

class Enemy(Component):
    def start(self):
        # Start multiple coroutines
        self.start_coroutine(self.patrol())
        self.start_coroutine(self.fade_in())

    def patrol(self):
        """Patrol between waypoints"""
        waypoints = [Vector2(100, 100), Vector2(400, 100),
                    Vector2(400, 300), Vector2(100, 300)]
        index = 0

        while True:
            target = waypoints[index]

            # Move towards waypoint
            while (self.transform.position - target).length() > 5:
                direction = (target - self.transform.position).normalized()
                self.transform.position += direction * self.speed * 0.016
                yield  # Wait one frame

            # Wait at waypoint
            yield WaitForSeconds(2.0)

            # Next waypoint
            index = (index + 1) % len(waypoints)

    def fade_in(self):
        """Fade sprite in over 1 second"""
        sprite = self.get_component(Sprite)
        duration = 1.0
        elapsed = 0.0

        while elapsed < duration:
            elapsed += 0.016  # Approximate frame time
            alpha = elapsed / duration
            sprite.color.a = alpha
            yield

        sprite.color.a = 1.0

    def take_damage(self, amount):
        self.health -= amount

        if self.health <= 0:
            self.start_coroutine(self.death_sequence())

    def death_sequence(self):
        """Play death animation then destroy"""
        # Flash red
        sprite = self.get_component(Sprite)
        original_color = sprite.color

        for _ in range(3):
            sprite.color = Color(255, 0, 0, 255)
            yield WaitForSeconds(0.1)
            sprite.color = original_color
            yield WaitForSeconds(0.1)

        # Fade out
        for i in range(10):
            sprite.color.a = 1.0 - (i / 10.0)
            yield WaitForSeconds(0.05)

        # Destroy GameObject
        GameObject.destroy(self.game_object)

    def wait_example(self):
        """Examples of different wait types"""
        # Wait for time
        yield WaitForSeconds(3.0)

        # Wait for condition
        yield WaitUntil(lambda: self.health <= 0)

        # Wait while condition is true
        yield WaitWhile(lambda: self.is_invincible)

        # Wait for physics update
        yield WaitForFixedUpdate()

        # Wait for another coroutine
        other_coroutine = self.start_coroutine(self.fade_in())
        yield other_coroutine  # Wait for it to finish
```

### Yield Instructions

| Instruction | Description | Usage |
|-------------|-------------|-------|
| `yield` | Wait one frame | `yield` |
| `WaitForSeconds(time)` | Wait for specified seconds | `yield WaitForSeconds(2.5)` |
| `WaitForFixedUpdate()` | Wait for next physics update | `yield WaitForFixedUpdate()` |
| `WaitUntil(predicate)` | Wait until condition is true | `yield WaitUntil(lambda: ready)` |
| `WaitWhile(predicate)` | Wait while condition is true | `yield WaitWhile(lambda: moving)` |
| `Coroutine` | Wait for another coroutine | `yield self.start_coroutine(...)` |

---

## Physics System Integration

```
┌────────────────────────────────────────────────────────────┐
│                    PUBLIC PHYSICS API                      │
│              (No pymunk exposure to users)                 │
└────────────────────────────────────────────────────────────┘
                             │
        ┌────────────────────┼────────────────────┐
        │                    │                    │
┌───────▼────────┐  ┌────────▼────────┐  ┌──────▼──────┐
│   RigidBody    │  │    Collider     │  │  Raycast    │
│   Component    │  │   Components    │  │  Utilities  │
├────────────────┤  ├─────────────────┤  └─────────────┘
│ - body_type    │  │ - BoxCollider   │
│ - mass         │  │ - CircleCollider│
│ - apply_force()│  │ - is_trigger    │
│ - velocity     │  │ - friction      │
└────────┬───────┘  └────────┬────────┘
         │                   │
         │  Internal pymunk objects (hidden from user)
         │                   │
         └──────────┬────────┘
                    │
         ┌──────────▼──────────┐
         │   PhysicsWorld      │
         │   ._space (pymunk)  │
         ├─────────────────────┤
         │ - gravity           │
         │ - step()            │
         │ - add_rigidbody()   │
         │ - remove_rigidbody()│
         └──────────┬──────────┘
                    │
         ┌──────────▼──────────┐
         │  pymunk.Space       │
         │  (Hidden Layer)     │
         │  - bodies           │
         │  - shapes           │
         │  - constraints      │
         └──────────┬──────────┘
                    │
         ┌──────────▼──────────┐
         │  Collision Events   │
         ├─────────────────────┤
         │ on_collision_enter()│
         │ on_collision_stay() │
         │ on_collision_exit() │
         │ on_trigger_enter()  │
         │ on_trigger_exit()   │
         └─────────────────────┘
```

### Physics API Usage

```python
from pyg import GameObject, RigidBody, BoxCollider, CircleCollider, Vector2
from pyg.physics import RigidBodyType, PhysicsMaterial

# Create GameObject with physics
ball = GameObject("Ball")

# Add RigidBody (required for physics)
rb = ball.add_component(RigidBody)
rb.body_type = RigidBodyType.DYNAMIC  # or STATIC, KINEMATIC
rb.mass = 10.0
rb.friction = 0.5
rb.restitution = 0.8  # Bounciness

# Add Collider (defines shape)
collider = ball.add_component(CircleCollider)
collider.radius = 25
collider.is_trigger = False  # Set to True for triggers

# Apply forces
rb.apply_force(Vector2(100, 0))           # Apply force at center
rb.apply_impulse(Vector2(0, 500))         # Instant force
rb.velocity = Vector2(50, 0)              # Direct velocity control

# Collision callbacks in custom component
class Ball(Component):
    def on_collision_enter(self, collision):
        print(f"Hit {collision.other.game_object.name}")
        print(f"Contact point: {collision.contact_point}")
        print(f"Normal: {collision.normal}")

    def on_collision_stay(self, collision):
        pass  # While collision continues

    def on_collision_exit(self, collision):
        pass  # When collision ends

    def on_trigger_enter(self, collider):
        if collider.game_object.tag == "Pickup":
            print("Entered pickup zone!")
```

---

## Complete Directory Structure

```
pyg-engine/
│
├── pyg/                                    # Main package
│   ├── __init__.py                        # Public API exports
│   │
│   ├── core/                              # Core engine systems
│   │   ├── __init__.py
│   │   ├── engine.py                      # Engine singleton (main loop)
│   │   ├── game_object.py                 # GameObject with hierarchy
│   │   ├── component.py                   # Base Component class
│   │   ├── transform.py                   # Transform with parent/child
│   │   └── time.py                        # Time & delta time management
│   │
│   ├── scene/                             # Scene management
│   │   ├── __init__.py
│   │   ├── scene.py                       # Scene class
│   │   ├── scene_manager.py              # Scene loading/switching
│   │   ├── camera.py                      # Camera system
│   │   └── scene_serializer.py           # Save/Load scenes (future)
│   │
│   ├── rendering/                         # Graphics system
│   │   ├── __init__.py
│   │   ├── renderer.py                    # Main renderer (SDL3 wrapper)
│   │   ├── sprite.py                      # Sprite component
│   │   ├── sprite_renderer.py            # Sprite rendering logic
│   │   ├── texture.py                     # Texture management
│   │   ├── material.py                    # Material system
│   │   ├── shader.py                      # Shader system
│   │   ├── lighting/
│   │   │   ├── __init__.py
│   │   │   ├── light.py                   # Light component
│   │   │   ├── point_light.py
│   │   │   ├── directional_light.py
│   │   │   └── lighting_system.py
│   │   ├── particle.py                    # Particle system
│   │   ├── camera_renderer.py            # Camera rendering
│   │   └── post_processing.py            # Post-processing effects
│   │
│   ├── physics/                           # Physics system (pymunk wrapper)
│   │   ├── __init__.py
│   │   ├── physics_world.py              # PhysicsWorld (wraps pymunk.Space)
│   │   ├── rigidbody.py                  # RigidBody component
│   │   ├── collider.py                   # Base Collider
│   │   ├── box_collider.py               # BoxCollider component
│   │   ├── circle_collider.py            # CircleCollider component
│   │   ├── polygon_collider.py           # PolygonCollider component
│   │   ├── physics_material.py           # Physics material properties
│   │   ├── collision.py                  # Collision data class
│   │   ├── raycast.py                    # Raycasting utilities
│   │   └── joints/                       # Physics joints
│   │       ├── __init__.py
│   │       ├── spring_joint.py
│   │       ├── hinge_joint.py
│   │       └── distance_joint.py
│   │
│   ├── audio/                            # Audio system
│   │   ├── __init__.py
│   │   ├── audio_manager.py              # Audio system (SDL3 wrapper)
│   │   ├── audio_source.py               # AudioSource component
│   │   ├── audio_listener.py             # AudioListener component
│   │   ├── audio_clip.py                 # Audio asset class
│   │   ├── audio_mixer.py                # Audio mixing
│   │   └── audio_effects.py              # Reverb, doppler, etc.
│   │
│   ├── input/                            # Input system
│   │   ├── __init__.py
│   │   ├── input_manager.py              # Unified input system
│   │   ├── keyboard.py                   # Keyboard input
│   │   ├── mouse.py                      # Mouse input
│   │   ├── gamepad.py                    # Gamepad/controller
│   │   ├── touch.py                      # Touch input (mobile)
│   │   └── input_axis.py                 # Virtual axis system
│   │
│   ├── ui/                               # UI system
│   │   ├── __init__.py
│   │   ├── ui_manager.py                 # ImGui wrapper/manager
│   │   ├── canvas.py                     # UI Canvas
│   │   ├── widgets/
│   │   │   ├── __init__.py
│   │   │   ├── button.py
│   │   │   ├── text.py
│   │   │   ├── image.py
│   │   │   ├── slider.py
│   │   │   └── panel.py
│   │   └── layout.py                     # Layout systems
│   │
│   ├── animation/                        # Animation system
│   │   ├── __init__.py
│   │   ├── animator.py                   # Animator component
│   │   ├── animation_clip.py             # Animation data
│   │   ├── animation_curve.py            # Curves/easing
│   │   ├── sprite_animator.py            # 2D sprite animation
│   │   ├── tween.py                      # Tweening utilities
│   │   └── state_machine/
│   │       ├── __init__.py
│   │       ├── animator_controller.py
│   │       ├── animation_state.py
│   │       └── transition.py
│   │
│   ├── resources/                        # Asset management
│   │   ├── __init__.py
│   │   ├── resource_manager.py           # Asset loading/caching
│   │   ├── asset.py                      # Base asset class
│   │   ├── asset_loader.py               # Format loaders
│   │   ├── prefab.py                     # Prefab system (future)
│   │   └── loaders/
│   │       ├── __init__.py
│   │       ├── texture_loader.py
│   │       ├── audio_loader.py
│   │       └── json_loader.py
│   │
│   ├── utils/                            # Utilities
│   │   ├── __init__.py
│   │   ├── math.py                       # Vector2, Vector3, Quaternion
│   │   ├── color.py                      # Color class
│   │   ├── rect.py                       # Rect/Bounds
│   │   ├── logger.py                     # Logging system
│   │   ├── events.py                     # Event system
│   │   ├── coroutine.py                  # Coroutine system
│   │   ├── singleton.py                  # Singleton decorator
│   │   └── debug.py                      # Debug utilities
│   │
│   ├── networking/                       # Networking (future)
│   │   ├── __init__.py
│   │   ├── network_manager.py
│   │   ├── client.py
│   │   ├── server.py
│   │   └── replication.py
│   │
│   └── editor/                           # Editor tools (future)
│       ├── __init__.py
│       ├── editor_window.py
│       ├── inspector.py
│       ├── hierarchy_view.py
│       └── scene_view.py
│
├── examples/                             # Example projects
│   ├── 01_hello_world.py
│   ├── 02_physics_demo.py
│   ├── 03_platformer/
│   │   ├── main.py
│   │   ├── player.py
│   │   └── enemy.py
│   ├── 04_ui_demo.py
│   ├── 05_coroutines_demo.py
│   ├── 06_events_demo.py
│   └── 07_hierarchy_demo.py
│
├── tests/                                # Unit tests
│   ├── test_game_object.py
│   ├── test_component.py
│   ├── test_hierarchy.py
│   ├── test_events.py
│   ├── test_coroutines.py
│   ├── test_physics.py
│   └── test_rendering.py
│
├── docs/                                 # Documentation
│   ├── index.md
│   ├── getting_started.md
│   ├── api/
│   │   ├── core.md
│   │   ├── components.md
│   │   └── events.md
│   ├── tutorials/
│   │   ├── first_game.md
│   │   ├── physics.md
│   │   └── coroutines.md
│   └── conf.py                          # Sphinx config
│
├── assets/                              # Example assets
│   ├── sprites/
│   ├── audio/
│   └── shaders/
│
├── setup.py                             # Setup script
├── pyproject.toml                       # Project config (PEP 621)
├── requirements.txt                     # Dependencies
├── requirements-dev.txt                 # Dev dependencies
├── README.md                            # Main README
├── LICENSE                              # License file
├── CHANGELOG.md                         # Version history
└── .gitignore
```

---

## Data Flow: Engine Update Cycle

```
┌─────────────────────────────────────────────────────────────┐
│                    ENGINE.RUN() - MAIN LOOP                 │
└─────────────────────────────────────────────────────────────┘
                            │
        ┌───────────────────┼───────────────────┐
        │                   │                   │
        ▼                   ▼                   ▼
┌───────────────┐   ┌───────────────┐   ┌──────────────┐
│ INPUT.PROCESS │   │  TIME.TICK()  │   │ COROUTINES.  │
│   _EVENTS()   │   │               │   │   UPDATE()   │
└───────┬───────┘   └───────┬───────┘   └──────┬───────┘
        │                   │                   │
        └─────────┬─────────┴──────────┬────────┘
                  │                    │
          ┌───────▼───────┐    ┌───────▼────────┐
          │ FIXED UPDATE  │    │    UPDATE      │
          │  (Physics)    │    │ (Game Logic)   │
          └───────┬───────┘    └───────┬────────┘
                  │                    │
          Scene._fixed_update()   Scene._update()
                  │                    │
          ┌───────▼───────┐    ┌───────▼────────┐
          │  GameObject   │    │  GameObject    │
          │  .fixed_      │    │  ._update()    │
          │  update()     │    └───────┬────────┘
          └───────┬───────┘            │
                  │            ┌───────▼────────┐
                  │            │ Component.     │
                  │            │ _internal_     │
                  │            │ update()       │
                  │            └───────┬────────┘
                  │                    │
          ┌───────▼────────────────────▼────────┐
          │     PhysicsWorld.step()             │
          │     - Collision detection           │
          │     - Physics simulation            │
          │     - Trigger events                │
          └───────┬─────────────────────────────┘
                  │
          ┌───────▼─────────────────────────────┐
          │  Collision Callbacks                │
          │  - on_collision_enter()             │
          │  - on_collision_stay()              │
          │  - on_collision_exit()              │
          │  - on_trigger_enter()               │
          │  - on_trigger_exit()                │
          └───────┬─────────────────────────────┘
                  │
          ┌───────▼─────────────────────────────┐
          │    Scene._late_update()             │
          │    - Post-processing logic          │
          │    - Camera follow                  │
          │    - Animation finalization         │
          └───────┬─────────────────────────────┘
                  │
          ┌───────▼─────────────────────────────┐
          │        RENDERING PHASE              │
          │  - Renderer.clear()                 │
          │  - Scene.render()                   │
          │    • Sort by layer/depth            │
          │    • Culling                        │
          │    • Sprite batching                │
          │  - UIManager.render()               │
          │  - Renderer.present()               │
          └─────────────────────────────────────┘
                  │
                  └──► Back to top (next frame)
```

### Frame Timing

| Phase | Rate | Purpose |
|-------|------|---------|
| Input Processing | Variable | Handle keyboard, mouse, gamepad events |
| Time.tick() | Variable | Calculate delta_time, track frame timing |
| Coroutines.update() | Variable | Progress all active coroutines |
| Fixed Update | 50 FPS (default) | Physics simulation, consistent rate |
| Update | Variable (target 60 FPS) | Main gameplay logic |
| Late Update | Variable | Post-update cleanup, camera following |
| Rendering | Variable | Draw to screen |

---

## Component Communication Patterns

```
┌────────────────────────────────────────────────────────────┐
│              COMPONENT COMMUNICATION                       │
└────────────────────────────────────────────────────────────┘
                            │
        ┌───────────────────┼───────────────────┐
        │                   │                   │
        ▼                   ▼                   ▼
┌───────────────┐   ┌───────────────┐   ┌──────────────┐
│ DIRECT ACCESS │   │ EVENT SYSTEM  │   │  FIND/SEARCH │
└───────┬───────┘   └───────┬───────┘   └──────┬───────┘
        │                   │                   │
        │                   │                   │
        ▼                   ▼                   ▼
```

### Pattern 1: Direct Component Access

```python
# Get component on same GameObject
rb = self.get_component(RigidBody)
rb.apply_force(Vector2(100, 0))

# Get component in children
weapon_sprite = self.get_component_in_children(Sprite)
weapon_sprite.color = Color(255, 0, 0)

# Get component in parent
player_health = self.get_component_in_parent(HealthComponent)
player_health.heal(10)
```

### Pattern 2: Local Events

```python
# Publisher Component
class Enemy(Component):
    def __init__(self, game_object):
        super().__init__(game_object)
        self.on_death = Event()

    def die(self):
        self.on_death.invoke(self)

# Subscriber Component
class ScoreManager(Component):
    def start(self):
        # Find all enemies and subscribe
        enemies = GameObject.find_game_objects_with_tag("Enemy")
        for enemy in enemies:
            enemy_comp = enemy.get_component(Enemy)
            enemy_comp.on_death += self.add_score

    def add_score(self, enemy):
        self.score += 100
```

### Pattern 3: Global Events

```python
# Publisher (anywhere)
EventBus().publish("enemy_died", position=self.transform.position, score=50)

# Subscriber (anywhere)
class GameManager(Component):
    def start(self):
        EventBus().subscribe("enemy_died", self.on_enemy_died)

    def on_enemy_died(self, position, score):
        self.total_score += score
        self.spawn_particles(position)
```

### Pattern 4: GameObject Finding

```python
# By name (searches root objects only)
player = GameObject.find("Player")

# By tag
enemies = GameObject.find_game_objects_with_tag("Enemy")
pickups = GameObject.find_game_objects_with_tag("Pickup")

# In hierarchy
weapon = self.game_object.find_child("Weapon")
head = self.game_object.find_child_recursive("Head")
```

### Communication Pattern Comparison

| Pattern | Use Case | Pros | Cons |
|---------|----------|------|------|
| Direct Access | Tightly coupled components | Fast, type-safe | Requires reference |
| Local Events | Object-specific notifications | Decoupled, flexible | Setup overhead |
| Global Events | Game-wide notifications | Completely decoupled | Hard to debug |
| GameObject Finding | Dynamic object lookup | No setup needed | Slower, fragile |

---

## Memory Management & Object Lifecycle

```
┌─────────────────────────────────────────────────────────┐
│              OBJECT LIFECYCLE MANAGEMENT                │
└─────────────────────────────────────────────────────────┘

Creation Phase:
┌──────────────────────────────────────┐
│ player = GameObject("Player")        │
│   └─► Added to GameObject.           │
│        _all_objects (set)            │
│                                      │
│ scene.add(player)                    │
│   └─► Added to Scene._objects (list) │
└──────────────────────────────────────┘
                │
                ▼
Active Lifetime:
┌──────────────────────────────────────┐
│ - Receives update() calls            │
│ - Receives collision events          │
│ - Coroutines running                 │
│ - Event listeners active             │
└──────────────────────────────────────┘
                │
                ▼
Destruction Request:
┌──────────────────────────────────────┐
│ GameObject.destroy(player)           │
│   or                                 │
│ GameObject.destroy(player, delay=2.0)│
└──────────────┬───────────────────────┘
               │
               ▼
┌──────────────────────────────────────┐
│ Destruction Sequence:                │
│ 1. Mark as _destroyed = True         │
│ 2. Call on_destroy() on components   │
│ 3. Stop all coroutines               │
│ 4. Destroy all children (recursive)  │
│ 5. Remove from parent                │
│ 6. Remove from scene                 │
│ 7. Remove from _all_objects          │
│ 8. Python GC collects                │
└──────────────────────────────────────┘
```

### Best Practices

```python
# Immediate destruction
GameObject.destroy(enemy)

# Delayed destruction (useful for death animations)
GameObject.destroy(enemy, delay=2.0)

# Proper cleanup in components
class MyComponent(Component):
    def start(self):
        # Subscribe to events
        EventBus().subscribe("game_over", self.on_game_over)

    def on_destroy(self):
        # Unsubscribe to prevent memory leaks
        EventBus().unsubscribe("game_over", self.on_game_over)
        # Stop coroutines (automatic, but can do manually)
        self.stop_all_coroutines()

# Don't reference destroyed objects
if not game_object._destroyed:
    game_object.transform.position = Vector2(0, 0)
```

---

## Rendering Pipeline

```
┌─────────────────────────────────────────────────────────┐
│                  RENDERING PIPELINE                     │
└─────────────────────────────────────────────────────────┘
                        │
                        ▼
        ┌───────────────────────────┐
        │   Camera.render(scene)    │
        └───────────┬───────────────┘
                    │
        ┌───────────▼───────────┐
        │  Gather Renderable    │
        │  Components           │
        │  (Sprite, Light, etc) │
        └───────────┬───────────┘
                    │
        ┌───────────▼───────────┐
        │  Frustum Culling      │
        │  (if enabled)         │
        └───────────┬───────────┘
                    │
        ┌───────────▼───────────┐
        │  Sort by Layer/Depth  │
        │  - Background (-10)   │
        │  - Default (0)        │
        │  - Foreground (10)    │
        └───────────┬───────────┘
                    │
        ┌───────────▼───────────┐
        │  Batch Sprites        │
        │  (same texture)       │
        └───────────┬───────────┘
                    │
        ┌───────────▼───────────┐
        │  Apply Lighting       │
        │  (if enabled)         │
        └───────────┬───────────┘
                    │
        ┌───────────▼───────────┐
        │  Shader Pass          │
        │  - Vertex shader      │
        │  - Fragment shader    │
        └───────────┬───────────┘
                    │
        ┌───────────▼───────────┐
        │  Post-Processing      │
        │  - Bloom              │
        │  - Color grading      │
        │  - SSAO               │
        └───────────┬───────────┘
                    │
        ┌───────────▼───────────┐
        │  UI Overlay           │
        │  (ImGui)              │
        └───────────┬───────────┘
                    │
                    ▼
        ┌───────────────────────┐
        │  SDL_RenderPresent()  │
        └───────────────────────┘
```

### Rendering Layers

```python
# Set sprite layer
sprite = player.add_component(Sprite)
sprite.layer = 5  # Higher = rendered on top

# Common layer setup
LAYER_BACKGROUND = -10
LAYER_TERRAIN = -5
LAYER_OBJECTS = 0
LAYER_PLAYER = 5
LAYER_EFFECTS = 10
LAYER_UI = 100
```

---


