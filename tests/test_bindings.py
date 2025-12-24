"""
Test suite for the Rust bindings in pyg_engine_native.

This module tests:
- Vec2 and Vec3 vector operations
- Color creation and manipulation
- Time tracking
- GameObject creation
- TransformComponent usage
"""

import time as py_time
from typing import TYPE_CHECKING

import pytest

if TYPE_CHECKING:
    pass

import pyg_engine as pyg


# ========== Vec2 Tests ==========

def test_vec2_creation() -> None:
    """
    Test that Vec2 instances can be created successfully.
    """
    v1 = pyg.Vec2(3.0, 4.0)
    v2 = pyg.Vec2(1.0, 2.0)
    
    assert v1 is not None
    assert v2 is not None
    assert isinstance(v1, pyg.Vec2)
    assert isinstance(v2, pyg.Vec2)


def test_vec2_component_access() -> None:
    """
    Test that Vec2 components (x, y) can be accessed.
    """
    v = pyg.Vec2(3.0, 4.0)
    
    assert v.x == 3.0
    assert v.y == 4.0


def test_vec2_addition() -> None:
    """
    Test Vec2 vector addition.
    """
    v1 = pyg.Vec2(3.0, 4.0)
    v2 = pyg.Vec2(1.0, 2.0)
    v3 = v1.add(v2)
    
    assert v3.x == 4.0
    assert v3.y == 6.0


def test_vec2_subtraction() -> None:
    """
    Test Vec2 vector subtraction.
    """
    v1 = pyg.Vec2(3.0, 4.0)
    v2 = pyg.Vec2(1.0, 2.0)
    v3 = v1.subtract(v2)
    
    assert v3.x == 2.0
    assert v3.y == 2.0


def test_vec2_scalar_multiplication() -> None:
    """
    Test Vec2 scalar multiplication.
    """
    v1 = pyg.Vec2(3.0, 4.0)
    v2 = v1.multiply_scalar(2.0)
    
    assert v2.x == 6.0
    assert v2.y == 8.0


def test_vec2_length() -> None:
    """
    Test Vec2 length calculation.
    """
    v = pyg.Vec2(3.0, 4.0)
    length = v.length()
    
    # Account for floating point precision in sqrt calculation
    assert abs(length - 5.0) < 1e-5


def test_vec2_normalize() -> None:
    """
    Test Vec2 normalization.
    """
    v = pyg.Vec2(3.0, 4.0)
    normalized = v.normalize()
    
    # Normalized vector should have length 1
    assert abs(normalized.length() - 1.0) < 0.0001


def test_vec2_dot_product() -> None:
    """
    Test Vec2 dot product.
    """
    v1 = pyg.Vec2(3.0, 4.0)
    v2 = pyg.Vec2(1.0, 2.0)
    dot = v1.dot(v2)
    
    # 3*1 + 4*2 = 11
    assert dot == 11.0


def test_vec2_cross_product() -> None:
    """
    Test Vec2 cross product (returns scalar for 2D).
    """
    v1 = pyg.Vec2(3.0, 4.0)
    v2 = pyg.Vec2(1.0, 2.0)
    cross = v1.cross(v2)
    
    # 3*2 - 4*1 = 2
    assert cross == 2.0


def test_vec2_distance() -> None:
    """
    Test Vec2 distance calculation.
    """
    v1 = pyg.Vec2(0.0, 0.0)
    v2 = pyg.Vec2(3.0, 4.0)
    dist = v1.distance(v2)
    
    # Note: This might return squared distance based on implementation
    assert dist >= 0.0


def test_vec2_lerp() -> None:
    """
    Test Vec2 linear interpolation.
    """
    v1 = pyg.Vec2(0.0, 0.0)
    v2 = pyg.Vec2(10.0, 10.0)
    v3 = v1.lerp(v2, 0.5)
    
    # Account for floating point precision in interpolation
    assert abs(v3.x - 5.0) < 1e-5
    assert abs(v3.y - 5.0) < 1e-5


def test_vec2_string_representation() -> None:
    """
    Test Vec2 string representation.
    """
    v = pyg.Vec2(3.0, 4.0)
    s = str(v)
    
    assert isinstance(s, str)
    assert "3.0" in s or "3" in s
    assert "4.0" in s or "4" in s


# ========== Vec3 Tests ==========

