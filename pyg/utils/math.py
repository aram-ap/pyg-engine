
try:
    from pyg import _native
except ImportError:
    # Fallback for direct import or weird paths
    import _native


class Vector2:
    def __init__(self, x: float = 0, y: float = 0):
        self._vector2_manager = _native.Vector2Manager()
        self._vector2 = _native.Vector2()
        self._vector2.x = x
        self._vector2.y = y

    def __str__(self):
        return f"Vector2(x={self.x}, y={self.y})"

    def __repr__(self):
        return f"Vector2(x={self.x}, y={self.y})"

    @property
    def x(self) -> float:
        return self._vector2.x

    @x.setter
    def x(self, value: float):
        self._vector2.x = value

    @property
    def y(self) -> float:
        return self._vector2.y

    @y.setter
    def y(self, value: float):
        self._vector2.y = value

    def __add__(self, other: "Vector2") -> "Vector2":
        return Vector2(self.x + other.x, self.y + other.y)

    def __sub__(self, other: "Vector2") -> "Vector2":
        return Vector2(self.x - other.x, self.y - other.y)

    def __mul__(self, other: float) -> "Vector2":
        return Vector2(self.x * other, self.y * other)

    def __truediv__(self, other: float) -> "Vector2":
        return Vector2(self.x / other, self.y / other)

    def __eq__(self, other: "Vector2") -> bool:
        return self.x == other.x and self.y == other.y

    def __ne__(self, other: "Vector2") -> bool:
        return self.x != other.x or self.y != other.y

    def __lt__(self, other: "Vector2") -> bool:
        return self.x < other.x and self.y < other.y

    def __le__(self, other: "Vector2") -> bool:
        return self.x <= other.x and self.y <= other.y

    def __gt__(self, other: "Vector2") -> bool:
        return self.x > other.x and self.y > other.y

    def __ge__(self, other: "Vector2") -> bool:
        return self.x >= other.x and self.y >= other.y

    def __hash__(self) -> int:
        return hash((self.x, self.y))

    def __copy__(self) -> "Vector2":
        return Vector2(self.x, self.y)

    def __deepcopy__(self, memo) -> "Vector2":
        return Vector2(self.x, self.y)


class Vector3:
    def __init__(self, x: float = 0, y: float = 0, z: float = 0):
        self._vector3_manager = _native.Vector3Manager()
        self._vector3 = _native.Vector3()
        self._vector3.x = x
        self._vector3.y = y
        self._vector3.z = z

    def __str__(self):
        return f"Vector3(x={self.x}, y={self.y}, z={self.z})"

    def __repr__(self):
        return f"Vector3(x={self.x}, y={self.y}, z={self.z})"

    @property
    def x(self) -> float:
        return self._vector3.x

    @x.setter
    def x(self, value: float):
        self._vector3.x = value

    @property
    def y(self) -> float:
        return self._vector3.y

    @y.setter
    def y(self, value: float):
        self._vector3.y = value

    @property
    def z(self) -> float:
        return self._vector3.z

    @z.setter
    def z(self, value: float):
        self._vector3.z = value

    def __add__(self, other: "Vector3") -> "Vector3":
        return Vector3(self.x + other.x, self.y + other.y, self.z + other.z)

    def __sub__(self, other: "Vector3") -> "Vector3":
        return Vector3(self.x - other.x, self.y - other.y, self.z - other.z)

    def __mul__(self, other: float) -> "Vector3":
        return Vector3(self.x * other, self.y * other, self.z * other)

    def __truediv__(self, other: float) -> "Vector3":
        return Vector3(self.x / other, self.y / other, self.z / other)

    def __eq__(self, other: "Vector3") -> bool:
        return self.x == other.x and self.y == other.y and self.z == other.z

    def __ne__(self, other: "Vector3") -> bool:
        return self.x != other.x or self.y != other.y or self.z != other.z  # or self.w != other.w

    def __lt__(self, other: "Vector3") -> bool:
        return self.x < other.x and self.y < other.y and self.z < other.z

    def __le__(self, other: "Vector3") -> bool:
        return self.x <= other.x and self.y <= other.y and self.z <= other.z

    def __gt__(self, other: "Vector3") -> bool:
        return self.x > other.x and self.y > other.y and self.z > other.z

    def __ge__(self, other: "Vector3") -> bool:
        return self.x >= other.x and self.y >= other.y and self.z >= other.z

    def __hash__(self) -> int:
        return hash((self.x, self.y, self.z))

    def __copy__(self) -> "Vector3":
        return Vector3(self.x, self.y, self.z)

    def __deepcopy__(self, memo) -> "Vector3":
        return Vector3(self.x, self.y, self.z)

