"""
UI System for pyg_engine

A complete UI system with anchoring, event handling, and reactive components.

Components:
- UICanvas: Manager for all UI elements
- UIElement: Base class for UI components
- UIButton: Interactive button with hover/click states
- UILabel: Text display component
- UIPanel: Container for grouping elements
- UITextInput: Editable text input field

Example:
```python
from pyg_engine.ui import UICanvas, UIButton, UILabel, Anchor

# Create canvas
canvas = UICanvas(engine)

# Create button
button = UIButton(
    text="Click Me",
    anchor=Anchor.CENTER,
    onClick=lambda: print("Clicked!")
)
canvas.add_element(button)

# In game loop
canvas.update()
canvas.render(engine.screen)
```
"""

from .anchors import Anchor, get_anchor_position
from .ui_element import UIElement
from .ui_label import UILabel
from .ui_button import UIButton, ButtonState
from .ui_panel import UIPanel
from .ui_canvas import UICanvas
from .ui_text_input import UITextInput

__all__ = [
    'Anchor',
    'get_anchor_position',
    'UIElement',
    'UILabel',
    'UIButton',
    'ButtonState',
    'UIPanel',
    'UICanvas',
    'UITextInput',
]
