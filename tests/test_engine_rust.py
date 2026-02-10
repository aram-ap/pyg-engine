"""
Test suite for the Rust-based engine functionality.

This module tests the core Rust bindings including the tracing-based logging
system and version information.

Note: The tracing library writes directly to stdout in a way that bypasses
Python's stdout capture, so we test for basic functionality rather than
captured output.
"""

import tempfile
from pathlib import Path
from typing import TYPE_CHECKING

import pytest

if TYPE_CHECKING:
    pass

import pyg_engine as pyg


def test_engine_creation() -> None:
    """
    Test that an Engine instance can be created successfully.
    """
    engine = pyg.Engine()
    assert engine is not None
    assert isinstance(engine, pyg.Engine)


def test_engine_version() -> None:
    """
    Test that the engine version is correctly exposed.
    """
    engine = pyg.Engine()
    version = engine.version
    
    assert isinstance(version, str)
    assert version == "1.2.0"


def test_engine_log_methods_exist() -> None:
    """
    Test that all log methods are accessible and callable.
    """
    engine = pyg.Engine()
    
    # Verify all log methods exist
    assert hasattr(engine, "log")
    assert hasattr(engine, "log_trace")
    assert hasattr(engine, "log_debug")
    assert hasattr(engine, "log_info")
    assert hasattr(engine, "log_warn")
    assert hasattr(engine, "log_error")
    
    # Verify they're callable
    assert callable(engine.log)
    assert callable(engine.log_trace)
    assert callable(engine.log_debug)
    assert callable(engine.log_info)
    assert callable(engine.log_warn)
    assert callable(engine.log_error)


def test_engine_render_api_methods_exist() -> None:
    """
    Test that new rendering control APIs are exposed on Python Engine.
    """
    engine = pyg.Engine()

    assert hasattr(engine, "run")
    assert hasattr(engine, "start_manual")
    assert hasattr(engine, "set_window_icon")
    assert hasattr(engine, "add_game_object")
    assert hasattr(engine, "create_game_object")
    assert hasattr(engine, "remove_game_object")
    assert hasattr(engine, "clear_draw_commands")
    assert hasattr(engine, "draw_pixel")
    assert hasattr(engine, "draw_line")
    assert hasattr(engine, "draw_rectangle")
    assert hasattr(engine, "draw_circle")
    assert not hasattr(engine, "run_with_update")
    assert not hasattr(engine, "initialize")


def test_set_window_icon_invalid_path_raises() -> None:
    """Test set_window_icon raises for missing icon files."""
    engine = pyg.Engine()
    with pytest.raises(RuntimeError):
        engine.set_window_icon("images/does_not_exist.png")


def test_start_manual_guard_raises_when_already_running() -> None:
    """Test that start_manual fails fast when engine is already running."""
    engine = pyg.Engine()
    engine._runtime_state = "manual"  # type: ignore[attr-defined]

    with pytest.raises(RuntimeError, match="Cannot call start_manual\\(\\)"):
        engine.start_manual()


def test_run_guard_raises_when_already_running() -> None:
    """Test that run fails fast when engine is already running."""
    engine = pyg.Engine()
    engine._runtime_state = "running_callback"  # type: ignore[attr-defined]

    with pytest.raises(RuntimeError, match="Cannot call run\\(\\)"):
        engine.run()


def test_is_running_reflects_runtime_state() -> None:
    """Test Engine.is_running reflects lifecycle state transitions."""
    engine = pyg.Engine()
    assert engine.is_running is False

    engine._runtime_state = "manual"  # type: ignore[attr-defined]
    assert engine.is_running is True

    engine._runtime_state = "idle"  # type: ignore[attr-defined]
    assert engine.is_running is False


def test_update_context_is_exposed() -> None:
    """Test that UpdateContext is importable from the top-level module."""
    assert hasattr(pyg, "UpdateContext")


def test_update_callback_compiler_supports_named_injection() -> None:
    """Test callback argument injection without creating a window."""
    from pyg_engine.engine import UpdateContext, _compile_update_callback

    engine = pyg.Engine()
    context = UpdateContext(engine, user_data={"speed": 4.0})
    context.delta_time = 0.016
    context.elapsed_time = 2.5
    context.frame = 10

    observed: dict[str, object] = {}

    def callback(dt: float, engine: pyg.Engine, frame: int, user_data: object) -> None:
        observed["dt"] = dt
        observed["engine"] = engine
        observed["frame"] = frame
        observed["user_data"] = user_data

    invoker = _compile_update_callback(callback)
    invoker(context)

    assert observed["dt"] == 0.016
    assert observed["engine"] is engine
    assert observed["frame"] == 10
    assert observed["user_data"] == {"speed": 4.0}


