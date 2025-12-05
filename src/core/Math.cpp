//
// Created by Aram Aprahamian on 11/22/25.
//

#include "Math.h"
#include <cmath>

namespace pyg {
    const float Math::PI = 3.14159265358979323846f;
    const float Math::EPSILON = 0.00001f;
    const float Math::DEG2RAD = PI / 180.0f;
    const float Math::RAD2DEG = 180.0f / PI;
    const float Math::INFINITY = std::numeric_limits<float>::infinity();
    const float Math::NAN = std::numeric_limits<float>::quiet_NaN();
    const float Math::SQRT2 = 1.41421356237309504880f;
    const float Math::SQRT3 = 1.73205080756887729352f;
    const float Math::E = 2.71828182845904523536f;
    const float Math::GOLDEN_RATIO = (1 + sqrt(5)) / 2;
    const float Math::PHI = (1 + sqrt(5)) / 2;
    const float Math::TAU = 2 * PI;
    const float Math::LOG2E = 1.44269504088896340736f;
    const float Math::LOG10E = 0.434294481903251827651f;
    const float Math::LN2 = 0.693147180559945309417f;
    const float Math::LN10 = 2.3025;

    Math::ZERO = Vector2(0, 0);
    Math::ONE = Vector2(1, 1);
    Math::UP = Vector2(0, 1);
    Math::DOWN = Vector2(0, -1);
    Math::LEFT = Vector2(-1, 0);
    Math::RIGHT = Vector2(1, 0);
    Math::ZERO3 = Vector3(0, 0, 0);
    Math::ONE3 = Vector3(1, 1, 1);
    Math::UP3 = Vector3(0, 1, 0);
    Math::DOWN3 = Vector3(0, -1, 0);
    Math::LEFT3 = Vector3(-1, 0, 0);
    Math::RIGHT3 = Vector3(1, 0, 0);
    Math::ZERO4 = Vector4(0, 0, 0, 0);
    Math::ONE4 = Vector4(1, 1, 1, 1);
    Math::UP4 = Vector4(0, 1, 0, 1);
    Math::DOWN4 = Vector4(0, -1, 0, 1);
    Math::LEFT4 = Vector4(-1, 0, 0, 1);
    Math::RIGHT4 = Vector4(1, 0, 0, 1);
    Math::FORWARD = Vector4(0, 0, -1, 0);
    Math::BACK = Vector4(0, 0, 1, 0);
    Math::ZEROV = Vector4(0, 0, 0, 0);
    Math::ONEV = Vector4(1, 1, 1, 1);
    Math::UPV = Vector4(0, 1, 0, 1);
    Math::DOWNV = Vector4(0, -1, 0, 1);
    Math::PI = 3.14159265358979323846f;
    Math::EPSILON = 0.00001f;
    Math::DEG2RAD = PI / 180.0f;
    Math::RAD2DEG = 180.0f / PI;
    Math::INFINITY = std::numeric_limits<float>::infinity();
    Math::NAN = std::numeric_limits<float>::quiet_NaN();
    Math::SQRT2 = 1.41421356237309504880f;
    Math::SQRT3 = 1.73205080756887729352f;
    Math::E = 2.71828182845904523536f;
    Math::GOLDEN_RATIO = (1 + sqrt(5)) / 2;
    Math::PHI = (1 + sqrt(5)) / 2;
    Math::TAU = 2 * PI;
    Math::LOG2E = 1.44269504088896340736f;
    Math::LOG10E = 0.434294481903251827651f;
    Math::LN2 = 0.693147180559945309417f;
    Math::LN10 = 2.3025;

    // Dot Product

    Math::dot(Vector2 a, Vector2 b) {
        return a.x * b.x + a.y * b.y;
    }

    Math::dot(Vector3 a, Vector3 b) {
        return a.x * b.x + a.y * b.y + a.z * b.z;
    }

    Math::dot(Vector4 a, Vector4 b) {
        return a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w;
    }

    // Cross Product

    Math::cross(Vector2 a, Vector2 b) {
        return Vector2(a.y * b.x - a.x * b.y, a.x * b.y - a.y * b.x);
    }

