"""
Anchor system for UI positioning
"""

from enum import Enum, auto

class Anchor(Enum):
    """Anchor points for UI element positioning."""
    TOP_LEFT = auto()
    TOP_CENTER = auto()
    TOP_RIGHT = auto()
    CENTER_LEFT = auto()
    CENTER = auto()
    CENTER_RIGHT = auto()
    BOTTOM_LEFT = auto()
    BOTTOM_CENTER = auto()
    BOTTOM_RIGHT = auto()

def get_anchor_position(anchor, screen_width, screen_height):
    """
    Get the screen position for an anchor point.
    
    Args:
        anchor: Anchor enum value
        screen_width: Width of the screen
        screen_height: Height of the screen
    
    Returns:
        tuple: (x, y) position in screen coordinates
    """
    positions = {
        Anchor.TOP_LEFT: (0, screen_height),
        Anchor.TOP_CENTER: (screen_width / 2, screen_height),
        Anchor.TOP_RIGHT: (screen_width, screen_height),
        Anchor.CENTER_LEFT: (0, screen_height / 2),
        Anchor.CENTER: (screen_width / 2, screen_height / 2),
        Anchor.CENTER_RIGHT: (screen_width, screen_height / 2),
        Anchor.BOTTOM_LEFT: (0, 0),
        Anchor.BOTTOM_CENTER: (screen_width / 2, 0),
        Anchor.BOTTOM_RIGHT: (screen_width, 0),
    }
    return positions.get(anchor, (0, 0))

