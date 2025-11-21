from __future__ import annotations
from dataclasses import dataclass
import math
from typing import Union, ClassVar

Number = Union[int, float]

@dataclass(slots=True)
class Vector2:
    """Vector2 class for 2D vector operations"""
    x: float
    y: float

    zero: ClassVar["Vector2"]
    one:  ClassVar["Vector2"]
    up:   ClassVar["Vector2"]
    down: ClassVar["Vector2"]
    left: ClassVar["Vector2"]
    right:ClassVar["Vector2"]

    def __str__(self) -> str:
        return f"({self.x}, {self.y})"

    def __add__(self, other: Vector2) -> Vector2:
        return Vector2(self.x + other.x, self.y + other.y)

    def __sub__(self, other: Vector2) -> Vector2:
        return Vector2(self.x - other.x, self.y - other.y)

    def __neg__(self) -> Vector2:
        return Vector2(-self.x, -self.y)

    def __mul__(self, scalar: Number) -> Vector2:
        """Scalar multiplication"""
        return Vector2(self.x * scalar, self.y * scalar)

    def __rmul__(self, scalar: Number) -> Vector2:
        """Scalar multiplication"""
        return self * scalar

    def __truediv__(self, scalar: Number) -> Vector2:
        """Scalar division"""
        if scalar == 0:
            raise ZeroDivisionError("division by zero")
        return Vector2(self.x / scalar, self.y / scalar)

    @property
    def magnitude(self) -> float:
        """Length of the vector"""
        return math.hypot(self.x, self.y)

    @property
    def sqrMagnitude(self) -> float:
        """Squared length for performance when comparing sizes"""
        return self.x * self.x + self.y * self.y

    @property
    def normalized(self) -> Vector2:
        """Unit vector in the same direction"""
        mag = self.magnitude
        if mag == 0:
            return Vector2.zero
        return self / mag

    def dot(self, other: Vector2) -> float:
        """Dot product"""
        return self.x * other.x + self.y * other.y

    def cross(self, other: Vector2) -> float:
        """2D cross product returns scalar z-component"""
        return self.x * other.y - self.y * other.x

    def angle(self, other: Vector2) -> float:
        """Returns unsigned angle in degrees between two vectors"""
        dot = self.dot(other)
        mag_product = self.magnitude * other.magnitude
        if mag_product == 0:
            return 0.0
        cos = max(min(dot / mag_product, 1.0), -1.0)
        return math.degrees(math.acos(cos))

    def signed_angle(self, other: Vector2) -> float:
        """Signed angle in degrees, positive is counter-clockwise"""
        angle = self.angle(other)
        sign = 1.0 if self.cross(other) >= 0 else -1.0
        return angle * sign

    @staticmethod
    def lerp(a: Vector2, b: Vector2, t: float) -> Vector2:
        """Linear interpolation, t is clamped between 0 and 1"""
        t = max(0.0, min(1.0, t))
        return a + (b - a) * t

    @staticmethod
    def move_towards(current: Vector2, target: Vector2, max_distance_delta: float) -> Vector2:
        """Move current toward target by at most max_distance_delta"""
        to_target = target - current
        dist = to_target.magnitude
        if dist <= max_distance_delta or dist == 0:
            return target
        return current + to_target / dist * max_distance_delta

    @staticmethod
    def clamp_magnitude(vector: Vector2, max_length: float) -> Vector2:
        """Returns vector with magnitude limited to max_length"""
        sqr_len = vector.sqrMagnitude
        if sqr_len > max_length * max_length:
            return vector.normalized * max_length
        return vector

    @staticmethod
    def reflect(direction: Vector2, normal: Vector2) -> Vector2:
        """Reflects a vector off the plane defined by normal"""
        n = normal.normalized
        return direction - 2 * direction.dot(n) * n

    @staticmethod
    def distance(a: Vector2, b: Vector2) -> float:
        """Euclidean distance between two points"""
        return (a - b).magnitude

    def rotate(self, angle_deg: float) -> Vector2:
        """Returns new vector rotated angle_deg degrees counter-clockwise"""
        rad = math.radians(angle_deg)
        cos_a, sin_a = math.cos(rad), math.sin(rad)
        return Vector2(
            self.x * cos_a - self.y * sin_a,
            self.x * sin_a + self.y * cos_a,
        )

Vector2.zero = Vector2(0.0, 0.0)
Vector2.one  = Vector2(1.0, 1.0)
Vector2.up   = Vector2(0.0, 1.0)
Vector2.down = Vector2(0.0, -1.0)
Vector2.left = Vector2(-1.0, 0.0)
Vector2.right= Vector2(1.0, 0.0)

