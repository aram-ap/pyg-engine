#!/usr/bin/env python3
"""
Snake game demo for pyg_engine.

Controls:
- Arrow keys or WASD: Move snake
- Space or P: Pause/Resume
- R: Restart after game over (or any time)
- Escape: Quit
"""

import random
from typing import List, Optional, Tuple

from pyg_engine import Color, Engine, Keys

GridPos = Tuple[int, int]

GRID_WIDTH = 28
GRID_HEIGHT = 20
CELL_SIZE = 40
MOVE_INTERVAL_SECONDS = 0.12
HUD_HEIGHT = 84.0
PADDING = 20.0


class SnakeGame:
    """Small state container for snake gameplay rules."""

    def __init__(self) -> None:
        self.reset()

    def reset(self) -> None:
        center_x = GRID_WIDTH // 2
        center_y = GRID_HEIGHT // 2
        self.snake: List[GridPos] = [
            (center_x, center_y),
            (center_x - 1, center_y),
            (center_x - 2, center_y),
        ]
        self.direction: GridPos = (1, 0)
        self.next_direction: GridPos = (1, 0)
        self.food: Optional[GridPos] = self._spawn_food()
        self.score = 0
        self.paused = False
        self.game_over = False
        self.has_won = False
        self.move_timer = 0.0

    def _spawn_food(self) -> Optional[GridPos]:
        occupied = set(self.snake)
        if len(occupied) >= GRID_WIDTH * GRID_HEIGHT:
            return None

        candidates: List[GridPos] = []
        for y in range(GRID_HEIGHT):
            for x in range(GRID_WIDTH):
                pos = (x, y)
                if pos not in occupied:
                    candidates.append(pos)

        return random.choice(candidates) if candidates else None

    def queue_direction(self, direction: GridPos) -> None:
        if self.game_over:
            return

        # Avoid reversing directly into the snake body.
        opposite = (-self.direction[0], -self.direction[1])
        if direction == opposite:
            return

        self.next_direction = direction

    def step(self) -> None:
        if self.game_over or self.paused:
            return

        self.direction = self.next_direction
        head_x, head_y = self.snake[0]
        dx, dy = self.direction
        new_head = (head_x + dx, head_y + dy)

        x, y = new_head
        hit_wall = x < 0 or y < 0 or x >= GRID_WIDTH or y >= GRID_HEIGHT
        # Moving into the old tail position is valid on non-growth moves.
        hit_body = new_head in self.snake[:-1]
        if hit_wall or hit_body:
            self.game_over = True
            return

        self.snake.insert(0, new_head)
        if self.food is not None and new_head == self.food:
            self.score += 1
            self.food = self._spawn_food()
            if self.food is None:
                self.has_won = True
                self.game_over = True
        else:
            self.snake.pop()


class KeyEdgeTracker:
    """Frame-edge detector built on key_down for reliable one-shot actions."""

    def __init__(self) -> None:
        self._previous: dict[str, bool] = {}

    def just_pressed_any(self, engine: Engine, keys: tuple[str, ...]) -> bool:
        triggered = False
        for key in keys:
            is_down = engine.input.key_down(key)
            was_down = self._previous.get(key, False)
            if is_down and not was_down:
                triggered = True
            self._previous[key] = is_down
        return triggered


def read_direction_input(engine: Engine) -> Optional[GridPos]:
    """Read one direction change request for this frame."""
    if engine.input.key_down("w") or engine.input.key_down(Keys.ARROW_UP):
        return (0, -1)
    if engine.input.key_down("s") or engine.input.key_down(Keys.ARROW_DOWN):
        return (0, 1)
    if engine.input.key_down("a") or engine.input.key_down(Keys.ARROW_LEFT):
        return (-1, 0)
    if engine.input.key_down("d") or engine.input.key_down(Keys.ARROW_RIGHT):
        return (1, 0)
    return None


def compute_board_origin(display_width: int, display_height: int) -> Tuple[float, float]:
    board_width = GRID_WIDTH * CELL_SIZE
    board_height = GRID_HEIGHT * CELL_SIZE
    board_x = max(PADDING, (display_width - board_width) * 0.5)
    board_y = max(HUD_HEIGHT, (display_height - board_height) * 0.5)
    return board_x, board_y


