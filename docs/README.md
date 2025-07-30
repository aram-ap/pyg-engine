# Documentation

This directory contains documentation for the Pyg Engine project.

## Documentation Files

### `PHYSICS_CONTROLS_GUIDE.md`
**Purpose**: Guide to the physics system controls and features
- **Topics Covered**:
  - Rigidbody component usage
  - Collider system (circles, boxes, materials)
  - Physics materials and properties
  - Force-based vs velocity-based movement
  - Rotation locking system
  - Collision detection and callbacks
  - Gravity and drag settings
  - Kinematic vs dynamic bodies

### `MOUSE_INPUT_GUIDE.md`
**Purpose**: Guide to mouse input handling
- **Topics Covered**:
  - Mouse position tracking
  - Mouse button states
  - Mouse wheel handling
  - Input event processing
  - Mouse-based object manipulation
  - Drag and drop functionality
  - Mouse coordinate systems

### `SPRITE_SYSTEM_UPGRADE.md`
**Purpose**: Documentation of sprite system improvements
- **Topics Covered**:
  - Sprite rendering optimizations
  - Texture management
  - Sprite scaling and rotation
  - Performance improvements
  - Memory management
  - Rendering pipeline enhancements

## Quick Reference

### Physics System
- **Rigidbody**: `PymunkRigidBody` component for physics simulation
- **Colliders**: `PymunkCircleCollider`, `PymunkBoxCollider` for collision detection
- **Materials**: `PhysicsMaterial` for friction, bounce, and collision properties
- **Rotation Lock**: `lock_rotation=True/False` for Unity-style rotation constraints

### Input System
- **Mouse**: `MouseInput` component for mouse handling
- **Keyboard**: Direct pygame key handling in scripts
- **Events**: Event-driven input processing

### Rendering System
- **Sprites**: Optimized sprite rendering with rotation and scaling
- **Cameras**: `Camera` component for viewport management
- **Layers**: Collision layer system for selective collision detection

## Getting Started

1. **Physics**: Read `PHYSICS_CONTROLS_GUIDE.md` for physics system usage
2. **Input**: Read `MOUSE_INPUT_GUIDE.md` for input handling
3. **Rendering**: Read `SPRITE_SYSTEM_UPGRADE.md` for rendering optimizations

## Examples

- `main.py`: Complete physics demo with multiple objects
- `tests/`: test suite for all systems (kinda broken for now)
- `scripts/`: Example scripts demonstrating component usage
