"""
Unit tests for math utilities.

Tests vector operations and mathematical functions.
"""

from typing import TYPE_CHECKING

import pytest

from pyg.utils.math import Vector2, Vector3, Vector4

if TYPE_CHECKING:
    pass


@pytest.mark.unit
class TestVector2:
    """Test Vector2 functionality."""

    def test_create_vector2_default(self) -> None:
        """Test creating a Vector2 with default values."""
        vec = Vector2()
        assert vec.x == 0
        assert vec.y == 0

    def test_create_vector2_with_values(self) -> None:
        """Test creating a Vector2 with specific values."""
        vec = Vector2(3.5, 7.2)
        assert vec.x == pytest.approx(3.5)
        assert vec.y == pytest.approx(7.2)

    def test_vector2_addition(self) -> None:
        """Test Vector2 addition."""
        vec1 = Vector2(1, 2)
        vec2 = Vector2(3, 4)
        result = vec1 + vec2
        assert result.x == 4
        assert result.y == 6

    def test_vector2_subtraction(self) -> None:
        """Test Vector2 subtraction."""
        vec1 = Vector2(5, 8)
        vec2 = Vector2(2, 3)
        result = vec1 - vec2
        assert result.x == 3
        assert result.y == 5

    def test_vector2_scalar_multiplication(self) -> None:
        """Test Vector2 multiplication by scalar."""
        vec = Vector2(2, 3)
        result = vec * 2
        assert result.x == 4
        assert result.y == 6

    def test_vector2_scalar_division(self) -> None:
        """Test Vector2 division by scalar."""
        vec = Vector2(10, 20)
        result = vec / 2
        assert result.x == 5
        assert result.y == 10

    def test_vector2_equality(self) -> None:
        """Test Vector2 equality comparison."""
        vec1 = Vector2(1, 2)
        vec2 = Vector2(1, 2)
        vec3 = Vector2(2, 3)
        assert vec1 == vec2
        assert vec1 != vec3

    def test_vector2_string_representation(self) -> None:
        """Test Vector2 string representation."""
        vec = Vector2(1.5, 2.5)
        string_repr = str(vec)
        assert "1.5" in string_repr
        assert "2.5" in string_repr
        assert "Vector2" in string_repr

    def test_vector2_copy(self) -> None:
        """Test Vector2 copy."""
        vec1 = Vector2(3, 4)
        vec2 = vec1.__copy__()
        assert vec1.x == pytest.approx(vec2.x)
        assert vec1.y == pytest.approx(vec2.y)


@pytest.mark.unit
class TestVector3:
    """Test Vector3 functionality."""

    def test_create_vector3_default(self) -> None:
        """Test creating a Vector3 with default values."""
        vec = Vector3()
        assert vec.x == 0
        assert vec.y == 0
        assert vec.z == 0

    def test_create_vector3_with_values(self) -> None:
        """Test creating a Vector3 with specific values."""
        vec = Vector3(1.5, 2.5, 3.5)
        assert vec.x == pytest.approx(1.5)
        assert vec.y == pytest.approx(2.5)
        assert vec.z == pytest.approx(3.5)

    def test_vector3_addition(self) -> None:
        """Test Vector3 addition."""
        vec1 = Vector3(1, 2, 3)
        vec2 = Vector3(4, 5, 6)
        result = vec1 + vec2
        assert result.x == pytest.approx(5)
        assert result.y == pytest.approx(7)
        assert result.z == pytest.approx(9)

    def test_vector3_subtraction(self) -> None:
        """Test Vector3 subtraction."""
        vec1 = Vector3(10, 8, 6)
        vec2 = Vector3(1, 2, 3)
        result = vec1 - vec2
        assert result.x == pytest.approx(9)
        assert result.y == pytest.approx(6)
        assert result.z == pytest.approx(3)

    def test_vector3_scalar_multiplication(self) -> None:
        """Test Vector3 multiplication by scalar."""
        vec = Vector3(2, 3, 4)
        result = vec * 3
        assert result.x == pytest.approx(6)
        assert result.y == pytest.approx(9)
        assert result.z == pytest.approx(12)

    def test_vector3_scalar_division(self) -> None:
        """Test Vector3 division by scalar."""
        vec = Vector3(12, 18, 24)
        result = vec / 3
        assert result.x == pytest.approx(4)
        assert result.y == pytest.approx(6)
        assert result.z == pytest.approx(8)

    def test_vector3_equality(self) -> None:
        """Test Vector3 equality comparison."""
        vec1 = Vector3(1, 2, 3)
        vec2 = Vector3(1, 2, 3)
        vec3 = Vector3(3, 2, 1)
        assert vec1 == vec2
        assert vec1 != vec3


