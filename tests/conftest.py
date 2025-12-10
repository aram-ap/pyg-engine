"""
Pytest configuration and shared fixtures.

This module provides shared fixtures and configuration for all tests.
"""

from typing import TYPE_CHECKING, Any, Generator

import pytest

if TYPE_CHECKING:
    from _pytest.config import Config
    from _pytest.config.argparsing import Parser
    from _pytest.fixtures import FixtureRequest


def pytest_configure(config: "Config") -> None:
    """
    Configure pytest with custom settings.

    Args:
        config: Pytest configuration object
    """
    config.addinivalue_line("markers", "unit: Unit tests")
    config.addinivalue_line("markers", "integration: Integration tests")
    config.addinivalue_line("markers", "slow: Slow running tests")
    config.addinivalue_line("markers", "cpp_binding: C++ binding tests")


def pytest_addoption(parser: "Parser") -> None:
    """
    Add custom command line options.

    Args:
        parser: Pytest argument parser
    """
    parser.addoption(
        "--run-slow",
        action="store_true",
        default=False,
        help="Run slow tests",
    )


def pytest_collection_modifyitems(config: "Config", items: list[Any]) -> None:
    """
    Modify test collection to handle custom markers.

    Args:
        config: Pytest configuration object
        items: List of collected test items
    """
    if config.getoption("--run-slow"):
        # --run-slow given in cli: do not skip slow tests
        return

    skip_slow = pytest.mark.skip(reason="need --run-slow option to run")
    for item in items:
        if "slow" in item.keywords:
            item.add_marker(skip_slow)


@pytest.fixture(scope="session")
def engine_initialized() -> Generator[bool, None, None]:
    """
    Initialize the game engine for testing.

    Yields:
        bool: True if engine is initialized successfully
    """
    # Setup: Initialize engine if needed
    initialized = True

    yield initialized

    # Teardown: Cleanup engine resources


@pytest.fixture
def temp_game_object() -> Generator[dict[str, Any], None, None]:
    """
    Create a temporary game object for testing.

    Yields:
        dict: A temporary game object dictionary
    """
    game_obj = {
        "id": "test_object",
        "position": (0, 0),
        "active": True,
    }

    yield game_obj

    # Cleanup if necessary

