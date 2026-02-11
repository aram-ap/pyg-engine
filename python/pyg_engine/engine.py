"""
Python wrapper for the Rust-based pyg_engine native module.

This module provides a clean Python interface to the Rust engine implementation
with comprehensive logging capabilities.
"""

import inspect
from collections.abc import Callable
from typing import TYPE_CHECKING, Any, Optional

if TYPE_CHECKING:
    pass

try:
    from .pyg_engine_native import DrawCommand as _RustDrawCommand
    from .pyg_engine_native import Engine as _RustEngine
    from .pyg_engine_native import EngineHandle as _RustEngineHandle
    from .pyg_engine_native import CameraAspectMode, Keys, MouseButton
except ImportError as e:
    raise ImportError(
        "Failed to import pyg_engine_native. "
        "The Rust extension may not be compiled. "
        "Run 'pip install -e .' to build the extension."
    ) from e

DrawCommand = _RustDrawCommand


class EngineHandle:
    """
    Thread-safe handle to the engine that can be passed to background threads.
    
    Use this handle to queue commands like adding objects or drawing from other threads.
    """
    def __init__(self, inner: _RustEngineHandle) -> None:
        self._inner = inner

    def add_game_object(self, game_object: Any) -> None:
        """
        Add a `pyg_engine.GameObject` to the runtime scene.
        
        This is thread-safe and will be processed on the next engine update.
        """
        self._inner.add_game_object(game_object)

    def remove_game_object(self, object_id: int) -> None:
        """Remove a runtime GameObject by id via command queue."""
        self._inner.remove_game_object(object_id)

    def set_game_object_position(self, object_id: int, position: Any) -> None:
        """Update a runtime GameObject position by id via command queue."""
        self._inner.set_game_object_position(object_id, position)

    def set_camera_position(self, position: Any) -> None:
        """Update the active camera world position via command queue."""
        self._inner.set_camera_position(position)

    def set_camera_viewport_size(self, width: float, height: float) -> None:
        """Update the active camera viewport size in world units via command queue."""
        self._inner.set_camera_viewport_size(width, height)

    def set_camera_aspect_mode(self, mode: str) -> None:
        """Update camera aspect handling mode via command queue."""
        self._inner.set_camera_aspect_mode(mode)

    def set_camera_background_color(self, color: Any) -> None:
        """Update the active camera background clear color via command queue."""
        self._inner.set_camera_background_color(color)

    def clear_draw_commands(self) -> None:
        """Clear all immediate-mode drawing commands via command queue."""
        self._inner.clear_draw_commands()

    def add_draw_commands(self, commands: list[Any]) -> None:
        """Submit many draw commands via command queue in one call."""
        self._inner.add_draw_commands(commands)

    def draw_pixel(
        self,
        x: int,
        y: int,
        color: Any,
        draw_order: float = 0.0,
    ) -> None:
        """Draw a pixel in window coordinates via command queue."""
        self._inner.draw_pixel(x, y, color, draw_order=draw_order)

    def draw_line(
        self,
        start_x: float,
        start_y: float,
        end_x: float,
        end_y: float,
        color: Any,
        thickness: float = 1.0,
        draw_order: float = 0.0,
    ) -> None:
        """Draw a line in window coordinates via command queue."""
        self._inner.draw_line(
            start_x,
            start_y,
            end_x,
            end_y,
            color,
            thickness=thickness,
            draw_order=draw_order,
        )

    def draw_rectangle(
        self,
        x: float,
        y: float,
        width: float,
        height: float,
        color: Any,
        filled: bool = True,
        thickness: float = 1.0,
        draw_order: float = 0.0,
    ) -> None:
        """Draw a rectangle in window coordinates via command queue."""
        self._inner.draw_rectangle(
            x,
            y,
            width,
            height,
            color,
            filled=filled,
            thickness=thickness,
            draw_order=draw_order,
        )

    def draw_circle(
        self,
        center_x: float,
        center_y: float,
        radius: float,
        color: Any,
        filled: bool = True,
        thickness: float = 1.0,
        segments: int = 32,
        draw_order: float = 0.0,
    ) -> None:
        """Draw a circle in window coordinates via command queue."""
        self._inner.draw_circle(
            center_x,
            center_y,
            radius,
            color,
            filled=filled,
            thickness=thickness,
            segments=segments,
            draw_order=draw_order,
        )

    def draw_gradient_rect(
        self,
        x: float,
        y: float,
        width: float,
        height: float,
        top_left: Any,
        bottom_left: Any,
        bottom_right: Any,
        top_right: Any,
        draw_order: float = 0.0,
    ) -> None:
        """Draw a gradient rectangle with per-corner colors via command queue."""
        self._inner.draw_gradient_rect(
            x,
            y,
            width,
            height,
            top_left,
            bottom_left,
            bottom_right,
            top_right,
            draw_order=draw_order,
        )

    def draw_image(
        self,
        x: float,
        y: float,
        width: float,
        height: float,
        texture_path: str,
        draw_order: float = 0.0,
    ) -> None:
        """Draw an image from a file path via command queue."""
        self._inner.draw_image(
            x,
            y,
            width,
            height,
            texture_path,
            draw_order=draw_order,
        )

    def draw_image_from_bytes(
        self,
        x: float,
        y: float,
        width: float,
        height: float,
        texture_key: str,
        rgba: bytes,
        texture_width: int,
        texture_height: int,
        draw_order: float = 0.0,
    ) -> None:
        """Draw an image from raw RGBA bytes via command queue."""
        self._inner.draw_image_from_bytes(
            x,
            y,
            width,
            height,
            texture_key,
            rgba,
            texture_width,
            texture_height,
            draw_order=draw_order,
        )

    def draw_text(
        self,
        text: str,
        x: float,
        y: float,
        color: Any,
        font_size: float = 24.0,
        font_path: Optional[str] = None,
        letter_spacing: float = 0.0,
        line_spacing: float = 0.0,
        draw_order: float = 0.0,
    ) -> None:
        """
        Draw text via command queue.

        By default, this uses the engine's built-in open-source font.
        Provide `font_path` to use a custom TTF/OTF font file.
        """
        self._inner.draw_text(
            text,
            x,
            y,
            color,
            font_size=font_size,
            font_path=font_path,
            letter_spacing=letter_spacing,
            line_spacing=line_spacing,
            draw_order=draw_order,
        )


