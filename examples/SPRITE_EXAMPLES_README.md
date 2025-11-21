# Sprite, Animation, and Audio Examples

This directory contains examples demonstrating the sprite rendering, animation, and audio systems in Pyg Engine.

## Examples

### 1. Simple Sprite Test (`simple_sprite_test.py`)

A basic test that verifies the sprite, animation, and sound systems are working correctly.

**Features demonstrated:**
- Static sprite rendering
- Animated sprite with frame-based animation
- Sound loading and playback
- Basic keyboard input

**Usage:**
```bash
python examples/simple_sprite_test.py
```

**Controls:**
- `SPACE` - Play sound effect
- `ESC` - Quit

---

### 2. Sprite, Animation & Sound Demo (`sprite_animation_sound_demo.py`)

A comprehensive demonstration of all sprite, animation, and audio features using Flappy Bird assets.

**Features demonstrated:**
- Multiple animated sprites with different settings
- Sprite scaling, flipping, tinting, and rotation
- Frame-based animations at different speeds
- Sound effects triggered by keyboard
- UI elements with sprite backgrounds
- FPS display

**Usage:**
```bash
python examples/sprite_animation_sound_demo.py
```

**Controls:**
- `1-4` - Play wing sound on bird 1-4
- `P` - Play point sound
- `H` - Play hit sound
- `S` - Play swoosh sound
- `ESC` - Quit

**What you'll see:**
- Background sprite (Flappy Bird day background)
- Ground sprite (scrolling base)
- 4 animated birds demonstrating different features:
  - Bird 1: Normal speed animation
  - Bird 2: Slow animation with 1.5x scale
  - Bird 3: Fast animation with horizontal flip
  - Bird 4: Animation with red tint and 15° rotation
- UI panel with controls
- Interactive button with hover effects
- Real-time FPS counter

---

## Asset Requirements

These examples use the Flappy Bird assets located in:
```
examples/flappy_bird/Flappy_Bird_assets by kosresetr55/
```

The assets should include:
- `Game Objects/` - Sprite images (bird, background, pipes, etc.)
- `Sound Efects/` - Audio files (wing, hit, point, swoosh, etc.)
- `UI/` - UI elements (game over, message, numbers)

If the assets are not found, the examples will display an error message with the expected location.

---

## Creating Your Own Sprites and Animations

### Basic Sprite Setup

```python
from pyg_engine import Engine, GameObject, Sprite, Vector2

engine = Engine(width=800, height=600, title="My Game")

# Create object with sprite
player = GameObject(name="Player", position=Vector2(400, 300))
sprite = player.add_component(
    Sprite,
    image_path="path/to/sprite.png",
    scale=Vector2(2, 2)  # 2x scale
)
engine.add_game_object(player)

engine.run()
```

### Adding Animation

```python
from pyg_engine import Animator
from pyg_engine.rendering import load_animation_frames

# Add animator to game object
animator = player.add_component(Animator)

# Load animation frames
frames = load_animation_frames([
    "frame1.png",
    "frame2.png",
    "frame3.png"
])

# Add animation
animator.add_animation(
    "idle",                  # Animation name
    frames,                  # Frame list
    frame_duration=0.1,      # 0.1 seconds per frame
    loop=True                # Loop animation
)

# Play animation
animator.play("idle")
```

### Adding Sound Effects

```python
from pyg_engine import audio_manager

# Load sound
audio_manager.load_sound("jump", "path/to/jump.wav")

# Play sound in your update function
def update(engine):
    if engine.input.get_key_down('space'):
        audio_manager.play_sound("jump", volume=0.8)

engine.add_runnable(update)
```

---

## Performance Notes

### Sprite Optimization

The Sprite component automatically optimizes images using pygame's `convert()` or `convert_alpha()`:
- Images with transparency → `convert_alpha()`
- Images without transparency → `convert()`

This provides significant performance improvements, especially with many sprites.

### Animation Tips