class Vector4:
    def __init__(self, x: float = 0, y: float = 0, z: float = 0, w: float = 0):
        self._vector4_manager = _native.Vector4Manager()
        self._vector4 = _native.Vector4()
        self._vector4.x = x
        self._vector4.y = y
        self._vector4.z = z
        self._vector4.w = w

    def __str__(self):
        return f"Vector4(x={self.x}, y={self.y}, z={self.z}, w={self.w})"

    def __repr__(self):
        return f"Vector4(x={self.x}, y={self.y}, z={self.z}, w={self.w})"

    @property
    def x(self) -> float:
        return self._vector4.x

    @x.setter
    def x(self, value: float):
        self._vector4.x = value

    @property
    def y(self) -> float:
        return self._vector4.y

    @y.setter
    def y(self, value: float):
        self._vector4.y = value

    @property
    def z(self) -> float:
        return self._vector4.z

    @z.setter
    def z(self, value: float):
        self._vector4.z = value

    @property
    def w(self) -> float:
        return self._vector4.w

    @w.setter
    def w(self, value: float):
        self._vector4.w = value

    def __add__(self, other: "Vector4") -> "Vector4":
        return Vector4(self.x + other.x, self.y + other.y, self.z + other.z, self.w + other.w)

    def __sub__(self, other: "Vector4") -> "Vector4":
        return Vector4(self.x - other.x, self.y - other.y, self.z - other.z, self.w - other.w)

    def __mul__(self, other: float) -> "Vector4":
        return Vector4(self.x * other, self.y * other, self.z * other, self.w * other)

    def __truediv__(self, other: float) -> "Vector4":
        return Vector4(self.x / other, self.y / other, self.z / other, self.w / other)

    def __eq__(self, other: "Vector4") -> bool:
        return self.x == other.x and self.y == other.y and self.z == other.z and self.w == other.w

    def __ne__(self, other: "Vector4") -> bool:
        return self.x != other.x or self.y != other.y or self.z != other.z or self.w != other.w

    def __lt__(self, other: "Vector4") -> bool:
        return self.x < other.x and self.y < other.y and self.z < other.z and self.w < other.w

    def __le__(self, other: "Vector4") -> bool:
        return self.x <= other.x and self.y <= other.y and self.z <= other.z and self.w <= other.w

    def __gt__(self, other: "Vector4") -> bool:
        return self.x > other.x and self.y > other.y and self.z > other.z and self.w > other.w

    def __ge__(self, other: "Vector4") -> bool:
        return self.x >= other.x and self.y >= other.y and self.z >= other.z and self.w >= other.w

    def __hash__(self) -> int:
        return hash((self.x, self.y, self.z, self.w))

    def __copy__(self) -> "Vector4":
        return Vector4(self.x, self.y, self.z, self.w)

    def __deepcopy__(self, memo) -> "Vector4":
        return Vector4(self.x, self.y, self.z, self.w)

def dot(a: "Vector2", b: "Vector2") -> float:
    return a.x * b.x + a.y * b.y

def dot(a: "Vector3", b: "Vector3") -> float:
    return a.x * b.x + a.y * b.y + a.z * b.z

def dot(a: "Vector4", b: "Vector4") -> float:
    return a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w

def cross(a: "Vector2", b: "Vector2") -> "Vector2":
    return Vector2(a.y * b.x - a.x * b.y, a.x * b.y - a.y * b.x)

def cross(a: "Vector3", b: "Vector3") -> "Vector3":
    return Vector3(a.y * b.z - a.z * b.y, a.z * b.x - a.x * b.z, a.x * b.y - a.y * b.x)

def cross(a: "Vector4", b: "Vector4") -> "Vector4":
    return Vector4(a.y * b.z - a.z * b.y, a.z * b.x - a.x * b.z, a.x * b.y - a.y * b.x, a.w * b.x - a.x * b.w - a.y * b.z + a.z * b.y)

def length(v: "Vector2") -> float:
    return math.sqrt(dot(v, v))

def length(v: "Vector3") -> float:
    return math.sqrt(dot(v, v))

def length(v: "Vector4") -> float:
    return math.sqrt(dot(v, v))

def distance(a: "Vector2", b: "Vector2") -> float:
    return length(a - b)

def distance(a: "Vector3", b: "Vector2") -> float:
    return length(a - b)

def distance(a: "Vector4", b: "Vector2") -> float:
    return length(a - b)

def normalize(v: "Vector2") -> "Vector2":
    return v / length(v)