    Math::cross(Vector3 a, Vector3 b) {
        return Vector3(a.y * b.z - a.z * b.y, a.z * b.x - a.x * b.z, a.x * b.y - a.y * b.x);
    }

    Math::cross(Vector4 a, Vector4 b) {
        return Vector4(a.y * b.z - a.z * b.y, a.z * b.x - a.x * b.z, a.x * b.y - a.y * b.x, a.w * b.x - a.x * b.w - a.y * b.z + a.z * b.y);
    }

    // Length

    Math::length(Vector2 v) {
        return sqrt(dot(v, v));
    }

    Math::length(Vector3 v) {
        return sqrt(dot(v, v));
    }

    Math::length(Vector4 v) {
        return sqrt(dot(v, v));
    }

    // Distance

    Math::distance(Vector2 a, Vector2 b) {
        return length(a - b);
    }

    Math::distance(Vector3 a, Vector3 b) {
        return length(a - b);
    }

    Math::distance(Vector4 a, Vector4 b) {
        return length(a - b);
    }

    // Normalize

    Math::normalize(Vector2 v) {
        return v / length(v);
    }

    Math::normalize(Vector3 v) {
        return v / length(v);
    }

    Math::normalize(Vector4 v) {
        return v / length(v);
    }

    // Is NaN

    bool Math::isNaN(Vector2 v) {
        return isNaN(v.x) || isNaN(v.y);
    }

    bool Math::isNaN(Vector3 v) {
        return isNaN(v.x) || isNaN(v.y) || isNaN(v.z);
    }

    bool Math::isNaN(Vector4 v) {
        return isNaN(v.x) || isNaN(v.y) || isNaN(v.z) || isNaN(v.w);
    }

    // Is Infinity

    bool Math::isInfinity(Vector2 v) {
        return isInfinity(v.x) || isInfinity(v.y);
    }

    bool Math::isInfinity(Vector3 v) {
        return isInfinity(v.x) || isInfinity(v.y) || isInfinity(v.z);
    }

    bool Math::isInfinity(Vector4 v) {
        return isInfinity(v.x) || isInfinity(v.y) || isInfinity(v.z) || isInfinity(v.w);
    }

    // Is Finite

    bool Math::isFinite(Vector2 v) {
        return isFinite(v.x) && isFinite(v.y);
    }

    bool Math::isFinite(Vector3 v) {
        return isFinite(v.x) && isFinite(v.y) && isFinite(v.z);
    }

    bool Math::isFinite(Vector4 v) {
        return isFinite(v.x) && isFinite(v.y) && isFinite(v.z) && isFinite(v.w);
    }

    // Is Equal (Vectors)

    bool Math::isEqual(Vector2 a, Vector2 b) {
        return isEqual(a.x, b.x) && isEqual(a.y, b.y);
    }

    bool Math::isEqual(Vector3 a, Vector3 b) {
        return isEqual(a.x, b.x) && isEqual(a.y, b.y) && isEqual(a.z, b.z);
    }

    bool Math::isEqual(Vector4 a, Vector4 b) {
        return isEqual(a.x, b.x) && isEqual(a.y, b.y) && isEqual(a.z, b.z) && isEqual(a.w, b.w);
    }

    // Is Greater (Vectors)

    bool Math::isGreater(Vector2 a, Vector2 b) {
        return a.x > b.x && a.y > b.y;
    }

    bool Math::isGreater(Vector3 a, Vector3 b) {
        return a.x > b.x && a.y > b.y && a.z > b.z;
    }

    bool Math::isGreater(Vector4 a, Vector4 b) {
        return a.x > b.x && a.y > b.y && a.z > b.z && a.w > b.w;
    }

    // Is Greater Equal (Vectors)

    bool Math::isGreaterEqual(Vector2 a, Vector2 b) {
        return a.x >= b.x && a.y >= b.y;
    }

    bool Math::isGreaterEqual(Vector3 a, Vector3 b) {
        return a.x >= b.x && a.y >= b.y && a.z >= b.z;
    }