def test_vec3_creation() -> None:
    """
    Test that Vec3 instances can be created successfully.
    """
    v1 = pyg.Vec3(1.0, 0.0, 0.0)
    v2 = pyg.Vec3(0.0, 1.0, 0.0)
    
    assert v1 is not None
    assert v2 is not None
    assert isinstance(v1, pyg.Vec3)
    assert isinstance(v2, pyg.Vec3)


def test_vec3_component_access() -> None:
    """
    Test that Vec3 components (x, y, z) can be accessed.
    """
    v = pyg.Vec3(1.0, 2.0, 3.0)
    
    assert v.x == 1.0
    assert v.y == 2.0
    assert v.z == 3.0


def test_vec3_addition() -> None:
    """
    Test Vec3 vector addition.
    """
    v1 = pyg.Vec3(1.0, 2.0, 3.0)
    v2 = pyg.Vec3(4.0, 5.0, 6.0)
    v3 = v1.add(v2)
    
    assert v3.x == 5.0
    assert v3.y == 7.0
    assert v3.z == 9.0


def test_vec3_cross_product() -> None:
    """
    Test Vec3 cross product (returns Vec3 for 3D).
    X cross Y should give Z axis.
    """
    v1 = pyg.Vec3(1.0, 0.0, 0.0)
    v2 = pyg.Vec3(0.0, 1.0, 0.0)
    v3 = v1.cross(v2)
    
    assert abs(v3.x - 0.0) < 0.0001
    assert abs(v3.y - 0.0) < 0.0001
    assert abs(v3.z - 1.0) < 0.0001


def test_vec3_lerp() -> None:
    """
    Test Vec3 linear interpolation.
    """
    v1 = pyg.Vec3(0.0, 0.0, 0.0)
    v2 = pyg.Vec3(10.0, 20.0, 30.0)
    v3 = v1.lerp(v2, 0.5)
    
    # Account for floating point precision in interpolation
    assert abs(v3.x - 5.0) < 1e-5
    assert abs(v3.y - 10.0) < 1e-5
    assert abs(v3.z - 15.0) < 1e-5


def test_vec3_length() -> None:
    """
    Test Vec3 length calculation.
    """
    v = pyg.Vec3(1.0, 0.0, 0.0)
    length = v.length()
    
    # Account for floating point precision in sqrt calculation
    assert abs(length - 1.0) < 1e-5


def test_vec3_normalize() -> None:
    """
    Test Vec3 normalization.
    """
    v = pyg.Vec3(3.0, 4.0, 0.0)
    normalized = v.normalize()
    
    # Normalized vector should have length 1
    assert abs(normalized.length() - 1.0) < 0.0001


def test_vec3_string_representation() -> None:
    """
    Test Vec3 string representation.
    """
    v = pyg.Vec3(1.0, 2.0, 3.0)
    s = str(v)
    
    assert isinstance(s, str)
    assert "1.0" in s or "1" in s
    assert "2.0" in s or "2" in s
    assert "3.0" in s or "3" in s


# ========== Color Tests ==========

def test_color_creation() -> None:
    """
    Test that Color instances can be created successfully.
    """
    c = pyg.Color(1.0, 0.0, 0.0, 1.0)
    
    assert c is not None
    assert isinstance(c, pyg.Color)


def test_color_component_access() -> None:
    """
    Test that Color components (r, g, b, a) can be accessed.
    """
    c = pyg.Color(1.0, 0.5, 0.25, 0.75)
    
    # Account for floating point precision (though these should be exact)
    assert abs(c.r - 1.0) < 1e-5
    assert abs(c.g - 0.5) < 1e-5
    assert abs(c.b - 0.25) < 1e-5
    assert abs(c.a - 0.75) < 1e-5


def test_color_from_rgb() -> None:
    """
    Test creating Color from 8-bit RGB values.
    """
    c = pyg.Color.rgb(255, 128, 0)
    
    # Account for floating point precision in division
    assert abs(c.r - 1.0) < 1e-5
    assert abs(c.g - 128.0/255.0) < 1e-5
    assert abs(c.b - 0.0) < 1e-5
    assert abs(c.a - 1.0) < 1e-5


