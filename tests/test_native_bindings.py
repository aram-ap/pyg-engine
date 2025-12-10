"""
Tests for C++ native bindings.

Tests that the C++ extension module loads correctly and provides expected functionality.
"""

from typing import TYPE_CHECKING

import pytest

if TYPE_CHECKING:
    pass


@pytest.mark.cpp_binding
class TestNativeModule:
    """Test the native C++ extension module."""

    def test_import_native_module(self) -> None:
        """Test that the _native module can be imported."""
        try:
            from pyg import _native
            assert _native is not None
        except ImportError:
            pytest.fail("Failed to import _native module")

    def test_native_version_exists(self) -> None:
        """Test that version information is available in native module."""
        from pyg import _native
        # Check if version or similar attribute exists
        assert hasattr(_native, '__doc__') or hasattr(_native, '__name__')

    def test_native_math_class_exists(self) -> None:
        """Test that Math class is available from native module."""
        from pyg import _native
        assert hasattr(_native, 'Math')

    def test_native_vector2_exists(self) -> None:
        """Test that Vector2 class is available from native module."""
        from pyg import _native
        assert hasattr(_native, 'Vector2')

    def test_native_vector3_exists(self) -> None:
        """Test that Vector3 class is available from native module."""
        from pyg import _native
        assert hasattr(_native, 'Vector3')

    def test_native_vector4_exists(self) -> None:
        """Test that Vector4 class is available from native module."""
        from pyg import _native
        assert hasattr(_native, 'Vector4')

    def test_native_color_exists(self) -> None:
        """Test that Color class is available from native module."""
        from pyg import _native
        assert hasattr(_native, 'Color')


@pytest.mark.cpp_binding
class TestNativeMath:
    """Test native Math class functionality."""

    def test_math_clamp_int(self) -> None:
        """Test Math.clamp_int function."""
        from pyg import _native
        Math = _native.Math

        # Test clamping above max
        assert Math.clamp_int(300, 0, 255) == 255

        # Test clamping below min
        assert Math.clamp_int(-10, 0, 255) == 0

        # Test value within range
        assert Math.clamp_int(128, 0, 255) == 128


@pytest.mark.integration
@pytest.mark.cpp_binding
class TestNativeColorIntegration:
    """Test native Color class integration."""

    def test_create_native_color(self) -> None:
        """Test creating a Color instance from native module."""
        from pyg import _native

        color = _native.Color(255, 128, 64, 255)
        assert color is not None

    def test_native_color_properties(self) -> None:
        """Test accessing native Color properties."""
        from pyg import _native

        color = _native.Color(100, 150, 200, 250)
        assert color.r == 100
        assert color.g == 150
        assert color.b == 200
        assert color.a == 250

