//
// Created by Aram Aprahamian on 11/22/25.
//

#include "Math.h"
#include <cmath>

namespace pyg {
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
        return std::floor(value);
    }

    float Math::ceil(float value) {
        return std::ceil(value);
    }

    float Math::round(float value) {
        return std::round(value);
    }

    int Math::clamp_int(int value, int min, int max) {
        return value < min ? min : value > max ? max : value;
    }

    float Math::clamp_float(float value, float min, float max) {
        return value < min ? min : value > max ? max : value;
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

    bool Math::isNaN(float value) {
        return std::isnan(value);
    }
    bool Math::isInfinity(float value) {
        return std::isinf(value);
    }
    bool Math::isFinite(float value) {
        return std::isfinite(value);
    }
    bool Math::isEqual(float a, float b) {
        return std::abs(a - b) < Math::EPSILON;
    }
    bool Math::isEqual(float a, float b, float epsilon) {
        return std::abs(a - b) < epsilon;
    }
    bool Math::isGreater(float a, float b) {
        return a > b;
    }
    bool Math::isGreaterEqual(float a, float b) {
        return a >= b;
    }
    bool Math::isLess(float a, float b) {
        return a < b;
    }
    bool Math::isLessEqual(float a, float b) {
        return a <= b;
    }
    bool Math::isZero(float value) {
        return std::abs(value) < Math::EPSILON;
    }
    bool Math::isNotZero(float value) {
        return std::abs(value) >= Math::EPSILON;
    }
    bool Math::isPositive(float value) {
        return value > 0;
    }
    bool Math::isNegative(float value) {
        return value < 0;
    }

} // pyg