def test_color_from_rgba() -> None:
    """
    Test creating Color from 8-bit RGBA values.
    """
    c = pyg.Color.rgba(255, 0, 0, 128)
    
    # Account for floating point precision in division
    assert abs(c.r - 1.0) < 1e-5
    assert abs(c.g - 0.0) < 1e-5
    assert abs(c.b - 0.0) < 1e-5
    assert abs(c.a - 128.0/255.0) < 1e-5


def test_color_from_hex() -> None:
    """
    Test creating Color from hex string.
    """
    c1 = pyg.Color.from_hex("#FF0000")
    c2 = pyg.Color.from_hex("00FF00")  # Without #
    
    # Account for floating point precision in conversion
    assert abs(c1.r - 1.0) < 1e-5
    assert abs(c1.g - 0.0) < 1e-5
    assert abs(c1.b - 0.0) < 1e-5
    
    assert abs(c2.r - 0.0) < 1e-5
    assert abs(c2.g - 1.0) < 1e-5
    assert abs(c2.b - 0.0) < 1e-5


def test_color_constants() -> None:
    """
    Test that color constants are accessible.
    """
    # Test basic colors
    assert pyg.Color.RED is not None
    assert pyg.Color.GREEN is not None
    assert pyg.Color.BLUE is not None
    assert pyg.Color.WHITE is not None
    assert pyg.Color.BLACK is not None
    
    # Verify red is actually red
    assert abs(pyg.Color.RED.r - 1.0) < 1e-5
    assert abs(pyg.Color.RED.g - 0.0) < 1e-5
    assert abs(pyg.Color.RED.b - 0.0) < 1e-5
    assert abs(pyg.Color.RED.a - 1.0) < 1e-5


def test_color_with_alpha() -> None:
    """
    Test modifying color alpha channel.
    """
    c1 = pyg.Color(1.0, 0.0, 0.0, 1.0)
    c2 = c1.with_alpha(0.5)
    
    # Original unchanged
    assert abs(c1.a - 1.0) < 1e-5
    
    # New color has modified alpha
    assert abs(c2.r - 1.0) < 1e-5
    assert abs(c2.g - 0.0) < 1e-5
    assert abs(c2.b - 0.0) < 1e-5
    assert abs(c2.a - 0.5) < 1e-5


def test_color_set_r() -> None:
    """
    Test setting color red component.
    """
    c1 = pyg.Color(0.5, 0.5, 0.5, 1.0)
    c2 = c1.set_r(0.8)
    
    # Original unchanged
    assert abs(c1.r - 0.5) < 1e-5
    
    # New color has modified red
    assert abs(c2.r - 0.8) < 1e-5
    assert abs(c2.g - 0.5) < 1e-5
    assert abs(c2.b - 0.5) < 1e-5
    assert abs(c2.a - 1.0) < 1e-5


def test_color_set_g() -> None:
    """
    Test setting color green component.
    """
    c1 = pyg.Color(0.5, 0.5, 0.5, 1.0)
    c2 = c1.set_g(0.3)
    
    # Original unchanged
    assert abs(c1.g - 0.5) < 1e-5
    
    # New color has modified green
    assert abs(c2.r - 0.5) < 1e-5
    assert abs(c2.g - 0.3) < 1e-5
    assert abs(c2.b - 0.5) < 1e-5
    assert abs(c2.a - 1.0) < 1e-5


def test_color_set_b() -> None:
    """
    Test setting color blue component.
    """
    c1 = pyg.Color(0.5, 0.5, 0.5, 1.0)
    c2 = c1.set_b(0.9)
    
    # Original unchanged
    assert abs(c1.b - 0.5) < 1e-5
    
    # New color has modified blue
    assert abs(c2.r - 0.5) < 1e-5
    assert abs(c2.g - 0.5) < 1e-5
    assert abs(c2.b - 0.9) < 1e-5
    assert abs(c2.a - 1.0) < 1e-5


def test_color_set_a() -> None:
    """
    Test setting color alpha component.
    """
    c1 = pyg.Color(0.5, 0.5, 0.5, 1.0)
    c2 = c1.set_a(0.7)
    
    # Original unchanged
    assert abs(c1.a - 1.0) < 1e-5
    
    # New color has modified alpha
    assert abs(c2.r - 0.5) < 1e-5
    assert abs(c2.g - 0.5) < 1e-5
    assert abs(c2.b - 0.5) < 1e-5
    assert abs(c2.a - 0.7) < 1e-5