def test_update_callback_compiler_single_arg_falls_back_to_context() -> None:
    """Test single unknown callback argument receives UpdateContext."""
    from pyg_engine.engine import UpdateContext, _compile_update_callback

    engine = pyg.Engine()
    context = UpdateContext(engine)

    received: list[object] = []

    def callback(state: object) -> None:
        received.append(state)

    invoker = _compile_update_callback(callback)
    invoker(context)

    assert received
    assert received[0] is context


def test_engine_log_info_no_crash() -> None:
    """
    Test that log_info doesn't crash.
    """
    engine = pyg.Engine()
    # Should not raise an exception
    engine.log_info("Test INFO message")


def test_engine_log_default_no_crash() -> None:
    """
    Test that default log() method doesn't crash.
    """
    engine = pyg.Engine()
    engine.log("Test default log message")


def test_engine_log_warn_no_crash() -> None:
    """
    Test that log_warn doesn't crash.
    """
    engine = pyg.Engine()
    engine.log_warn("Test WARN message")


def test_engine_log_error_no_crash() -> None:
    """
    Test that log_error doesn't crash.
    """
    engine = pyg.Engine()
    engine.log_error("Test ERROR message")


def test_engine_log_debug_no_crash() -> None:
    """
    Test that log_debug doesn't crash.
    """
    engine = pyg.Engine(log_level="DEBUG")
    engine.log_debug("Test DEBUG message")


def test_engine_log_trace_no_crash() -> None:
    """
    Test that log_trace doesn't crash.
    """
    engine = pyg.Engine(log_level="TRACE")
    engine.log_trace("Test TRACE message")


def test_engine_log_multiple_messages() -> None:
    """
    Test that multiple log messages at different levels work.
    """
    engine = pyg.Engine()
    
    # Should not raise exceptions
    engine.log_info("First message")
    engine.log_warn("Second message")
    engine.log_error("Third message with special chars: !@#$%")


def test_engine_log_empty_string() -> None:
    """
    Test that logging an empty string doesn't crash.
    """
    engine = pyg.Engine()
    engine.log_info("")  # Should not crash


def test_multiple_engine_instances() -> None:
    """
    Test that multiple Engine instances can coexist.
    
    Note: The logging system is initialized once globally, but multiple
    engines can share it.
    """
    engine1 = pyg.Engine()
    engine2 = pyg.Engine()
    
    assert engine1 is not engine2
    assert engine1.version == engine2.version
    assert engine1.version == "1.2.0"


def test_engine_version_property_immutable() -> None:
    """
    Test that the version property is read-only.
    """
    engine = pyg.Engine()
    
    with pytest.raises(AttributeError):
        engine.version = "9.9.9"  # type: ignore


def test_engine_with_file_logging() -> None:
    """
    Test that engine can be initialized with file logging enabled.
    
    Note: Since logging is initialized once globally, this test verifies
    that the engine accepts file logging parameters without crashing.
    """
    with tempfile.TemporaryDirectory() as tmpdir:
        log_dir = str(Path(tmpdir) / "test_logs")
        
        # Should not crash even if logging is already initialized
        engine = pyg.Engine(
            enable_file_logging=True,
            log_directory=log_dir,
            log_level="INFO"
        )
        
        # Log some messages (should not crash)
        engine.log_info("Test file logging")
        engine.log_warn("Warning in file")
        
        # Verify engine was created successfully
        assert engine.version == "1.2.0"


def test_engine_with_custom_log_level() -> None:
    """
    Test that engine can be initialized with custom log levels.
    """
    # These should not crash with different log levels
    engine_info = pyg.Engine(log_level="INFO")
    engine_warn = pyg.Engine(log_level="WARN")
    engine_debug = pyg.Engine(log_level="DEBUG")
    engine_trace = pyg.Engine(log_level="TRACE")
    
    # All should be valid
    assert engine_info.version == "1.2.0"
    assert engine_warn.version == "1.2.0"
    assert engine_debug.version == "1.2.0"
    assert engine_trace.version == "1.2.0"


def test_engine_initialization_variants() -> None:
    """
    Test different ways to initialize the engine.
    """
    # Default initialization
    engine1 = pyg.Engine()
    assert engine1.version == "1.2.0"
    
    # With log level only
    engine2 = pyg.Engine(log_level="DEBUG")
    assert engine2.version == "1.2.0"
    
    # With file logging
    with tempfile.TemporaryDirectory() as tmpdir:
        engine3 = pyg.Engine(
            enable_file_logging=True,
            log_directory=str(tmpdir)
        )
        assert engine3.version == "1.2.0"


def test_engine_with_invalid_log_level_defaults() -> None:
    """
    Test that invalid log levels default to INFO.
    """
    # Invalid log level should default to INFO without crashing
    engine = pyg.Engine(log_level="INVALID")
    assert engine.version == "1.2.0"
    engine.log_info("This should work with default INFO level")