class Input:
    """
    Handles keyboard, mouse, and joystick input.
    
    This class provides methods to check the current state of input devices
    and events. It is accessed via the `engine.input` property.
    """
    def __init__(self, engine: "Engine") -> None:
        self._engine = engine._engine
    
    def key_down(self, key: str) -> bool:
        """Check if a keyboard key is currently held down."""
        return self._engine.key_down(key)
    
    def key_pressed(self, key: str) -> bool:
        """Check if a keyboard key was pressed this frame."""
        return self._engine.key_pressed(key)
    
    def key_released(self, key: str) -> bool:
        """Check if a keyboard key was released this frame."""
        return self._engine.key_released(key)
    
    def mouse_button_down(self, button: MouseButton) -> bool:
        """Check if a mouse button is currently held down."""
        return self._engine.mouse_button_down(button)
    
    def mouse_button_pressed(self, button: MouseButton) -> bool:
        """Check if a mouse button was pressed this frame."""
        return self._engine.mouse_button_pressed(button)
    
    def mouse_button_released(self, button: MouseButton) -> bool:
        """Check if a mouse button was released this frame."""
        return self._engine.mouse_button_released(button)
    
    @property
    def mouse_position(self) -> tuple[float, float]:
        """Get the current mouse position in window coordinates."""
        return self._engine.mouse_position()
    
    @property
    def mouse_delta(self) -> tuple[float, float]:
        """Get the mouse movement delta for this frame."""
        return self._engine.mouse_delta()
    
    @property
    def mouse_wheel(self) -> tuple[float, float]:
        """Get the mouse wheel delta accumulated this frame."""
        return self._engine.mouse_wheel()
    
    def axis(self, name: str) -> float:
        """Get the current value of a logical axis (-1.0 to 1.0)."""
        return self._engine.axis(name)
    
    def axis_previous(self, name: str) -> float:
        """Get the previous frame's value of a logical axis."""
        return self._engine.axis_previous(name)

    def axis_names(self) -> list[str]:
        """List all configured logical axis names."""
        return self._engine.axis_names()

    def action_down(self, action_name: str) -> bool:
        """Check whether an action is currently active (held)."""
        return self._engine.action_down(action_name)

    def action_pressed(self, action_name: str) -> bool:
        """Check whether an action was pressed this frame."""
        return self._engine.action_pressed(action_name)

    def action_released(self, action_name: str) -> bool:
        """Check whether an action was released this frame."""
        return self._engine.action_released(action_name)

    def action_names(self) -> list[str]:
        """List all configured action names."""
        return self._engine.action_names()

    def reset_bindings_to_defaults(self) -> None:
        """Restore default axis and action bindings."""
        self._engine.reset_input_bindings_to_defaults()

    def set_axis_keys(
        self,
        name: str,
        positive_keys: list[str],
        negative_keys: list[str],
        sensitivity: float = 1.0,
    ) -> None:
        """Set keyboard keys for an axis (replaces existing keyboard keys)."""
        self._engine.set_axis_keys(name, positive_keys, negative_keys, sensitivity=sensitivity)

    def add_axis_positive_key(self, axis_name: str, key: str) -> None:
        """Add one positive key to an axis binding."""
        self._engine.add_axis_positive_key(axis_name, key)

    def add_axis_negative_key(self, axis_name: str, key: str) -> None:
        """Add one negative key to an axis binding."""
        self._engine.add_axis_negative_key(axis_name, key)

    def remove_axis_positive_key(self, axis_name: str, key: str) -> bool:
        """Remove one positive key from an axis binding."""
        return self._engine.remove_axis_positive_key(axis_name, key)

    def remove_axis_negative_key(self, axis_name: str, key: str) -> bool:
        """Remove one negative key from an axis binding."""
        return self._engine.remove_axis_negative_key(axis_name, key)

    def remove_axis(self, axis_name: str) -> bool:
        """Remove a logical axis binding."""
        return self._engine.remove_axis(axis_name)

    def set_axis_mouse(
        self,
        name: str,
        mouse_axis: str,
        sensitivity: float = 1.0,
        invert: bool = False,
    ) -> bool:
        """Bind an axis to mouse input (x/y/wheel)."""
        return self._engine.set_axis_mouse(
            name,
            mouse_axis,
            sensitivity=sensitivity,
            invert=invert,
        )

    def set_action_keys(self, action_name: str, keys: list[str]) -> None:
        """Set keyboard bindings for an action (replaces existing keyboard keys)."""
        self._engine.set_action_keys(action_name, keys)

    def add_action_key(self, action_name: str, key: str) -> None:
        """Add one keyboard key to an action binding."""
        self._engine.add_action_key(action_name, key)

    def remove_action_key(self, action_name: str, key: str) -> bool:
        """Remove one keyboard key from an action binding."""
        return self._engine.remove_action_key(action_name, key)

    def set_action_mouse_buttons(self, action_name: str, buttons: list[str]) -> None:
        """Set mouse-button bindings for an action (replaces existing mouse buttons)."""
        self._engine.set_action_mouse_buttons(action_name, buttons)

    def add_action_mouse_button(self, action_name: str, button: MouseButton) -> None:
        """Add one mouse button to an action binding."""
        self._engine.add_action_mouse_button(action_name, button)

    def remove_action_mouse_button(self, action_name: str, button: MouseButton) -> bool:
        """Remove one mouse button from an action binding."""
        return self._engine.remove_action_mouse_button(action_name, button)

    def clear_action_bindings(self, action_name: str) -> None:
        """Clear all bindings (keyboard/mouse/joystick) for an action."""
        self._engine.clear_action_bindings(action_name)