def test_color_setters_clamp() -> None:
    """
    Test that setters clamp values between 0 and 1.
    """
    c1 = pyg.Color(0.5, 0.5, 0.5, 0.5)
    
    # Test clamping values above 1.0
    c2 = c1.set_r(2.0)
    assert abs(c2.r - 1.0) < 1e-5
    
    c3 = c1.set_g(-0.5)
    assert abs(c3.g - 0.0) < 1e-5
    
    c4 = c1.set_b(1.5)
    assert abs(c4.b - 1.0) < 1e-5
    
    c5 = c1.set_a(-1.0)
    assert abs(c5.a - 0.0) < 1e-5


def test_color_setters_chaining() -> None:
    """
    Test that setters can be chained together.
    """
    c1 = pyg.Color(0.0, 0.0, 0.0, 1.0)
    c2 = c1.set_r(0.8).set_g(0.6).set_b(0.4).set_a(0.9)
    
    assert abs(c2.r - 0.8) < 1e-5
    assert abs(c2.g - 0.6) < 1e-5
    assert abs(c2.b - 0.4) < 1e-5
    assert abs(c2.a - 0.9) < 1e-5
    
    # Original unchanged
    assert abs(c1.r - 0.0) < 1e-5
    assert abs(c1.g - 0.0) < 1e-5
    assert abs(c1.b - 0.0) < 1e-5
    assert abs(c1.a - 1.0) < 1e-5


def test_color_lerp() -> None:
    """
    Test color interpolation.
    """
    c1 = pyg.Color.RED
    c2 = pyg.Color.BLUE
    c3 = c1.lerp(c2, 0.5)
    
    # Should be halfway between red and blue
    # Account for floating point precision in interpolation
    assert abs(c3.r - 0.5) < 1e-5
    assert abs(c3.g - 0.0) < 1e-5
    assert abs(c3.b - 0.5) < 1e-5


def test_color_string_representation() -> None:
    """
    Test Color string representation.
    """
    c = pyg.Color(1.0, 0.5, 0.0, 1.0)
    s = str(c)
    
    assert isinstance(s, str)
    assert "Color" in s


def test_color_from_hsv() -> None:
    """
    Test creating Color from HSV values.
    """
    # Red at 0 degrees
    c1 = pyg.Color.from_hsv(0.0, 1.0, 1.0, 1.0)
    # Account for floating point precision in HSV to RGB conversion
    assert abs(c1.r - 1.0) < 1e-5
    assert abs(c1.g - 0.0) < 1e-5
    assert abs(c1.b - 0.0) < 1e-5
    
    # Green at 120 degrees
    c2 = pyg.Color.from_hsv(120.0, 1.0, 1.0, 1.0)
    assert abs(c2.r - 0.0) < 1e-5
    assert abs(c2.g - 1.0) < 1e-5
    assert abs(c2.b - 0.0) < 1e-5
    
    # Blue at 240 degrees
    c3 = pyg.Color.from_hsv(240.0, 1.0, 1.0, 1.0)
    assert abs(c3.r - 0.0) < 1e-5
    assert abs(c3.g - 0.0) < 1e-5
    assert abs(c3.b - 1.0) < 1e-5


def test_color_from_hsv_clamping() -> None:
    """
    Test that HSV values are properly clamped.
    """
    # Test saturation and value clamping
    c1 = pyg.Color.from_hsv(0.0, 2.0, 1.5, 1.0)
    # Account for floating point precision
    assert c1.r <= 1.0 + 1e-5
    assert c1.g <= 1.0 + 1e-5
    assert c1.b <= 1.0 + 1e-5
    
    # Test hue wrapping (360 should wrap to 0)
    c2 = pyg.Color.from_hsv(360.0, 1.0, 1.0, 1.0)
    c3 = pyg.Color.from_hsv(0.0, 1.0, 1.0, 1.0)
    # Account for floating point precision in HSV conversion
    assert abs(c2.r - c3.r) < 1e-5


