# Flappy Bird - Pyg Engine Implementation

A complete Flappy Bird clone built using the pyg_engine game framework.

## Files Created

1. **flappy_bird.py** - Main game entry point
2. **scripts/flappy_bird_controller.py** - Game controller managing state, pipes, and scoring
3. **scripts/bird_script.py** - Bird physics and input handling
4. **scripts/pipe_script.py** - Pipe movement and scoring logic

## How to Play

```bash
python examples/flappy_bird.py
```

### Reducing Console Output

The game displays minimal console output by default. If you see debug messages from the engine (like "Added script", "Destroyed GameObject"), these come from the core engine's GameObject class. To completely disable them, you can:

1. Modify `src/core/gameobject.py` to comment out print statements
2. Or redirect output: `python examples/flappy_bird.py > /dev/null 2>&1` (Unix/Mac)

The game is designed to be played visually - all important information (score, game state) is displayed on screen.

### Controls
- **SPACE** or **LEFT CLICK** - Flap wings / Start game
- **R** - Restart game (when game over)
- **ESC** - Quit game

### Gameplay
- Guide the bird through pipes without hitting them
- Each pipe you pass increases your score by 1
- Game ends if you hit a pipe, ground, or ceiling
- Try to beat your high score!

## Game Architecture

### FlappyBirdController (Main Game Manager)
- Manages game states: ready, playing, game_over
- Spawns pipes at regular intervals
- Handles collision detection
- Tracks score and high score
- Renders UI (score, game over screen, instructions)

### BirdScript (Player Character)
- Applies gravity physics
- Handles flap input (spacebar/click)
- Smooth rotation based on velocity
- Idle floating animation in ready state

### PipeScript (Obstacles)
- Moves pipes from right to left
- Upper pipe triggers scoring when bird passes
- Automatically cleaned up when off-screen

## Coordinate System

**Important:** This engine uses an inverted Y-axis coordinate system:
- **Positive Y goes UP** (towards top of screen)
- **Negative Y goes DOWN** (towards bottom of screen)
- This is like traditional math coordinates, opposite from standard pygame

## Game Parameters

You can modify these in `flappy_bird_controller.py`:

- `pipe_spawn_interval` - Time between pipe spawns (default: 2.0 seconds)
- `pipe_speed` - Pipe movement speed (default: 200 px/s)
- `pipe_gap` - Gap between upper and lower pipes (default: 180 px)

In `bird_script.py`:

- `gravity` - Downward acceleration (default: -800 px/s² - negative in inverted Y coords)
- `flap_strength` - Upward velocity on flap (default: 350 px/s - positive in inverted Y coords)
- `max_rotation` - Maximum bird tilt angle (default: 25°)

## Features

✅ Smooth physics-based bird movement  
✅ Procedural pipe generation  
✅ Collision detection with pipes and boundaries  
✅ Score tracking with high score persistence  
✅ Visual bird rotation based on velocity  
✅ Game state management (ready, playing, game over)  
✅ Clean UI with score display and instructions  
✅ Restart functionality  

## Engine Features Demonstrated

This game showcases several pyg_engine features:

1. **Script System** - Modular game logic through scripts
2. **GameObject System** - Dynamic creation and management of game objects
3. **Input System** - Keyboard and mouse input handling
4. **Update Loop** - Frame-based game logic
5. **Rendering** - Custom UI rendering with pygame
6. **Vector Math** - Using Vector2 for physics and movement
7. **Game State Management** - Clean state transitions

## Extending the Game

Ideas for enhancements:

- Add sound effects (flap, score, collision)
- Add sprite graphics for bird and pipes
- Add background scrolling
- Add different difficulty levels
- Add power-ups
- Add particle effects
- Add animation for bird wings
- Add day/night cycle
- Add different bird skins
- Add leaderboard system

Enjoy playing Flappy Bird on pyg_engine! 🐦

