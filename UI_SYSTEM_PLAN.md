# UI System Implementation Plan

## Phase 1: Core Foundation ✅
- [x] UIElement base class with anchoring
- [x] State management (normal, hover, pressed, disabled)
- [x] Event callback system
- [x] Screen-responsive positioning

## Phase 2: Essential Components ✅
- [x] UIButton - Interactive buttons with callbacks
- [x] UILabel - Text display with styling
- [x] UIPanel - Container for grouping elements
- [x] UIImage - Image display component

## Phase 3: Canvas Manager ✅
- [x] UICanvas - Manages all UI elements
- [x] Input event routing to UI elements
- [x] Z-order rendering
- [x] Screen resize handling

## Phase 4: Advanced Components (Future)
- [ ] UISlider - Value input
- [ ] UICheckbox - Toggle states
- [ ] UITextField - Text input
- [ ] UIProgressBar - Progress display
- [ ] UITooltip - Hover information

## Phase 5: Layout System (Future)
- [ ] Flexbox-like layout
- [ ] Grid layout
- [ ] Auto-sizing containers

## Implementation Details

### UIElement (Base Class)
```python
Properties:
- position: Vector2 (screen coordinates)
- size: Vector2 (width, height)
- anchor: Anchor enum (positioning)
- offset: Vector2 (from anchor point)
- visible: bool
- enabled: bool
- layer: int (z-order)
- parent: UIElement or None

Methods:
- update(engine) - Update logic
- render(screen) - Draw to screen
- handle_event(event) - Process input
- get_rect() - Get bounding box
- contains_point(x, y) - Hit testing
```

### UIButton
```python
Additional Properties:
- text: str
- onClick: callable
- onHover: callable
- state: ButtonState (normal/hover/pressed/disabled)
- colors: dict (per state)
- font_size: int

States:
- NORMAL - Default state
- HOVER - Mouse over button
- PRESSED - Mouse button down
- DISABLED - Not interactive
```

### UILabel
```python
Properties:
- text: str
- font_size: int
- font_name: str or None
- color: Color
- align: str (left/center/right)
- bold: bool
- italic: bool
```

### UIPanel
```python
Properties:
- background_color: Color
- border_color: Color or None
- border_width: int
- children: list[UIElement]
- padding: int

Methods:
- add_child(element)
- remove_child(element)
- clear_children()
```

### UICanvas
```python
Properties:
- elements: list[UIElement]
- screen_size: Size
- input_enabled: bool

Methods:
- add_element(element)
- remove_element(element)
- update(engine) - Update all elements
- render(screen) - Render all elements
- handle_input(engine) - Route input to elements
- clear() - Remove all elements
```

## Usage Example

```python
from pyg_engine.ui import UICanvas, UIButton, UILabel, Anchor

# Create UI canvas
canvas = UICanvas(engine)

# Create title label
title = UILabel(
    text="Game Title",
    font_size=48,
    color=Color(255, 255, 255),
    anchor=Anchor.TOP_CENTER,
    offset=Vector2(0, -50)
)
canvas.add_element(title)

# Create play button
play_btn = UIButton(
    text="Play",
    size=Vector2(200, 60),
    anchor=Anchor.CENTER,
    onClick=lambda: start_game()
)
canvas.add_element(play_btn)

# In game loop
canvas.update(engine)
canvas.render(engine.screen)
```

## Integration with Flappy Bird

### Main Menu
- Title label (centered top)
- Play button (centered)
- High score display (top right)
- Credits (bottom)

### In-Game HUD
- Score label (top center)
- Pause button (top right)

### Game Over Screen
- "Game Over" title
- Score display
- High score display
- Restart button
- Quit button

## File Structure

```
src/ui/
├── __init__.py
├── ui_element.py      # Base class
├── ui_button.py       # Button component
├── ui_label.py        # Text component
├── ui_panel.py        # Container component
├── ui_image.py        # Image component
├── ui_canvas.py       # Canvas manager
└── anchors.py         # Anchor enum
```

