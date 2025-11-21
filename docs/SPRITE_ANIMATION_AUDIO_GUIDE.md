# Sprite, Animation, and Audio System Guide

This guide covers the sprite rendering, animation, and audio systems in Pyg Engine.

## Table of Contents

1. [Sprite System](#sprite-system)
2. [Animation System](#animation-system)
3. [Audio System](#audio-system)
4. [UI Sprite Support](#ui-sprite-support)
5. [Examples](#examples)

---

## Sprite System

The Sprite component provides optimized image rendering with automatic `convert()` optimization for performance.

### Basic Sprite Usage

```python
from pyg_engine import GameObject, Sprite, Vector2

# Create a game object with a sprite
player = GameObject(name="Player", position=Vector2(100, 100))
sprite = player.add_component(
    Sprite,
    image_path="assets/player.png",
    scale=Vector2(2, 2)  # Scale 2x
)
engine.add_game_object(player)
```

### Sprite Features

#### Scaling
```python
# Uniform scaling
sprite = player.add_component(Sprite, image_path="player.png", scale=2.0)

# Non-uniform scaling
sprite = player.add_component(Sprite, image_path="player.png", scale=Vector2(2, 1.5))

# Change scale at runtime
sprite.set_scale(Vector2(3, 3))
```

#### Flipping
```python
# Horizontal flip
sprite = player.add_component(Sprite, image_path="player.png", flip_x=True)

# Vertical flip
sprite = player.add_component(Sprite, image_path="player.png", flip_y=True)

# Change flip at runtime
sprite.set_flip(flip_x=True, flip_y=False)
```

#### Color Tinting
```python
from pyg_engine import Color

# Add red tint
sprite = player.add_component(Sprite, image_path="player.png", tint=Color(255, 100, 100))

# Change tint at runtime
sprite.set_tint(Color(100, 100, 255))
```

#### Alpha Transparency
```python
# Semi-transparent sprite
sprite = player.add_component(Sprite, image_path="player.png", alpha=128)

# Change alpha at runtime
sprite.set_alpha(200)  # 0-255
```

#### Sprite Offset
```python
# Offset sprite from GameObject position
sprite = player.add_component(
    Sprite, 
    image_path="player.png",
    offset=Vector2(10, 5)
)
```

### Performance Optimization

The Sprite component automatically uses `convert()` or `convert_alpha()` based on whether the image has transparency:

```python
# This is done automatically:
# - Images with alpha channel → convert_alpha()
# - Images without alpha → convert()
```

This provides significant performance improvements for rendering.

---

## Animation System

The Animator component provides frame-based sprite animation with timing control.

### Basic Animation

```python
from pyg_engine import GameObject, Sprite, Animator
from pyg_engine.rendering import load_animation_frames

# Create game object with sprite and animator
player = GameObject(name="Player", position=Vector2(100, 100))
sprite = player.add_component(Sprite)
animator = player.add_component(Animator)

# Load animation frames
idle_frames = load_animation_frames([
    "assets/player_idle_1.png",
    "assets/player_idle_2.png",
    "assets/player_idle_3.png"
])

walk_frames = load_animation_frames([
    "assets/player_walk_1.png",
    "assets/player_walk_2.png",
    "assets/player_walk_3.png",
    "assets/player_walk_4.png"
])

# Add animations
animator.add_animation("idle", idle_frames, frame_duration=0.2, loop=True)
animator.add_animation("walk", walk_frames, frame_duration=0.1, loop=True)

# Play animation
animator.play("idle")

engine.add_game_object(player)
```

### Animation Control

```python
# Play an animation
animator.play("walk")

# Play animation from beginning even if already playing
animator.play("walk", restart=True)

# Pause animation
animator.pause()

# Resume animation
animator.resume()

# Stop animation
animator.stop()

# Change animation speed
animator.set_speed(2.0)  # 2x speed
animator.set_speed(0.5)  # Half speed
```

### Animation State Queries

```python
# Check if animation is playing
if animator.is_playing():
    print("Animation is playing")

# Check specific animation
if animator.is_playing("walk"):
    print("Walk animation is playing")

# Get current animation name
current = animator.get_current_animation_name()

# Check if animation has finished (for non-looping animations)
if animator.is_finished():
    print("Animation finished")

# Check if animation exists
if animator.has_animation("jump"):
    animator.play("jump")
```

### One-Shot Animations

```python
# Animation that plays once and triggers a callback
def on_attack_complete():
    print("Attack animation finished!")
    animator.play("idle")

animator.add_animation(
    "attack", 
    attack_frames, 
    frame_duration=0.1, 
    loop=False,  # Don't loop
    on_complete=on_attack_complete
)

animator.play("attack")
```

### Using Sprite Sheets

```python
from pyg_engine.rendering import SpriteSheet

# Load sprite sheet
sheet = SpriteSheet("assets/player_sheet.png", sprite_width=32, sprite_height=32)

# Extract frames by index (left-to-right, top-to-bottom)
idle_frames = sheet.get_frames_range(0, 3)  # Frames 0-3

# Extract a row
walk_frames = sheet.get_row(1)  # Second row

# Extract specific frames
jump_frames = [
    sheet.get_frame_at_grid(0, 2),  # Column 0, Row 2
    sheet.get_frame_at_grid(1, 2),
    sheet.get_frame_at_grid(2, 2)
]

# Add to animator
animator.add_animation("idle", idle_frames, frame_duration=0.2, loop=True)
animator.add_animation("walk", walk_frames, frame_duration=0.1, loop=True)
animator.add_animation("jump", jump_frames, frame_duration=0.15, loop=False)
```

---

## Audio System

The audio system provides sound effects and music playback through the global AudioManager.

### Basic Sound Effects

```python
from pyg_engine import audio_manager

# Load sound effects
audio_manager.load_sound("jump", "assets/sounds/jump.wav")
audio_manager.load_sound("coin", "assets/sounds/coin.wav")
audio_manager.load_sound("hurt", "assets/sounds/hurt.wav")

# Play sounds
audio_manager.play_sound("jump", volume=1.0)
audio_manager.play_sound("coin", volume=0.7)

# Loop a sound
audio_manager.play_sound("ambient", volume=0.5, loops=-1)  # Loop forever

# Stop a sound
audio_manager.stop_sound("ambient")
```

### Music Playback

```python
# Load and play music
audio_manager.play_music("assets/music/theme.mp3", volume=0.8, loops=-1)

# Fade in music
audio_manager.play_music("assets/music/theme.mp3", volume=0.8, loops=-1, fade_ms=2000)

# Stop music
audio_manager.stop_music()

# Fade out music
audio_manager.stop_music(fade_ms=1000)

# Pause/unpause music
audio_manager.pause_music()
audio_manager.unpause_music()

# Check if music is playing
if audio_manager.is_music_playing():
    print("Music is playing")
```

### Volume Control

```python
# Set master volume (affects all audio)
audio_manager.set_master_volume(0.8)  # 0.0 to 1.0

# Set music volume
audio_manager.set_music_volume(0.6)

# Set sound effects volume
audio_manager.set_sfx_volume(0.9)

# Get current volumes
master = audio_manager.get_master_volume()
music = audio_manager.get_music_volume()
sfx = audio_manager.get_sfx_volume()
```

### Sound Component

For sounds attached to GameObjects:

```python
from pyg_engine import Sound

# Add Sound component to GameObject
player = GameObject(name="Player", position=Vector2(100, 100))
jump_sound = player.add_component(
    Sound,
    sound_name="player_jump",
    file_path="assets/sounds/jump.wav",
    volume=0.8,
    play_on_start=False  # Don't play automatically
)

# Play the sound
jump_sound.play()

# Play with different volume
jump_sound.play(volume=0.5)

# Stop the sound
jump_sound.stop()

# Check if playing
if jump_sound.is_playing:
    print("Sound is playing")
```

### One-Shot Sounds

For quick sound effects without creating a component:

```python
from pyg_engine.audio import SoundOneShot

# Play a sound once
SoundOneShot.play("assets/sounds/click.wav", volume=0.7)
```

---

## UI Sprite Support

UI elements (UIButton, UIPanel, UILabel) now support sprite backgrounds.

### UIButton with Sprites

```python
from pyg_engine.ui import UIButton, Anchor
from pyg_engine import Vector2

# Button with state-based sprites
button = UIButton(
    text="Start Game",
    size=Vector2(200, 60),
    anchor=Anchor.CENTER,
    normal_sprite="assets/ui/button_normal.png",
    hover_sprite="assets/ui/button_hover.png",
    pressed_sprite="assets/ui/button_pressed.png",
    onClick=lambda: print("Button clicked!")
)
engine.canvas.add_element(button)

# Button with single sprite for all states
button = UIButton(
    text="Options",
    size=Vector2(150, 50),
    sprite="assets/ui/button.png",  # Used for all states
    onClick=on_options_click
)
```

### UIPanel with Sprites

```python
from pyg_engine.ui import UIPanel

# Panel with stretched sprite background
panel = UIPanel(
    size=Vector2(400, 300),
    sprite="assets/ui/panel_bg.png",
    sprite_scale_mode="stretch"  # Default
)

# Panel with tiled sprite
panel = UIPanel(
    size=Vector2(400, 300),
    sprite="assets/ui/tile.png",
    sprite_scale_mode="tile"  # Repeat sprite
)

# Panel with centered sprite
panel = UIPanel(
    size=Vector2(400, 300),
    sprite="assets/ui/decoration.png",
    sprite_scale_mode="center"  # Center without scaling
)
```

### UILabel with Background Sprite

```python
from pyg_engine.ui import UILabel

# Label with background sprite
label = UILabel(
    text="Score: 100",
    font_size=24,
    background_sprite="assets/ui/label_bg.png"
)
```

---

## Examples

### Complete Animated Character

```python
from pyg_engine import Engine, GameObject, Sprite, Animator, Sound, audio_manager
from pyg_engine import Vector2, Color
from pyg_engine.rendering import load_animation_frames

# Create engine
engine = Engine(width=800, height=600, title="Animated Character")

# Create character
player = GameObject(name="Player", position=Vector2(400, 300))

# Add sprite and animator
sprite = player.add_component(Sprite)
animator = player.add_component(Animator)

# Load animations
idle_frames = load_animation_frames([
    "assets/player/idle_1.png",
    "assets/player/idle_2.png",
    "assets/player/idle_3.png"
])

walk_frames = load_animation_frames([
    "assets/player/walk_1.png",
    "assets/player/walk_2.png",
    "assets/player/walk_3.png",
    "assets/player/walk_4.png"
])

# Add animations
animator.add_animation("idle", idle_frames, frame_duration=0.2, loop=True)
animator.add_animation("walk", walk_frames, frame_duration=0.1, loop=True)
animator.play("idle")

# Add footstep sound
footstep_sound = player.add_component(
    Sound,
    sound_name="footstep",
    file_path="assets/sounds/footstep.wav",
    volume=0.6
)

# Add to engine
engine.add_game_object(player)

# Load jump sound
audio_manager.load_sound("jump", "assets/sounds/jump.wav")

# Update function
def update(engine):
    speed = 200 * (engine.clock.get_time() / 1000.0)
    
    # Movement and animation
    if engine.input.get_key('left'):
        player.position.x -= speed
        sprite.set_flip(flip_x=True)
        if not animator.is_playing("walk"):
            animator.play("walk")
    elif engine.input.get_key('right'):
        player.position.x += speed
        sprite.set_flip(flip_x=False)
        if not animator.is_playing("walk"):
            animator.play("walk")
    else:
        if animator.is_playing("walk"):
            animator.play("idle")
    
    # Jump
    if engine.input.get_key_down('space'):
        audio_manager.play_sound("jump", volume=0.8)

engine.add_runnable(update, priority=0)
engine.run()
```

### Using the Flappy Bird Assets

```python
from pathlib import Path
from pyg_engine import Engine, GameObject, Sprite, Animator, audio_manager
from pyg_engine import Vector2, Color
from pyg_engine.rendering import load_animation_frames

# Assets path
ASSETS = Path("examples/flappy_bird/Flappy_Bird_assets by kosresetr55")

# Create engine
engine = Engine(width=800, height=600, title="Flappy Bird Demo")

# Create animated bird
bird = GameObject(name="Bird", position=Vector2(200, 300))
sprite = bird.add_component(Sprite)
animator = bird.add_component(Animator)

# Load bird animation
bird_frames = load_animation_frames([
    str(ASSETS / "Game Objects" / "yellowbird-downflap.png"),
    str(ASSETS / "Game Objects" / "yellowbird-midflap.png"),
    str(ASSETS / "Game Objects" / "yellowbird-upflap.png")
])

animator.add_animation("fly", bird_frames, frame_duration=0.1, loop=True)
animator.play("fly")
engine.add_game_object(bird)

# Load sounds
audio_manager.load_sound("wing", str(ASSETS / "Sound Efects" / "wing.wav"))
audio_manager.load_sound("point", str(ASSETS / "Sound Efects" / "point.wav"))

# Handle flap
def update(engine):
    if engine.input.get_key_down('space'):
        audio_manager.play_sound("wing", volume=0.7)
        bird.position.y += 50  # Flap up

engine.add_runnable(update, priority=0)
engine.run()
```

---

## Best Practices

### Performance Tips

1. **Use convert()**: The Sprite component automatically uses `convert()` or `convert_alpha()` for optimal performance.

2. **Preload assets**: Load all sprites and sounds during initialization, not during gameplay.

3. **Reuse sprite sheets**: Use `SpriteSheet` to load multiple animation frames from a single image file.

4. **Limit simultaneous sounds**: Too many sounds playing at once can impact performance.

### Organization Tips

1. **Animation naming**: Use consistent naming conventions for animations (e.g., "idle", "walk", "jump", "attack").

2. **Sound categories**: Organize sounds by category (sfx, music, ui_sounds).

3. **Asset structure**: Keep assets organized in folders:
   ```
   assets/
     sprites/
       player/
       enemies/
       environment/
     sounds/
       sfx/
       music/
     ui/
   ```

### Common Patterns

#### Character Controller with Animations

```python
class PlayerController:
    def __init__(self, player_object):
        self.player = player_object
        self.animator = player_object.get_component(Animator)
        self.sprite = player_object.get_component(Sprite)
        
    def update(self, engine):
        # Handle movement and animation
        if self.is_moving:
            self.animator.play("walk")
        elif self.is_jumping:
            self.animator.play("jump")
        else:
            self.animator.play("idle")
```

#### Sound Manager Pattern

```python
class GameSounds:
    def __init__(self):
        self.load_all_sounds()
    
    def load_all_sounds(self):
        audio_manager.load_sound("jump", "assets/sounds/jump.wav")
        audio_manager.load_sound("coin", "assets/sounds/coin.wav")
        audio_manager.load_sound("hurt", "assets/sounds/hurt.wav")
    
    def play_jump(self):
        audio_manager.play_sound("jump", volume=0.8)
    
    def play_coin(self):
        audio_manager.play_sound("coin", volume=0.7)
```

---

## API Reference

### Sprite Component

- `Sprite(image_path, scale, offset, flip_x, flip_y, tint, alpha, layer)`
- `set_image(image_path)` - Load new image
- `set_image_from_surface(surface)` - Set from pygame Surface
- `set_scale(scale)` - Change scale
- `set_flip(flip_x, flip_y)` - Change flip
- `set_tint(color)` - Apply color tint
- `set_alpha(alpha)` - Set transparency
- `get_image()` - Get current surface
- `get_size()` - Get sprite size

### Animator Component

- `Animator(default_animation)`
- `add_animation(name, frames, frame_duration, loop, on_complete)` - Add animation
- `play(animation_name, restart)` - Play animation
- `stop()` - Stop animation
- `pause()` - Pause animation
- `resume()` - Resume animation
- `set_speed(speed)` - Change playback speed
- `is_playing(animation_name)` - Check if playing
- `is_finished()` - Check if finished
- `get_current_animation_name()` - Get current animation

### AudioManager

- `load_sound(name, file_path)` - Load sound
- `play_sound(name, volume, loops)` - Play sound
- `stop_sound(name)` - Stop sound
- `play_music(file_path, volume, loops, fade_ms)` - Play music
- `stop_music(fade_ms)` - Stop music
- `pause_music()` - Pause music
- `unpause_music()` - Resume music
- `set_master_volume(volume)` - Set master volume
- `set_music_volume(volume)` - Set music volume
- `set_sfx_volume(volume)` - Set SFX volume

### Sound Component

- `Sound(sound_name, file_path, volume, loops, play_on_start)`
- `play(volume, loops)` - Play sound
- `stop()` - Stop sound
- `set_volume(volume)` - Change volume
- `check_playing()` - Check if playing

---

## Troubleshooting

### Sprites not appearing
- Check file paths are correct
- Ensure GameObject is added to engine
- Verify sprite layer ordering

### Animations not playing
- Ensure Animator has Sprite component on same GameObject
- Check animation was added before playing
- Verify frame_duration is reasonable (0.05 - 0.3 typical)

### Sounds not playing
- Check file format (WAV, OGG supported)
- Verify sound was loaded before playing
- Check volume levels (master, sfx, individual)

### Performance issues
- Verify convert() is being used (automatic in Sprite component)
- Limit number of simultaneous GameObjects
- Reduce animation frame count if needed
- Use sprite sheets instead of individual files