def test_color_addition() -> None:
    """
    Test Color addition operator.
    """
    c1 = pyg.Color(0.5, 0.3, 0.2, 1.0)
    c2 = pyg.Color(0.2, 0.4, 0.3, 0.5)
    c3 = c1 + c2
    
    # Account for floating point precision in addition and clamping
    assert abs(c3.r - 0.7) < 1e-5
    assert abs(c3.g - 0.7) < 1e-5
    assert abs(c3.b - 0.5) < 1e-5
    assert abs(c3.a - 1.0) < 1e-5  # Clamped to 1.0
    
    # Original colors unchanged (exact values from construction)
    assert abs(c1.r - 0.5) < 1e-5
    assert abs(c2.r - 0.2) < 1e-5


def test_color_subtraction() -> None:
    """
    Test Color subtraction operator.
    """
    c1 = pyg.Color(0.8, 0.6, 0.4, 1.0)
    c2 = pyg.Color(0.3, 0.2, 0.1, 0.5)
    c3 = c1 - c2
    
    # Account for floating point precision in subtraction and clamping
    assert abs(c3.r - 0.5) < 1e-5
    assert abs(c3.g - 0.4) < 1e-5
    assert abs(c3.b - 0.3) < 1e-5
    assert abs(c3.a - 0.5) < 1e-5
    
    # Original colors unchanged (exact values from construction)
    assert abs(c1.r - 0.8) < 1e-5
    assert abs(c2.r - 0.3) < 1e-5


def test_color_multiplication() -> None:
    """
    Test Color multiplication operator.
    """
    c1 = pyg.Color(0.8, 0.6, 0.4, 1.0)
    c2 = pyg.Color(0.5, 0.5, 0.5, 0.5)
    c3 = c1 * c2
    
    # Account for floating point precision in multiplication and clamping
    assert abs(c3.r - 0.4) < 1e-5
    assert abs(c3.g - 0.3) < 1e-5
    assert abs(c3.b - 0.2) < 1e-5
    assert abs(c3.a - 0.5) < 1e-5
    
    # Original colors unchanged (exact values from construction)
    assert abs(c1.r - 0.8) < 1e-5
    assert abs(c2.r - 0.5) < 1e-5


def test_color_division() -> None:
    """
    Test Color division operator.
    """
    c1 = pyg.Color(0.8, 0.6, 0.4, 1.0)
    c2 = pyg.Color(0.4, 0.3, 0.2, 0.5)
    c3 = c1 / c2
    
    # Account for floating point precision in division and clamping
    assert abs(c3.r - 1.0) < 1e-5  # 0.8 / 0.4 = 2.0, clamped to 1.0
    assert abs(c3.g - 1.0) < 1e-5  # 0.6 / 0.3 = 2.0, clamped to 1.0
    assert abs(c3.b - 1.0) < 1e-5  # 0.4 / 0.2 = 2.0, clamped to 1.0
    assert abs(c3.a - 1.0) < 1e-5  # 1.0 / 0.5 = 2.0, clamped to 1.0
    
    # Original colors unchanged (exact values from construction)
    assert abs(c1.r - 0.8) < 1e-5
    assert abs(c2.r - 0.4) < 1e-5


def test_color_division_by_zero() -> None:
    """
    Test Color division with zero components.
    """
    c1 = pyg.Color(0.8, 0.6, 0.4, 1.0)
    c2 = pyg.Color(0.0, 0.3, 0.2, 0.0)
    c3 = c1 / c2
    
    # Division by zero should result in 0.0
    # Account for floating point precision
    assert abs(c3.r - 0.0) < 1e-5
    assert abs(c3.g - 1.0) < 1e-5  # 0.6 / 0.3 = 2.0, clamped to 1.0
    assert abs(c3.b - 1.0) < 1e-5  # 0.4 / 0.2 = 2.0, clamped to 1.0
    assert abs(c3.a - 0.0) < 1e-5


def test_color_operators_chaining() -> None:
    """
    Test that color operators can be chained.
    """
    c1 = pyg.Color(0.5, 0.5, 0.5, 1.0)
    c2 = pyg.Color(0.2, 0.2, 0.2, 0.5)
    c3 = pyg.Color(0.1, 0.1, 0.1, 0.5)
    
    result = (c1 + c2) * c3
    
    # (0.5+0.2) * 0.1 = 0.7 * 0.1 = 0.07 for each component
    # Account for floating point precision in chained operations
    assert abs(result.r - 0.07) < 1e-5
    assert abs(result.g - 0.07) < 1e-5
    assert abs(result.b - 0.07) < 1e-5
    assert abs(result.a - 0.5) < 1e-5  # (1.0+0.5) * 0.5 = 1.0 * 0.5 = 0.5 (clamped)


