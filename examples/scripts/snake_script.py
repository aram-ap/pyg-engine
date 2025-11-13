"""
Snake script for the Snake game example
"""

import pygame as pg
from pygame import Color
import random
from pyg_engine import Script, Input, Vector2

class SnakeScript(Script):
    """Script for controlling the snake in the Snake game."""

    def __init__(self, game_object, grid_size=20, game_speed=10):
        super().__init__(game_object)
        self.grid_size = grid_size
        self.game_speed = game_speed
        self.game_state = "playing"  # "playing", "game_over", "paused"
        self.score = 0
        self.high_score = 0

        # Snake properties
        self.snake_body = []
        self.snake_direction = Vector2(1, 0)  # Start moving right
        self.next_direction = Vector2(1, 0)
        self.snake_speed = 0.15  # Seconds between moves
        self.time_since_last_move = 0

        # Food properties
        self.food = None
        self.food_spawn_timer = 0

        # Game area
        self.game_width = 40
        self.game_height = 30
        self.cell_size = 20  # Each cell is 20x20 pixels

        # Engine reference (will be set in start method)
        self.engine = None

        self.restarting = False
        self.paused = False

        # Track snake segments to avoid recreating them unnecessarily
        self.snake_segments = {}
        self.last_snake_body = []

        print("Snake Script initialized!")
        print("Controls: Arrow keys or WASD to move, ESC to pause, R to restart")

    def start(self, engine):
        """Called when the script starts."""
        super().start(engine)
        self.engine = engine
        self._initialize_snake()
        self._spawn_food()

    def _initialize_snake(self):
        """Initialize the snake with starting body segments."""
        # Clear existing snake
        self.snake_body = []
        self.snake_segments = {}
        self.last_snake_body = []

        # Create initial snake (3 segments) - start in the middle of the game area
        start_x = self.game_width // 2
        start_y = self.game_height // 2

        # Create snake segments from left to right (head is on the left)
        for i in range(3):
            segment_pos = Vector2(start_x - i, start_y)  # Head at start_x, body extending left
            self.snake_body.append(segment_pos)

        # Reset direction
        self.snake_direction = Vector2(1, 0)
        self.next_direction = Vector2(1, 0)

        # Create initial snake segments
        self._update_snake_visuals()
        print(f"Snake initialized at position: {self.snake_body[0]} (head)")
        print(f"Snake direction: {self.snake_direction}")
        print(f"Game area: {self.game_width}x{self.game_height} cells")

    def _spawn_food(self):
        """Spawn food at a random location."""
        if self.food:
            # Remove existing food game object
            self.engine.removeGameObject(self.food)

        # Find empty position
        while True:
            food_x = random.randint(0, self.game_width - 1)
            food_y = random.randint(0, self.game_height - 1)
            food_pos = Vector2(food_x, food_y)

            # Check if position is not occupied by snake
            if food_pos not in self.snake_body:
                break

        # Create food game object
        from pyg_engine import GameObject, Tag, BasicShape
        self.food = GameObject(
            name="Food",
            position=self._grid_to_world(food_pos),
            size=Vector2(self.cell_size, self.cell_size),  # Full cell size for better visibility
            color=Color(255, 0, 0),  # Red food
            tag=Tag.Environment,
            basicShape=BasicShape.Circle
        )

        # Add to engine
        self.engine.addGameObject(self.food)

    def _grid_to_world(self, grid_pos):
        """Convert grid position to world position."""
        world_x = grid_pos.x * self.cell_size + self.cell_size // 2
        world_y = grid_pos.y * self.cell_size + self.cell_size // 2
        return Vector2(world_x, world_y)

    def _world_to_grid(self, world_pos):
        """Convert world position to grid position."""
        grid_x = int(world_pos.x // self.cell_size)
        grid_y = int(world_pos.y // self.cell_size)
        return Vector2(grid_x, grid_y)

    def update(self, engine):
        """Update game logic."""
        # Handle input every frame (regardless of game state)
        self._handle_input(engine)


        # Only update snake movement if not paused and game is not over
        if self.game_state != "game_over" and not (self.restarting or self.paused):
            self.time_since_last_move += engine.dt()
            if self.time_since_last_move >= self.snake_speed:
                self._move_snake()
                self.time_since_last_move = 0

    def _handle_input(self, engine):
        """Handle keyboard input for snake movement using the new input system."""
        input = engine.input

        # Prevent 180-degree turns
        if input.get(Input.Keybind.K_UP) or input.get(Input.Keybind.W):
            if self.snake_direction.y == 0:  # Not moving up/down
                self.next_direction = Vector2(0, 1)
        elif input.get(Input.Keybind.K_DOWN) or input.get(Input.Keybind.S):
            if self.snake_direction.y == 0:  # Not moving up/down
                self.next_direction = Vector2(0, -1)
        elif input.get(Input.Keybind.K_LEFT) or input.get(Input.Keybind.A):
            if self.snake_direction.x == 0:  # Not moving left/right
                self.next_direction = Vector2(-1, 0)
        elif input.get(Input.Keybind.K_RIGHT) or input.get(Input.Keybind.D):
            if self.snake_direction.x == 0:  # Not moving left/right
                self.next_direction = Vector2(1, 0)

        # Restart
        if input.get(Input.Keybind.R) and (self.game_state == "game_over") and not self.restarting:
            self.restarting = True
            print("🔄 Restart key pressed!")
            self._restart_game()
            self.paused = False
            self.restarting = False
        elif input.get(Input.Keybind.K_ESCAPE) and (self.game_state == "game_over"):
            print("🚫 Exiting game...")
            self.engine.stop()
        elif input.get_event_state('key_up', Input.Keybind.K_ESCAPE.value):
            self.paused = not self.paused
            print("Game is {}".format("PAUSED" if self.paused else "PLAYING"))

    def _move_snake(self):
        """Move the snake one step."""
        # Update direction
        self.snake_direction = self.next_direction

        # Calculate new head position
        head = self.snake_body[0]
        new_head = head + self.snake_direction

        # Check wall collision
        if (new_head.x < 0 or new_head.x >= self.game_width or
            new_head.y < 0 or new_head.y >= self.game_height):
            self._game_over()
            return

        # Check self collision
        if new_head in self.snake_body:
            self._game_over()
            return

        # Check food collision
        ate_food = False
        if self.food:
            food_grid_pos = self._world_to_grid(self.food.position)
            if new_head == food_grid_pos:
                ate_food = True
                self.score += 10
                if self.score > self.high_score:
                    self.high_score = self.score
                print(f"🍎 Food eaten! Score: {self.score} | High Score: {self.high_score}")
                print(f"Snake length: {len(self.snake_body) + 1} segments")

        # Add new head
        self.snake_body.insert(0, new_head)

        # Remove tail if didn't eat food
        if not ate_food:
            self.snake_body.pop()
        else:
            # Spawn new food
            self._spawn_food()
            # Increase speed slightly
            self.snake_speed = max(0.05, self.snake_speed - 0.002)

        # Update snake body game objects only if the snake changed
        if self.snake_body != self.last_snake_body:
            self._update_snake_visuals()
            self.last_snake_body = self.snake_body.copy()

    def _update_snake_visuals(self):
        """Update the visual representation of the snake."""
        # Remove old snake segments that are no longer needed
        current_segment_keys = set()
        for i, segment_pos in enumerate(self.snake_body):
            segment_key = f"SnakeSegment_{i}"
            current_segment_keys.add(segment_key)

            # If this segment doesn't exist or is in the wrong position, create/update it
            if segment_key not in self.snake_segments:
                # Create new segment
                from pyg_engine import GameObject, Tag, BasicShape
                is_head = i == 0
                segment = GameObject(
                    name=segment_key,
                    position=self._grid_to_world(segment_pos),
                    size=Vector2(self.cell_size, self.cell_size),  # Full cell size for better visibility
                    color=Color(0, 255, 0) if is_head else Color(0, 200, 0),  # Green, darker for body
                    tag=Tag.Player,
                    basicShape=BasicShape.Rectangle
                )
                self.engine.addGameObject(segment)
                self.snake_segments[segment_key] = segment
                print(f"Added snake segment: {segment_key} at position {segment_pos}")
            else:
                # Update existing segment position and color
                segment = self.snake_segments[segment_key]
                segment.position = self._grid_to_world(segment_pos)
                is_head = i == 0
                segment.color = Color(0, 255, 0) if is_head else Color(0, 200, 0)

        # Remove segments that are no longer part of the snake
        segments_to_remove = []
        for segment_key, segment in self.snake_segments.items():
            if segment_key not in current_segment_keys:
                self.engine.removeGameObject(segment)
                segments_to_remove.append(segment_key)

        for segment_key in segments_to_remove:
            del self.snake_segments[segment_key]

    def _game_over(self):
        """Handle game over."""
        self.game_state = "game_over"
        print(f"💀 GAME OVER! Final Score: {self.score} | High Score: {self.high_score}")
        print("Press R to restart or ESC to quit.")

    def _restart_game(self):
        """Restart the game."""
        # Clean up existing snake segments
        for segment in self.snake_segments.values():
            self.engine.removeGameObject(segment)
        self.snake_segments.clear()

        self.score = 0
        self.snake_speed = 0.15
        self.game_state = "playing"
        self._initialize_snake()
        self._spawn_food()
        print("🔄 Game restarted! Score reset to 0")