def draw_scene(engine: Engine, game: SnakeGame) -> None:
    display_width, display_height = engine.get_display_size()
    board_x, board_y = compute_board_origin(display_width, display_height)
    board_width = GRID_WIDTH * CELL_SIZE
    board_height = GRID_HEIGHT * CELL_SIZE

    engine.clear_draw_commands()

    # Background and board frame
    engine.draw_gradient_rect(
        0.0,
        0.0,
        float(display_width),
        float(display_height),
        Color(0.06, 0.08, 0.12, 1.0),
        Color(0.07, 0.12, 0.10, 1.0),
        Color(0.13, 0.09, 0.08, 1.0),
        Color(0.10, 0.07, 0.12, 1.0),
        draw_order=0.0,
    )
    engine.draw_rectangle(
        board_x - 3.0,
        board_y - 3.0,
        float(board_width + 6),
        float(board_height + 6),
        Color(0.90, 0.95, 1.00, 0.25),
        draw_order=1.0,
    )
    engine.draw_rectangle(
        board_x,
        board_y,
        float(board_width),
        float(board_height),
        Color(0.08, 0.10, 0.13, 0.96),
        draw_order=1.0,
    )

    # Grid lines
    grid_color = Color(0.98, 1.00, 1.00, 0.06)
    for x in range(GRID_WIDTH + 1):
        px = board_x + x * CELL_SIZE
        engine.draw_line(
            px,
            board_y,
            px,
            board_y + board_height,
            grid_color,
            thickness=1.0,
            draw_order=2.0,
        )
    for y in range(GRID_HEIGHT + 1):
        py = board_y + y * CELL_SIZE
        engine.draw_line(
            board_x,
            py,
            board_x + board_width,
            py,
            grid_color,
            thickness=1.0,
            draw_order=2.0,
        )

    # Food
    if game.food is not None:
        food_x, food_y = game.food
        engine.draw_rectangle(
            board_x + food_x * CELL_SIZE + 4.0,
            board_y + food_y * CELL_SIZE + 4.0,
            float(CELL_SIZE - 8),
            float(CELL_SIZE - 8),
            Color(0.96, 0.28, 0.20, 1.0),
            draw_order=4.1,
        )

    # Snake
    for idx, (seg_x, seg_y) in enumerate(game.snake):
        color = Color(0.28, 0.90, 0.44, 1.0)
        if idx == 0:
            color = Color(0.44, 0.99, 0.58, 1.0)

        engine.draw_rectangle(
            board_x + seg_x * CELL_SIZE + 2.0,
            board_y + seg_y * CELL_SIZE + 2.0,
            float(CELL_SIZE - 4),
            float(CELL_SIZE - 4),
            color,
            draw_order=5.3,
        )

    # HUD text
    engine.draw_text(
        f"Snake  |  Score: {game.score}  |  Length: {len(game.snake)}",
        22.0,
        18.0,
        Color(0.95, 0.99, 1.00, 0.97),
        font_size=30.0,
        draw_order=10.0,
    )
    engine.draw_text(
        "Move: WASD/Arrows   Pause: Space/P   Restart: R   Quit: Esc",
        22.0,
        52.0,
        Color(0.82, 0.90, 0.96, 0.95),
        font_size=16.0,
        draw_order=10.0,
    )

    if game.paused and not game.game_over:
        center_x = board_x + board_width * 0.5 - 62.0
        center_y = board_y + board_height * 0.5 - 20.0
        engine.draw_rectangle(
            center_x - 24.0,
            center_y - 10.0,
            230.0,
            54.0,
            Color(0.0, 0.0, 0.0, 0.45),
            draw_order=11.0,
        )
        engine.draw_text("PAUSED", center_x, center_y, Color.WHITE, font_size=28.0, draw_order=12.0)

    if game.game_over:
        overlay_message = "YOU WIN! Press R to play again" if game.has_won else "GAME OVER! Press R to restart"
        center_x = board_x + board_width * 0.5 - 220.0
        center_y = board_y + board_height * 0.5 - 22.0
        engine.draw_rectangle(
            center_x - 16.0,
            center_y - 16.0,
            730.0,
            62.0,
            Color(0.0, 0.0, 0.0, 0.60),
            draw_order=11.0,
        )
        engine.draw_text(
            overlay_message,
            center_x,
            center_y,
            Color(1.0, 0.98, 0.90, 1.0),
            font_size=26.0,
            draw_order=12.0,
        )


def main() -> None:
    engine = Engine(log_level="INFO")
    engine.start_manual(
        title="PyG Engine - Snake Demo",
        width=980,
        height=760,
        background_color=Color.rgb(18, 18, 26),
        vsync=True,
        redraw_on_change_only=False,
        show_fps_in_title=True,
    )

    game = SnakeGame()
    key_edges = KeyEdgeTracker()
    engine.log_info("Snake demo started.")

    while engine.poll_events():
        if key_edges.just_pressed_any(engine, (Keys.ESCAPE,)):
            break

        if key_edges.just_pressed_any(engine, ("r",)):
            game.reset()

        if key_edges.just_pressed_any(engine, ("p", Keys.SPACE)):
            if not game.game_over:
                game.paused = not game.paused

        direction_change = read_direction_input(engine)
        if direction_change is not None:
            game.queue_direction(direction_change)

        dt = engine.delta_time
        if dt > 0.25:
            dt = 0.25

        if not game.paused and not game.game_over:
            game.move_timer += dt
            while game.move_timer >= MOVE_INTERVAL_SECONDS:
                game.move_timer -= MOVE_INTERVAL_SECONDS
                game.step()
                if game.game_over:
                    break
        else:
            game.move_timer = 0.0

        draw_scene(engine, game)
        engine.update()
        engine.render()

    engine.log_info("Snake demo finished.")


if __name__ == "__main__":
    main()
