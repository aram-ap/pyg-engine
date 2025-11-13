import pyg_engine
from pyg_engine import Engine, Size, GameObject
from enum import Enum
from typing import Dict


class Alignment(Enum):
    # Where to set the screen's origin point
    TopLeft = "Top Left"
    TopCenter= "Top Center"
    TopRight = "Top Right"
    CenterLeft = "Center Left"
    Center = "Center"
    CenterRight = "Center Right"
    BottomLeft = "Bottom Left"
    BottomCenter = "Bottom Center"
    BottomRight = "Bottom Right"

class UI_Element(GameObject):

    def __init__(self, engine: Engine):
        super.__init__(engine)

        # size and position are already defined in the GameObject class
        self.layer = 0          # 0 is ground layer, anything higher than it will cover it.
        self.priority = 0
        self._enabled = True
        self.visible = True
        self.children : list[UI_Element]
        self.alignment = Alignment.Center
        self.input_enabled = False

    def add_child(self, child: UI_Element, enabled: bool = True):
        if child in self.children:
            return

        self.children.append(child)

    @property
    def enabled(self) -> bool:
        return self._enabled

    @enabled.setter
    def enabled(self, value: bool) -> None:
        """Runs whenever ``obj.enabled = …`` is executed."""
        if self._enabled != value:
            self._enabled = value
            self._on_enabled_change(value)

    def _on_enabled_change(self, is_enabled: bool) -> None:
        if self._enabled == is_enabled:
            return

        if is_enabled:
            for child in self._children:
                self._chil