    bool Math::isGreaterEqual(Vector4 a, Vector4 b) {
        return a.x >= b.x && a.y >= b.y && a.z >= b.z && a.w >= b.w;
    }

    // Is Less (Vectors)

    bool Math::isLess(Vector2 a, Vector2 b) {
        return a.x < b.x && a.y < b.y;
    }

    bool Math::isLess(Vector3 a, Vector3 b) {
        return a.x < b.x && a.y < b.y && a.z < b.z;
    }

    bool Math::isLess(Vector4 a, Vector4 b) {
        return a.x < b.x && a.y < b.y && a.z < b.z && a.w < b.w;
    }

    // Is Less Equal (Vectors)

    bool Math::isLessEqual(Vector2 a, Vector2 b) {
        return a.x <= b.x && a.y <= b.y;
    }

    bool Math::isLessEqual(Vector3 a, Vector3 b) {
        return a.x <= b.x && a.y <= b.y && a.z <= b.z;
    }

    bool Math::isLessEqual(Vector4 a, Vector4 b) {
        return a.x <= b.x && a.y <= b.y && a.z <= b.z && a.w <= b.w;
    }

    // Is Zero (Vectors)

    bool Math::isZero(Vector2 v) {
        return isEqual(v, ZERO);
    }

    bool Math::isZero(Vector3 v) {
        return isEqual(v, ZERO3);
    }

    bool Math::isZero(Vector4 v) {
        return isEqual(v, ZERO4);
    }

    // Is Not Zero (Vectors)

    bool Math::isNotZero(Vector2 v) {
        return !isZero(v);
    }

    bool Math::isNotZero(Vector3 v) {
        return !isZero(v);
    }

    bool Math::isNotZero(Vector4 v) {
        return !isZero(v);
    }

    // Is Positive (Vectors)

    bool Math::isPositive(Vector2 v) {
        return v.x > 0 && v.y > 0;
    }

    bool Math::isPositive(Vector3 v) {
        return v.x > 0 && v.y > 0 && v.z > 0;
    }

    bool Math::isPositive(Vector4 v) {
        return v.x > 0 && v.y > 0 && v.z > 0 && v.w > 0;
    }

    // Is Negative (Vectors)

    bool Math::isNegative(Vector2 v) {
        return v.x < 0 && v.y < 0;
    }

    bool Math::isNegative(Vector3 v) {
        return v.x < 0 && v.y < 0 && v.z < 0;
    }

    bool Math::isNegative(Vector4 v) {
        return v.x < 0 && v.y < 0 && v.z < 0 && v.w < 0;
    }

    float Math::random() {
        return rand() / (float) RAND_MAX;
    }

    float Math::random(float min, float max) {
        return min + (max - min) * random();
    }

    float Math::abs(float value) {
        return value < 0 ? -value : value;
    }

    float Math::sign(float value) {
        return value < 0 ? -1 : 1;
    }

    float Math::floor(float value) {
        return (float) floor(value);
    }

    float Math::ceil(float value) {
        return (float) ceil(value);
    }

    float Math::round(float value) {
        return (float) round(value);
    }

    float Math::frac(float value) {
        return value - floor(value);
    }

    float Math::mod(float x, float y) {
        return x - y * floor(x / y);
    }

    float Math::min(float a, float b) {
        return a < b ? a : b;
    }

    float Math::max(float a, float b) {
        return a > b ? a : b;
    }

    float Math::pow(float x, float y) {
        return powf(x, y);
    }

    float Math::sqrt(float x) {
        return sqrtf(x);
    }

    float Math::sin(float x) {
        return sinf(x);
    }

    float Math::cos(float x) {
        return cosf(x);
    }

    float Math::tan(float x) {
        return tanf(x);
    }

    float Math::asin(float x) {
        return asinf(x);
    }

    float Math::acos(float x) {
        return acosf(x);
    }

    float Math::atan(float x) {
        return atanf(x);
    }

    float Math::atan2(float y, float x) {
        return atan2f(y, x);
    }