@pytest.mark.unit
class TestVector4:
    """Test Vector4 functionality."""

    def test_create_vector4_default(self) -> None:
        """Test creating a Vector4 with default values."""
        vec = Vector4()
        assert vec.x == 0
        assert vec.y == 0
        assert vec.z == 0
        assert vec.w == 0

    def test_create_vector4_with_values(self) -> None:
        """Test creating a Vector4 with specific values."""
        vec = Vector4(1, 2, 3, 4)
        assert vec.x == pytest.approx(1)
        assert vec.y == pytest.approx(2)
        assert vec.z == pytest.approx(3)
        assert vec.w == pytest.approx(4)

    def test_vector4_addition(self) -> None:
        """Test Vector4 addition."""
        vec1 = Vector4(1, 2, 3, 4)
        vec2 = Vector4(5, 6, 7, 8)
        result = vec1 + vec2
        assert result.x == pytest.approx(6)
        assert result.y == pytest.approx(8)
        assert result.z == pytest.approx(10)
        assert result.w == pytest.approx(12)

    def test_vector4_subtraction(self) -> None:
        """Test Vector4 subtraction."""
        vec1 = Vector4(10, 20, 30, 40)
        vec2 = Vector4(1, 2, 3, 4)
        result = vec1 - vec2
        assert result.x == pytest.approx(9)
        assert result.y == pytest.approx(18)
        assert result.z == pytest.approx(27)
        assert result.w == pytest.approx(36)

    def test_vector4_scalar_multiplication(self) -> None:
        """Test Vector4 multiplication by scalar."""
        vec = Vector4(1, 2, 3, 4)
        result = vec * 2
        assert result.x == pytest.approx(2)
        assert result.y == pytest.approx(4)
        assert result.z == pytest.approx(6)
        assert result.w == pytest.approx(8)

    def test_vector4_equality(self) -> None:
        """Test Vector4 equality comparison."""
        vec1 = Vector4(1, 2, 3, 4)
        vec2 = Vector4(1, 2, 3, 4)
        vec3 = Vector4(4, 3, 2, 1)
        assert vec1 == vec2
        assert vec1 != vec3


@pytest.mark.unit
class TestVectorEdgeCases:
    """Test edge cases for vector operations."""

    def test_vector2_zero_division(self) -> None:
        """Test that dividing by zero raises appropriate error."""
        vec = Vector2(10, 20)
        with pytest.raises(RuntimeError):
            _ = vec / 0

    def test_vector3_negative_values(self) -> None:
        """Test Vector3 with negative values."""
        vec = Vector3(-1, -2, -3)
        assert vec.x == pytest.approx(-1)
        assert vec.y == pytest.approx(-2)
        assert vec.z == pytest.approx(-3)

    def test_vector4_large_values(self) -> None:
        """Test Vector4 with large values."""
        vec = Vector4(1000000, 2000000, 3000000, 4000000)
        assert vec.x == pytest.approx(1000000)
        assert vec.y == pytest.approx(2000000)
        assert vec.z == pytest.approx(3000000)
        assert vec.w == pytest.approx(4000000)

