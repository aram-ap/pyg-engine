"""
Tests for Vector2 class
"""

import pytest
import math
from pyg_engine import Vector2


def test_vector2_creation():
    """Test basic Vector2 creation"""
    v = Vector2(1.0, 2.0)
    assert v.x == 1.0
    assert v.y == 2.0


def test_vector2_mutability():
    """Test that Vector2 is mutable"""
    v = Vector2(1.0, 2.0)
    v.x = 5.0
    v.y = 10.0
    assert v.x == 5.0
    assert v.y == 10.0


def test_vector2_string_representation():
    """Test string representation"""
    v = Vector2(1.5, 2.5)
    assert str(v) == "(1.5, 2.5)"


def test_vector2_class_variables():
    """Test class variable constants"""
    assert Vector2.zero.x == 0.0 and Vector2.zero.y == 0.0
    assert Vector2.one.x == 1.0 and Vector2.one.y == 1.0
    assert Vector2.up.x == 0.0 and Vector2.up.y == 1.0
    assert Vector2.down.x == 0.0 and Vector2.down.y == -1.0
    assert Vector2.left.x == -1.0 and Vector2.left.y == 0.0
    assert Vector2.right.x == 1.0 and Vector2.right.y == 0.0


def test_vector2_addition():
    """Test vector addition"""
    v1 = Vector2(1.0, 2.0)
    v2 = Vector2(3.0, 4.0)
    result = v1 + v2
    assert result.x == 4.0
    assert result.y == 6.0


def test_vector2_subtraction():
    """Test vector subtraction"""
    v1 = Vector2(5.0, 7.0)
    v2 = Vector2(2.0, 3.0)
    result = v1 - v2
    assert result.x == 3.0
    assert result.y == 4.0


def test_vector2_negation():
    """Test vector negation"""
    v = Vector2(3.0, -4.0)
    result = -v
    assert result.x == -3.0
    assert result.y == 4.0


def test_vector2_scalar_multiplication():
    """Test scalar multiplication"""
    v = Vector2(2.0, 3.0)
    result = v * 2.0
    assert result.x == 4.0
    assert result.y == 6.0


def test_vector2_right_scalar_multiplication():
    """Test right-hand scalar multiplication"""
    v = Vector2(2.0, 3.0)
    result = 2.0 * v
    assert result.x == 4.0
    assert result.y == 6.0


def test_vector2_scalar_division():
    """Test scalar division"""
    v = Vector2(6.0, 8.0)
    result = v / 2.0
    assert result.x == 3.0
    assert result.y == 4.0


def test_vector2_division_by_zero():
    """Test that division by zero raises error"""
    v = Vector2(1.0, 2.0)
    with pytest.raises(ZeroDivisionError):
        v / 0.0


def test_vector2_magnitude():
    """Test magnitude calculation"""
    v = Vector2(3.0, 4.0)
    assert v.magnitude == 5.0


def test_vector2_sqr_magnitude():
    """Test squared magnitude calculation"""
    v = Vector2(3.0, 4.0)
    assert v.sqrMagnitude == 25.0


def test_vector2_normalized():
    """Test normalized vector"""
    v = Vector2(3.0, 4.0)
    normalized = v.normalized
    assert abs(normalized.magnitude - 1.0) < 0.0001
    assert abs(normalized.x - 0.6) < 0.0001
    assert abs(normalized.y - 0.8) < 0.0001


def test_vector2_normalized_zero():
    """Test normalized zero vector returns zero"""
    v = Vector2(0.0, 0.0)
    normalized = v.normalized
    assert normalized == Vector2.zero


def test_vector2_dot_product():
    """Test dot product"""
    v1 = Vector2(1.0, 2.0)
    v2 = Vector2(3.0, 4.0)
    result = v1.dot(v2)
    assert result == 11.0


def test_vector2_cross_product():
    """Test cross product"""
    v1 = Vector2(1.0, 2.0)
    v2 = Vector2(3.0, 4.0)
    result = v1.cross(v2)
    assert result == -2.0