1. **Frame Duration**: Typical values are 0.05 - 0.3 seconds
   - Fast: 0.05 - 0.1 (action animations)
   - Medium: 0.1 - 0.2 (walking, idle)
   - Slow: 0.2 - 0.3 (breathing, ambient)

2. **Frame Count**: Keep animation frames reasonable
   - Idle: 2-4 frames
   - Walk: 4-8 frames
   - Attack: 3-6 frames

3. **Sprite Sheets**: Use `SpriteSheet` to load multiple frames from one file:
   ```python
   from pyg_engine.rendering import SpriteSheet
   
   sheet = SpriteSheet("spritesheet.png", sprite_width=32, sprite_height=32)
   frames = sheet.get_frames_range(0, 7)  # Get frames 0-7
   ```

### Sound Tips

1. **Format**: Use WAV or OGG formats for best compatibility
2. **Size**: Keep sound files small (< 1MB for SFX)
3. **Volume**: Test volume levels (0.6 - 0.8 is usually good)
4. **Preload**: Load all sounds at startup, not during gameplay

---

## Common Patterns

### Character with Multiple Animations

```python
# Setup
player = GameObject(name="Player", position=Vector2(400, 300))
sprite = player.add_component(Sprite)
animator = player.add_component(Animator)

# Load animations
idle = load_animation_frames(["idle1.png", "idle2.png", "idle3.png"])
walk = load_animation_frames(["walk1.png", "walk2.png", "walk3.png", "walk4.png"])
jump = load_animation_frames(["jump1.png", "jump2.png", "jump3.png"])

animator.add_animation("idle", idle, frame_duration=0.2, loop=True)
animator.add_animation("walk", walk, frame_duration=0.1, loop=True)
animator.add_animation("jump", jump, frame_duration=0.1, loop=False)

# In update function
def update(engine):
    if is_jumping:
        animator.play("jump")
    elif is_moving:
        animator.play("walk")
    else:
        animator.play("idle")
```

### Animated UI with Sounds

```python
from pyg_engine.ui import UIButton, Anchor

# Button with hover sound
def on_button_hover():
    audio_manager.play_sound("hover", volume=0.5)

def on_button_click():
    audio_manager.play_sound("click", volume=0.7)
    # Do button action

button = UIButton(
    text="Start Game",
    size=Vector2(200, 60),
    anchor=Anchor.CENTER,
    sprite="button_bg.png",
    onHover=on_button_hover,
    onClick=on_button_click
)
engine.canvas.add_element(button)
```

---

## Troubleshooting

### "Flappy Bird assets not found"
Make sure the assets are in the correct location:
```
examples/flappy_bird/Flappy_Bird_assets by kosresetr55/
```

### Sprites appear as magenta squares
This means the image file couldn't be loaded. Check:
- File path is correct
- File exists at that location
- File is a valid image format (PNG, JPG, etc.)

### Animation not playing
Check:
- Animator component has a Sprite component on the same GameObject
- Animation was added before playing
- Animation name is correct (case-sensitive)

### No sound playing
Check:
- Sound file exists at the specified path
- File format is supported (WAV, OGG)
- Volume is not 0
- Master/SFX volume is not muted

### Low FPS
If performance is poor:
- Check how many GameObjects are active
- Verify sprites are being optimized (automatic in Sprite component)
- Consider using sprite sheets instead of individual files
- Reduce number of simultaneous sound effects

---

## Next Steps

1. **Read the full guide**: See `docs/SPRITE_ANIMATION_AUDIO_GUIDE.md` for complete API reference
2. **Experiment**: Try modifying the examples to learn how things work
3. **Create your own**: Use these examples as templates for your own games
4. **Check other examples**: Look at `flappy_bird/` for a complete game example

---

## Questions or Issues?

If you encounter problems or have questions:
1. Check the main documentation in `docs/SPRITE_ANIMATION_AUDIO_GUIDE.md`
2. Look at the example code for patterns and usage
3. Verify your asset files are in the correct locations
4. Check console output for error messages

