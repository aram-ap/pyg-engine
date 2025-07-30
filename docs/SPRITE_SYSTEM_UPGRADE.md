# GameObject Sprite System Upgrade

## Overview

The GameObject system has been upgraded to inherit from Pygame's `sprite.Sprite` class, providing better performance, cleaner code organization, and access to Pygame's built-in sprite management features.

## What Changed

### 1. GameObject Class Inheritance
- **Before**: `class BasicObject:` (custom class)
- **After**: `class GameObject(pg.sprite.Sprite):` (inherits from Pygame Sprite)

### 2. Engine Management
- **Before**: Used a simple list `self.__gameobjects = []`
- **After**: Uses Pygame sprite groups `self.__gameobjects = pg.sprite.Group()`

### 3. New Features Added

#### Sprite Properties
- `self.image`: Pygame surface for rendering
- `self.rect`: Pygame rect for collision detection and positioning
- Automatic sprite group management

#### New Methods
- `update_position()`: Updates both position and rect
- `update_size()`: Recreates sprite surface with new size
- `update_rotation()`: Applies rotation to sprite image
- `update_color()`: Updates sprite color and recreates surface
- `kill()`: Pygame sprite kill method with cleanup

#### Backward Compatibility
- `BasicObject = GameObject` alias maintains existing code compatibility
- All existing scripts and components continue to work unchanged

## Benefits

### 1. Performance Improvements
- **Sprite Groups**: Pygame's optimized sprite group rendering
- **Collision Detection**: Built-in rect-based collision detection
- **Memory Management**: Automatic sprite cleanup and management

### 2. Code Organization
- **Cleaner Architecture**: Follows Pygame's established patterns
- **Better Separation**: Rendering logic separated from game logic
- **Standard Practices**: Uses industry-standard sprite patterns

### 3. New Capabilities
- **Sprite Groups**: Easy filtering and management of sprites
- **Built-in Collision**: `pg.sprite.spritecollide()` and related functions
- **Automatic Cleanup**: Sprites automatically removed from groups when killed
- **Batch Operations**: Efficient operations on sprite groups

### 4. Maintained Functionality
- ✅ Component system (unchanged)
- ✅ Script system (unchanged)
- ✅ Physics system (unchanged)
- ✅ Camera system (unchanged)
- ✅ Mouse input system (unchanged)
- ✅ All existing examples work without modification

## Technical Details

### Sprite Surface Creation
```python
def _create_sprite_surface(self):
    """Create pygame surface and rect for the sprite."""
    if self.basicShape == BasicShape.Circle:
        radius = max(1, int(max(self.size.x, self.size.y) / 2))
        diameter = radius * 2
        self.image = pg.Surface((diameter, diameter), pg.SRCALPHA)
        pg.draw.circle(self.image, self.color, (radius, radius), radius)
    else:  # Rectangle
        width = max(1, int(self.size.x)) if self.size.x > 0 else 40
        height = max(1, int(self.size.y)) if self.size.y > 0 else 40
        self.image = pg.Surface((width, height), pg.SRCALPHA)
        self.image.fill(self.color)
    
    self.rect = self.image.get_rect()
    self.rect.center = (int(self.position.x), int(self.position.y))
```

### Rotation Support
```python
def _apply_rotation(self):
    """Apply rotation to the sprite image."""
    if hasattr(self, 'image') and self.image:
        if not hasattr(self, '_original_image'):
            self._original_image = self.image.copy()
        else:
            self.image = self._original_image.copy()
        
        rotated_image = pg.transform.rotate(self.image, self.rotation)
        self.image = rotated_image
        self.rect = self.image.get_rect()
        self.rect.center = (int(self.position.x), int(self.position.y))
```

## Usage Examples

### Creating GameObjects (Same as Before)
```python
# Create a basic GameObject
player = GameObject(
    name="Player",
    position=Vector2(400, 300),
    size=Vector2(50, 50),
    color=Color(255, 0, 0)
)

# Add to engine (same as before)
engine.addGameObject(player)
```

### Using New Sprite Features
```python
# Update position (updates both position and rect)
player.update_position(Vector2(500, 400))

# Update size (recreates sprite surface)
player.update_size(Vector2(100, 100))

# Update rotation (applies rotation to sprite)
player.update_rotation(45.0)

# Update color (recreates sprite surface)
player.update_color(Color(0, 255, 0))

# Kill sprite (removes from groups and cleans up)
player.kill()
```

### Using Pygame Sprite Groups
```python
# Get all sprites in a group
all_sprites = engine.getGameObjects()

# Filter sprites by type
player_sprites = [sprite for sprite in all_sprites if sprite.tag == Tag.Player]

# Use Pygame's sprite collision functions
collisions = pg.sprite.spritecollide(player, enemies, False)
```

## Migration Guide

### For Existing Code

**No changes required!** The upgrade is fully backward compatible:

```python
# This still works exactly the same
from pyg_engine import BasicObject  # Still available as alias

player = BasicObject("Player", Vector2(400, 300), Vector2(50, 50), Color(255, 0, 0))
engine.addGameObject(player)
```

### For New Code

Use the new GameObject class directly:

```python
from pyg_engine import GameObject

player = GameObject("Player", Vector2(400, 300), Vector2(50, 50), Color(255, 0, 0))
engine.addGameObject(player)
```

## Performance Benefits

### Before (Custom Implementation)
- Manual sprite management
- Custom collision detection
- Manual memory cleanup
- No batch rendering optimizations

### After (Pygame Sprite System)
- Automatic sprite group management
- Built-in collision detection
- Automatic memory cleanup
- Optimized batch rendering
- Hardware acceleration support

## Testing

All existing tests continue to pass:

```bash
# Run the test suite
python -m pytest tests/

# Test sprite-specific functionality
python tests/test_sprite_system.py
```

## Future Enhancements

The sprite system upgrade provides a foundation for future improvements:

1. **Texture Support**: Easy integration with texture atlases
2. **Animation System**: Built on top of sprite surface management
3. **Particle Systems**: Leverage sprite groups for particle management
4. **UI System**: Use sprites for UI elements
5. **Optimization**: Further performance improvements using Pygame's sprite optimizations

## Summary

The sprite system upgrade provides:

- **Better Performance**: Pygame's optimized sprite rendering
- **Cleaner Code**: Standard Pygame patterns
- **More Features**: Built-in collision, groups, batch operations
- **Full Compatibility**: No breaking changes to existing code
- **Future-Proof**: Foundation for advanced features

The upgrade maintains all existing functionality while providing significant performance and organizational benefits. 