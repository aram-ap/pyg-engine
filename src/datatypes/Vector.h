// Vector.h
// Created by Aram Aprahamian on 12/05/25.
#pragma once

#include <array>
#include <cmath>
#include <sstream>
#include <stdexcept>
#include <string>

namespace pyg {
// A generic fixed-size vector class
template <size_t N, typename T = float> class Vector {
public:
  std::array<T, N> components;

  // Default constructor (zero-initialized)
  Vector() : components{} {}

  // Variadic constructor for list initialization (e.g., Vector(1, 2, 3))
  template <typename... Args>
  Vector(Args... args) : components{static_cast<T>(args)...} {
    static_assert(sizeof...(args) == N,
                  "Number of arguments must match Vector dimension");
  }

  // Element access
  T &operator[](size_t index) {
    if (index >= N)
      throw std::out_of_range("Index out of bounds");
    return components[index];
  }

  const T &operator[](size_t index) const {
    if (index >= N)
      throw std::out_of_range("Index out of bounds");
    return components[index];
  }

  // Addition
  Vector operator+(const Vector &other) const {
    Vector result;
    for (size_t i = 0; i < N; ++i) {
      result.components[i] = components[i] + other.components[i];
    }
    return result;
  }

  // Subtraction
  Vector operator-(const Vector &other) const {
    Vector result;
    for (size_t i = 0; i < N; ++i) {
      result.components[i] = components[i] - other.components[i];
    }
    return result;
  }

  // Scalar multiplication
  Vector operator*(T scalar) const {
    Vector result;
    for (size_t i = 0; i < N; ++i) {
      result.components[i] = components[i] * scalar;
    }
    return result;
  }

  // Scalar division
  Vector operator/(T scalar) const {
    Vector result;
    for (size_t i = 0; i < N; ++i) {
        if (scalar == 0) {
            throw std::runtime_error("Division by zero");
        }
      result.components[i] = components[i] / scalar;
    }
    return result;
  }

  // Vector multiplication
  Vector operator*(const Vector &other) const {
    Vector result;
    for (size_t i = 0; i < N; ++i) {
      result.components[i] = components[i] * other.components[i];
    }
    return result;
  }

  // Vector division
  Vector operator/(const Vector &other) const {
    Vector result;
    for (size_t i = 0; i < N; ++i) {
        if (other.components[i] == 0) {
            throw std::runtime_error("Division by zero");
        }
      result.components[i] = components[i] / other.components[i];
    }
    return result;
  }

  // Dot product: \( a \cdot b = \sum a_i b_i \)
  T dot(const Vector &other) const {
    T sum = 0;
    for (size_t i = 0; i < N; ++i) {
      sum += components[i] * other.components[i];
    }
    return sum;
  }

  // Magnitude (Length): \( ||v|| = \sqrt{v \cdot v} \)
  T length() const { return std::sqrt(dot(*this)); }

  // String representation for Python
  std::string toString() const {
    std::stringstream ss;
    ss << "(";
    for (size_t i = 0; i < N; ++i) {
      ss << components[i] << (i < N - 1 ? ", " : "");
    }
    ss << ")";
    return ss.str();
  }
};

} // namespace pyg