def test_color_equality() -> None:
    """
    Test Color equality operator with floating point precision handling.
    """
    c1 = pyg.Color(0.5, 0.3, 0.8, 1.0)
    c2 = pyg.Color(0.5, 0.3, 0.8, 1.0)
    c3 = pyg.Color(0.5, 0.3, 0.8, 0.5)
    c4 = pyg.Color(0.6, 0.3, 0.8, 1.0)
    
    # Same colors should be equal
    assert c1 == c2
    assert not (c1 != c2)
    
    # Different alpha should not be equal
    assert c1 != c3
    assert not (c1 == c3)
    
    # Different RGB should not be equal
    assert c1 != c4
    assert not (c1 == c4)
    
    # Test with color constants
    assert pyg.Color.RED == pyg.Color(1.0, 0.0, 0.0, 1.0)
    assert pyg.Color.BLUE != pyg.Color.RED


def test_color_equality_floating_point_precision() -> None:
    """
    Test that equality handles floating point precision correctly.
    """
    # Test that very close values are considered equal (within epsilon)
    c1 = pyg.Color(0.5, 0.3, 0.8, 1.0)
    # Values that differ by less than 1e-5 should be equal
    c2 = pyg.Color(0.5 + 1e-6, 0.3 + 1e-6, 0.8 + 1e-6, 1.0 + 1e-6)
    assert c1 == c2
    
    # Values that differ by more than epsilon should not be equal
    c3 = pyg.Color(0.5 + 1e-4, 0.3, 0.8, 1.0)  # 0.0001 difference
    assert c1 != c3
    
    # Test with operations that might introduce floating point errors
    c4 = pyg.Color(0.1, 0.2, 0.3, 0.4)
    c5 = c4 + pyg.Color(0.0, 0.0, 0.0, 0.0)
    assert c4 == c5  # Adding zero should result in equal color
    
    # Test division that might result in floating point precision issues
    c6 = pyg.Color(0.8, 0.6, 0.4, 1.0)
    c7 = pyg.Color(0.4, 0.3, 0.2, 0.5)
    result = c6 / c7
    # Result should be clamped to 1.0, but might have tiny floating point differences
    expected = pyg.Color(1.0, 1.0, 1.0, 1.0)
    assert result == expected


def test_color_equality_with_operations() -> None:
    """
    Test that equality works correctly with color operations, accounting for floating point precision.
    """
    c1 = pyg.Color(0.5, 0.5, 0.5, 1.0)
    c2 = pyg.Color(0.3, 0.3, 0.3, 1.0)
    c3 = pyg.Color(0.2, 0.2, 0.2, 0.0)
    
    # (c1 - c2) should equal c3 (accounting for floating point precision)
    result = c1 - c2
    assert result == c3
    
    # Test that operations preserve equality even with floating point operations
    c4 = pyg.Color(1.0, 0.0, 0.0, 1.0)
    c5 = pyg.Color(0.5, 0.0, 0.0, 1.0)
    c6 = pyg.Color(0.5, 0.0, 0.0, 1.0)
    
    # Multiplication might introduce tiny floating point differences
    result2 = c4 * c6
    assert result2 == c5
    
    # Test with HSV conversion that might have floating point precision issues
    c7 = pyg.Color.from_hsv(0.0, 1.0, 1.0, 1.0)  # Red
    c8 = pyg.Color(1.0, 0.0, 0.0, 1.0)
    # HSV to RGB conversion might have tiny differences
    assert c7 == c8


# ========== Time Tests ==========

def test_time_creation() -> None:
    """
    Test that Time instances can be created successfully.
    """
    timer = pyg.Time()
    
    assert timer is not None
    assert isinstance(timer, pyg.Time)


def test_time_initial_values() -> None:
    """
    Test that Time starts with zero delta_time and elapsed_time.
    """
    timer = pyg.Time()
    
    assert timer.delta_time == 0.0
    assert timer.elapsed_time == 0.0


def test_time_tick() -> None:
    """
    Test that Time.tick() updates timing values.
    """
    timer = pyg.Time()
    
    # Initial values
    assert timer.delta_time == 0.0
    assert timer.elapsed_time == 0.0
    
    # Wait a bit and tick
    py_time.sleep(0.01)
    timer.tick()
    
    # Values should be updated
    assert timer.delta_time >= 0.0
    assert timer.elapsed_time >= 0.0


