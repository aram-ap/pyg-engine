"""
UI system for PyG Engine - buttons, panels, labels, and more.
"""

from typing import Callable, Optional
from .pyg_engine_native import (
    ButtonComponent,
    PanelComponent,
    LabelComponent,
    GameObject,
)


class Button:
    """
    A clickable button UI element.

    Example:
        >>> def on_click():
        ...     print("Button clicked!")
        >>> button = Button("Click Me", x=100, y=50, width=120, height=40, on_click=on_click)
        >>> engine.ui.add(button)
    """

    def __init__(
        self,
        text: str = "",
        x: float = 0,
        y: float = 0,
        width: float = 100,
        height: float = 30,
        on_click: Optional[Callable[..., None]] = None,
        enabled: bool = True,
        depth: float = 0,
        trigger_on: str = "release",
        repeat_interval_ms: Optional[float] = None,
    ):
        """
        Create a new button.

        Args:
            text: Button label text
            x: X position in screen coordinates
            y: Y position in screen coordinates
            width: Button width in pixels
            height: Button height in pixels
            on_click: Callback function called when button is clicked.
                Can be `def callback():` or `def callback(engine_handle):` to receive engine access.
            enabled: Whether the button is enabled
            depth: Rendering depth (higher = in front)
            trigger_on: When to trigger the callback - "press" (on mouse down) or "release" (on mouse up, default)
            repeat_interval_ms: If set, the callback will repeat every X milliseconds while the button is held.
                Set to None (default) to disable repeating.
        """
        self._component = ButtonComponent(text, x, y, width, height)
        self._game_object = None
        self._engine_handle = None
        self._user_callback = on_click

        self._component.set_enabled(enabled)
        self._component.set_depth(depth)
        self._component.set_trigger_on(trigger_on)
        if repeat_interval_ms is not None:
            self._component.set_repeat_interval(repeat_interval_ms)

    def add_to_engine(self, engine) -> int:
        """
        Add this button to the engine and return its object ID.

        .. deprecated::
            Use ``engine.ui.add(button)`` instead.

        Args:
            engine: The Engine instance

        Returns:
            The GameObject ID
        """
        # Store engine handle for callbacks
        self._engine_handle = engine.get_handle()

        # Set up callback wrapper if user provided a callback
        if self._user_callback:
            self._setup_callback()

        self._game_object = GameObject()
        self._game_object.set_object_type("UIObject")
        self._game_object.add_component(self._component)
        return engine.add_game_object(self._game_object)

    def _setup_callback(self):
        """Internal: Set up the callback wrapper to pass engine handle if needed."""
        import inspect

        # Check if callback accepts parameters
        sig = inspect.signature(self._user_callback)
        num_params = len(sig.parameters)

        if num_params == 0:
            # No parameters - call as-is
            self._component.set_on_click(self._user_callback)
        elif num_params == 1:
            # One parameter - pass engine handle
            def wrapper():
                self._user_callback(self._engine_handle)
            self._component.set_on_click(wrapper)
        else:
            raise ValueError(
                f"Button callback must accept 0 or 1 parameters, got {num_params}. "
                f"Use `def callback():` or `def callback(engine):`"
            )

    @property
    def text(self) -> str:
        """Get the button text."""
        return self._component.get_text()

    @text.setter
    def text(self, value: str):
        """Set the button text."""
        self._component.set_text(value)

    @property
    def enabled(self) -> bool:
        """Get whether the button is enabled."""
        return self._enabled

    @enabled.setter
    def enabled(self, value: bool):
        """Set whether the button is enabled."""
        self._enabled = value
        self._component.set_enabled(value)

    def set_position(self, x: float, y: float):
        """Set the button position."""
        self._component.set_position(x, y)

    def set_size(self, width: float, height: float):
        """Set the button size."""
        self._component.set_size(width, height)

    def set_on_click(self, callback: Optional[Callable[..., None]]):
        """
        Set the click callback.

        Args:
            callback: Function called when clicked. Can be:
                - `def callback():` (no parameters)
                - `def callback(engine):` (receives EngineHandle)
        """
        self._user_callback = callback
        if callback and self._engine_handle:
            self._setup_callback()
        elif callback:
            # Not yet added to engine, will be set up in add_to_engine
            pass
        else:
            # Clear callback
            self._component.set_on_click(lambda: None)

    def set_trigger_on(self, trigger: str):
        """
        Set when the button callback is triggered.

        Args:
            trigger: "press" to trigger on mouse down, "release" to trigger on mouse up (default)
        """
        self._component.set_trigger_on(trigger)

    def set_repeat_interval(self, interval_ms: Optional[float]):
        """
        Set the repeat interval for when the button is held down.

        Args:
            interval_ms: Interval in milliseconds between repeats, or None to disable repeating
        """
        self._component.set_repeat_interval(interval_ms)


