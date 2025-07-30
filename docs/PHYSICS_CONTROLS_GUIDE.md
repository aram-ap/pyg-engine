# Pymunk Physics Controls Guide

## Overview

This guide explains how to handle user controls with Pymunk physics, based on research of how other developers implement controls in their projects.

## The Problem with Direct Position Manipulation

**Original Problem**: Your control system was erratic because it directly manipulated `game_object.position`, which conflicts with Pymunk's physics simulation.

```python
# ❌ BAD: Direct position manipulation
self.game_object.position += movement * self.speed * engine.dt()
```

This approach:
- Bypasses physics simulation
- Creates conflicts between manual positioning and physics
- Results in erratic, unpredictable movement
- Ignores collisions and physics constraints

## World-Axis Movement Fix

**Critical Issue**: Movement was relative to object rotation instead of world axes.

### The Problem
When using `add_force()` with Pymunk, forces are applied relative to the body's local coordinate system. If the object is rotated, the force direction is also rotated:

```python
# ❌ BAD: Force applied in local coordinates
self.rigidbody.add_force(Vector2(-move_force, 0))  # Relative to object rotation
```

### The Solution
Use `add_force_at_point()` to apply forces in world coordinates:

```python
# ✅ GOOD: Force applied in world coordinates
world_force = Vector2(-move_force, 0)
self.rigidbody.add_force_at_point(world_force, self.game_object.position)
```

### Implementation
All movement methods now use world-coordinate forces:

```python
def _force_based_movement(self, keys, mouse_pos, mouse_buttons, engine):
    # Horizontal movement - ALWAYS in world coordinates
    if any(keys[key] for key in self.movement_keys['left']):
        if current_vel.x > -self.max_speed:
            world_force = Vector2(-self.move_force, 0)
            self.rigidbody.add_force_at_point(world_force, self.game_object.position)
    
    # Jumping - ALWAYS in world coordinates
    if any(keys[key] for key in self.movement_keys['up']):
        if current_vel.y > -50:
            world_impulse = Vector2(0, -self.jump_force)
            self.rigidbody.add_impulse_at_point(world_impulse, self.game_object.position)
```

## Solution: Physics-Based Controls

### Approach 1: Force-Based Movement (Recommended)

**Best for**: Realistic physics, platformers, games with momentum

```python
# ✅ GOOD: Force-based movement in world coordinates
current_vel = self.rigidbody.velocity

if keys[pg.K_LEFT] and current_vel.x > -max_speed:
    world_force = Vector2(-move_force, 0)
    self.rigidbody.add_force_at_point(world_force, self.game_object.position)
if keys[pg.K_RIGHT] and current_vel.x < max_speed:
    world_force = Vector2(move_force, 0)
    self.rigidbody.add_force_at_point(world_force, self.game_object.position)
```

**Advantages**:
- Realistic physics behavior
- Proper momentum and inertia
- Natural collision response
- Speed limiting prevents excessive velocity
- **Movement always relative to world axes**

**Use cases**:
- Platformers with realistic physics
- Racing games
- Any game where momentum matters

### Approach 2: Velocity-Based Movement

**Best for**: Responsive controls, arcade-style games

```python
# ✅ GOOD: Velocity-based movement
target_velocity = Vector2(0, self.rigidbody.velocity.y)

if keys[pg.K_LEFT]:
    target_velocity.x = -self.speed
if keys[pg.K_RIGHT]:
    target_velocity.x = self.speed

# Smooth velocity change
current_vel = self.rigidbody.velocity
new_velocity = current_vel.lerp(target_velocity, 0.1)
self.rigidbody.set_velocity(new_velocity)
```

**Advantages**:
- Responsive controls
- Predictable movement
- Easy to implement
- Good for arcade-style games

**Use cases**:
- Arcade games
- Top-down games
- Games where immediate response is needed

## Rotation Controls

### Approach 1: Torque-Based Rotation

```python
# ✅ GOOD: Torque-based rotation
if keys[pg.K_q]:  # Rotate left
    self.rigidbody.add_torque(-rotation_force)
if keys[pg.K_e]:  # Rotate right
    self.rigidbody.add_torque(rotation_force)
```

### Approach 2: Direct Angular Velocity

```python
# ✅ GOOD: Direct angular velocity control
if keys[pg.K_q]:
    self.rigidbody.angular_velocity = -rotation_speed
if keys[pg.K_e]:
    self.rigidbody.angular_velocity = rotation_speed
```

## Best Practices

### 1. Always Use World Coordinates
```python
# ✅ GOOD: World coordinates
world_force = Vector2(-move_force, 0)
self.rigidbody.add_force_at_point(world_force, self.game_object.position)

# ❌ BAD: Local coordinates
self.rigidbody.add_force(Vector2(-move_force, 0))
```

### 2. Limit Maximum Speeds
```python
# ✅ GOOD: Speed limiting
current_vel = self.rigidbody.velocity
if current_vel.length() > max_speed:
    self.rigidbody.set_velocity(current_vel.normalize() * max_speed)
```

### 3. Use Appropriate Movement Type
- **Force-based**: For realistic physics and momentum
- **Velocity-based**: For responsive, arcade-style controls

### 4. Handle Ground Detection
```python
# ✅ GOOD: Ground detection for jumping
if self.is_grounded and keys[pg.K_SPACE]:
    jump_impulse = Vector2(0, -jump_force)
    self.rigidbody.add_impulse_at_point(jump_impulse, self.game_object.position)
```

## Complete Example

```python
import pygame as pg
from pygame import Vector2
from pyg_engine import RigidBody, PhysicsMaterial

class PlayerController:
    def __init__(self, game_object):
        self.game_object = game_object
        self.rigidbody = game_object.get_component(RigidBody)
        
        # Movement settings
        self.move_force = 500
        self.max_speed = 200
        self.jump_force = 400
        
    def update(self, engine):
        keys = pg.key.get_pressed()
        current_vel = self.rigidbody.velocity
        
        # Horizontal movement (force-based)
        if keys[pg.K_LEFT] and current_vel.x > -self.max_speed:
            world_force = Vector2(-self.move_force, 0)
            self.rigidbody.add_force_at_point(world_force, self.game_object.position)
            
        if keys[pg.K_RIGHT] and current_vel.x < self.max_speed:
            world_force = Vector2(self.move_force, 0)
            self.rigidbody.add_force_at_point(world_force, self.game_object.position)
        
        # Jumping (impulse-based)
        if keys[pg.K_SPACE] and self.is_grounded():
            jump_impulse = Vector2(0, -self.jump_force)
            self.rigidbody.add_impulse_at_point(jump_impulse, self.game_object.position)
    
    def is_grounded(self):
        # Implement ground detection logic
        return True  # Simplified for example
```

## Troubleshooting

### Common Issues

1. **Object moves in wrong direction**: Check if you're using world coordinates
2. **Movement feels sluggish**: Increase force values or use velocity-based movement
3. **Object spins unexpectedly**: Check for unintended torque application
4. **Collision response is wrong**: Verify physics material settings

### Debug Tips

1. **Print velocity values**: `print(f"Velocity: {self.rigidbody.velocity}")`
2. **Check forces**: `print(f"Forces: {self.rigidbody._forces}")`
3. **Monitor position**: `print(f"Position: {self.game_object.position}")`

## Summary

- Use `add_force_at_point()` for world-coordinate forces
- Choose force-based or velocity-based movement based on your game type
- Always limit maximum speeds to prevent physics instability
- Test movement thoroughly with different physics materials
- Use ground detection for jumping mechanics 