    float Math::exp(float x) {
        return expf(x);
    }

    float Math::log(float x) {
        return logf(x);
    }

    float Math::log2(float x) {
        return log2f(x);
    }

    float Math::log10(float x) {
        return log10f(x);
    }

    float Math::deg2rad(float degrees) {
        return degrees * DEG2RAD;
    }

    float Math::rad2deg(float radians) {
        return radians * RAD2DEG;
    }

    float Math::lerp(float a, float b, float t) {
        return a * (1 - t) + b * t;
    }

    float Math::clamp(float value, float min, float max) {
        return value < min ? min : value > max ? max : value;
    }

    float Math::smoothstep(float edge0, float edge1, float x) {
        float t = clamp((x - edge0) / (edge1 - edge0), 0.0f, 1.0f);
        return t * t * (3 - 2 * t);
    }

    float Math::smootherstep(float edge0, float edge1, float x) {
        x = clamp((x - edge0) / (edge1 - edge0), 0.0f, 1.0f);
        return x * x * x * (x * (x * 6 - 15) + 10);
    }

    Math::Vector2 Math::ZERO = Vector2(0, 0);
    Math::Vector2 Math::ONE = Vector2(1, 1);
    Math::Vector2 Math::UP = Vector2(0, 1);
    Math::Vector2 Math::DOWN = Vector2(0, -1);
    Math::Vector2 Math::LEFT = Vector2(-1, 0);
    Math::Vector2 Math::RIGHT = Vector2(1, 0);
    Math::Vector3 Math::ZERO3 = Vector3(0, 0, 0);
    Math::Vector3 Math::ONE3 = Vector3(1, 1, 1);
    Math::Vector3 Math::UP3 = Vector3(0, 1, 0);
    Math::Vector3 Math::DOWN3 = Vector3(0, -1, 0);
    Math::Vector3 Math::LEFT3 = Vector3(-1, 0, 0);
    Math::Vector3 Math::RIGHT3 = Vector3(1, 0, 0);
    Math::Vector4 Math::ZERO4 = Vector4(0, 0, 0, 0);
    Math::Vector4 Math::ONE4 = Vector4(1, 1, 1, 1);
    Math::Vector4 Math::UP4 = Vector4(0, 1, 0, 1);
    Math::Vector4 Math::DOWN4 = Vector4(0, -1, 0, 1);
    Math::Vector4 Math::LEFT4 = Vector4(-1, 0, 0, 1);
    Math::Vector4 Math::RIGHT4 = Vector4(1, 0, 0, 1);
    Math::Vector4 Math::FORWARD = Vector4(0, 0, -1, 0);
    Math::Vector4 Math::BACK = Vector4(0, 0, 1, 0);
    Math::Vector4 Math::ZEROV = Vector4(0, 0, 0, 0);
    Math::Vector4 Math::ONEV = Vector4(1, 1, 1, 1);
    Math::Vector4 Math::UPV = Vector4(0, 1, 0, 1);
    Math::Vector4 Math::DOWNV = Vector4(0, -1, 0, 1);
    Math::Vector4 Math::PI = 3.14159265358979323846f;
    Math::Vector4 Math::EPSILON = 0.00001f;
    Math::Vector4 Math::DEG2RAD = PI / 180.0f;
    Math::Vector4 Math::RAD2DEG = 180.0f / PI;
    Math::Vector4 Math::INFINITY = std::numeric_limits<float>::infinity();
    Math::Vector4 Math::NAN = std::numeric_limits<float>::quiet_NaN();
    Math::Vector4 Math::SQRT2 = 1.41421356237309504880f;
    Math::Vector4 Math::SQRT3 = 1.73205080756887729352f;
    Math::Vector4 Math::E = 2.71828182845904523536f;
    Math::Vector4 Math::GOLDEN_RATIO = (1 + sqrt(5)) / 2;
    Math::Vector4 Math::PHI = (1 + sqrt(5)) / 2;
    Math::Vector4 Math::TAU = 2 * PI;
    Math::Vector4 Math::LOG2E = 1.44269504088896340736f;



} // pyg