class Panel:
    """
    A rectangular panel/container UI element.

    Example:
        >>> panel = Panel(x=50, y=50, width=300, height=200)
        >>> panel.set_background_color(0.9, 0.9, 0.9, 1.0)
        >>> panel.set_border(2, 0.5, 0.5, 0.5, 1.0)
        >>> engine.ui.add(panel)
    """

    def __init__(
        self,
        x: float = 0,
        y: float = 0,
        width: float = 200,
        height: float = 200,
        depth: float = 0,
    ):
        """
        Create a new panel.

        Args:
            x: X position in screen coordinates
            y: Y position in screen coordinates
            width: Panel width in pixels
            height: Panel height in pixels
            depth: Rendering depth (higher = in front)
        """
        self._component = PanelComponent(x, y, width, height)
        self._game_object = None
        self._component.set_depth(depth)

    def add_to_engine(self, engine) -> int:
        """
        Add this panel to the engine and return its object ID.

        .. deprecated::
            Use ``engine.ui.add(panel)`` instead.

        Args:
            engine: The Engine instance

        Returns:
            The GameObject ID
        """
        self._game_object = GameObject()
        self._game_object.set_object_type("UIObject")
        self._game_object.add_component(self._component)
        return engine.add_game_object(self._game_object)

    def set_position(self, x: float, y: float):
        """Set the panel position."""
        self._component.set_position(x, y)

    def set_size(self, width: float, height: float):
        """Set the panel size."""
        self._component.set_size(width, height)

    def set_background_color(self, r: float, g: float, b: float, a: float = 1.0):
        """
        Set the panel background color.

        Args:
            r: Red component (0-1)
            g: Green component (0-1)
            b: Blue component (0-1)
            a: Alpha component (0-1)
        """
        self._component.set_background_color(r, g, b, a)

    def set_border(self, width: float, r: float, g: float, b: float, a: float = 1.0):
        """
        Set the panel border.

        Args:
            width: Border width in pixels
            r: Red component (0-1)
            g: Green component (0-1)
            b: Blue component (0-1)
            a: Alpha component (0-1)
        """
        self._component.set_border(width, r, g, b, a)


class Label:
    """
    A text label UI element.

    Example:
        >>> label = Label("Hello World", x=100, y=100, font_size=16)
        >>> label.set_color(0, 0, 0, 1)
        >>> label.set_align("center")
        >>> engine.ui.add(label)
    """

    def __init__(
        self,
        text: str = "",
        x: float = 0,
        y: float = 0,
        font_size: float = 14,
        align: str = "left",
        depth: float = 0,
    ):
        """
        Create a new label.

        Args:
            text: Label text
            x: X position in screen coordinates
            y: Y position in screen coordinates
            font_size: Font size in pixels
            align: Text alignment ("left", "center", "right")
            depth: Rendering depth (higher = in front)
        """
        self._component = LabelComponent(text, x, y, font_size)
        self._game_object = None
        self._engine = None
        self._object_id = None
        self._component.set_align(align)
        self._component.set_depth(depth)

    def add_to_engine(self, engine) -> int:
        """
        Add this label to the engine and return its object ID.

        .. deprecated::
            Use ``engine.ui.add(label)`` instead.

        Args:
            engine: The Engine instance

        Returns:
            The GameObject ID
        """
        # Store engine handle instead of engine to avoid borrow checker issues in callbacks
        self._engine = engine.get_handle()
        self._game_object = GameObject()
        self._game_object.set_object_type("UIObject")
        self._game_object.add_component(self._component)
        self._object_id = engine.add_game_object(self._game_object)
        return self._object_id

    @property
    def text(self) -> str:
        """Get the label text."""
        return self._component.get_text()

    @text.setter
    def text(self, value: str):
        """Set the label text."""
        self._component.set_text(value)
        # Update runtime component if already added to engine
        if self._engine is not None and self._object_id is not None:
            self._engine.update_ui_label_text(self._object_id, value)

    def set_position(self, x: float, y: float):
        """Set the label position."""
        self._component.set_position(x, y)

    def set_font_size(self, size: float):
        """Set the label font size."""
        self._component.set_font_size(size)

    def set_color(self, r: float, g: float, b: float, a: float = 1.0):
        """
        Set the label text color.

        Args:
            r: Red component (0-1)
            g: Green component (0-1)
            b: Blue component (0-1)
            a: Alpha component (0-1)
        """
        self._component.set_color(r, g, b, a)

    def set_align(self, align: str):
        """
        Set the text alignment.

        Args:
            align: "left", "center", or "right"
        """
        self._component.set_align(align)


__all__ = ["Button", "Panel", "Label"]
