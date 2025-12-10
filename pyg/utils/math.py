"""Math utilities for the pyg engine.

This module provides native C++ vector classes (Vector2, Vector3, Vector4)
and the native Math class with optimized scalar and vector operations.
"""

import math as _math

try:
    from pyg import _native
except ImportError:
    # Fallback for direct import or weird paths
    import _native

# Import native vector classes directly from C++
Vector2 = _native.Vector2
Vector3 = _native.Vector3
Vector4 = _native.Vector4

# Import native Math class
Math = _native.Math


# Vector utility functions
def dot(a: Vector2 | Vector3 | Vector4, b: Vector2 | Vector3 | Vector4) -> float:
    """Calculate the dot product of two vectors."""
    return a.dot(b)


def cross(a: Vector3, b: Vector3) -> Vector3:
    """Calculate the cross product of two 3D vectors."""
    return Vector3(
        a.y * b.z - a.z * b.y,
        a.z * b.x - a.x * b.z,
        a.x * b.y - a.y * b.x
    )


def length(v: Vector2 | Vector3 | Vector4) -> float:
    """Calculate the length (magnitude) of a vector."""
    return v.length()


def distance(a: Vector2 | Vector3 | Vector4, b: Vector2 | Vector3 | Vector4) -> float:
    """Calculate the distance between two vectors."""
    return (a - b).length()


def normalize(v: Vector2 | Vector3 | Vector4) -> Vector2 | Vector3 | Vector4:
    """Return a normalized (unit length) version of the vector."""
    return v / v.length()

# Vector2 Constants
ZERO = Vector2(0.0, 0.0)
ONE = Vector2(1.0, 1.0)
UP = Vector2(0.0, 1.0)
DOWN = Vector2(0.0, -1.0)
LEFT = Vector2(-1.0, 0.0)
RIGHT = Vector2(1.0, 0.0)

# Vector3 Constants
ZERO3 = Vector3(0.0, 0.0, 0.0)
ONE3 = Vector3(1.0, 1.0, 1.0)
UP3 = Vector3(0.0, 1.0, 0.0)
DOWN3 = Vector3(0.0, -1.0, 0.0)
LEFT3 = Vector3(-1.0, 0.0, 0.0)
RIGHT3 = Vector3(1.0, 0.0, 0.0)
FORWARD3 = Vector3(0.0, 0.0, 1.0)
BACK3 = Vector3(0.0, 0.0, -1.0)

# Vector4 Constants
ZERO4 = Vector4(0.0, 0.0, 0.0, 0.0)
ONE4 = Vector4(1.0, 1.0, 1.0, 1.0)
UP4 = Vector4(0.0, 1.0, 0.0, 1.0)
DOWN4 = Vector4(0.0, -1.0, 0.0, 1.0)
LEFT4 = Vector4(-1.0, 0.0, 0.0, 1.0)
RIGHT4 = Vector4(1.0, 0.0, 0.0, 1.0)
FORWARD4 = Vector4(0.0, 0.0, 1.0, 1.0)
BACK4 = Vector4(0.0, 0.0, -1.0, 1.0)

# Scalar Math Constants (from native C++ Math class)
PI = Math.PI
EPSILON = Math.EPSILON
DEG2RAD = Math.DEG2RAD
RAD2DEG = Math.RAD2DEG
SQRT2 = Math.SQRT2
SQRT3 = Math.SQRT3
E = Math.E
GOLDEN_RATIO = Math.GOLDEN_RATIO
PHI = Math.PHI
TAU = Math.TAU
LOG2E = Math.LOG2E
LOG10E = Math.LOG10E
LN2 = Math.LN2
LN10 = Math.LN10
INFINITY = float('inf')
NAN = float('nan')
