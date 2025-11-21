# Flappy Bird - Pyg Engine

A complete implementation of Flappy Bird built with the Pyg Engine framework.

## Features

- 🎮 Classic Flappy Bird gameplay
- 🏆 Leaderboard system with persistent storage
- 👤 Username tracking for high scores
- 📝 Text input UI component
- 🎨 Beautiful UI with anchored elements
- 💾 Automatic score saving (replaces existing username scores)

## How to Play

### From the repository root:
```bash
python -m examples.flappy_bird.flappy_bird
```

### From this directory:
```bash
python flappy_bird.py
```

## Controls

### Ready Screen
- **SPACE or CLICK** - Start game

### Playing
- **SPACE or CLICK** - Flap wings
- Avoid the pipes and ground/ceiling

### High Score Entry (if applicable)
- **Type** - Enter your username
- **ENTER** - Submit name
- **SPACE or ESC** - Skip name entry (use "Anonymous")

### Game Over
- **R** - Restart game
- **Click Restart button** - Restart game
- **ESC** - Quit game
- **Click Quit button** - Quit game

## Game Mechanics

- Score increases by 1 for each pipe passed
- Only high scores (top 10) require username entry
- Usernames are unique - new score replaces old score for same username
- Leaderboard shows top 5 scores on game over screen
- Collision with pipes, ground, or ceiling ends the game

## File Structure

```
flappy_bird/
├── flappy_bird.py              # Main game entry point
├── flappy_bird_leaderboard.json # Saved scores (auto-generated)
├── README.md                    # This file
├── FLAPPY_BIRD_README.md       # Detailed technical documentation
├── LEADERBOARD_FEATURE.md      # Leaderboard system documentation
└── scripts/
    ├── flappy_bird_controller.py # Main game controller
    ├── bird_script.py           # Bird physics and controls
    ├── pipe_script.py           # Pipe movement logic
    └── leaderboard.py           # Leaderboard data management
```

## Documentation

- **[FLAPPY_BIRD_README.md](FLAPPY_BIRD_README.md)** - Complete technical guide
- **[LEADERBOARD_FEATURE.md](LEADERBOARD_FEATURE.md)** - Leaderboard system details

## Requirements

- Python 3.7+
- pygame-ce (pygame community edition)
- pyg_engine (included in parent directory)

## Tips

- The bird falls due to gravity - tap frequently to stay airborne
- The gap between pipes is 180 pixels
- Pipes spawn every 2 seconds
- Try to maintain a steady rhythm for best control
- Your best score per username is saved - replay to improve!

## Credits

Built using the Pyg Engine framework as a demonstration of:
- Component-based game architecture
- UI system with anchors and event handling
- Script attachment system
- Physics and collision detection
- Persistent data storage

Enjoy the game! 🐦

