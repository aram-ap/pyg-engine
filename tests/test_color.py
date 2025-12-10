"""
Unit tests for the Color class.

Tests color creation, manipulation, clamping, and color constants.
"""

from typing import TYPE_CHECKING

import pytest

from pyg.types.color import Color

if TYPE_CHECKING:
    from _pytest.fixtures import FixtureRequest


@pytest.mark.unit
class TestColorCreation:
    """Test Color initialization and creation."""

    def test_create_color_with_rgb(self) -> None:
        """Test creating a color with RGB values."""
        color = Color(255, 128, 64)
        assert color.r == 255
        assert color.g == 128
        assert color.b == 64
        assert color.a == 255  # Default alpha

    def test_create_color_with_rgba(self) -> None:
        """Test creating a color with RGBA values."""
        color = Color(255, 128, 64, 200)
        assert color.r == 255
        assert color.g == 128
        assert color.b == 64
        assert color.a == 200

    def test_color_clamping_upper_bound(self) -> None:
        """Test that color values are clamped to maximum of 255."""
        color = Color(300, 400, 500, 600)
        assert color.r == 255
        assert color.g == 255
        assert color.b == 255
        assert color.a == 255

    def test_color_clamping_lower_bound(self) -> None:
        """Test that color values are clamped to minimum of 0."""
        color = Color(-10, -20, -30, -40)
        assert color.r == 0
        assert color.g == 0
        assert color.b == 0
        assert color.a == 0

    def test_color_float_conversion(self) -> None:
        """Test that float values are converted to integers."""
        color = Color(127.8, 64.2, 32.9)
        assert isinstance(color.r, int)
        assert isinstance(color.g, int)
        assert isinstance(color.b, int)


@pytest.mark.unit
class TestColorProperties:
    """Test Color property setters and getters."""

    def test_set_red_component(self) -> None:
        """Test setting the red component."""
        color = Color(0, 0, 0)
        color.r = 200
        assert color.r == 200

    def test_set_green_component(self) -> None:
        """Test setting the green component."""
        color = Color(0, 0, 0)
        color.g = 150
        assert color.g == 150

    def test_set_blue_component(self) -> None:
        """Test setting the blue component."""
        color = Color(0, 0, 0)
        color.b = 100
        assert color.b == 100

    def test_set_alpha_component(self) -> None:
        """Test setting the alpha component."""
        color = Color(0, 0, 0)
        color.a = 128
        assert color.a == 128

    def test_setter_clamping(self) -> None:
        """Test that setters also apply clamping."""
        color = Color(128, 128, 128)
        color.r = 300
        color.g = -10
        assert color.r == 255
        assert color.g == 0


@pytest.mark.unit
class TestColorConstants:
    """Test predefined color constants."""

    def test_white_color(self) -> None:
        """Test WHITE color constant."""
        assert Color.WHITE.r == 255
        assert Color.WHITE.g == 255
        assert Color.WHITE.b == 255
        assert Color.WHITE.a == 255

    def test_black_color(self) -> None:
        """Test BLACK color constant."""
        assert Color.BLACK.r == 0
        assert Color.BLACK.g == 0
        assert Color.BLACK.b == 0
        assert Color.BLACK.a == 255

    def test_red_color(self) -> None:
        """Test RED color constant."""
        assert Color.RED.r == 255
        assert Color.RED.g == 0
        assert Color.RED.b == 0
        assert Color.RED.a == 255

    def test_green_color(self) -> None:
        """Test GREEN color constant."""
        assert Color.GREEN.r == 0
        assert Color.GREEN.g == 255
        assert Color.GREEN.b == 0
        assert Color.GREEN.a == 255

    def test_blue_color(self) -> None:
        """Test BLUE color constant."""
        assert Color.BLUE.r == 0
        assert Color.BLUE.g == 0
        assert Color.BLUE.b == 255
        assert Color.BLUE.a == 255

    def test_transparent_color(self) -> None:
        """Test TRANSPARENT color constant."""
        assert Color.TRANSPARENT.a == 0


@pytest.mark.unit
class TestColorEdgeCases:
    """Test edge cases and boundary conditions."""

    def test_all_zero_color(self) -> None:
        """Test creating a fully transparent black color."""
        color = Color(0, 0, 0, 0)
        assert color.r == 0
        assert color.g == 0
        assert color.b == 0
        assert color.a == 0

    def test_all_max_color(self) -> None:
        """Test creating a fully opaque white color."""
        color = Color(255, 255, 255, 255)
        assert color.r == 255
        assert color.g == 255
        assert color.b == 255
        assert color.a == 255

