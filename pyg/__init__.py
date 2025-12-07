import math
import sys

try:
    from pyg import _native
except ImportError:
    import _native # Fallback?

Engine   = _native.Engine
Logger   = _native.Logger
LogType  = _native.LogType
Window   = _native.Window
Math     = _native.Math

# from .core.engine import Engine, LogType
from .input.input_manager import Input
from .types.color import Color

Vector2  = _native.Vec2
Vector3  = _native.Vec3
Vector4  = _native.Vec4

# Add Vector constants as class attributes
Vector2.zero = Vector2(0.0, 0.0)
Vector2.one = Vector2(1.0, 1.0)
Vector2.up = Vector2(0.0, 1.0)
Vector2.down = Vector2(0.0, -1.0)
Vector2.left = Vector2(-1.0, 0.0)
Vector2.right = Vector2(1.0, 0.0)

Vector3.zero = Vector3(0.0, 0.0, 0.0)
Vector3.one = Vector3(1.0, 1.0, 1.0)
Vector3.up = Vector3(0.0, 1.0, 0.0)
Vector3.down = Vector3(0.0, -1.0, 0.0)
Vector3.left = Vector3(-1.0, 0.0, 0.0)
Vector3.right = Vector3(1.0, 0.0, 0.0)
Vector3.forward = Vector3(0.0, 0.0, 1.0)
Vector3.back = Vector3(0.0, 0.0, -1.0)

Vector4.zero = Vector4(0.0, 0.0, 0.0, 0.0)
Vector4.one = Vector4(1.0, 1.0, 1.0, 1.0)
Vector4.up = Vector4(0.0, 1.0, 0.0, 1.0)
Vector4.down = Vector4(0.0, -1.0, 0.0, 1.0)
Vector4.left = Vector4(-1.0, 0.0, 0.0, 1.0)
Vector4.right = Vector4(1.0, 0.0, 0.0, 1.0)
Vector4.forward = Vector4(0.0, 0.0, 1.0, 1.0)
Vector4.back = Vector4(0.0, 0.0, -1.0, 1.0)

# Math Constants
PI           = math.pi
TAU          = 2.0 * math.pi
E            = math.e
EPSILON      = sys.float_info.epsilon
INFINITY     = float('inf')
NAN          = float('nan')
DEG2RAD      = math.pi / 180.0
RAD2DEG      = 180.0 / math.pi
SQRT2        = math.sqrt(2)
SQRT3        = math.sqrt(3)
GOLDEN_RATIO = (1.0 + math.sqrt(5.0)) / 2.0
PHI          = GOLDEN_RATIO
LOG2E        = math.log2(math.e)
LOG10E       = math.log10(math.e)
LN2          = math.log(2)
LN10         = math.log(10)

# Module-level log function
def log(message):
    """Log a message using the engine's logger."""
    _native.log(str(message))

def log_type(log_type: LogType, message):
    """Log a message using the engine's logger with LogType enum"""
    _native.log_type(log_type, str(message))

def log_error(message):
    """Log a message using the engine's logger with LogType.Error"""
    _native.log_type(LogType.Error, str(message))

def log_warning(message):
    """Log a message using the engine's logger with LogType.Warning"""
    _native.log_type(LogType.Warning, str(message))

def log_info(message):
    """Log a message using the engine's logger with LogType.Info"""
    _native.log_type(LogType.Info, str(message))

def log_debug(message):
    """Log a message using the engine's logger with LogType.Debug"""
    _native.log_type(LogType.Debug, str(message))

def log_trace(message):
    """Log a message using the engine's logger with LogType.Trace"""
    _native.log_type(LogType.Trace, str(message))

# """
#     , "dot", "cross", "length", "distance", "normalize", "isNaN", "isInfinity", "isFinite", "isEqual", "isGreater", "isGreaterEqual", "isLess", "isLessEqual", "isZero", "isNotZero", "isPositive", "isNegative", "random", "abs", "sign", "floor", "ceil", "round", "frac", "mod", "min",
#     "max", "pow", "sqrt", "sin", "cos", "tan", "asin", "acos", "atan", "atan2", "exp", "log", "log2", "log10", "deg2rad", "rad2deg", "lerp", "clamp", "smoothstep", "smootherstep",
# """

__all__ = [
    # Classes
    "Engine", "Logger", "LogType", "Window", "Input",
    "Vector2", "Vector3", "Vector4",

    # Functions
    "log", "log_type", "log_error", "log_warning", "log_info", "log_debug", "log_trace",

    # Math
    "Math",

    # Types
    "Color",

    # Scalar Constants
    "PI", "TAU", "EPSILON", "DEG2RAD", "RAD2DEG", "INFINITY", "NAN",
    "SQRT2", "SQRT3", "E", "GOLDEN_RATIO", "PHI", "LOG2E", "LOG10E", "LN2", "LN10"
]

