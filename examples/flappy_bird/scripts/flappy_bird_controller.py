"""
Flappy Bird Controller Script
Manages the game state, bird, pipes, and scoring
"""

import pygame as pg
from pygame import Color
import random
import os
import sys
from pyg_engine import Script, Input, Vector2, GameObject, Tag, BasicShape, Sprite, Animator, audio_manager
from pyg_engine.rendering import load_animation_frames
from pyg_engine.ui import UICanvas, UIButton, UILabel, UIPanel, UITextInput, Anchor

# Import leaderboard module dynamically to work with script loading
import importlib.util
_script_dir = os.path.dirname(__file__)
_leaderboard_path = os.path.join(_script_dir, "leaderboard.py")
_spec = importlib.util.spec_from_file_location("leaderboard", _leaderboard_path)
_leaderboard_module = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(_leaderboard_module)
Leaderboard = _leaderboard_module.Leaderboard

class FlappyBirdController(Script):
    """Main controller script for the Flappy Bird game."""

    def __init__(self, game_object):
        super().__init__(game_object)
        
        # Game state
        self.game_state = "ready"  # "ready", "playing", "game_over", "enter_name"
        self.score = 0
        self.username = ""
        self.is_new_high_score = False
        
        # Leaderboard system
        self.leaderboard = Leaderboard()
        
        # Game objects
        self.bird = None
        self.pipes = []
        self.ground = None
        self.ceiling = None
        self.background = None
        self.ground_sprite_objs = []  # Multiple ground sprites for scrolling
        
        # Ground scrolling
        self.ground_scroll_speed = 200  # Same as pipe speed
        self.ground_sprite_width = 0
        self.ground_sprite_start_x = 0
        self.ground_sprite_height = 0
        
        # Pipe spawning
        self.pipe_spawn_timer = 0
        self.pipe_spawn_interval = 2.0  # Spawn pipe every 2 seconds
        self.pipe_speed = 200  # Pixels per second
        self.pipe_gap = 180  # Gap between upper and lower pipes
        
        # Engine reference
        self.engine = None
        
        # Event storage for UI processing
        self.pending_events = []
        
        # UI System
        self.ui_canvas = None
        self.ui_elements = {
            'title': None,
            'username_input': None,
            'username_label': None,
            'username_instruction': None,
            'start_instruction': None,
            'score_label': None,
            'game_over_panel': None,
            'game_over_title': None,
            'final_score': None,
            'your_rank': None,
            'high_score_label': None,
            'leaderboard_title': None,
            'leaderboard_entries': [],
            'restart_button': None,
            'quit_button': None
        }

    def start(self, engine):
        """Called when the script starts."""
        super().start(engine)
        self.engine = engine
        
        # Enable text input mode for pygame
        pg.key.start_text_input()
        
        # Load audio assets
        self._load_audio()
        
        # Create UI canvas
        self.ui_canvas = UICanvas(engine)
        self._create_ui()
        
        # Register UI rendering with the engine's render system
        engine.add_runnable(self._render_ui, event_type='render', key='flappy_bird_ui')
        
        # Create background and ground sprites
        self._create_background()
        
        # Create the bird
        self._create_bird()
        
        # Create ground and ceiling (invisible collision boundaries)
        self._create_boundaries()
    
    def _get_screen_size(self):
        """Get current screen dimensions from engine."""
        if self.engine:
            size = self.engine.getWindowSize()
            return size.w, size.h
        return 800, 600  # Default fallback
    
    def _load_audio(self):
        """Load all audio assets."""
        assets_dir = os.path.join(os.path.dirname(__file__), "..", "Flappy_Bird_assets by kosresetr55")
        sound_dir = os.path.join(assets_dir, "Sound Efects")
        
        # Load sound effects
        try:
            audio_manager.load_sound("wing", os.path.join(sound_dir, "wing.wav"))
            audio_manager.load_sound("point", os.path.join(sound_dir, "point.wav"))
            audio_manager.load_sound("hit", os.path.join(sound_dir, "hit.wav"))
            audio_manager.load_sound("swoosh", os.path.join(sound_dir, "swoosh.wav"))
            print("✓ Audio assets loaded successfully")
        except Exception as e:
            print(f"Warning: Could not load audio assets: {e}")
    
    def _create_background(self):
        """Create background and ground sprites with scrolling support."""
        assets_dir = os.path.join(os.path.dirname(__file__), "..", "Flappy_Bird_assets by kosresetr55")
        game_objects_dir = os.path.join(assets_dir, "Game Objects")
        
        screen_width, screen_height = self._get_screen_size()
        
        # Background - centered in screen
        try:
            self.background = GameObject(
                name="Background",
                position=Vector2(screen_width / 2, screen_height / 2),
                tag=Tag.Environment
            )
            bg_sprite = self.background.add_component(
                Sprite,
                image_path=os.path.join(game_objects_dir, "background-day.png"),
                scale=Vector2(2.8, 2.8),  # Scale to fill screen
                layer=-10  # Behind everything
            )
            self.engine.addGameObject(self.background)
            print("✓ Background sprite created")
        except Exception as e:
            print(f"Warning: Could not load background sprite: {e}")
        
        # Ground sprites (visual) - create 3 for seamless scrolling
        try:
            # Load ground image to get its width
            import pygame as pg
            ground_img = pg.image.load(os.path.join(game_objects_dir, "base.png"))
            ground_scale = 2.0
            ground_width = ground_img.get_width() * ground_scale
            ground_height = ground_img.get_height() * ground_scale
            self.ground_sprite_width = ground_width
            self.ground_sprite_height = ground_height
            
            # Ground height - position it at the bottom to align with collision
            ground_y = ground_height / 2 - 5  # Slightly above collision ground
            self.ground_sprite_start_x = screen_width / 2
            
            # Create 3 ground sprites to ensure continuous scrolling
            for i in range(3):
                ground_x = self.ground_sprite_start_x + i * ground_width
                ground_sprite_obj = GameObject(
                    name=f"GroundSprite{i}",
                    position=Vector2(ground_x, ground_y),
                    tag=Tag.Environment
                )
                ground_sprite = ground_sprite_obj.add_component(
                    Sprite,
                    image_path=os.path.join(game_objects_dir, "base.png"),
                    scale=Vector2(ground_scale, ground_scale),
                    layer=10  # In front of background
                )
                self.engine.addGameObject(ground_sprite_obj)
                self.ground_sprite_objs.append(ground_sprite_obj)
            
            print(f"✓ Ground sprites created (width: {ground_width})")
        except Exception as e:
            print(f"Warning: Could not load ground sprite: {e}")
    
    def _create_ui(self):
        """Create all UI elements."""
        # Title label for ready state
        self.ui_elements['title'] = UILabel(
            text="FLAPPY BIRD",
            font_size=64,
            color=Color(255, 255, 255),
            anchor=Anchor.TOP_CENTER,
            offset=Vector2(0, -80),
            bold=True,
            layer=10
        )
        self.ui_canvas.add_element(self.ui_elements['title'])
        
        # Username input label (for game over state when high score achieved)
        self.ui_elements['username_label'] = UILabel(
            text="NEW HIGH SCORE! Enter your name:",
            font_size=24,
            color=Color(255, 255, 100),
            anchor=Anchor.CENTER,
            offset=Vector2(0, 70),
            layer=10,
            visible=False
        )
        self.ui_canvas.add_element(self.ui_elements['username_label'])
        
        # Instruction label for name entry
        self.ui_elements['username_instruction'] = UILabel(
            text="(Press Enter to submit, Space/Esc to skip)",
            font_size=16,
            color=Color(200, 200, 200),
            anchor=Anchor.CENTER,
            offset=Vector2(0, -50),
            layer=10,
            visible=False
        )
        self.ui_canvas.add_element(self.ui_elements['username_instruction'])
        
        # Username text input (for game over state when high score achieved)
        def on_username_submit(text):
            if text.strip():
                self.username = text.strip()
                # Add to leaderboard (will only replace if score is higher)
                rank = self.leaderboard.add_entry(self.username, self.score)
                
                # Check if this was actually a new personal best
                existing_score = None
                for entry in self.leaderboard.get_top_entries():
                    if entry.username == self.username:
                        existing_score = entry.score
                        break
                
                if existing_score and existing_score > self.score:
                    print(f"Score: {self.score} - Kept your better score of {existing_score}, Rank: #{rank}")
                else:
                    print(f"Score added to leaderboard for: {self.username}, Rank: #{rank}")
                
                # Transition to normal game over screen
                self.game_state = "game_over"
        
        self.ui_elements['username_input'] = UITextInput(
            placeholder="Your name",
            text="",
            max_length=15,
            size=Vector2(300, 50),
            font_size=28,
            anchor=Anchor.CENTER,
            offset=Vector2(0, 0),
            onSubmit=on_username_submit,
            onChange=None,
            background_color=Color(40, 40, 60),
            active_color=Color(60, 60, 80),
            text_color=Color(255, 255, 255),
            placeholder_color=Color(150, 150, 150),
            border_color=Color(100, 150, 200),
            active_border_color=Color(150, 200, 255),
            border_width=3,
            layer=10,
            visible=False
        )
        self.ui_canvas.add_element(self.ui_elements['username_input'])
        
        # Start instruction for ready state
        self.ui_elements['start_instruction'] = UILabel(
            text="Press SPACE or CLICK to start",
            font_size=28,
            color=Color(200, 255, 200),
            anchor=Anchor.CENTER,
            offset=Vector2(0, 0),
            layer=10
        )
        self.ui_canvas.add_element(self.ui_elements['start_instruction'])
        
        # Score label for playing state
        self.ui_elements['score_label'] = UILabel(
            text="0",
            font_size=72,
            color=Color(255, 255, 255),
            anchor=Anchor.TOP_CENTER,
            offset=Vector2(0, -60),
            bold=True,
            layer=10,
            visible=False
        )
        self.ui_canvas.add_element(self.ui_elements['score_label'])
        
        # Game over panel 
        self.ui_elements['game_over_panel'] = UIPanel(
            size=Vector2(500, 500),
            background_color=Color(0, 0, 0, 200),
            border_color=Color(255, 255, 255),
            border_width=3,
            anchor=Anchor.CENTER,
            offset=Vector2(0, 0),
            layer=9,
            visible=False
        )
        self.ui_canvas.add_element(self.ui_elements['game_over_panel'])
        
        # Game over title
        self.ui_elements['game_over_title'] = UILabel(
            text="GAME OVER",
            font_size=48,
            color=Color(255, 50, 50),
            anchor=Anchor.CENTER,
            offset=Vector2(0, 210),
            bold=True,
            layer=10,
            visible=False
        )
        self.ui_canvas.add_element(self.ui_elements['game_over_title'])
        
        # Final score label
        self.ui_elements['final_score'] = UILabel(
            text="Score: 0",
            font_size=28,
            color=Color(255, 255, 255),
            anchor=Anchor.CENTER,
            offset=Vector2(0, 160),
            layer=10,
            visible=False
        )
        self.ui_canvas.add_element(self.ui_elements['final_score'])
        
        # Your rank label
        self.ui_elements['your_rank'] = UILabel(
            text="",
            font_size=24,
            color=Color(255, 255, 100),
            anchor=Anchor.CENTER,
            offset=Vector2(0, 130),
            layer=10,
            visible=False
        )
        self.ui_canvas.add_element(self.ui_elements['your_rank'])
        
        # Leaderboard title
        self.ui_elements['leaderboard_title'] = UILabel(
            text="=== LEADERBOARD ===",
            font_size=24,
            color=Color(100, 200, 255),
            anchor=Anchor.CENTER,
            offset=Vector2(0, 90),
            bold=True,
            layer=10,
            visible=False
        )
        self.ui_canvas.add_element(self.ui_elements['leaderboard_title'])
        
        # Create leaderboard entry labels (we'll update these dynamically)
        for i in range(5):
            entry_label = UILabel(
                text="",
                font_size=20,
                color=Color(255, 255, 255),
                anchor=Anchor.CENTER,
                offset=Vector2(0, 60 - (i * 30)),
                layer=10,
                visible=False
            )
            self.ui_canvas.add_element(entry_label)
            self.ui_elements['leaderboard_entries'].append(entry_label)
        
        # High score label (moved higher)
        self.ui_elements['high_score_label'] = UILabel(
            text="All-Time High: 0",
            font_size=20,
            color=Color(255, 200, 100),
            anchor=Anchor.CENTER,
            offset=Vector2(0, -110),
            layer=10,
            visible=False
        )
        self.ui_canvas.add_element(self.ui_elements['high_score_label'])
        
        # Restart button (moved lower)
        def on_restart_click():
            print("Restart button clicked!")
            self.restart_game()
        
        self.ui_elements['restart_button'] = UIButton(
            text="Restart (R)",
            size=Vector2(180, 50),
            font_size=24,
            anchor=Anchor.CENTER,
            offset=Vector2(0, -160),
            onClick=on_restart_click,
            normal_color=Color(50, 150, 50),
            hover_color=Color(70, 200, 70),
            pressed_color=Color(30, 100, 30),
            layer=10,
            visible=False
        )
        self.ui_canvas.add_element(self.ui_elements['restart_button'])
        
        # Quit button (moved lower)
        self.ui_elements['quit_button'] = UIButton(
            text="Quit (ESC)",
            size=Vector2(180, 50),
            font_size=24,
            anchor=Anchor.CENTER,
            offset=Vector2(0, -220),
            onClick=lambda: self.engine.stop(),
            normal_color=Color(150, 50, 50),
            hover_color=Color(200, 70, 70),
            pressed_color=Color(100, 30, 30),
            layer=10,
            visible=False
        )
        self.ui_canvas.add_element(self.ui_elements['quit_button'])

    def _create_bird(self):
        """Create the bird game object with sprite animation."""
        screen_width, screen_height = self._get_screen_size()
        self.bird = GameObject(
            name="Bird",
            position=Vector2(screen_width * 0.25, screen_height * 0.5),
            size=Vector2(34, 24),
            color=Color(255, 255, 0),  # Yellow bird (fallback)
            tag=Tag.Player,
            basicShape=BasicShape.Circle
        )
        
        # Add sprite and animator components
        try:
            assets_dir = os.path.join(os.path.dirname(__file__), "..", "Flappy_Bird_assets by kosresetr55")
            game_objects_dir = os.path.join(assets_dir, "Game Objects")
            
            # Load bird animation frames
            bird_frames = load_animation_frames([
                os.path.join(game_objects_dir, "yellowbird-downflap.png"),
                os.path.join(game_objects_dir, "yellowbird-midflap.png"),
                os.path.join(game_objects_dir, "yellowbird-upflap.png"),
            ])
            
            # Add sprite component with layer
            sprite = self.bird.add_component(Sprite, layer=8)  # Above pipes and ground
            
            # Add animator component
            animator = self.bird.add_component(Animator)
            animator.add_animation("fly", bird_frames, frame_duration=0.1, loop=True)
            animator.play("fly")
            
            print("✓ Bird sprite and animation created")
        except Exception as e:
            print(f"Warning: Could not load bird sprites, using basic shape: {e}")
        
        # Add bird script
        scripts_dir = os.path.dirname(__file__)
        bird_script_path = os.path.join(scripts_dir, "bird_script.py")
        self.bird.add_script(bird_script_path, "BirdScript", controller=self)
        
        self.engine.addGameObject(self.bird)

    def _create_boundaries(self):
        """Create ground and ceiling boundaries."""
        screen_width, screen_height = self._get_screen_size()
        
        # Ground (at bottom - low/negative Y in engine coords)
        self.ground = GameObject(
            name="Ground",
            position=Vector2(screen_width / 2, 10),
            size=Vector2(screen_width, 20),
            color=Color(139, 69, 19),  # Brown ground
            tag=Tag.Environment,
            basicShape=BasicShape.Rectangle
        )
        self.engine.addGameObject(self.ground)
        
        # Ceiling (at top - high Y in engine coords)
        self.ceiling = GameObject(
            name="Ceiling",
            position=Vector2(screen_width / 2, screen_height - 10),
            size=Vector2(screen_width, 20),
            color=Color(135, 206, 235, 100),  # Semi-transparent
            tag=Tag.Environment,
            basicShape=BasicShape.Rectangle
        )
        self.engine.addGameObject(self.ceiling)
    
    def _update_boundaries(self):
        """Update boundary positions when screen is resized."""
        if not (self.ground and self.ceiling):
            return
        
        screen_width, screen_height = self._get_screen_size()
        
        # Update ground
        self.ground.position = Vector2(screen_width / 2, 10)
        self.ground.size = Vector2(screen_width, 20)
        
        # Update ceiling
        self.ceiling.position = Vector2(screen_width / 2, screen_height - 10)
        self.ceiling.size = Vector2(screen_width, 20)
    
    def _update_ground_scroll(self, dt):
        """Update scrolling ground sprites."""
        if not self.ground_sprite_objs or self.ground_sprite_width == 0:
            return
        
        # Only scroll when playing
        if self.game_state == "playing":
            # Move each ground sprite left
            for ground_obj in self.ground_sprite_objs:
                ground_obj.position.x -= self.ground_scroll_speed * dt
                
                # Reset position when sprite goes off-screen to the left
                if ground_obj.position.x < -self.ground_sprite_width:
                    # Find the rightmost ground sprite
                    max_x = max(g.position.x for g in self.ground_sprite_objs)
                    # Position this sprite to the right of it
                    ground_obj.position.x = max_x + self.ground_sprite_width

    def _spawn_pipe(self):
        """Spawn a pair of pipes (upper and lower)."""
        screen_width, screen_height = self._get_screen_size()
        
        # Random gap position (in inverted Y coords: higher value = higher on screen)
        # Keep gap in the middle 60% of the screen
        min_gap_y = int(screen_height * 0.25)
        max_gap_y = int(screen_height * 0.75)
        gap_center_y = random.randint(min_gap_y, max_gap_y)
        
        # Lower pipe (bottom of screen - low Y in engine coords)
        lower_pipe_height = gap_center_y - self.pipe_gap / 2
        lower_pipe = GameObject(
            name="PipeLower",
            position=Vector2(screen_width + 50, lower_pipe_height / 2),
            size=Vector2(52, lower_pipe_height),
            color=Color(0, 200, 0),  # Green pipe (fallback)
            tag=Tag.Environment,
            basicShape=BasicShape.Rectangle
        )
        
        # Add sprite to lower pipe
        try:
            assets_dir = os.path.join(os.path.dirname(__file__), "..", "Flappy_Bird_assets by kosresetr55")
            game_objects_dir = os.path.join(assets_dir, "Game Objects")
            lower_sprite = lower_pipe.add_component(
                Sprite,
                image_path=os.path.join(game_objects_dir, "pipe-green.png"),
                scale=Vector2(1.0, lower_pipe_height / 320),  # 320 is original pipe height
                flip_y=True,  # Flip for lower pipe
                layer=5  # Above background, below ground
            )
        except Exception as e:
            print(f"Warning: Could not load lower pipe sprite: {e}")
            pass  # Use basic shape if sprite load fails
        
        # Upper pipe (top of screen - high Y in engine coords)
        upper_pipe_y = gap_center_y + self.pipe_gap / 2
        upper_pipe_height = screen_height - upper_pipe_y - 10
        upper_pipe = GameObject(
            name="PipeUpper",
            position=Vector2(screen_width + 50, upper_pipe_y + upper_pipe_height / 2),
            size=Vector2(52, upper_pipe_height),
            color=Color(0, 200, 0),  # Green pipe (fallback)
            tag=Tag.Environment,
            basicShape=BasicShape.Rectangle
        )
        
        # Add sprite to upper pipe
        try:
            assets_dir = os.path.join(os.path.dirname(__file__), "..", "Flappy_Bird_assets by kosresetr55")
            game_objects_dir = os.path.join(assets_dir, "Game Objects")
            upper_sprite = upper_pipe.add_component(
                Sprite,
                image_path=os.path.join(game_objects_dir, "pipe-green.png"),
                scale=Vector2(1.0, upper_pipe_height / 320),  # 320 is original pipe height
                layer=5  # Above background, below ground
            )
        except Exception as e:
            print(f"Warning: Could not load upper pipe sprite: {e}")
            pass  # Use basic shape if sprite load fails
        
        # Add pipe script to move them
        scripts_dir = os.path.dirname(__file__)
        pipe_script_path = os.path.join(scripts_dir, "pipe_script.py")
        
        lower_pipe.add_script(pipe_script_path, "PipeScript", 
                            controller=self, speed=self.pipe_speed, is_score_pipe=True)
        upper_pipe.add_script(pipe_script_path, "PipeScript", 
                            controller=self, speed=self.pipe_speed, is_score_pipe=False)
        
        # Add to engine and track
        self.engine.addGameObject(lower_pipe)
        self.engine.addGameObject(upper_pipe)
        self.pipes.append(lower_pipe)
        self.pipes.append(upper_pipe)

    def _check_collisions(self):
        """Check if bird collided with pipes or boundaries."""
        if not self.bird:
            return False
        
        screen_width, screen_height = self._get_screen_size()
        bird_pos = self.bird.position
        bird_radius = 15  # Half of bird size
        
        # Check ground collision (bottom - low Y value)
        if bird_pos.y <= 25:
            return True
        
        # Check ceiling collision (top - high Y value)
        if bird_pos.y >= screen_height - 25:
            return True
        
        # Check pipe collisions
        for pipe in self.pipes:
            pipe_pos = pipe.position
            pipe_size = pipe.size
            
            # Simple AABB collision detection
            # Bird circle vs pipe rectangle
            closest_x = max(pipe_pos.x - pipe_size.x/2, 
                          min(bird_pos.x, pipe_pos.x + pipe_size.x/2))
            closest_y = max(pipe_pos.y - pipe_size.y/2, 
                          min(bird_pos.y, pipe_pos.y + pipe_size.y/2))
            
            distance_x = bird_pos.x - closest_x
            distance_y = bird_pos.y - closest_y
            distance_squared = distance_x * distance_x + distance_y * distance_y
            
            if distance_squared < (bird_radius * bird_radius):
                return True
        
        return False

    def add_score(self):
        """Increment score when bird passes through pipes."""
        self.score += 1
        # Update score label
        if self.ui_elements['score_label']:
            self.ui_elements['score_label'].set_text(str(self.score))
        # Play point sound
        audio_manager.play_sound("point", volume=0.7)

    def game_over(self):
        """Handle game over state."""
        if self.game_state not in ["game_over", "enter_name"]:
            # Play hit/die sound
            audio_manager.play_sound("hit", volume=0.7)
            
            # Check if this is a high score
            self.is_new_high_score = self.leaderboard.is_high_score(self.score)
            
            if self.is_new_high_score:
                # Go to name entry state
                self.game_state = "enter_name"
                print("=" * 50)
                print(f"GAME OVER! Final Score: {self.score}")
                print("NEW HIGH SCORE! Enter your name.")
                print("=" * 50)
            else:
                # No high score, go straight to game over
                self.game_state = "game_over"
                print("=" * 50)
                print(f"GAME OVER! Final Score: {self.score}")
                print(f"All-Time High: {self.leaderboard.get_high_score()}")
                print("\nLeaderboard:")
                print(self.leaderboard)
                print("Press R to restart or ESC to quit")
                print("=" * 50)

    def restart_game(self):
        """Restart the game."""
        # Play swoosh sound
        audio_manager.play_sound("swoosh", volume=0.6)
        print("Restarting game...")
        # Remove all pipes
        for pipe in self.pipes:
            self.engine.removeGameObject(pipe)
        self.pipes.clear()
        
        # Reset bird
        if self.bird:
            self.engine.removeGameObject(self.bird)
        self._create_bird()
        
        # Reset ground sprite positions for scrolling
        if self.ground_sprite_objs and self.ground_sprite_width > 0:
            for i, ground_obj in enumerate(self.ground_sprite_objs):
                ground_obj.position.x = self.ground_sprite_start_x + i * self.ground_sprite_width
        
        # Reset game state
        self.score = 0
        self.pipe_spawn_timer = 0
        self.game_state = "ready"

    def update(self, engine):
        """Update game logic."""
        # Capture TEXTINPUT and KEYDOWN events that weren't consumed by engine
        # TEXTINPUT events are not processed by the engine, so they'll still be in queue
        self.pending_events = []
        for event in pg.event.get([pg.TEXTINPUT, pg.KEYDOWN]):
            self.pending_events.append(event)
        
        dt = engine.dt()
        
        # Update boundaries if screen size changed
        self._update_boundaries()
        
        # Update ground scrolling
        self._update_ground_scroll(dt)
        
        # Handle input
        self._handle_input(engine)
        
        # Game state logic
        if self.game_state == "ready":
            # Wait for player input to start
            if engine.input.get(Input.Keybind.K_SPACE) or \
               engine.input.get(Input.Keybind.MOUSE_LEFT):
                self.game_state = "playing"
        
        elif self.game_state == "enter_name":
            # Waiting for username input after high score
            # Check if text input was submitted (handled by callback)
            # Or allow skipping with Space or Escape
            if engine.input.get_event_state('key_down', Input.Keybind.K_SPACE.value) or \
               engine.input.get_event_state('key_down', Input.Keybind.K_ESCAPE.value):
                # Skip name entry, use default
                if not self.username.strip():
                    self.username = "Anonymous"
                rank = self.leaderboard.add_entry(self.username, self.score)
                print(f"Score added to leaderboard for: {self.username}, Rank: #{rank}")
                self.game_state = "game_over"
        
        elif self.game_state == "playing":
            # Update pipe spawning
            self.pipe_spawn_timer += dt
            if self.pipe_spawn_timer >= self.pipe_spawn_interval:
                self._spawn_pipe()
                self.pipe_spawn_timer = 0
            
            # Check collisions
            if self._check_collisions():
                self.game_over()
            
            # Clean up off-screen pipes
            pipes_to_remove = []
            for pipe in self.pipes:
                if pipe.position.x < -100:
                    pipes_to_remove.append(pipe)
            
            for pipe in pipes_to_remove:
                self.engine.removeGameObject(pipe)
                self.pipes.remove(pipe)
        
        # Update UI visibility and state (rendering happens in render runnable)
        self._update_ui_visibility()
        if self.ui_canvas:
            self.ui_canvas.update()
            # Process UI input events by checking recent pygame events
            self._process_ui_events(engine)

    def _handle_input(self, engine):
        """Handle keyboard and mouse input."""
        # Restart on R key (only when game over)
        if self.game_state == "game_over":
            if engine.input.get_event_state('key_down', Input.Keybind.R.value):
                self.restart_game()
            # Quit on ESC
            if engine.input.get_event_state('key_down', Input.Keybind.K_ESCAPE.value):
                engine.stop()

    def _render_ui(self, engine):
        """Render the UI canvas - called by engine during render phase."""
        if self.ui_canvas and engine._Engine__useDisplay:
            self.ui_canvas.render(engine.screen)
    
    def _process_text_input_events(self, engine):
        """Process keyboard events for text input."""
        if not self.ui_canvas:
            return
        
        # Process pending KEYDOWN events captured earlier
        for event in self.pending_events:
            self.ui_canvas.handle_event(event)
        
        # Clear processed events
        self.pending_events.clear()
    
    def _process_ui_events(self, engine):
        """Process input events for UI elements."""
        if not self.ui_canvas:
            return
        
        # Process text input events (keyboard)
        self._process_text_input_events(engine)
        
        # Create synthetic events for mouse button clicks
        # Check for mouse button down
        if engine.input.get_event_state('mouse_button_down', 0):
            event = pg.event.Event(pg.MOUSEBUTTONDOWN, {'button': 1, 'pos': engine.input.mouse.get_pos()})
            self.ui_canvas.handle_event(event)
        
        # Check for mouse button up
        if engine.input.get_event_state('mouse_button_up', 0):
            event = pg.event.Event(pg.MOUSEBUTTONUP, {'button': 1, 'pos': engine.input.mouse.get_pos()})
            self.ui_canvas.handle_event(event)
    
    def _update_ui_visibility(self):
        """Update UI element visibility based on game state."""
        if not self.ui_elements:
            return
        
        # Ready state - show title and instructions only
        if self.game_state == "ready":
            self.ui_elements['title'].visible = True
            self.ui_elements['username_label'].visible = False
            self.ui_elements['username_input'].visible = False
            self.ui_elements['username_instruction'].visible = False
            self.ui_elements['start_instruction'].visible = True
            self.ui_elements['score_label'].visible = False
            self.ui_elements['game_over_panel'].visible = False
            self.ui_elements['game_over_title'].visible = False
            self.ui_elements['final_score'].visible = False
            self.ui_elements['your_rank'].visible = False
            self.ui_elements['leaderboard_title'].visible = False
            for entry_label in self.ui_elements['leaderboard_entries']:
                entry_label.visible = False
            self.ui_elements['high_score_label'].visible = False
            self.ui_elements['restart_button'].visible = False
            self.ui_elements['quit_button'].visible = False
        
        # Playing state - show score only
        elif self.game_state == "playing":
            self.ui_elements['title'].visible = False
            self.ui_elements['username_label'].visible = False
            self.ui_elements['username_input'].visible = False
            self.ui_elements['username_instruction'].visible = False
            self.ui_elements['start_instruction'].visible = False
            self.ui_elements['score_label'].visible = True
            self.ui_elements['game_over_panel'].visible = False
            self.ui_elements['game_over_title'].visible = False
            self.ui_elements['final_score'].visible = False
            self.ui_elements['your_rank'].visible = False
            self.ui_elements['leaderboard_title'].visible = False
            for entry_label in self.ui_elements['leaderboard_entries']:
                entry_label.visible = False
            self.ui_elements['high_score_label'].visible = False
            self.ui_elements['restart_button'].visible = False
            self.ui_elements['quit_button'].visible = False
            
            # Update score text
            self.ui_elements['score_label'].set_text(str(self.score))
        
        # Enter name state - show username input for high score
        elif self.game_state == "enter_name":
            self.ui_elements['title'].visible = False
            self.ui_elements['username_label'].visible = True
            self.ui_elements['username_input'].visible = True
            self.ui_elements['username_instruction'].visible = True
            self.ui_elements['start_instruction'].visible = False
            self.ui_elements['score_label'].visible = False
            self.ui_elements['game_over_panel'].visible = True
            self.ui_elements['game_over_title'].visible = True
            self.ui_elements['final_score'].visible = True
            self.ui_elements['your_rank'].visible = False
            self.ui_elements['leaderboard_title'].visible = False
            for entry_label in self.ui_elements['leaderboard_entries']:
                entry_label.visible = False
            self.ui_elements['high_score_label'].visible = False
            self.ui_elements['restart_button'].visible = False
            self.ui_elements['quit_button'].visible = False
            
            # Update score display
            self.ui_elements['final_score'].set_text(f"Your Score: {self.score}")
            
            # Auto-focus the text input
            if self.ui_elements['username_input'] and not self.ui_elements['username_input'].is_focused:
                self.ui_elements['username_input'].focus()
                self.ui_elements['username_input'].clear()
        
        # Game over state - show game over UI with leaderboard
        elif self.game_state == "game_over":
            self.ui_elements['title'].visible = False
            self.ui_elements['username_label'].visible = False
            self.ui_elements['username_input'].visible = False
            self.ui_elements['username_instruction'].visible = False
            self.ui_elements['start_instruction'].visible = False
            self.ui_elements['score_label'].visible = False
            self.ui_elements['game_over_panel'].visible = True
            self.ui_elements['game_over_title'].visible = True
            self.ui_elements['final_score'].visible = True
            self.ui_elements['your_rank'].visible = True
            self.ui_elements['leaderboard_title'].visible = True
            self.ui_elements['high_score_label'].visible = True
            self.ui_elements['restart_button'].visible = True
            self.ui_elements['quit_button'].visible = True
            
            # Update score displays
            self.ui_elements['final_score'].set_text(f"Your Score: {self.score}")
            
            # Find current player's rank
            rank_text = ""
            if self.username:
                for i, entry in enumerate(self.leaderboard.get_top_entries()):
                    if entry.username == self.username and entry.score == self.score:
                        rank_text = f"Rank: #{i+1}"
                        break
            self.ui_elements['your_rank'].set_text(rank_text)
            
            # Update leaderboard entries
            top_entries = self.leaderboard.get_top_entries(5)
            for i, entry_label in enumerate(self.ui_elements['leaderboard_entries']):
                if i < len(top_entries):
                    entry = top_entries[i]
                    # Highlight current player's entry (only if username was entered)
                    if self.username and entry.username == self.username and entry.score == self.score:
                        entry_label.set_text(f"► {i+1}. {entry.username}: {entry.score}")
                        entry_label.set_color(Color(255, 255, 100))
                    else:
                        entry_label.set_text(f"{i+1}. {entry.username}: {entry.score}")
                        entry_label.set_color(Color(255, 255, 255))
                    entry_label.visible = True
                else:
                    entry_label.visible = False
            
            # Update all-time high score
            self.ui_elements['high_score_label'].set_text(f"All-Time High: {self.leaderboard.get_high_score()}")
