//
// Created by Aram Aprahamian on 12/7/25.
//

#ifndef COLOR_H
#define COLOR_H

#include <algorithm> // std::min, std::max, std::clamp
#include <cmath>     // std::pow, std::round, std::fmod
#include <cstdint>   // uint8_t
#include <string>

namespace pyg {

struct Color {
    uint8_t r{0};
    uint8_t g{0};
    uint8_t b{0};
    uint8_t a{255};

    // --- Constructors ---

    constexpr Color() = default;

    constexpr Color(uint8_t r, uint8_t g, uint8_t b, uint8_t a = 255)
        : r(r), g(g), b(b), a(a) {}

    // --- Arithmetic Operators (Color vs Color) ---

    constexpr Color& operator+=(const Color& other) {
        r = qadd(r, other.r);
        g = qadd(g, other.g);
        b = qadd(b, other.b);
        a = qadd(a, other.a);
        return *this;
    }

    constexpr Color& operator-=(const Color& other) {
        r = qsub(r, other.r);
        g = qsub(g, other.g);
        b = qsub(b, other.b);
        a = qsub(a, other.a);
        return *this;
    }

    constexpr Color& operator*=(const Color& other) {
        r = qmul(r, other.r);
        g = qmul(g, other.g);
        b = qmul(b, other.b);
        a = qmul(a, other.a);
        return *this;
    }

    constexpr Color& operator/=(const Color& other) {
        r = qdiv(r, other.r);
        g = qdiv(g, other.g);
        b = qdiv(b, other.b);
        a = qdiv(a, other.a);
        return *this;
    }

    // --- Arithmetic Operators (Color vs Scalar) ---

    Color& operator*=(float scalar) {
        scalar = std::max(0.0f, scalar); // Prevent negative colors
        r = static_cast<uint8_t>(std::min(static_cast<float>(r) * scalar, 255.0f));
        g = static_cast<uint8_t>(std::min(static_cast<float>(g) * scalar, 255.0f));
        b = static_cast<uint8_t>(std::min(static_cast<float>(b) * scalar, 255.0f));
        a = static_cast<uint8_t>(std::min(static_cast<float>(a) * scalar, 255.0f));
        return *this;
    }

    Color& operator/=(float scalar) {
        if (scalar <= 0.00001f) {
            // Handle division by zero/negative by saturating or ignoring
            if (scalar == 0.0f) r = g = b = a = 255;
        } else {
            r = static_cast<uint8_t>(std::min(static_cast<float>(r) / scalar, 255.0f));
            g = static_cast<uint8_t>(std::min(static_cast<float>(g) / scalar, 255.0f));
            b = static_cast<uint8_t>(std::min(static_cast<float>(b) / scalar, 255.0f));
            a = static_cast<uint8_t>(std::min(static_cast<float>(a) / scalar, 255.0f));
        }
        return *this;
    }

    // --- Binary Friends ---

    friend constexpr Color operator+(Color lhs, const Color& rhs) { lhs += rhs; return lhs; }
    friend constexpr Color operator-(Color lhs, const Color& rhs) { lhs -= rhs; return lhs; }
    friend constexpr Color operator*(Color lhs, const Color& rhs) { lhs *= rhs; return lhs; }
    friend constexpr Color operator/(Color lhs, const Color& rhs) { lhs /= rhs; return lhs; }

    // Scalar friends (allow float * Color and Color * float)
    friend Color operator*(Color lhs, float scalar) { lhs *= scalar; return lhs; }
    friend Color operator*(float scalar, Color rhs) { rhs *= scalar; return rhs; }
    friend Color operator/(Color lhs, float scalar) { lhs /= scalar; return lhs; }

    constexpr bool operator==(const Color& other) const {
        return r == other.r && g == other.g && b == other.b && a == other.a;
    }

    constexpr bool operator!=(const Color& other) const {
        return !(*this == other);
    }

    // --- Utilities ---

    [[nodiscard]] std::string toString() const {
        return "RGBA(" + std::to_string(r) + ", " + std::to_string(g) +
               ", " + std::to_string(b) + ", " + std::to_string(a) + ")";
    }

