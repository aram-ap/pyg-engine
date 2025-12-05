//
// Created by Aram Aprahamian on 11/22/25.
//

#ifndef MATH_H
#define MATH_H

namespace pyg {

class Math {
    public:
        /* Vector2
         *
         * x = x-component
         * y = y-component
         */
        struct Vector2 {
            const float x;
            const float y;
        };

        /* Vector3
         *
         * x = x-component
         * y = y-component
         * z = z-component
         */
        struct Vector3 {
            const float x;
            const float y;
            const float z;
        };

        /* Vector4
         *
         * x = x-component
         * y = y-component
         * z = z-component
         * w = w-component
         */
        struct Vector4 {
            const float x;
            const float y;
            const float z;
            const float w;
        };

        Math();
        virtual ~Math();
        static float PI;
        static float EPSILON;
        static float DEG2RAD;
        static float RAD2DEG;
        static float INFINITY;
        static float NAN;
        static float SQRT2;
        static float SQRT3;
        static float E;
        static float GOLDEN_RATIO;
        static float PHI;
        static float TAU;
        static float LOG2E;
        static float LOG10E;
        static float LN2;
        static float LN10;
        static float INVSQRT2;
        static float INVSQRT3;

        static Vector2 ZERO;
        static Vector2 ONE;
        static Vector2 UP;
        static Vector2 DOWN;
        static Vector2 LEFT;
        static Vector2 RIGHT;

        static Vector3 ZERO3;
        static Vector3 ONE3;
        static Vector3 UP3;
        static Vector3 DOWN3;
        static Vector3 LEFT3;
        static Vector3 RIGHT3;

        static Vector4 ZERO4;
        static Vector4 ONE4;
        static Vector4 UP4;
        static Vector4 DOWN4;
        static Vector4 LEFT4;
        static Vector4 RIGHT4;
        static Vector4 FORWARD;
        static Vector4 BACK;
        static Vector4 ZEROV;
        static Vector4 ONEV;
        static Vector4 UPV;
        static Vector4 DOWNV;

        static float dot(Vector2 a, Vector2 b);
        static float dot(Vector3 a, Vector3 b);
        static float dot(Vector4 a, Vector4 b);

        static Vector2 cross(Vector2 a, Vector2 b);
        static Vector3 cross(Vector3 a, Vector3 b);
        static Vector4 cross(Vector4 a, Vector4 b);

        static float length(Vector2 v);
        static float length(Vector3 v);
        static float length(Vector4 v);

        static float distance(Vector2 a, Vector2 b);
        static float distance(Vector3 a, Vector3 b);
        static float distance(Vector4 a, Vector4 b);

        static Vector2 normalize(Vector2 v);
        static Vector3 normalize(Vector3 v);
        static Vector4 normalize(Vector4 v);

        static bool isNaN(Vector2 v);
        static bool isNaN(Vector3 v);
        static bool isNaN(Vector4 v);

        static bool isInfinity(Vector2 v);
        static bool isInfinity(Vector3 v);
        static bool isInfinity(Vector4 v);

        static bool isFinite(Vector2 v);
        static bool isFinite(Vector3 v);
        static bool isFinite(Vector4 v);

        static bool isEqual(Vector2 a, Vector2 b);
        static bool isEqual(Vector3 a, Vector3 b);
        static bool isEqual(Vector4 a, Vector4 b);

        static bool isGreater(Vector2 a, Vector2 b);
        static bool isGreater(Vector3 a, Vector3 b);
        static bool isGreater(Vector4 a, Vector4 b);

        static bool isGreaterEqual(Vector2 a, Vector2 b);
        static bool isGreaterEqual(Vector3 a, Vector3 b);
        static bool isGreaterEqual(Vector4 a, Vector4 b);

        static bool isLess(Vector2 a, Vector2 b);
        static bool isLess(Vector3 a, Vector3 b);
        static bool isLess(Vector4 a, Vector4 b);

        static bool isLessEqual(Vector2 a, Vector2 b);
        static bool isLessEqual(Vector3 a, Vector3 b);
        static bool isLessEqual(Vector4 a, Vector4 b);

        static bool isZero(Vector2 v);
        static bool isZero(Vector3 v);
        static bool isZero(Vector4 v);

        static bool isNotZero(Vector2 v);
        static bool isNotZero(Vector3 v);
        static bool isNotZero(Vector4 v);

        static bool isPositive(Vector2 v);
        static bool isPositive(Vector3 v);
        static bool isPositive(Vector4 v);

        static bool isNegative(Vector2 v);
        static bool isNegative(Vector3 v);
        static bool isNegative(Vector4 v);

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
