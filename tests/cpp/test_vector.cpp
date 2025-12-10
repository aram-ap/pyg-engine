/**
 * @file test_vector.cpp
 * @brief Unit tests for Vector class
 *
 * Tests vector operations and mathematical functions.
 */

#include <gtest/gtest.h>
#include "datatypes/Vector.h"

using namespace pyg;

/**
 * @brief Test fixture for Vector class
 */
class VectorTest : public ::testing::Test {
protected:
    void SetUp() override {
        // Setup code if needed
    }

    void TearDown() override {
        // Cleanup code if needed
    }
};

// Test Vector2 creation
TEST_F(VectorTest, TestVector2Creation) {
    Vector<2> v1;
    EXPECT_FLOAT_EQ(v1[0], 0.0f);
    EXPECT_FLOAT_EQ(v1[1], 0.0f);

    Vector<2> v2(3.0f, 4.0f);
    EXPECT_FLOAT_EQ(v2[0], 3.0f);
    EXPECT_FLOAT_EQ(v2[1], 4.0f);
}

// Test Vector3 creation
TEST_F(VectorTest, TestVector3Creation) {
    Vector<3> v1;
    EXPECT_FLOAT_EQ(v1[0], 0.0f);
    EXPECT_FLOAT_EQ(v1[1], 0.0f);
    EXPECT_FLOAT_EQ(v1[2], 0.0f);

    Vector<3> v2(1.0f, 2.0f, 3.0f);
    EXPECT_FLOAT_EQ(v2[0], 1.0f);
    EXPECT_FLOAT_EQ(v2[1], 2.0f);
    EXPECT_FLOAT_EQ(v2[2], 3.0f);
}

// Test Vector4 creation
TEST_F(VectorTest, TestVector4Creation) {
    Vector<4> v(1.0f, 2.0f, 3.0f, 4.0f);
    EXPECT_FLOAT_EQ(v[0], 1.0f);
    EXPECT_FLOAT_EQ(v[1], 2.0f);
    EXPECT_FLOAT_EQ(v[2], 3.0f);
    EXPECT_FLOAT_EQ(v[3], 4.0f);
}

// Test vector addition
TEST_F(VectorTest, TestVectorAddition) {
    Vector<3> v1(1.0f, 2.0f, 3.0f);
    Vector<3> v2(4.0f, 5.0f, 6.0f);
    Vector<3> result = v1 + v2;

    EXPECT_FLOAT_EQ(result[0], 5.0f);
    EXPECT_FLOAT_EQ(result[1], 7.0f);
    EXPECT_FLOAT_EQ(result[2], 9.0f);
}

// Test vector subtraction
TEST_F(VectorTest, TestVectorSubtraction) {
    Vector<3> v1(10.0f, 8.0f, 6.0f);
    Vector<3> v2(1.0f, 2.0f, 3.0f);
    Vector<3> result = v1 - v2;

    EXPECT_FLOAT_EQ(result[0], 9.0f);
    EXPECT_FLOAT_EQ(result[1], 6.0f);
    EXPECT_FLOAT_EQ(result[2], 3.0f);
}

// Test scalar multiplication
TEST_F(VectorTest, TestScalarMultiplication) {
    Vector<3> v(2.0f, 3.0f, 4.0f);
    Vector<3> result = v * 2.0f;

    EXPECT_FLOAT_EQ(result[0], 4.0f);
    EXPECT_FLOAT_EQ(result[1], 6.0f);
    EXPECT_FLOAT_EQ(result[2], 8.0f);
}

// Test scalar division
TEST_F(VectorTest, TestScalarDivision) {
    Vector<3> v(10.0f, 20.0f, 30.0f);
    Vector<3> result = v / 2.0f;

    EXPECT_FLOAT_EQ(result[0], 5.0f);
    EXPECT_FLOAT_EQ(result[1], 10.0f);
    EXPECT_FLOAT_EQ(result[2], 15.0f);
}

// Test dot product
TEST_F(VectorTest, TestDotProduct) {
    Vector<3> v1(1.0f, 2.0f, 3.0f);
    Vector<3> v2(4.0f, 5.0f, 6.0f);
    float dot = v1.dot(v2);

    // 1*4 + 2*5 + 3*6 = 4 + 10 + 18 = 32
    EXPECT_FLOAT_EQ(dot, 32.0f);
}

// Test vector length
TEST_F(VectorTest, TestVectorLength) {
    Vector<3> v(3.0f, 4.0f, 0.0f);
    float length = v.length();

    // sqrt(3^2 + 4^2 + 0^2) = sqrt(9 + 16) = sqrt(25) = 5
    EXPECT_FLOAT_EQ(length, 5.0f);
}

// Test vector length for unit vector
TEST_F(VectorTest, TestUnitVectorLength) {
    Vector<3> v(1.0f, 0.0f, 0.0f);
    float length = v.length();

    EXPECT_FLOAT_EQ(length, 1.0f);
}

// Test vector multiplication (component-wise)
TEST_F(VectorTest, TestVectorMultiplication) {
    Vector<3> v1(2.0f, 3.0f, 4.0f);
    Vector<3> v2(5.0f, 6.0f, 7.0f);
    Vector<3> result = v1 * v2;

    EXPECT_FLOAT_EQ(result[0], 10.0f);
    EXPECT_FLOAT_EQ(result[1], 18.0f);
    EXPECT_FLOAT_EQ(result[2], 28.0f);
}

// Test vector division (component-wise)
TEST_F(VectorTest, TestVectorDivision) {
    Vector<3> v1(10.0f, 20.0f, 30.0f);
    Vector<3> v2(2.0f, 4.0f, 5.0f);
    Vector<3> result = v1 / v2;

    EXPECT_FLOAT_EQ(result[0], 5.0f);
    EXPECT_FLOAT_EQ(result[1], 5.0f);
    EXPECT_FLOAT_EQ(result[2], 6.0f);
}

// Test toString
TEST_F(VectorTest, TestToString) {
    Vector<3> v(1.5f, 2.5f, 3.5f);
    std::string str = v.toString();

    EXPECT_FALSE(str.empty());
    EXPECT_NE(str.find("1.5"), std::string::npos);
}

// Test out of bounds access
TEST_F(VectorTest, TestOutOfBoundsAccess) {
    Vector<3> v(1.0f, 2.0f, 3.0f);

    EXPECT_THROW(v[10], std::out_of_range);
}

// Test division by zero
TEST_F(VectorTest, TestDivisionByZero) {
    Vector<3> v(10.0f, 20.0f, 30.0f);

    EXPECT_THROW(v / 0.0f, std::runtime_error);
}

// Test vector division by zero component
TEST_F(VectorTest, TestVectorDivisionByZeroComponent) {
    Vector<3> v1(10.0f, 20.0f, 30.0f);
    Vector<3> v2(2.0f, 0.0f, 5.0f);

    EXPECT_THROW(v1 / v2, std::runtime_error);
}

// Test negative values
TEST_F(VectorTest, TestNegativeValues) {
    Vector<3> v(-1.0f, -2.0f, -3.0f);

    EXPECT_FLOAT_EQ(v[0], -1.0f);
    EXPECT_FLOAT_EQ(v[1], -2.0f);
    EXPECT_FLOAT_EQ(v[2], -3.0f);
}

// Test large dimension vector
TEST_F(VectorTest, TestLargeDimensionVector) {
    Vector<10> v;
    for (size_t i = 0; i < 10; ++i) {
        EXPECT_FLOAT_EQ(v[i], 0.0f);
    }
}

