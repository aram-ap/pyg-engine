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
    A clickable button UI element with customizable appearance and behavior.

    Buttons can trigger callbacks on press or release, and optionally repeat
    while held down. The callback can optionally receive an `EngineHandle`
    parameter for thread-safe engine access.

    **Basic Example:**

        ```python
        from pyg_engine import Engine, Button

        engine = Engine()

        def on_click():
            print("Button clicked!")

        button = Button("Click Me", x=100, y=50, width=120, height=40, on_click=on_click)
        engine.ui.add(button)

        engine.run()
        ```

    **Advanced Example with Engine Access:**

        ```python
        # Callback with engine handle parameter
        def on_click(engine_handle):
            engine_handle.log("Button clicked with engine access!")
            engine_handle.draw_circle(200, 200, 50, Color(1.0, 0.0, 0.0, 1.0))

        button = Button("Action", x=100, y=100, on_click=on_click)
        engine.ui.add(button)
        ```

    **Hold and Repeat Example:**

        ```python
        counter = [0]
        label = Label(f"Count: {counter[0]}", x=300, y=100)

        def increment():
            counter[0] += 1
            label.text = f"Count: {counter[0]}"

        # Button repeats callback every 100ms while held
        button = Button(
            "Hold to Increment",
            x=100, y=100,
            on_click=increment,
            repeat_interval_ms=100
        )
        engine.ui.add(button)
        engine.ui.add(label)
        ```
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
        """
        Set the button position in screen coordinates.

        Args:
            x: X coordinate in pixels (0 = left edge).
            y: Y coordinate in pixels (0 = top edge).

        Example:
            ```python
            button = Button("Move Me", x=100, y=100)
            engine.ui.add(button)

            # Move button to new position
            button.set_position(200, 150)
            ```
        """
        self._component.set_position(x, y)

    def set_size(self, width: float, height: float):
        """
        Set the button size in pixels.

        Args:
            width: Button width in pixels.
            height: Button height in pixels.

        Example:
            ```python
            button = Button("Resize Me", x=100, y=100)
            engine.ui.add(button)

            # Make button larger
            button.set_size(200, 60)
            ```
        """
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
            trigger: "press" to trigger on mouse down, "release" to trigger on mouse up (default).

        Example:
            ```python
            # Fire on press for more responsive feel
            button = Button("Quick Action", x=100, y=100)
            button.set_trigger_on("press")
            engine.ui.add(button)

            # Standard button behavior (fire on release)
            button2 = Button("Standard", x=100, y=150)
            button2.set_trigger_on("release")
            engine.ui.add(button2)
            ```
        """
        self._component.set_trigger_on(trigger)

    def set_repeat_interval(self, interval_ms: Optional[float]):
        """
        Set the repeat interval for when the button is held down.

        When set, the callback will be repeatedly called while the button is held,
        with the specified time between each call.

        Args:
            interval_ms: Interval in milliseconds between repeats, or None to disable repeating.

        Example:
            ```python
            volume = [50]

            def increase_volume():
                volume[0] = min(100, volume[0] + 1)
                print(f"Volume: {volume[0]}")

            button = Button("Volume +", x=100, y=100, on_click=increase_volume)
            button.set_repeat_interval(100)  # Repeat every 100ms while held
            engine.ui.add(button)

            # Disable repeating
            # button.set_repeat_interval(None)
            ```
        """
        self._component.set_repeat_interval(interval_ms)


class Panel:
    """
    A rectangular panel/container UI element with customizable background and border.

    Panels are useful for creating UI backgrounds, separators, and visual containers
    for other UI elements. They support depth ordering for layering.

    **Basic Example:**

        ```python
        from pyg_engine import Engine, Panel

        engine = Engine()

        # Create a simple gray panel
        panel = Panel(x=50, y=50, width=300, height=200)
        panel.set_background_color(0.9, 0.9, 0.9, 1.0)
        panel.set_border(2, 0.5, 0.5, 0.5, 1.0)
        engine.ui.add(panel)

        engine.run()
        ```

    **Layered Panels Example:**

        ```python
        # Background panel (lower depth)
        bg_panel = Panel(x=100, y=100, width=400, height=300, depth=0)
        bg_panel.set_background_color(0.2, 0.2, 0.2, 0.8)
        engine.ui.add(bg_panel)

        # Foreground panel (higher depth, drawn on top)
        fg_panel = Panel(x=150, y=150, width=300, height=200, depth=1)
        fg_panel.set_background_color(0.9, 0.9, 0.9, 1.0)
        fg_panel.set_border(3, 0.0, 0.5, 1.0, 1.0)  # Blue border
        engine.ui.add(fg_panel)
        ```
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
        """
        Set the panel position in screen coordinates.

        Args:
            x: X coordinate in pixels (0 = left edge).
            y: Y coordinate in pixels (0 = top edge).

        Example:
            ```python
            panel = Panel(x=100, y=100, width=200, height=150)
            engine.ui.add(panel)

            # Move panel to new position
            panel.set_position(300, 200)
            ```
        """
        self._component.set_position(x, y)

    def set_size(self, width: float, height: float):
        """
        Set the panel size in pixels.

        Args:
            width: Panel width in pixels.
            height: Panel height in pixels.

        Example:
            ```python
            panel = Panel(x=100, y=100, width=200, height=150)
            engine.ui.add(panel)

            # Resize panel
            panel.set_size(400, 300)
            ```
        """
        self._component.set_size(width, height)

    def set_background_color(self, r: float, g: float, b: float, a: float = 1.0):
        """
        Set the panel background color.

        Args:
            r: Red component (0.0-1.0).
            g: Green component (0.0-1.0).
            b: Blue component (0.0-1.0).
            a: Alpha/transparency component (0.0=transparent, 1.0=opaque).

        Example:
            ```python
            panel = Panel(x=100, y=100, width=300, height=200)

            # Semi-transparent blue background
            panel.set_background_color(0.0, 0.5, 1.0, 0.7)

            # Solid red background
            panel.set_background_color(1.0, 0.0, 0.0, 1.0)

            engine.ui.add(panel)
            ```
        """
        self._component.set_background_color(r, g, b, a)

    def set_border(self, width: float, r: float, g: float, b: float, a: float = 1.0):
        """
        Set the panel border width and color.

        Args:
            width: Border width in pixels (0 = no border).
            r: Red component (0.0-1.0).
            g: Green component (0.0-1.0).
            b: Blue component (0.0-1.0).
            a: Alpha/transparency component (0.0=transparent, 1.0=opaque).

        Example:
            ```python
            panel = Panel(x=100, y=100, width=300, height=200)
            panel.set_background_color(0.95, 0.95, 0.95, 1.0)

            # Add a 3-pixel black border
            panel.set_border(3, 0.0, 0.0, 0.0, 1.0)

            # Add a thick red border
            panel.set_border(5, 1.0, 0.0, 0.0, 1.0)

            engine.ui.add(panel)
            ```
        """
        self._component.set_border(width, r, g, b, a)


