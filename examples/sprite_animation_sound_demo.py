"""
Sprite, Animation, and Sound System Demo

This example demonstrates:
- Sprite rendering with convert() optimization
- Frame-based animations with the Animator component
- Sound effects using the AudioManager
- UI elements with sprite backgrounds
- Multiple animated game objects

Uses the Flappy Bird assets to showcase the new systems.
"""

import sys
import os
from pathlib import Path

# Add parent directory to path to import pyg_engine
sys.path.insert(0, str(Path(__file__).parent.parent))

from pyg_engine import Engine, GameObject, Sprite, Animator, Sound, audio_manager, Priority
from pyg_engine import Vector2, Color
from pyg_engine.rendering import load_animation_frames
from pyg_engine.ui import UILabel, UIButton, UIPanel, UICanvas, Anchor
from pyg_engine.input.input import Input

# Path to Flappy Bird assets
ASSETS_DIR = Path(__file__).parent / "flappy_bird" / "Flappy_Bird_assets by kosresetr55"
GAME_OBJECTS_DIR = ASSETS_DIR / "Game Objects"
SOUND_DIR = ASSETS_DIR / "Sound Efects"
UI_DIR = ASSETS_DIR / "UI"


class AnimatedBirdDemo:
    """Demo showing animated sprites, sounds, and UI with sprites."""
    
    def __init__(self):
        # Create engine
        from pyg_engine.utilities.object_types import Size
        self.engine = Engine(
            size=Size(w=800, h=600),
            windowName="Sprite, Animation & Sound Demo",
            backgroundColor=Color(135, 206, 235)  # Sky blue
        )
        
        # Create UI canvas
        self.canvas = UICanvas(self.engine)
        
        # Setup scene
        self._setup_background()
        self._setup_animated_birds()
        self._setup_sounds()
        self._setup_ui()
        
    def _setup_background(self):
        """Setup background and ground sprites."""
        # Background
        background = GameObject(
            name="Background",
            position=Vector2(400, 300)
        )
        bg_sprite = background.add_component(
            Sprite,
            image_path=str(GAME_OBJECTS_DIR / "background-day.png"),
            scale=Vector2(2.5, 2.5)
        )
        self.engine.addGameObject(background)
        
        # Ground/Base (scrolling effect could be added)
        ground = GameObject(
            name="Ground",
            position=Vector2(400, 50)
        )
        ground_sprite = ground.add_component(
            Sprite,
            image_path=str(GAME_OBJECTS_DIR / "base.png"),
            scale=Vector2(2.5, 2.5)
        )
        self.engine.addGameObject(ground)
    
    def _setup_animated_birds(self):
        """Create multiple animated birds demonstrating different features."""
        # Bird animation frames
        bird_frames = load_animation_frames([
            str(GAME_OBJECTS_DIR / "yellowbird-downflap.png"),
            str(GAME_OBJECTS_DIR / "yellowbird-midflap.png"),
            str(GAME_OBJECTS_DIR / "yellowbird-upflap.png"),
        ])
        
        # Bird 1: Normal speed animation
        bird1 = GameObject(name="Bird1", position=Vector2(150, 400))
        sprite1 = bird1.add_component(Sprite)
        animator1 = bird1.add_component(Animator)
        animator1.add_animation("fly", bird_frames, frame_duration=0.1, loop=True)
        animator1.play("fly")
        self.engine.addGameObject(bird1)
        
        # Bird 2: Slow animation with scale
        bird2 = GameObject(name="Bird2", position=Vector2(300, 350))
        sprite2 = bird2.add_component(Sprite, scale=Vector2(1.5, 1.5))
        animator2 = bird2.add_component(Animator)
        animator2.add_animation("fly", bird_frames, frame_duration=0.2, loop=True)
        animator2.play("fly")
        self.engine.addGameObject(bird2)
        
        # Bird 3: Fast animation with flip
        bird3 = GameObject(name="Bird3", position=Vector2(450, 400))
        sprite3 = bird3.add_component(Sprite, flip_x=True)
        animator3 = bird3.add_component(Animator)
        animator3.add_animation("fly", bird_frames, frame_duration=0.05, loop=True)
        animator3.play("fly")
        self.engine.addGameObject(bird3)
        
        # Bird 4: With tint and rotation
        bird4 = GameObject(name="Bird4", position=Vector2(600, 350), rotation=15)
        sprite4 = bird4.add_component(
            Sprite, 
            tint=Color(255, 150, 150),
            scale=Vector2(1.2, 1.2)
        )
        animator4 = bird4.add_component(Animator)
        animator4.add_animation("fly", bird_frames, frame_duration=0.15, loop=True)
        animator4.play("fly")
        self.engine.addGameObject(bird4)
        
        # Store birds for sound triggers
        self.birds = [bird1, bird2, bird3, bird4]
    
    def _setup_sounds(self):
        """Load sound effects."""
        # Load sounds into AudioManager
        audio_manager.load_sound("wing", str(SOUND_DIR / "wing.wav"))
        audio_manager.load_sound("point", str(SOUND_DIR / "point.wav"))
        audio_manager.load_sound("hit", str(SOUND_DIR / "hit.wav"))
        audio_manager.load_sound("swoosh", str(SOUND_DIR / "swoosh.wav"))
        
        # Add Sound component to one bird
        sound_component = self.birds[0].add_component(
            Sound,
            sound_name="wing_sound",
            file_path=str(SOUND_DIR / "wing.wav"),
            volume=0.5
        )
        
        print("Sounds loaded successfully!")
    
    def _setup_ui(self):
        """Create UI with sprite backgrounds."""
        # Title panel with sprite background (optional - only if you want)
        title_label = UILabel(
            text="Sprite & Animation Demo",
            font_size=36,
            color=Color(255, 255, 255),
            anchor=Anchor.TOP_CENTER,
            offset=Vector2(0, -50),
            bold=True
        )
        self.canvas.add_element(title_label)
        
        # Info panel
        info_panel = UIPanel(
            size=Vector2(350, 180),
            background_color=Color(40, 40, 40, 200),
            border_color=Color(255, 255, 255),
            border_width=2,
            anchor=Anchor.TOP_LEFT,
            offset=Vector2(20, -20)
        )
        self.canvas.add_element(info_panel)
        
        # Info text
        info_lines = [
            "Controls:",
            "1-4: Play wing sound on bird 1-4",
            "P: Play point sound",
            "H: Play hit sound",
            "S: Play swoosh sound",
            "ESC: Quit"
        ]
        
        y_offset = -20
        for line in info_lines:
            label = UILabel(
                text=line,
                font_size=18 if line == "Controls:" else 16,
                color=Color(255, 255, 255),
                bold=line == "Controls:",
                anchor=Anchor.TOP_LEFT,
                offset=Vector2(30, y_offset),
                align="left"
            )
            self.canvas.add_element(label)
            y_offset -= 25
        
        # Button with sprite example (using game over sprite as button bg)
        play_button = UIButton(
            text="Play Wing Sound",
            size=Vector2(200, 60),
            font_size=20,
            anchor=Anchor.BOTTOM_CENTER,
            offset=Vector2(0, 30),
            onClick=lambda: audio_manager.play_sound("wing", volume=0.7),
            normal_color=Color(70, 130, 180),
            hover_color=Color(100, 160, 210),
            pressed_color=Color(50, 110, 160)
        )
        self.canvas.add_element(play_button)
        
        # Stats label
        self.stats_label = UILabel(
            text="FPS: 0",
            font_size=20,
            color=Color(255, 255, 0),
            anchor=Anchor.TOP_RIGHT,
            offset=Vector2(-20, -20)
        )
        self.canvas.add_element(self.stats_label)
    
    def update(self, engine):
        """Update game state."""
        # Update UI canvas
        self.canvas.update()
        
        # Update FPS display
        fps = engine.clock.get_fps()
        self.stats_label.set_text(f"FPS: {int(fps)}")
        
        # Handle keyboard input for sound testing
        if engine.input.get_event_state('key_down', Input.Keybind.K_1.value):
            audio_manager.play_sound("wing", volume=0.7)
            print("🎵 Playing wing sound (bird 1)")
        
        if engine.input.get_event_state('key_down', Input.Keybind.K_2.value):
            audio_manager.play_sound("wing", volume=0.7)
            print("🎵 Playing wing sound (bird 2)")
        
        if engine.input.get_event_state('key_down', Input.Keybind.K_3.value):
            audio_manager.play_sound("wing", volume=0.7)
            print("🎵 Playing wing sound (bird 3)")
        
        if engine.input.get_event_state('key_down', Input.Keybind.K_4.value):
            audio_manager.play_sound("wing", volume=0.7)
            print("🎵 Playing wing sound (bird 4)")
        
        if engine.input.get_event_state('key_down', Input.Keybind.P.value):
            audio_manager.play_sound("point", volume=0.7)
            print("🎵 Playing point sound")
        
        if engine.input.get_event_state('key_down', Input.Keybind.H.value):
            audio_manager.play_sound("hit", volume=0.7)
            print("🎵 Playing hit sound")
        
        if engine.input.get_event_state('key_down', Input.Keybind.S.value):
            audio_manager.play_sound("swoosh", volume=0.7)
            print("🎵 Playing swoosh sound")
        
        if engine.input.get_event_state('key_down', Input.Keybind.K_ESCAPE.value):
            engine.stop()
    
    def render(self, engine):
        """Render UI elements."""
        self.canvas.render(engine.screen)
    
    def run(self):
        """Start the demo."""
        print("=" * 60)
        print("SPRITE, ANIMATION & SOUND DEMO")
        print("=" * 60)
        print("\nSprite system initialized with convert() optimization")
        print("Animator system with frame-based animations")
        print("AudioManager with sound effects")
        print("UI elements with sprite support")
        print("\nPress 1-4 or P/H/S to test sounds!")
        print("Press ESC to quit\n")
        
        # Add update and render callbacks
        self.engine.add_runnable(self.update, priority=Priority.NORMAL)
        self.engine.add_runnable(self.render, priority=Priority.LOW)  # Render UI after everything else
        
        # Start the engine
        self.engine.start()


if __name__ == "__main__":
    # Check if assets exist
    if not GAME_OBJECTS_DIR.exists():
        print("❌ Error: Flappy Bird assets not found!")
        print(f"Expected location: {ASSETS_DIR}")
        print("\nPlease ensure the Flappy Bird assets are in the correct location.")
        sys.exit(1)
    
    # Run the demo
    demo = AnimatedBirdDemo()
    demo.run()

