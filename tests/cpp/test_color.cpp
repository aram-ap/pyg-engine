/**
 * @file test_color.cpp
 * @brief Unit tests for Color class
 *
 * Tests color creation, manipulation, and operations.
 */

#include <gtest/gtest.h>
#include "datatypes/Color.h"

using namespace pyg;

/**
 * @brief Test fixture for Color class
 */
class ColorTest : public ::testing::Test {
protected:
    void SetUp() override {
        // Setup code if needed
    }

    void TearDown() override {
        // Cleanup code if needed
    }
};

// Test color creation
TEST_F(ColorTest, TestColorCreation) {
    Color c1;
    EXPECT_EQ(c1.r, 0);
    EXPECT_EQ(c1.g, 0);
    EXPECT_EQ(c1.b, 0);
    EXPECT_EQ(c1.a, 255);

    Color c2(255, 128, 64);
    EXPECT_EQ(c2.r, 255);
    EXPECT_EQ(c2.g, 128);
    EXPECT_EQ(c2.b, 64);
    EXPECT_EQ(c2.a, 255);

    Color c3(100, 150, 200, 250);
    EXPECT_EQ(c3.r, 100);
    EXPECT_EQ(c3.g, 150);
    EXPECT_EQ(c3.b, 200);
    EXPECT_EQ(c3.a, 250);
}

// Test color addition
TEST_F(ColorTest, TestColorAddition) {
    Color c1(100, 50, 25, 200);
    Color c2(50, 25, 10, 50);
    Color result = c1 + c2;

    EXPECT_EQ(result.r, 150);
    EXPECT_EQ(result.g, 75);
    EXPECT_EQ(result.b, 35);
    EXPECT_EQ(result.a, 250);
}

// Test color addition with saturation
TEST_F(ColorTest, TestColorAdditionSaturation) {
    Color c1(200, 200, 200, 200);
    Color c2(100, 100, 100, 100);
    Color result = c1 + c2;

    // Should saturate at 255
    EXPECT_EQ(result.r, 255);
    EXPECT_EQ(result.g, 255);
    EXPECT_EQ(result.b, 255);
    EXPECT_EQ(result.a, 255);
}

// Test color subtraction
TEST_F(ColorTest, TestColorSubtraction) {
    Color c1(100, 80, 60, 200);
    Color c2(50, 30, 10, 50);
    Color result = c1 - c2;

    EXPECT_EQ(result.r, 50);
    EXPECT_EQ(result.g, 50);
    EXPECT_EQ(result.b, 50);
    EXPECT_EQ(result.a, 150);
}

// Test color subtraction with underflow
TEST_F(ColorTest, TestColorSubtractionUnderflow) {
    Color c1(50, 30, 10, 100);
    Color c2(100, 80, 60, 150);
    Color result = c1 - c2;

    // Should not go below 0
    EXPECT_EQ(result.r, 0);
    EXPECT_EQ(result.g, 0);
    EXPECT_EQ(result.b, 0);
    EXPECT_EQ(result.a, 0);
}

// Test scalar multiplication
TEST_F(ColorTest, TestScalarMultiplication) {
    Color c(100, 50, 25, 200);
    Color result = c * 2.0f;

    EXPECT_EQ(result.r, 200);
    EXPECT_EQ(result.g, 100);
    EXPECT_EQ(result.b, 50);
    EXPECT_EQ(result.a, 255); // Saturates at 255
}

// Test scalar division
TEST_F(ColorTest, TestScalarDivision) {
    Color c(200, 100, 50, 240);
    Color result = c / 2.0f;

    EXPECT_EQ(result.r, 100);
    EXPECT_EQ(result.g, 50);
    EXPECT_EQ(result.b, 25);
    EXPECT_EQ(result.a, 120);
}

// Test color equality
TEST_F(ColorTest, TestColorEquality) {
    Color c1(100, 150, 200, 250);
    Color c2(100, 150, 200, 250);
    Color c3(101, 150, 200, 250);

    EXPECT_TRUE(c1 == c2);
    EXPECT_FALSE(c1 == c3);
    EXPECT_TRUE(c1 != c3);
}

// Test color lerp
TEST_F(ColorTest, TestColorLerp) {
    Color c1(0, 0, 0, 0);
    Color c2(100, 200, 255, 255);

    Color mid = Color::lerp(c1, c2, 0.5f);
    EXPECT_EQ(mid.r, 50);
    EXPECT_EQ(mid.g, 100);

    Color start = Color::lerp(c1, c2, 0.0f);
    EXPECT_EQ(start.r, 0);
    EXPECT_EQ(start.g, 0);

    Color end = Color::lerp(c1, c2, 1.0f);
    EXPECT_EQ(end.r, 100);
    EXPECT_EQ(end.g, 200);
}

// Test toString method
TEST_F(ColorTest, TestToString) {
    Color c(255, 128, 64, 255);
    std::string str = c.toString();

    EXPECT_FALSE(str.empty());
    EXPECT_NE(str.find("255"), std::string::npos);
    EXPECT_NE(str.find("128"), std::string::npos);
}

// Test toHex method
TEST_F(ColorTest, TestToHex) {
    Color c(255, 128, 64, 255);
    std::string hex = c.toHex();

    EXPECT_EQ(hex, "#FF8040FF");
}

// Test toRGBHex method
TEST_F(ColorTest, TestToRGBHex) {
    Color c(255, 128, 64, 255);
    std::string hex = c.toRGBHex();

    EXPECT_EQ(hex, "#FF8040");
}

// Test compound assignment operators
TEST_F(ColorTest, TestCompoundAssignment) {
    Color c(100, 100, 100, 100);
    c += Color(50, 50, 50, 50);

    EXPECT_EQ(c.r, 150);
    EXPECT_EQ(c.g, 150);
    EXPECT_EQ(c.b, 150);
    EXPECT_EQ(c.a, 150);
}

// Test edge cases
TEST_F(ColorTest, TestEdgeCases) {
    // All black
    Color black(0, 0, 0, 0);
    EXPECT_EQ(black.r, 0);
    EXPECT_EQ(black.a, 0);

    // All white
    Color white(255, 255, 255, 255);
    EXPECT_EQ(white.r, 255);
    EXPECT_EQ(white.a, 255);

    // Negative scalar multiplication should not produce negative colors
    Color c(100, 100, 100, 100);
    Color result = c * -1.0f;
    EXPECT_EQ(result.r, 0);
}