def test_vector2_angle():
    """Test angle calculation"""
    v1 = Vector2(1.0, 0.0)
    v2 = Vector2(0.0, 1.0)
    angle = v1.angle(v2)
    assert abs(angle - 90.0) < 0.0001


def test_vector2_angle_zero():
    """Test angle with zero vectors"""
    v1 = Vector2(1.0, 0.0)
    v2 = Vector2(0.0, 0.0)
    angle = v1.angle(v2)
    assert angle == 0.0


def test_vector2_signed_angle():
    """Test signed angle calculation"""
    v1 = Vector2(1.0, 0.0)
    v2 = Vector2(0.0, 1.0)
    angle = v1.signed_angle(v2)
    assert abs(angle - 90.0) < 0.0001


def test_vector2_lerp():
    """Test linear interpolation"""
    a = Vector2(0.0, 0.0)
    b = Vector2(10.0, 10.0)
    result = Vector2.lerp(a, b, 0.5)
    assert result.x == 5.0
    assert result.y == 5.0


def test_vector2_lerp_clamped():
    """Test lerp with t values outside 0-1 range"""
    a = Vector2(0.0, 0.0)
    b = Vector2(10.0, 10.0)
    result1 = Vector2.lerp(a, b, -0.5)
    result2 = Vector2.lerp(a, b, 1.5)
    assert result1 == a
    assert result2 == b


def test_vector2_move_towards():
    """Test move towards"""
    current = Vector2(0.0, 0.0)
    target = Vector2(10.0, 0.0)
    result = Vector2.move_towards(current, target, 5.0)
    assert result.x == 5.0
    assert result.y == 0.0


def test_vector2_move_towards_reached():
    """Test move towards when target is reached"""
    current = Vector2(0.0, 0.0)
    target = Vector2(3.0, 0.0)
    result = Vector2.move_towards(current, target, 5.0)
    assert result == target


def test_vector2_clamp_magnitude():
    """Test clamp magnitude"""
    v = Vector2(10.0, 0.0)
    result = Vector2.clamp_magnitude(v, 5.0)
    assert abs(result.magnitude - 5.0) < 0.0001


def test_vector2_clamp_magnitude_unchanged():
    """Test clamp magnitude when already within limit"""
    v = Vector2(3.0, 0.0)
    result = Vector2.clamp_magnitude(v, 5.0)
    assert result == v


def test_vector2_reflect():
    """Test vector reflection"""
    direction = Vector2(1.0, -1.0)
    normal = Vector2(0.0, 1.0)
    result = Vector2.reflect(direction, normal)
    assert abs(result.x - 1.0) < 0.0001
    assert abs(result.y - 1.0) < 0.0001


def test_vector2_distance():
    """Test distance calculation"""
    a = Vector2(0.0, 0.0)
    b = Vector2(3.0, 4.0)
    dist = Vector2.distance(a, b)
    assert dist == 5.0


def test_vector2_rotate():
    """Test vector rotation"""
    v = Vector2(1.0, 0.0)
    result = v.rotate(90.0)
    assert abs(result.x) < 0.0001
    assert abs(result.y - 1.0) < 0.0001


def test_vector2_rotate_180():
    """Test 180 degree rotation"""
    v = Vector2(1.0, 0.0)
    result = v.rotate(180.0)
    assert abs(result.x - (-1.0)) < 0.0001
    assert abs(result.y) < 0.0001


def test_vector2_rotate_360():
    """Test 360 degree rotation returns original"""
    v = Vector2(1.0, 0.0)
    result = v.rotate(360.0)
    assert abs(result.x - 1.0) < 0.0001
    assert abs(result.y) < 0.0001


def test_vector2_int_coordinates():
    """Test Vector2 with integer coordinates"""
    v = Vector2(1, 2)
    assert isinstance(v.x, float)
    assert isinstance(v.y, float)
    assert v.x == 1.0
    assert v.y == 2.0


def test_vector2_operations_preserve_original():
    """Test that operations don't modify original vectors"""
    v1 = Vector2(1.0, 2.0)
    v2 = Vector2(3.0, 4.0)
    result = v1 + v2
    assert v1.x == 1.0 and v1.y == 2.0
    assert v2.x == 3.0 and v2.y == 4.0