    /**
     * @brief Returns the RGBA value as a hex string (e.g., "#RRGGBBAA").
     *
     * @return std::string The color represented as a hex string.
     */
    [[nodiscard]] std::string toHex() const {
        char buf[10]; // Enough for "#RRGGBBAA" + null terminator
        snprintf(buf, sizeof(buf), "#%02X%02X%02X%02X", r, g, b, a);
        return std::string(buf);
    }

    [[nodiscard]] std::string toRGBHex() const {
        char buf[10]; // Enough for "#RRGGBB" + null terminator
        snprintf(buf, sizeof(buf), "#%02X%02X%02X", r, g, b);
        return std::string(buf);
    }

    static Color lerp(const Color& a, const Color& b, float t) {
        t = std::clamp(t, 0.0f, 1.0f);
        return Color(
            static_cast<uint8_t>(a.r + (b.r - a.r) * t),
            static_cast<uint8_t>(a.g + (b.g - a.g) * t),
            static_cast<uint8_t>(a.b + (b.b - a.b) * t),
            static_cast<uint8_t>(a.a + (b.a - a.a) * t)
        );
    }

    // --- Conversions ---

    [[nodiscard]] Color SRGB() const {
        return { linearToSRGB(r), linearToSRGB(g), linearToSRGB(b), a };
    }

    [[nodiscard]] Color Linear() const {
        return { sRGBToLinear(r), sRGBToLinear(g), sRGBToLinear(b), a };
    }

    [[nodiscard]] Color HSV() const {
        float rn = r / 255.0f, gn = g / 255.0f, bn = b / 255.0f;
        float max = std::max({rn, gn, bn}), min = std::min({rn, gn, bn});
        float delta = max - min;
        float h = 0.0f;
        if (delta > 1e-5f) {
            if (max == rn) h = 60.0f * std::fmod(((gn - bn) / delta), 6.0f);
            else if (max == gn) h = 60.0f * (((bn - rn) / delta) + 2.0f);
            else h = 60.0f * (((rn - gn) / delta) + 4.0f);
        }
        if (h < 0.0f) h += 360.0f;
        float s = (max == 0.0f) ? 0.0f : (delta / max);
        return {
            static_cast<uint8_t>((h / 360.0f) * 255.0f),
            static_cast<uint8_t>(s * 255.0f),
            static_cast<uint8_t>(max * 255.0f),
            a
        };
    }

    [[nodiscard]] Color CMYK() const {
        float rn = r / 255.0f, gn = g / 255.0f, bn = b / 255.0f;
        float k = 1.0f - std::max({rn, gn, bn});
        float den = 1.0f - k;
        float c = 0.0f, m = 0.0f, y = 0.0f;
        if (den > 1e-5f) {
            c = (1.0f - rn - k) / den;
            m = (1.0f - gn - k) / den;
            y = (1.0f - bn - k) / den;
        }
        return {
            static_cast<uint8_t>(c * 255.0f),
            static_cast<uint8_t>(m * 255.0f),
            static_cast<uint8_t>(y * 255.0f),
            static_cast<uint8_t>(k * 255.0f)
        };
    }

private:
    // Helper helpers
    static constexpr uint8_t qadd(uint8_t a, uint8_t b) { return (a + b > 255) ? 255 : (a + b); }
    static constexpr uint8_t qsub(uint8_t a, uint8_t b) { return (a - b < 0) ? 0 : (a - b); }
    static constexpr uint8_t qmul(uint8_t a, uint8_t b) { return (a * b > 255) ? 255 : (a * b); }
    static constexpr uint8_t qdiv(uint8_t a, uint8_t b) { return (b == 0) ? 255 : std::min(255, a / b); }

    static uint8_t linearToSRGB(uint8_t c) {
        float v = c / 255.0f;
        v = (v <= 0.0031308f) ? (v * 12.92f) : (1.055f * std::pow(v, 1.0f / 2.4f) - 0.055f);
        return static_cast<uint8_t>(std::clamp(v * 255.0f, 0.0f, 255.0f));
    }

    static uint8_t sRGBToLinear(uint8_t c) {
        float v = c / 255.0f;
        v = (v <= 0.04045f) ? (v / 12.92f) : std::pow((v + 0.055f) / 1.055f, 2.4f);
        return static_cast<uint8_t>(std::clamp(v * 255.0f, 0.0f, 255.0f));
    }
};

} // namespace pyg

#endif // COLOR_H
