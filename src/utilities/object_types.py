from dataclasses import dataclass
from enum import Enum


@dataclass
class Size:
    """Width and height dimensions."""
    w: int
    h: int

    def __str__(self):
        return f"({self.w}, {self.h})"

class BasicShape(Enum):
    """Basic geometric shapes for game objects."""
    Rectangle = "rectangle"
    Circle = "circle"

class Tag(Enum):
    """Object tags for categorization and collision filtering."""
    Player = "player"
    Environment = "environment"
    Other = "other"