def normalize(v: "Vector3") -> "Vector3":
    return v / length(v)

def normalize(v: "Vector4") -> "Vector4":
    return v / length(v)

def isNaN(v: "Vector2") -> bool:
    return isNaN(v.x) or isNaN(v.y)

def isNaN(v: "Vector3") -> bool:
    return isNaN(v.x) or isNaN(v.y) or isNaN(v.z)

def isNaN(v: "Vector4") -> bool:
    return isNaN(v.x) or isNaN(v.y) or isNaN(v.z) or isNaN(v.w)

def isInfinity(v: "Vector2") -> bool:
    return isInfinity(v.x) or isInfinity(v.y)

def isInfinity(v: "Vector3") -> bool:
    return isInfinity(v.x) or isInfinity(v.y) or isInfinity(v.z)

def isInfinity(v: "Vector4") -> bool:
    return isInfinity(v.x) or isInfinity(v.y) or isInfinity(v.z) or isInfinity(v.w)

def isFinite(v: "Vector2") -> bool:
    return isFinite(v.x) and isFinite(v.y)

def isFinite(v: "Vector3") -> bool:
    return isFinite(v.x) and isFinite(v.y) and isFinite(v.z)

def isFinite(v: "Vector4") -> bool:
    return isFinite(v.x) and isFinite(v.y) and isFinite(v.z) and isFinite(v.w)

def isEqual(a: "Vector2", b: "Vector2") -> bool:
    return isEqual(a.x, b.x) and isEqual(a.y, b.y)

def isEqual(a: "Vector3", b: "Vector3") -> bool:
    return isEqual(a.x, b.x) and isEqual(a.y, b.y) and isEqual(a.z, b.z)

def isEqual(a: "Vector4", b: "Vector4") -> bool:
    return isEqual(a.x, b.x) and isEqual(a.y, b.y) and isEqual(a.z, b.z) and isEqual(a.w, b.w)

def isGreater(a: "Vector2", b: "Vector2") -> bool:
    return a.x > b.x and a.y > b.y

def isGreater(a: "Vector3", b: "Vector3") -> bool:  # TODO: Implement
    return a.x > b.x and a.y > b.y and a.z > b.z

def isGreater(a: "Vector4", b: "Vector4") -> bool:  # TODO: Implement
    return a.x > b.x and a.y > b.y and a.z > b.z and a.w > b.w

def isGreaterEqual(a: "Vector2", b: "Vector2") -> bool:
    return a.x >= b.x and a.y >= b.y

def isGreaterEqual(a: "Vector3", b: "Vector3") -> bool:  # TODO: Implement
    return a.x >= b.x and a.y >= b.y and a.z >= b.z

def isGreaterEqual(a: "Vector4", b: "Vector4") -> bool:  # TODO: Implement
    return a.x >= b.x and a.y >= b.y and a.z >= b.z and a.w >= b.w

def isLess(a: "Vector2", b: "Vector2") -> bool:
    return a.x < b.x and a.y < b.y

def isLess(a: "Vector3", b: "Vector3") -> bool:  # TODO: Implement
    return a.x < b.x and a.y < b.y and a.z < b.z

def isLess(a: "Vector4", b: "Vector4") -> bool:  # TODO: Implement
    return a.x < b.x and a.y < b.y and a.z < b.z and a.w < b.w

def isLessEqual(a: "Vector2", b: "Vector2") -> bool:
    return a.x <= b.x and a.y <= b.y

def isLessEqual(a: "Vector3", b: "Vector3") -> bool:  # TODO: Implement
    return a.x <= b.x and a.y <= b.y and a.z <= b.z

def isLessEqual(a: "Vector4", b: "Vector4") -> bool:  # TODO: Implement
    return a.x <= b.x and a.y <= b.y and a.z <= b.z and a.w <= b.w

def isZero(v: "Vector2") -> bool:
    return isEqual(v, ZERO)

def isZero(v: "Vector3") -> bool:
    return isEqual(v, ZERO3)

def isZero(v: "Vector4") -> bool:
    return isEqual(v, ZERO4)

def isNotZero(v: "Vector2") -> bool:
    return not isZero(v)

def isNotZero(v: "Vector3") -> bool:
    return not isZero(v)

def isNotZero(v: "Vector4") -> bool:
    return not isZero(v)

def isPositive(v: "Vector2") -> bool:
    return v.x > 0 and v.y > 0

def isPositive(v: "Vector3") -> bool:
    return v.x > 0 and v.y > 0 and v.z > 0

def isPositive(v: "Vector4") -> bool:
    return v.x > 0 and v.y > 0 and v.z > 0 and v.w > 0

