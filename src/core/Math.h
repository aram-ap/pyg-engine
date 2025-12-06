//
// Created by Aram Aprahamian on 11/22/25.
//

#pragma once
#include <cmath>
#include <cfloat>
#include <string>

#ifndef MATH_H
#define MATH_H

namespace pyg {

class Math {
    private:

    public:
        static constexpr float PI = 3.14159265358979323846f;
        static constexpr float EPSILON = 0.00001f;
        static constexpr float DEG2RAD = PI / 180.0f;
        static constexpr float RAD2DEG = 180.0f / PI;
        static constexpr float SQRT2 = 1.41421356237309504880f;
        static constexpr float SQRT3 = 1.73205080756887729352f;
        static constexpr float E = 2.71828182845904523536f;
        static constexpr float GOLDEN_RATIO = (1 + 2.23606797749979f) / 2;
        static constexpr float PHI = (1 + 2.23606797749979f) / 2;
        static constexpr float TAU = 2 * PI;
        static constexpr float LOG2E = 1.44269504088896340736f;
        static constexpr float LOG10E = 0.434294481903251827651f;
        static constexpr float LN2 = 0.693147180559945309417f;
        static constexpr float LN10 = 2.3025;
        static constexpr float INVSQRT2 = 0.707106781186547524401f;
        static constexpr float INVSQRT3 = 0.577350269189625764509f;

        static bool isNaN(float value);
        static bool isInfinity(float value);
        static bool isFinite(float value);
        static bool isEqual(float a, float b);
        static bool isEqual(float a, float b, float epsilon);
        static bool isGreater(float a, float b);
        static bool isGreaterEqual(float a, float b);
        static bool isLess(float a, float b);
        static bool isLessEqual(float a, float b);
        static bool isZero(float value);
        static bool isNotZero(float value);
        static bool isPositive(float value);
        static bool isNegative(float value);

        static float random();
        static float random(float min, float max);
        static float abs(float value);
        static float sign(float value);
        static float floor(float value);
        static float ceil(float value);
        static float round(float value);
        static float frac(float value);
        static float mod(float x, float y);
        static float min(float a, float b);
        static float max(float a, float b);
        static float pow(float x, float y);
        static float sqrt(float x);
        static float sin(float x);
        static float cos(float x);
        static float tan(float x);
        static float asin(float x);
        static float acos(float x);
        static float atan(float x);
        static float atan2(float y, float x);
        static float exp(float x);
        static float log(float x);
        static float log2(float x);
        static float log10(float x);
        static float deg2rad(float degrees);
        static float rad2deg(float radians);
        static float lerp(float a, float b, float t);
        static float clamp(float value, float min, float max);
        static float smoothstep(float edge0, float edge1, float x);
        static float smootherstep(float edge0, float edge1, float x);
};

} // pyg

#endif //MATH_H