class UpdateContext:
    """
    Mutable frame context passed to function-based engine update callbacks.

    This object is reused each frame to keep callback overhead low.
    """

    __slots__ = (
        "engine",
        "input",
        "delta_time",
        "elapsed_time",
        "frame",
        "user_data",
        "_should_stop",
    )

    def __init__(self, engine: "Engine", user_data: Any = None) -> None:
        self.engine = engine
        self.input = engine.input
        self.delta_time = 0.0
        self.elapsed_time = 0.0
        self.frame = 0
        self.user_data = user_data
        self._should_stop = False

    def stop(self) -> None:
        """Request that `Engine.run(update=...)` exits after this frame."""
        self._should_stop = True

    @property
    def should_stop(self) -> bool:
        """Return whether `stop()` has been requested."""
        return self._should_stop


_CallbackInvoker = Callable[[UpdateContext], object]
_CallbackValueProvider = Callable[[UpdateContext], object]

_CALLBACK_VALUE_PROVIDERS: dict[str, _CallbackValueProvider] = {
    "ctx": lambda context: context,
    "context": lambda context: context,
    "engine": lambda context: context.engine,
    "input": lambda context: context.input,
    "dt": lambda context: context.delta_time,
    "delta_time": lambda context: context.delta_time,
    "elapsed": lambda context: context.elapsed_time,
    "elapsed_time": lambda context: context.elapsed_time,
    "frame": lambda context: context.frame,
    "user_data": lambda context: context.user_data,
}

