"""
Python wrapper for the Rust-based pyg_engine native module.

This module provides a clean Python interface to the Rust engine implementation
with comprehensive logging capabilities.
"""

from typing import TYPE_CHECKING, Any, Optional

if TYPE_CHECKING:
    pass

try:
    from .pyg_engine_native import DrawCommand as _RustDrawCommand
    from .pyg_engine_native import Engine as _RustEngine
    from .pyg_engine_native import EngineHandle as _RustEngineHandle
    from .pyg_engine_native import MouseButton, Keys
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
        layer: int = 0,
        z_index: float = 0.0,
    ) -> None:
        """Draw a pixel in window coordinates via command queue."""
        self._inner.draw_pixel(x, y, color, layer=layer, z_index=z_index)

    def draw_line(
        self,
        start_x: float,
        start_y: float,
        end_x: float,
        end_y: float,
        color: Any,
        thickness: float = 1.0,
        layer: int = 0,
        z_index: float = 0.0,
    ) -> None:
        """Draw a line in window coordinates via command queue."""
        self._inner.draw_line(
            start_x,
            start_y,
            end_x,
            end_y,
            color,
            thickness=thickness,
            layer=layer,
            z_index=z_index,
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
        layer: int = 0,
        z_index: float = 0.0,
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
            layer=layer,
            z_index=z_index,
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
        layer: int = 0,
        z_index: float = 0.0,
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
            layer=layer,
            z_index=z_index,
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
        layer: int = 0,
        z_index: float = 0.0,
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
            layer=layer,
            z_index=z_index,
        )

    def draw_image(
        self,
        x: float,
        y: float,
        width: float,
        height: float,
        texture_path: str,
        layer: int = 0,
        z_index: float = 0.0,
    ) -> None:
        """Draw an image from a file path via command queue."""
        self._inner.draw_image(
            x,
            y,
            width,
            height,
            texture_path,
            layer=layer,
            z_index=z_index,
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
        layer: int = 0,
        z_index: float = 0.0,
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
            layer=layer,
            z_index=z_index,
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
    
    @property
    def input(self) -> Input:
        """
        Get the input manager for the engine.
        
        Returns:
            Input: The input manager instance.
        """
        return self._input

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

    def get_display_size(self) -> tuple[int, int]:
        """Get the current display size (window client size) in pixels."""
        return self._engine.get_display_size()

    def initialize(
        self,
        title: str = "PyG Engine",
        width: int = 1280,
        height: int = 720,
        resizable: bool = True,
        background_color: Optional[Any] = None,
        vsync: bool = True,
        redraw_on_change_only: bool = True,
        show_fps_in_title: bool = False,
    ) -> None:
        """
        Initialize the engine with window configuration without starting the loop.
        
        This method is used when you want to control the game loop manually using
        `poll_events()`, `update()`, and `render()`.
        
        Args:
            title: Window title.
            width: Initial window width.
            height: Initial window height.
            resizable: Whether the window can be resized.
            background_color: Optional `pyg_engine.Color`.
            vsync: Enable/disable vertical sync.
            redraw_on_change_only: When True (default), only redraw on scene changes.
            show_fps_in_title: When True, appends current FPS to window title.
        """
        self._engine.initialize(
            title=title,
            width=width,
            height=height,
            resizable=resizable,
            background_color=background_color,
            vsync=vsync,
            redraw_on_change_only=redraw_on_change_only,
            show_fps_in_title=show_fps_in_title,
        )

    def poll_events(self) -> bool:
        """
        Poll events from the window system.
        
        Returns:
            bool: True if the loop should continue, False if exit requested.
        """
        return self._engine.poll_events()

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
    ) -> None:
        """
        Run the engine window and enter the event loop.

        Args:
            title: Window title.
            width: Initial window width.
            height: Initial window height.
            resizable: Whether the window can be resized.
            background_color: Optional `pyg_engine.Color`.
            vsync: Enable/disable vertical sync.
            redraw_on_change_only: When True (default), only redraw on scene changes.
            show_fps_in_title: When True, appends current FPS to window title.
        """
        self._engine.run(
            title=title,
            width=width,
            height=height,
            resizable=resizable,
            background_color=background_color,
            vsync=vsync,
            redraw_on_change_only=redraw_on_change_only,
            show_fps_in_title=show_fps_in_title,
        )

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
        layer: int = 0,
        z_index: float = 0.0,
    ) -> None:
        """Draw a pixel in window coordinates."""
        self._engine.draw_pixel(x, y, color, layer=layer, z_index=z_index)

    def draw_line(
        self,
        start_x: float,
        start_y: float,
        end_x: float,
        end_y: float,
        color: Any,
        thickness: float = 1.0,
        layer: int = 0,
        z_index: float = 0.0,
    ) -> None:
        """Draw a line in window coordinates."""
        self._engine.draw_line(
            start_x,
            start_y,
            end_x,
            end_y,
            color,
            thickness=thickness,
            layer=layer,
            z_index=z_index,
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
        layer: int = 0,
        z_index: float = 0.0,
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
            layer=layer,
            z_index=z_index,
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
        layer: int = 0,
        z_index: float = 0.0,
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
            layer=layer,
            z_index=z_index,
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
        layer: int = 0,
        z_index: float = 0.0,
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
            layer=layer,
            z_index=z_index,
        )

    def draw_image(
        self,
        x: float,
        y: float,
        width: float,
        height: float,
        texture_path: str,
        layer: int = 0,
        z_index: float = 0.0,
    ) -> None:
        """Draw an image from a file path."""
        self._engine.draw_image(
            x,
            y,
            width,
            height,
            texture_path,
            layer=layer,
            z_index=z_index,
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
        layer: int = 0,
        z_index: float = 0.0,
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
            layer=layer,
            z_index=z_index,
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