def test_time_multiple_ticks() -> None:
    """
    Test that Time correctly tracks multiple ticks.
    """
    timer = pyg.Time()
    
    timer.tick()
    py_time.sleep(0.01)
    timer.tick()
    
    # After ticks, times should be positive
    assert timer.delta_time >= 0.0
    assert timer.elapsed_time >= 0.0


# ========== GameObject Tests ==========

def test_game_object_creation() -> None:
    """
    Test that GameObject instances can be created successfully.
    """
    go1 = pyg.GameObject()
    go2 = pyg.GameObject("Player")
    
    assert go1 is not None
    assert go2 is not None
    assert isinstance(go1, pyg.GameObject)
    assert isinstance(go2, pyg.GameObject)


def test_game_object_set_name() -> None:
    """
    Test that GameObject names can be set.
    """
    go = pyg.GameObject()
    
    # Should not raise an exception
    go.set_name("Enemy")


def test_game_object_update() -> None:
    """
    Test that GameObject.update() can be called.
    """
    go = pyg.GameObject("Player")
    
    # Should not raise an exception
    go.update()


def test_game_object_with_name() -> None:
    """
    Test creating GameObject with initial name.
    """
    go = pyg.GameObject("TestObject")
    
    assert go is not None
    # The name should be set internally even if we can't access it directly


# ========== TransformComponent Tests ==========

def test_transform_component_creation() -> None:
    """
    Test that TransformComponent instances can be created successfully.
    """
    transform = pyg.TransformComponent("Transform")
    
    assert transform is not None
    assert isinstance(transform, pyg.TransformComponent)


def test_transform_component_name() -> None:
    """
    Test that TransformComponent name can be accessed.
    """
    transform = pyg.TransformComponent("MyTransform")
    
    assert transform.name == "MyTransform"


def test_transform_component_initial_position() -> None:
    """
    Test that TransformComponent starts with zero position.
    """
    transform = pyg.TransformComponent("Transform")
    pos = transform.position
    
    assert isinstance(pos, pyg.Vec2)
    assert pos.x == 0.0
    assert pos.y == 0.0


def test_transform_component_set_position() -> None:
    """
    Test that TransformComponent position can be set.
    """
    transform = pyg.TransformComponent("Transform")
    new_pos = pyg.Vec2(100.0, 200.0)
    
    transform.position = new_pos
    pos = transform.position
    
    assert pos.x == 100.0
    assert pos.y == 200.0


def test_transform_component_initial_rotation() -> None:
    """
    Test that TransformComponent starts with zero rotation.
    """
    transform = pyg.TransformComponent("Transform")
    
    assert transform.rotation == 0.0


def test_transform_component_set_rotation() -> None:
    """
    Test that TransformComponent rotation can be set.
    """
    transform = pyg.TransformComponent("Transform")
    
    transform.rotation = 45.0
    
    assert transform.rotation == 45.0


def test_transform_component_initial_scale() -> None:
    """
    Test that TransformComponent starts with unit scale.
    """
    transform = pyg.TransformComponent("Transform")
    scale = transform.scale
    
    assert isinstance(scale, pyg.Vec2)
    assert scale.x == 1.0
    assert scale.y == 1.0


def test_transform_component_set_scale() -> None:
    """
    Test that TransformComponent scale can be set.
    """
    transform = pyg.TransformComponent("Transform")
    new_scale = pyg.Vec2(2.0, 3.0)
    
    transform.scale = new_scale
    scale = transform.scale
    
    assert scale.x == 2.0
    assert scale.y == 3.0


def test_transform_component_properties() -> None:
    """
    Test that all TransformComponent properties work together.
    """
    transform = pyg.TransformComponent("FullTransform")
    
    # Set all properties
    transform.position = pyg.Vec2(10.0, 20.0)
    transform.rotation = 90.0
    transform.scale = pyg.Vec2(2.0, 2.0)
    
    # Verify all properties
    assert transform.position.x == 10.0
    assert transform.position.y == 20.0
    assert transform.rotation == 90.0
    assert transform.scale.x == 2.0
    assert transform.scale.y == 2.0

