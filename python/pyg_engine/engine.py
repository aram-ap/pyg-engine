"""
Python wrapper for the Rust-based pyg_engine native module.

This module provides a clean Python interface to the Rust engine implementation
with comprehensive logging capabilities.
"""

from typing import TYPE_CHECKING, Optional

if TYPE_CHECKING:
    pass

try:
    from .pyg_engine_native import Engine as _RustEngine
except ImportError as e:
    raise ImportError(
        "Failed to import pyg_engine_native. "
        "The Rust extension may not be compiled. "
        "Run 'pip install -e .' to build the extension."
    ) from e


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
    
    @property
    def version(self) -> str:
        """
        Get the engine version.
        
        Returns:
            The version string (e.g., "1.2.0").
        """
        return self._engine.version