def isNegative(v: "Vector2") -> bool:
    return v.x < 0 and v.y < 0

def isNegative(v: "Vector3") -> bool:
    return v.x < 0 and v.y < 0 and v.z < 0

def isNegative(v: "Vector4") -> bool:
    return v.x < 0 and v.y < 0 and v.z < 0 and v.w < 0

def random() -> float:
    return rand() / (float) RAND_MAX

def random(min: float, max: float) -> float:
    return min + (max - min) * random()

def abs(value: float) -> float:
    return value < 0 ? -value : value

def sign(value: float) -> float:
    return value < 0 ? -1 : 1

def floor(value: float) -> float:
    return (float) floor(value)

def ceil(value: float) -> float:
    return (float) ceil(value)

def round(value: float) -> float:
    return (float) round(value)

def frac(value: float) -> float:
    return value - floor(value)

def mod(x: float, y: float) -> float:
    return x - y * floor(x / y)

def min(a: float, b: float) -> float:
    return a < b ? a : b

def max(a: float, b: float) -> float:
    return a > b ? a : b

def pow(x: float, y: float) -> float:
    return powf(x, y)

def sqrt(x: float) -> float:
    return sqrtf(x)

def sin(x: float) -> float:
    return sinf(x)

def cos(x: float) -> float:
    return cosf(x)

def tan(x: float) -> float:
    return tanf(x)

def asin(x: float) -> float:
    return asinf(x)

def acos(x: float) -> float:
    return acosf(x)

def atan(x: float) -> float:
    return atanf(x)

def atan2(y: float, x: float) -> float:
    return atan2f(y, x)

def exp(x: float) -> float:
    return expf(x)

def log(x: float) -> float:
    return logf(x)

def log2(x: float) -> float:
    return log2f(x)

def log10(x: float) -> float:
    return log10f(x)

def deg2rad(degrees: float) -> float:
    return degrees * DEG2RAD

def rad2deg(radians: float) -> float:
    return radians * RAD2DEG

def lerp(a: float, b: float, t: float) -> float:
    return a * (1 - t) + b * t

def clamp(value: float, min: float, max: float) -> float:
    return value < min ? min : value > max ? max : value

def smoothstep(edge0: float, edge1: float, x: float) -> float:
    t = clamp((x - edge0) / (edge1 - edge0), 0.0f, 1.0f)
    return t * t * (3 - 2 * t)

def smootherstep(edge0: float, edge1: float, x: float) -> float:
    x = clamp((x - edge0) / (edge1 - edge0), 0.0f, 1.0f)
    return x * x * x * (x * (x * 6 - 15) + 10)

ZERO = Vector2(0, 0)
ONE = Vector2(1, 1)
UP = Vector2(0, 1)
DOWN = Vector2(0, -1)
LEFT = Vector2(-1, 0)
RIGHT = Vector2(1, 0)
ZERO3 = Vector3(0, 0, 0)
ONE3 = Vector3(1, 1, 1)
UP3 = Vector3(0, 1, 0)
DOWN3 = Vector3(0, -1, 0)
LEFT3 = Vector3(-1, 0, 0)
RIGHT3 = Vector3(1, 0, 0)
ZERO4 = Vector4(0, 0, 0, 0)
ONE4 = Vector4(1, 1, 1, 1)
UP4 = Vector4(0, 1, 0, 1)
DOWN4 = Vector4(0, -1, 0, 1)
LEFT4 = Vector4(-1, 0, 0, 1)
RIGHT4 = Vector4(1, 0, 0, 1)
FORWARD = Vector4(0, 0, -1, 0)
BACK = Vector4(0, 0, 1, 0)
ZEROV = Vector4(0, 0, 0, 0)
ONEV = Vector4(1, 1, 1, 1)
UPV = Vector4(0, 1, 0, 1)
DOWNV = Vector4(0, -1, 0, 1)
PI = 3.14159265358979323846f
EPSILON = 0.00001f
DEG2RAD = PI / 180.0f
RAD2DEG = 180.0f / PI
INFINITY = std::numeric_limits<float>::infinity()
NAN = std::numeric_limits<float>::quiet_NaN()
SQRT2 = 1.41421356237309504880f
SQRT3 = 1.73205080756887729352f
E = 2.71828182845904523536f
GOLDEN_RATIO = (1 + sqrt(5)) / 2;
PHI = (1 + sqrt(5)) / 2;
TAU = 2 * PI
LOG2E = 1.44269504088896340736f
LOG10E = 0.434294481903251827651f
LN2 = 0.693147180559945309417f
LN10 = 2.3025
