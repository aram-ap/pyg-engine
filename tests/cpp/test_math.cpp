/**
 * @file test_math.cpp
 * @brief Unit tests for Math utilities
 *
 * Tests mathematical functions, constants, and utilities.
 */

#include <gtest/gtest.h>
#include "core/Math.h"

using namespace pyg;

/**
 * @brief Test fixture for Math class
 */
class MathTest : public ::testing::Test {
protected:
    void SetUp() override {
        // Setup code if needed
    }

    void TearDown() override {
        // Cleanup code if needed
    }
};

// Test mathematical constants
TEST_F(MathTest, TestConstants) {
    EXPECT_FLOAT_EQ(Math::PI, 3.14159265358979323846f);
    EXPECT_FLOAT_EQ(Math::E, 2.71828182845904523536f);
    EXPECT_FLOAT_EQ(Math::TAU, 2 * Math::PI);
    EXPECT_GT(Math::EPSILON, 0.0f);
}

// Test clamp function
TEST_F(MathTest, TestClampFloat) {
    EXPECT_FLOAT_EQ(Math::clamp_float(5.0f, 0.0f, 10.0f), 5.0f);
    EXPECT_FLOAT_EQ(Math::clamp_float(-5.0f, 0.0f, 10.0f), 0.0f);
    EXPECT_FLOAT_EQ(Math::clamp_float(15.0f, 0.0f, 10.0f), 10.0f);
}

TEST_F(MathTest, TestClampInt) {
    EXPECT_EQ(Math::clamp_int(128, 0, 255), 128);
    EXPECT_EQ(Math::clamp_int(-10, 0, 255), 0);
    EXPECT_EQ(Math::clamp_int(300, 0, 255), 255);
}

// Test absolute value
TEST_F(MathTest, TestAbs) {
    EXPECT_FLOAT_EQ(Math::abs(5.0f), 5.0f);
    EXPECT_FLOAT_EQ(Math::abs(-5.0f), 5.0f);
    EXPECT_FLOAT_EQ(Math::abs(0.0f), 0.0f);
}

// Test min/max
TEST_F(MathTest, TestMinMax) {
    EXPECT_FLOAT_EQ(Math::min(3.0f, 5.0f), 3.0f);
    EXPECT_FLOAT_EQ(Math::max(3.0f, 5.0f), 5.0f);
    EXPECT_FLOAT_EQ(Math::min(-2.0f, -1.0f), -2.0f);
}

// Test trigonometric functions
TEST_F(MathTest, TestTrigonometry) {
    EXPECT_NEAR(Math::sin(0.0f), 0.0f, Math::EPSILON);
    EXPECT_NEAR(Math::cos(0.0f), 1.0f, Math::EPSILON);
    EXPECT_NEAR(Math::sin(Math::PI / 2), 1.0f, Math::EPSILON);
    EXPECT_NEAR(Math::cos(Math::PI / 2), 0.0f, Math::EPSILON);
}

// Test degree/radian conversion
TEST_F(MathTest, TestAngleConversion) {
    EXPECT_NEAR(Math::deg2rad(180.0f), Math::PI, Math::EPSILON);
    EXPECT_NEAR(Math::rad2deg(Math::PI), 180.0f, Math::EPSILON);
    EXPECT_NEAR(Math::deg2rad(90.0f), Math::PI / 2, Math::EPSILON);
}

// Test lerp (linear interpolation)
TEST_F(MathTest, TestLerp) {
    EXPECT_FLOAT_EQ(Math::lerp(0.0f, 10.0f, 0.0f), 0.0f);
    EXPECT_FLOAT_EQ(Math::lerp(0.0f, 10.0f, 1.0f), 10.0f);
    EXPECT_FLOAT_EQ(Math::lerp(0.0f, 10.0f, 0.5f), 5.0f);
}

// Test comparison functions
TEST_F(MathTest, TestComparisons) {
    EXPECT_TRUE(Math::isEqual(1.0f, 1.0f));
    EXPECT_FALSE(Math::isEqual(1.0f, 2.0f));
    EXPECT_TRUE(Math::isZero(0.0f));
    EXPECT_FALSE(Math::isZero(0.1f));
    EXPECT_TRUE(Math::isPositive(1.0f));
    EXPECT_FALSE(Math::isPositive(-1.0f));
    EXPECT_TRUE(Math::isNegative(-1.0f));
    EXPECT_FALSE(Math::isNegative(1.0f));
}

// Test power and sqrt
TEST_F(MathTest, TestPowerAndSqrt) {
    EXPECT_FLOAT_EQ(Math::pow(2.0f, 3.0f), 8.0f);
    EXPECT_FLOAT_EQ(Math::sqrt(4.0f), 2.0f);
    EXPECT_FLOAT_EQ(Math::sqrt(9.0f), 3.0f);
}

// Test floor, ceil, round
TEST_F(MathTest, TestRounding) {
    EXPECT_FLOAT_EQ(Math::floor(3.7f), 3.0f);
    EXPECT_FLOAT_EQ(Math::ceil(3.2f), 4.0f);
    EXPECT_FLOAT_EQ(Math::round(3.5f), 4.0f);
    EXPECT_FLOAT_EQ(Math::round(3.4f), 3.0f);
}

// Test sign
TEST_F(MathTest, TestSign) {
    EXPECT_FLOAT_EQ(Math::sign(5.0f), 1.0f);
    EXPECT_FLOAT_EQ(Math::sign(-5.0f), -1.0f);
}

// Test smoothstep
TEST_F(MathTest, TestSmoothstep) {
    EXPECT_FLOAT_EQ(Math::smoothstep(0.0f, 1.0f, 0.0f), 0.0f);
    EXPECT_FLOAT_EQ(Math::smoothstep(0.0f, 1.0f, 1.0f), 1.0f);
    float mid = Math::smoothstep(0.0f, 1.0f, 0.5f);
    EXPECT_GT(mid, 0.0f);
    EXPECT_LT(mid, 1.0f);
}

// Test edge cases
TEST_F(MathTest, TestEdgeCases) {
    EXPECT_TRUE(Math::isFinite(1.0f));
    EXPECT_FALSE(Math::isNaN(1.0f));
    
    // Division by very small number shouldn't cause NaN in our functions
    float result = Math::clamp_float(100.0f, 0.0f, 50.0f);
    EXPECT_FALSE(Math::isNaN(result));
}