_RUNTIME_STATE_IDLE = "idle"
_RUNTIME_STATE_MANUAL = "manual"
_RUNTIME_STATE_RUNNING_BLOCKING = "running_blocking"
_RUNTIME_STATE_RUNNING_CALLBACK = "running_callback"


def _compile_update_callback(update_callback: Callable[..., object]) -> _CallbackInvoker:
    """
    Compile callback argument injection once for low per-frame overhead.

    Supported styles:
    - `def update(): ...`
    - `def update(dt): ...`
    - `def update(delta_time, engine): ...`
    - `def update(context): ...`
    - `def update(any_name): ...`  # single unknown arg receives UpdateContext
    """
    if not callable(update_callback):
        raise TypeError("update_callback must be callable")

    try:
        signature = inspect.signature(update_callback)
    except (TypeError, ValueError) as exc:
        raise TypeError("Could not inspect update callback signature") from exc

    parameters = list(signature.parameters.values())
    if not parameters:
        return lambda _context: update_callback()

    positional_providers: list[_CallbackValueProvider] = []
    keyword_providers: list[tuple[str, _CallbackValueProvider]] = []
    unknown_required_parameters: list[inspect.Parameter] = []

    for parameter in parameters:
        if parameter.kind is inspect.Parameter.VAR_POSITIONAL:
            continue
        if parameter.kind is inspect.Parameter.VAR_KEYWORD:
            continue

        provider = _CALLBACK_VALUE_PROVIDERS.get(parameter.name)
        if provider is None:
            if parameter.default is inspect.Parameter.empty:
                unknown_required_parameters.append(parameter)
            continue

        if parameter.kind in (
            inspect.Parameter.POSITIONAL_ONLY,
            inspect.Parameter.POSITIONAL_OR_KEYWORD,
        ):
            positional_providers.append(provider)
        elif parameter.kind is inspect.Parameter.KEYWORD_ONLY:
            keyword_providers.append((parameter.name, provider))

    if unknown_required_parameters:
        if len(parameters) == 1 and len(unknown_required_parameters) == 1:
            single_unknown = unknown_required_parameters[0]
            if single_unknown.kind is inspect.Parameter.KEYWORD_ONLY:
                unknown_name = single_unknown.name

                def invoke_with_context_alias_kw(context: UpdateContext) -> object:
                    return update_callback(**{unknown_name: context})

                return invoke_with_context_alias_kw

            return lambda context: update_callback(context)

        unsupported_names = ", ".join(
            sorted(parameter.name for parameter in unknown_required_parameters)
        )
        raise TypeError(
            "Unsupported required update callback parameter(s): "
            f"{unsupported_names}. "
            "Supported names: "
            f"{', '.join(sorted(_CALLBACK_VALUE_PROVIDERS))}"
        )

    if not positional_providers and not keyword_providers:
        return lambda _context: update_callback()

    # Fast path: purely positional injection (no kwargs dict allocation per frame).
    if not keyword_providers:
        provider_count = len(positional_providers)
        if provider_count == 1:
            getter_0 = positional_providers[0]
            return lambda context: update_callback(getter_0(context))
        if provider_count == 2:
            getter_0, getter_1 = positional_providers
            return lambda context: update_callback(getter_0(context), getter_1(context))
        if provider_count == 3:
            getter_0, getter_1, getter_2 = positional_providers
            return lambda context: update_callback(
                getter_0(context), getter_1(context), getter_2(context)
            )
        if provider_count == 4:
            getter_0, getter_1, getter_2, getter_3 = positional_providers
            return lambda context: update_callback(
                getter_0(context),
                getter_1(context),
                getter_2(context),
                getter_3(context),
            )

        positional_getters = tuple(positional_providers)

        def invoke_with_positional(context: UpdateContext) -> object:
            return update_callback(*[getter(context) for getter in positional_getters])

        return invoke_with_positional

    # Slow path: keyword-only parameters require kwargs.
    positional_getters = tuple(positional_providers)
    keyword_getters = tuple(keyword_providers)

    def invoke_with_mixed(context: UpdateContext) -> object:
        positional_values = [getter(context) for getter in positional_getters]
        kwargs = {name: getter(context) for name, getter in keyword_getters}
        return update_callback(*positional_values, **kwargs)

    return invoke_with_mixed


