"""
Simple Sprite System Test

Tests basic sprite, animation, and sound functionality.
"""

import sys
from pathlib import Path

# Add parent directory to path
sys.path.insert(0, str(Path(__file__).parent.parent))

from pyg_engine import Engine, GameObject, Sprite, Animator, Sound, audio_manager, Priority
from pyg_engine import Vector2, Color
from pyg_engine.rendering import load_animation_frames
from pyg_engine.input.input import Input

# Path to Flappy Bird assets
ASSETS_DIR = Path(__file__).parent / "flappy_bird" / "Flappy_Bird_assets by kosresetr55"
GAME_OBJECTS_DIR = ASSETS_DIR / "Game Objects"
SOUND_DIR = ASSETS_DIR / "Sound Efects"


def main():
    """Run a simple sprite test."""
    print("Testing Sprite System...")
    
    # Create engine
    from pyg_engine.utilities.object_types import Size
    engine = Engine(
        size=Size(w=600, h=400),
        windowName="Simple Sprite Test",
        backgroundColor=Color(135, 206, 235)
    )
    
    # Test 1: Static sprite
    print("Test 1: Creating static sprite...")
    static_bird = GameObject(name="StaticBird", position=Vector2(200, 200))
    sprite = static_bird.add_component(
        Sprite,
        image_path=str(GAME_OBJECTS_DIR / "yellowbird-midflap.png"),
        scale=Vector2(2, 2)
    )
    engine.addGameObject(static_bird)
    print("  Static sprite created successfully")
    
    # Test 2: Animated sprite
    print("Test 2: Creating animated sprite...")
    bird_frames = load_animation_frames([
        str(GAME_OBJECTS_DIR / "yellowbird-downflap.png"),
        str(GAME_OBJECTS_DIR / "yellowbird-midflap.png"),
        str(GAME_OBJECTS_DIR / "yellowbird-upflap.png"),
    ])
    
    animated_bird = GameObject(name="AnimatedBird", position=Vector2(400, 200))
    anim_sprite = animated_bird.add_component(Sprite)
    animator = animated_bird.add_component(Animator)
    animator.add_animation("fly", bird_frames, frame_duration=0.1, loop=True)
    animator.play("fly")
    engine.addGameObject(animated_bird)
    print("  Animated sprite created successfully")
    
    # Test 3: Sound loading
    print("Test 3: Loading sounds...")
    audio_manager.load_sound("test_sound", str(SOUND_DIR / "wing.wav"))
    print("  Sound loaded successfully")
    
    # Test 4: Play sound once
    print("Test 4: Playing test sound...")
    audio_manager.play_sound("test_sound", volume=0.5)
    print("  Sound played successfully")
    
    print("\n" + "=" * 50)
    print("ALL TESTS PASSED! ✓")
    print("=" * 50)
    print("\nStarting visual demo...")
    print("Press ESC to quit\n")
    
    # Simple update function
    def update(engine):
        if engine.input.get_event_state('key_down', Input.Keybind.K_ESCAPE.value):
            engine.stop()
        if engine.input.get_event_state('key_down', Input.Keybind.K_SPACE.value):
            audio_manager.play_sound("test_sound", volume=0.7)
            print("🎵 Sound played!")
    
    engine.add_runnable(update, priority=Priority.NORMAL)
    engine.start()


if __name__ == "__main__":
    if not GAME_OBJECTS_DIR.exists():
        print("❌ Error: Flappy Bird assets not found!")
        print(f"Expected: {ASSETS_DIR}")
        sys.exit(1)
    
    main()

