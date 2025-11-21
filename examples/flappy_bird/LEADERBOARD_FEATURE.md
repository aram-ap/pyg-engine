# Flappy Bird - Leaderboard Feature

## Overview
The Flappy Bird game now includes a complete leaderboard system with username tracking and persistent storage.

## New Features

### 1. Username Input
- **Location**: Ready screen (before game starts)
- **Features**:
  - Editable text field with placeholder text
  - Real-time username updates as you type
  - Maximum length of 15 characters
  - Submit with Enter key or click outside to unfocus
  - Default username: "Player"

### 2. Leaderboard System
- **Persistent Storage**: Scores are saved to `flappy_bird_leaderboard.json`
- **Top 10 Scores**: Keeps track of the best 10 scores
- **Ranked Display**: Shows your rank on the leaderboard
- **Timestamp Tracking**: Each score includes a timestamp

### 3. Game Over Screen Updates
The game over screen now displays:
- Your final score
- Your rank on the leaderboard (if you made it to top 10)
- **Leaderboard**: Top 5 scores with usernames
- Your entry is highlighted with a "►" marker and yellow text
- All-time high score
- Restart and Quit buttons

## UI Components Added

### UITextInput Component
A new reusable UI component for text input fields:

**Features**:
- Placeholder text support
- Character limit enforcement
- Cursor blinking animation
- Keyboard navigation (Home, End, Arrow keys)
- Text selection and editing
- Focus/unfocus states
- Customizable colors and styling
- Submit and change callbacks

**Usage Example**:
```python
from pyg_engine.ui import UITextInput, Anchor
from pyg_engine.utilities import Vector2, Color

text_input = UITextInput(
    placeholder="Enter name...",
    text="Player",
    max_length=15,
    size=Vector2(300, 50),
    font_size=28,
    anchor=Anchor.CENTER,
    offset=Vector2(0, 0),
    onSubmit=lambda text: print(f"Submitted: {text}"),
    onChange=lambda text: print(f"Changed: {text}"),
    background_color=Color(40, 40, 60),
    active_color=Color(60, 60, 80)
)
```

### Leaderboard System
A standalone leaderboard manager with JSON persistence:

**Features**:
- Automatic sorting by score
- Entry limit enforcement
- Rank calculation
- High score tracking
- Save/load functionality

**Usage Example**:
```python
from examples.scripts.leaderboard import Leaderboard

# Create leaderboard
leaderboard = Leaderboard(
    save_file="my_game_scores.json",
    max_entries=10
)

# Add entry
rank = leaderboard.add_entry("Alice", 42)

# Get top entries
top_scores = leaderboard.get_top_entries(5)

# Check if score qualifies
if leaderboard.is_high_score(50):
    print("New high score!")
```

## Game Controls

### Ready Screen
- **Type**: Enter your username in the text field
- **Click Text Field**: Focus the input for typing
- **Enter**: Submit username
- **Escape**: Unfocus text field
- **Space/Click**: Start game (when not typing)

### Playing
- **Space/Click**: Flap wings
- **Objective**: Fly through pipes without crashing

### Game Over
- **R Key**: Restart game
- **Click Restart Button**: Restart game
- **ESC Key**: Quit game
- **Click Quit Button**: Quit game

## File Structure

### New Files
```
examples/scripts/
├── leaderboard.py              # Leaderboard system implementation
└── flappy_bird_controller.py   # Updated with username and leaderboard

src/ui/
└── ui_text_input.py           # New text input UI component

examples/
└── flappy_bird_leaderboard.json  # Auto-generated leaderboard data
```

## Leaderboard Data Format
```json
{
  "entries": [
    {
      "username": "Alice",
      "score": 42,
      "timestamp": "2025-11-14T12:30:45.123456"
    }
  ],
  "max_entries": 10,
  "last_updated": "2025-11-14T12:30:45.123456"
}
```

## Technical Details

### Text Input Implementation
- Uses pygame's KEYDOWN events for character input
- Supports Unicode character input
- Implements cursor position tracking
- Blinking cursor animation with configurable speed
- Handles special keys (Backspace, Delete, Arrow keys, Home, End, Escape, Enter)
- Text clipping for overflow

### Leaderboard Implementation
- Uses JSON for persistent storage
- Automatic sorting on insertion
- Thread-safe file operations with error handling
- ISO timestamp format for entry tracking
- Maintains top N entries (configurable)

### Event Handling
The game now processes two types of events:
1. **Keyboard Events**: For text input (KEYDOWN)
2. **Mouse Events**: For button clicks and game controls

### Integration Notes
- Text input focus blocks game start (prevents accidental starts while typing)
- Leaderboard loads on game start and saves automatically on game over
- Username persists across restarts within the same session
- Leaderboard data persists across game sessions

## Future Enhancement Ideas
- Online leaderboard synchronization
- Different difficulty levels with separate leaderboards
- Achievement system
- Player profiles with avatars
- Score history graphs
- Export leaderboard to CSV/Excel
- Social sharing of high scores