class Engine:
    """
    Main game engine class providing core functionality with structured logging.
    
    This class wraps the Rust implementation and provides a Python-friendly API
    with full access to the tracing-based logging system.
    
    Attributes:
        version (str): The engine version number.
    
    Example:
        Basic usage with console logging:
        >>> engine = Engine()
        >>> engine.log_info("Engine started")
        >>> engine.log_warn("Low memory warning")
        >>> engine.log_error("Failed to load resource")
        
        With file logging enabled:
        >>> engine = Engine(
        ...     enable_file_logging=True,
        ...     log_directory="./logs",
        ...     log_level="DEBUG"
        ... )
        >>> engine.log_debug("Debug information")
    """
    
    def __init__(
        self,
        enable_file_logging: bool = False,
        log_directory: Optional[str] = None,
        log_level: Optional[str] = None,
    ) -> None:
        """
        Initialize a new Engine instance with optional logging configuration.
        
        Args:
            enable_file_logging: Enable logging to files (default: False).
                Files are rotated daily and stored in the log directory.
            log_directory: Directory path for log files (default: "./logs").
                Only used if enable_file_logging is True.
            log_level: Minimum log level to display. Options:
                - "TRACE": Most verbose, includes all logs
                - "DEBUG": Debug and higher
                - "INFO": Info and higher (default)
                - "WARN": Warnings and errors only
                - "ERROR": Errors only
        
        Example:
            >>> # Console only with INFO level (default)
            >>> engine = Engine()
            
            >>> # Enable file logging with DEBUG level
            >>> engine = Engine(
            ...     enable_file_logging=True,
            ...     log_directory="./my_logs",
            ...     log_level="DEBUG"
            ... )
        """
        self._engine = _RustEngine(
            enable_file_logging=enable_file_logging,
            log_directory=log_directory,
            log_level=log_level,
        )
        self._input = Input(self)
        self._runtime_state = _RUNTIME_STATE_IDLE
        self._window_icon_path: Optional[str] = None
    
    @property
    def input(self) -> Input:
        """
        Get the input manager for the engine.
        
        Returns:
            Input: The input manager instance.
        """
        return self._input

    @property
    def is_running(self) -> bool:
        """Return whether the engine is currently running in any loop mode."""
        return self._runtime_state != _RUNTIME_STATE_IDLE

    def _ensure_not_running(self, entrypoint_name: str) -> None:
        if self._runtime_state != _RUNTIME_STATE_IDLE:
            raise RuntimeError(
                f"Cannot call {entrypoint_name} while engine is already running "
                f"in '{self._runtime_state}' mode."
            )

    def get_handle(self) -> EngineHandle:
        """
        Get a thread-safe handle to the engine that can be passed to background threads.
        
        Returns:
            EngineHandle: A handle used to queue commands from other threads.
        """
        return EngineHandle(self._engine.get_handle())
    
    def log(self, message: str) -> None:
        """
        Log a message at INFO level (default log method).
        
        This is an alias for log_info() for backward compatibility.
        
        Args:
            message: The message to log.
        """
        self._engine.log(message)
    
    def log_trace(self, message: str) -> None:
        """
        Log a message at TRACE level (most verbose).
        
        Use for very detailed debugging information.
        Only visible when log level is set to TRACE.
        
        Args:
            message: The message to log.
        """
        self._engine.log_trace(message)
    
    def log_debug(self, message: str) -> None:
        """
        Log a message at DEBUG level.
        
        Use for debugging information that's useful during development.
        Visible when log level is DEBUG or TRACE.
        
        Args:
            message: The message to log.
        """
        self._engine.log_debug(message)
    
    def log_info(self, message: str) -> None:
        """
        Log a message at INFO level.
        
        Use for general informational messages about engine operation.
        This is the default log level.
        
        Args:
            message: The message to log.
        """
        self._engine.log_info(message)
    
    def log_warn(self, message: str) -> None:
        """
        Log a message at WARN level.
        
        Use for warnings that don't prevent operation but indicate
        potential issues.
        
        Args:
            message: The message to log.
        """
        self._engine.log_warn(message)
    
    def log_error(self, message: str) -> None:
        """
        Log a message at ERROR level.
        
        Use for errors that may affect engine operation.
        
        Args:
            message: The message to log.
        """
        self._engine.log_error(message)

    def set_window_title(self, title: str) -> None:
        """
        Set the window title.
        
        Args:
            title: The new window title.
        """
        self._engine.set_window_title(title)

    def set_window_icon(self, icon_path: str) -> None:
        """
        Set the window icon from an image file path.

        The path is remembered and will be reused for future `run()` /
        `start_manual()` calls unless overridden per call.
        """
        self._window_icon_path = icon_path
        self._engine.set_window_icon(icon_path)

    def get_display_size(self) -> tuple[int, int]:
        """Get the current display size (window client size) in pixels."""
        return self._engine.get_display_size()

    def start_manual(
        self,
        title: str = "PyG Engine",
        width: int = 1280,
        height: int = 720,
        resizable: bool = True,
        background_color: Optional[Any] = None,
        vsync: bool = True,
        redraw_on_change_only: bool = True,
        show_fps_in_title: bool = False,
        icon_path: Optional[str] = None,
    ) -> None:
        """
        Start the engine in manual-loop mode without entering a blocking loop.
        
        This mode is for advanced use cases where Python controls the frame loop
        manually using:
        `poll_events()`, `update()`, and `render()`.

        Raises:
            RuntimeError: If the engine is already running in another loop mode.
        
        Args:
            title: Window title.
            width: Initial window width.
            height: Initial window height.
            resizable: Whether the window can be resized.
            background_color: Optional `pyg_engine.Color`.
            vsync: Enable/disable vertical sync.
            redraw_on_change_only: When True (default), only redraw on scene changes.
            show_fps_in_title: When True, appends current FPS to window title.
            icon_path: Optional icon file path. If omitted, uses the most recent
                `set_window_icon(...)` value when present, otherwise the built-in
                default icon.
        """
        self._ensure_not_running("start_manual()")
        resolved_icon_path = (
            icon_path if icon_path is not None else self._window_icon_path
        )
        self._engine.initialize(
            title=title,
            width=width,
            height=height,
            resizable=resizable,
            background_color=background_color,
            vsync=vsync,
            redraw_on_change_only=redraw_on_change_only,
            show_fps_in_title=show_fps_in_title,
            icon_path=resolved_icon_path,
        )
        self._runtime_state = _RUNTIME_STATE_MANUAL

    def poll_events(self) -> bool:
        """
        Poll events from the window system.
        
        Returns:
            bool: True if the loop should continue, False if exit requested.
        """
        should_continue = self._engine.poll_events()
        if not should_continue and self._runtime_state in (
            _RUNTIME_STATE_MANUAL,
            _RUNTIME_STATE_RUNNING_CALLBACK,
        ):
            self._runtime_state = _RUNTIME_STATE_IDLE
        return should_continue

    def update(self) -> None:
        """Run a single update step."""
        self._engine.update()

    def render(self) -> None:
        """Render a single frame."""
        self._engine.render()

    def run(
        self,
        title: str = "PyG Engine",
        width: int = 1280,
        height: int = 720,
        resizable: bool = True,
        background_color: Optional[Any] = None,
        vsync: bool = True,
        redraw_on_change_only: bool = True,
        show_fps_in_title: bool = False,
        icon_path: Optional[str] = None,
        *,
        update: Optional[Callable[..., object]] = None,
        max_delta_time: Optional[float] = 0.1,
        user_data: Any = None,
    ) -> None:
        """
        Run the engine and start the frame loop.

        Default mode (no callback):
        - Enter the native blocking loop until close.

        Callback mode (`update=...`):
        - Start a Python-managed loop and invoke callback once per frame.
        - Callback can return `False` or call `context.stop()` to exit.
        - See `UpdateContext` for injected values.
        - Frame order is: poll events -> native update -> callback -> render.
          (Future GameObject script updates are intended to run in native update.)

        Raises:
            RuntimeError: If the engine is already running in another loop mode.

        Args:
            title: Window title.
            width: Initial window width.
            height: Initial window height.
            resizable: Whether the window can be resized.
            background_color: Optional `pyg_engine.Color`.
            vsync: Enable/disable vertical sync.
            redraw_on_change_only: When True (default), only redraw on scene changes.
            show_fps_in_title: When True, appends current FPS to window title.
            icon_path: Optional icon file path. If omitted, uses the most recent
                `set_window_icon(...)` value when present, otherwise the built-in
                default icon.
            update: Optional callback invoked once per frame. When omitted,
                the native blocking run loop is used.
            max_delta_time: Clamp callback `dt` to this value in seconds.
                Only used in callback mode (`update` provided).
            user_data: Arbitrary object exposed via callback context.
                Only used in callback mode (`update` provided).
        """
        self._ensure_not_running("run()")

        if update is None:
            resolved_icon_path = (
                icon_path if icon_path is not None else self._window_icon_path
            )
            self._runtime_state = _RUNTIME_STATE_RUNNING_BLOCKING
            try:
                self._engine.run(
                    title=title,
                    width=width,
                    height=height,
                    resizable=resizable,
                    background_color=background_color,
                    vsync=vsync,
                    redraw_on_change_only=redraw_on_change_only,
                    show_fps_in_title=show_fps_in_title,
                    icon_path=resolved_icon_path,
                )
            finally:
                self._runtime_state = _RUNTIME_STATE_IDLE
            return

        if max_delta_time is not None and max_delta_time <= 0.0:
            raise ValueError("max_delta_time must be > 0.0 or None")

        invoke_callback = _compile_update_callback(update)

        self.start_manual(
            title=title,
            width=width,
            height=height,
            resizable=resizable,
            background_color=background_color,
            vsync=vsync,
            redraw_on_change_only=redraw_on_change_only,
            show_fps_in_title=show_fps_in_title,
            icon_path=icon_path,
        )
        self._runtime_state = _RUNTIME_STATE_RUNNING_CALLBACK

        context = UpdateContext(self, user_data=user_data)

        native_engine = self._engine
        poll_events = native_engine.poll_events
        update_step = native_engine.update
        render_frame = native_engine.render

        try:
            while True:
                if not poll_events():
                    break

                # Update native systems first so callback gets current dt/input.
                update_step()

                context.delta_time = native_engine.delta_time
                if max_delta_time is not None and context.delta_time > max_delta_time:
                    context.delta_time = max_delta_time
                context.elapsed_time = native_engine.elapsed_time

                callback_result = invoke_callback(context)
                if callback_result is False or context._should_stop:
                    break

                render_frame()
                context.frame += 1
        finally:
            self._runtime_state = _RUNTIME_STATE_IDLE

    def add_game_object(self, game_object: Any) -> Optional[int]:
        """
        Add a `pyg_engine.GameObject` to the runtime scene.

        Returns:
            The runtime object id, or None if add failed.
        """
        return self._engine.add_game_object(game_object)

    def create_game_object(self, name: Optional[str] = None) -> Optional[int]:
        """
        Create and add a runtime GameObject.

        Returns:
            The runtime object id, or None if creation failed.
        """
        return self._engine.create_game_object(name)

    def remove_game_object(self, object_id: int) -> None:
        """Remove a runtime GameObject by id."""
        self._engine.remove_game_object(object_id)

    def set_game_object_position(self, object_id: int, position: Any) -> bool:
        """
        Update a runtime GameObject position by id.

        Returns:
            True if the object exists and was updated, False otherwise.
        """
        return self._engine.set_game_object_position(object_id, position)

    @property
    def camera_object_id(self) -> Optional[int]:
        """Get the runtime id of the active camera GameObject."""
        return self._engine.camera_object_id()

    def get_camera_position(self) -> Any:
        """Get the active camera world position."""
        return self._engine.get_camera_position()

    def set_camera_position(self, position: Any) -> bool:
        """Set the active camera world position."""
        return self._engine.set_camera_position(position)

    def get_camera_viewport_size(self) -> tuple[float, float]:
        """Get the active camera viewport size in world units."""
        return self._engine.get_camera_viewport_size()

    def set_camera_viewport_size(self, width: float, height: float) -> bool:
        """Set the active camera viewport size in world units."""
        return self._engine.set_camera_viewport_size(width, height)

    def get_camera_aspect_mode(self) -> str:
        """Get the camera aspect handling mode."""
        return self._engine.get_camera_aspect_mode()

    def set_camera_aspect_mode(self, mode: str) -> bool:
        """Set the camera aspect handling mode."""
        return self._engine.set_camera_aspect_mode(mode)

    def set_camera_background_color(self, color: Any) -> None:
        """Set the active camera background clear color."""
        self._engine.set_camera_background_color(color)

    def get_camera_background_color(self) -> Any:
        """Get the active camera background clear color."""
        return self._engine.get_camera_background_color()

    def world_to_screen(self, world_position: Any) -> tuple[float, float]:
        """Convert world-space coordinates to screen-space pixel coordinates."""
        return self._engine.world_to_screen(world_position)

    def screen_to_world(self, screen_x: float, screen_y: float) -> Any:
        """Convert screen-space pixel coordinates to world-space coordinates."""
        return self._engine.screen_to_world(screen_x, screen_y)

    def clear_draw_commands(self) -> None:
        """Clear all immediate-mode drawing commands."""
        self._engine.clear_draw_commands()

    def add_draw_commands(self, commands: list[Any]) -> None:
        """Submit many draw commands in one call."""
        self._engine.add_draw_commands(commands)

    def draw_pixel(
        self,
        x: int,
        y: int,
        color: Any,
        draw_order: float = 0.0,
    ) -> None:
        """Draw a pixel in window coordinates."""
        self._engine.draw_pixel(x, y, color, draw_order=draw_order)

    def draw_line(
        self,
        start_x: float,
        start_y: float,
        end_x: float,
        end_y: float,
        color: Any,
        thickness: float = 1.0,
        draw_order: float = 0.0,
    ) -> None:
        """Draw a line in window coordinates."""
        self._engine.draw_line(
            start_x,
            start_y,
            end_x,
            end_y,
            color,
            thickness=thickness,
            draw_order=draw_order,
        )

    def draw_rectangle(
        self,
        x: float,
        y: float,
        width: float,
        height: float,
        color: Any,
        filled: bool = True,
        thickness: float = 1.0,
        draw_order: float = 0.0,
    ) -> None:
        """Draw a rectangle in window coordinates."""
        self._engine.draw_rectangle(
            x,
            y,
            width,
            height,
            color,
            filled=filled,
            thickness=thickness,
            draw_order=draw_order,
        )

    def draw_circle(
        self,
        center_x: float,
        center_y: float,
        radius: float,
        color: Any,
        filled: bool = True,
        thickness: float = 1.0,
        segments: int = 32,
        draw_order: float = 0.0,
    ) -> None:
        """Draw a circle in window coordinates."""
        self._engine.draw_circle(
            center_x,
            center_y,
            radius,
            color,
            filled=filled,
            thickness=thickness,
            segments=segments,
            draw_order=draw_order,
        )

    def draw_gradient_rect(
        self,
        x: float,
        y: float,
        width: float,
        height: float,
        top_left: Any,
        bottom_left: Any,
        bottom_right: Any,
        top_right: Any,
        draw_order: float = 0.0,
    ) -> None:
        """Draw a gradient rectangle with per-corner colors."""
        self._engine.draw_gradient_rect(
            x,
            y,
            width,
            height,
            top_left,
            bottom_left,
            bottom_right,
            top_right,
            draw_order=draw_order,
        )

    def draw_image(
        self,
        x: float,
        y: float,
        width: float,
        height: float,
        texture_path: str,
        draw_order: float = 0.0,
    ) -> None:
        """Draw an image from a file path."""
        self._engine.draw_image(
            x,
            y,
            width,
            height,
            texture_path,
            draw_order=draw_order,
        )

    def draw_image_from_bytes(
        self,
        x: float,
        y: float,
        width: float,
        height: float,
        texture_key: str,
        rgba: bytes,
        texture_width: int,
        texture_height: int,
        draw_order: float = 0.0,
    ) -> None:
        """Draw an image from raw RGBA bytes."""
        self._engine.draw_image_from_bytes(
            x,
            y,
            width,
            height,
            texture_key,
            rgba,
            texture_width,
            texture_height,
            draw_order=draw_order,
        )

    def draw_text(
        self,
        text: str,
        x: float,
        y: float,
        color: Any,
        font_size: float = 24.0,
        font_path: Optional[str] = None,
        letter_spacing: float = 0.0,
        line_spacing: float = 0.0,
        draw_order: float = 0.0,
    ) -> None:
        """
        Draw text in window coordinates.

        By default, this uses the engine's built-in open-source font.
        Provide `font_path` to use a custom TTF/OTF font file.
        """
        self._engine.draw_text(
            text,
            x,
            y,
            color,
            font_size=font_size,
            font_path=font_path,
            letter_spacing=letter_spacing,
            line_spacing=line_spacing,
            draw_order=draw_order,
        )
    
    @property
    def version(self) -> str:
        """
        Get the engine version.
        
        Returns:
            The version string (e.g., "1.2.0").
        """
        return self._engine.version

    @property
    def delta_time(self) -> float:
        """Get the time since the last frame in seconds."""
        return self._engine.delta_time

    @property
    def elapsed_time(self) -> float:
        """Get the total elapsed time in seconds since the engine started."""
        return self._engine.elapsed_time