class Label:
    """
    A text label UI element for displaying static or dynamic text.

    Labels support customizable font size, color, and alignment. The text
    can be updated at runtime using the `text` property, which is useful
    for displaying scores, status information, or other dynamic content.

    **Basic Example:**

        ```python
        from pyg_engine import Engine, Label

        engine = Engine()

        label = Label("Hello World", x=100, y=100, font_size=16)
        label.set_color(0.0, 0.0, 0.0, 1.0)  # Black text
        label.set_align("center")
        engine.ui.add(label)

        engine.run()
        ```

    **Dynamic Text Example:**

        ```python
        score = [0]
        score_label = Label(f"Score: {score[0]}", x=400, y=30, font_size=24, align="center")
        score_label.set_color(1.0, 1.0, 1.0, 1.0)  # White text
        engine.ui.add(score_label)

        def on_click():
            score[0] += 10
            # Update label text dynamically
            score_label.text = f"Score: {score[0]}"

        button = Button("Add Points", x=350, y=100, on_click=on_click)
        engine.ui.add(button)
        ```

    **Multi-Style Labels Example:**

        ```python
        # Title (large, centered)
        title = Label("Game Title", x=400, y=50, font_size=32, align="center")
        title.set_color(0.0, 0.5, 1.0, 1.0)  # Blue
        engine.ui.add(title)

        # Left-aligned info text
        info = Label("Health: 100", x=20, y=20, font_size=14, align="left")
        info.set_color(0.0, 1.0, 0.0, 1.0)  # Green
        engine.ui.add(info)

        # Right-aligned timer
        timer = Label("Time: 0:00", x=760, y=20, font_size=14, align="right")
        timer.set_color(1.0, 1.0, 1.0, 1.0)  # White
        engine.ui.add(timer)
        ```
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
        """
        Get the current label text.

        Returns:
            The current text string displayed by the label.
        """
        return self._component.get_text()

    @text.setter
    def text(self, value: str):
        """
        Set the label text dynamically.

        If the label has been added to the engine, the text is updated immediately
        and will be reflected in the next frame.

        Args:
            value: The new text to display.

        Example:
            ```python
            label = Label("Initial Text", x=100, y=100)
            engine.ui.add(label)

            # Update text dynamically (e.g., in a callback or update loop)
            label.text = "Updated Text"
            label.text = f"Score: {player_score}"
            ```
        """
        self._component.set_text(value)
        # Update runtime component if already added to engine
        if self._engine is not None and self._object_id is not None:
            self._engine.update_ui_label_text(self._object_id, value)

    def set_position(self, x: float, y: float):
        """
        Set the label position in screen coordinates.

        The position interpretation depends on the alignment:
        - "left": x is the left edge of the text
        - "center": x is the center of the text
        - "right": x is the right edge of the text

        Args:
            x: X coordinate in pixels.
            y: Y coordinate in pixels (top of the text).

        Example:
            ```python
            label = Label("Move Me", x=100, y=100)
            engine.ui.add(label)

            # Move label to new position
            label.set_position(300, 200)
            ```
        """
        self._component.set_position(x, y)

    def set_font_size(self, size: float):
        """
        Set the label font size in pixels.

        Args:
            size: Font size in pixels (e.g., 12, 16, 24, 32).

        Example:
            ```python
            label = Label("Resize Me", x=100, y=100, font_size=14)
            engine.ui.add(label)

            # Make text larger
            label.set_font_size(24)

            # Make text smaller
            label.set_font_size(10)
            ```
        """
        self._component.set_font_size(size)

    def set_color(self, r: float, g: float, b: float, a: float = 1.0):
        """
        Set the label text color.

        Args:
            r: Red component (0.0-1.0).
            g: Green component (0.0-1.0).
            b: Blue component (0.0-1.0).
            a: Alpha/transparency component (0.0=transparent, 1.0=opaque).

        Example:
            ```python
            label = Label("Colorful Text", x=100, y=100)

            # White text
            label.set_color(1.0, 1.0, 1.0, 1.0)

            # Red text
            label.set_color(1.0, 0.0, 0.0, 1.0)

            # Semi-transparent blue text
            label.set_color(0.0, 0.5, 1.0, 0.7)

            engine.ui.add(label)
            ```
        """
        self._component.set_color(r, g, b, a)

    def set_align(self, align: str):
        """
        Set the text alignment relative to the label's position.

        Args:
            align: Text alignment mode - "left", "center", or "right".

        Example:
            ```python
            # Left-aligned at x=100
            left_label = Label("Left", x=100, y=100, align="left")
            engine.ui.add(left_label)

            # Centered at x=400 (middle of 800px window)
            center_label = Label("Center", x=400, y=150, align="center")
            engine.ui.add(center_label)

            # Right-aligned at x=700 (near right edge)
            right_label = Label("Right", x=700, y=200, align="right")
            engine.ui.add(right_label)

            # Change alignment dynamically
            center_label.set_align("left")
            ```
        """
        self._component.set_align(align)


__all__ = ["Button", "Panel", "Label"]
