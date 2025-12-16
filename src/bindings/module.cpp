#include <pybind11/native_enum.h>
#include <pybind11/operators.h>
#include <pybind11/pybind11.h>
#include <pybind11/stl.h>
#include <stdlib.h>

#include "core/Engine.h"
#include "core/Math.h"
#include "datatypes/Vector.h"
#include "datatypes/Color.h"
#include "logging/Logger.h"
#include "rendering/Window.h"

namespace py = pybind11;

// --- Helper to bind Vector<N, T> instantiations ---
template<size_t N, typename T>
void bind_vector(py::module &m, const std::string &name) {
  using Vec = pyg::Vector<N, T>;

  auto class_def = py::class_<Vec>(m, name.c_str())
      .def(py::init<>())
      .def(py::init([](const std::array<T, N> &args) {
        Vec v;
        v.components = args;
        return v;
      }));

  // Add constructors with individual arguments based on dimension
  if constexpr (N == 2) {
    class_def.def(py::init([](T x, T y) { return Vec(x, y); }));
  } else if constexpr (N == 3) {
    class_def.def(py::init([](T x, T y, T z) { return Vec(x, y, z); }));
  } else if constexpr (N == 4) {
    class_def.def(py::init([](T x, T y, T z, T w) { return Vec(x, y, z, w); }));
  }

  class_def
      // Standard Math Operators
      .def(py::self + py::self)
      .def(py::self - py::self)
      .def(py::self * T())
      .def(py::self / T())
      .def(py::self * py::self) // Vector multiplication
      .def(py::self / py::self) // Vector division
      .def(
        "__rmul__", [](const Vec &v, T s) { return v * s; },
        py::is_operator())

      // Comparison Operators
      .def(py::self == py::self)
      .def(py::self != py::self)
      .def("__lt__", [](const Vec &a, const Vec &b) {
        for (size_t i = 0; i < N; ++i) {
          if (a.components[i] >= b.components[i]) return false;
        }
        return true;
      })
      .def("__le__", [](const Vec &a, const Vec &b) {
        for (size_t i = 0; i < N; ++i) {
          if (a.components[i] > b.components[i]) return false;
        }
        return true;
      })
      .def("__gt__", [](const Vec &a, const Vec &b) {
        for (size_t i = 0; i < N; ++i) {
          if (a.components[i] <= b.components[i]) return false;
        }
        return true;
      })
      .def("__ge__", [](const Vec &a, const Vec &b) {
        for (size_t i = 0; i < N; ++i) {
          if (a.components[i] < b.components[i]) return false;
        }
        return true;
      })

      // Hash support
      .def("__hash__", [](const Vec &v) {
        size_t hash = 0;
        for (size_t i = 0; i < N; ++i) {
          hash ^= std::hash<T>{}(v.components[i]) + 0x9e3779b9 + (hash << 6) + (hash >> 2);
        }
        return hash;
      })

      // Copy support
      .def("__copy__", [](const Vec &v) { return Vec(v); })
      .def("__deepcopy__", [](const Vec &v, py::dict) { return Vec(v); })

      // Common Vector Functions
      .def("dot", &Vec::dot)
      .def("length", &Vec::length)

      // Array Access
      .def("__getitem__", [](const Vec &v, size_t i) { return v[i]; })
      .def("__setitem__", [](Vec &v, size_t i, T val) { v[i] = val; })

      // String representation
      .def("__repr__", [name](const Vec &v) {
        std::stringstream ss;
        ss << name << "(";
        for (size_t i = 0; i < N; ++i) {
          if (i == 0) ss << "x=" << v.components[i];
          else if (i == 1) ss << ", y=" << v.components[i];
          else if (i == 2) ss << ", z=" << v.components[i];
          else if (i == 3) ss << ", w=" << v.components[i];
        }
        ss << ")";
        return ss.str();
      })
      .def("__str__", [name](const Vec &v) {
        std::stringstream ss;
        ss << name << "(";
        for (size_t i = 0; i < N; ++i) {
          if (i == 0) ss << "x=" << v.components[i];
          else if (i == 1) ss << ", y=" << v.components[i];
          else if (i == 2) ss << ", z=" << v.components[i];
          else if (i == 3) ss << ", w=" << v.components[i];
        }
        ss << ")";
        return ss.str();
      })

      // Properties
      .def_property(
        "x", [](Vec &v) { return v[0]; }, [](Vec &v, T val) { v[0] = val; })
      .def_property(
        "y", [](Vec &v) { return N > 1 ? v[1] : 0; },
        [](Vec &v, T val) {
          if (N > 1)
            v[1] = val;
        })
      .def_property(
        "z", [](Vec &v) { return N > 2 ? v[2] : 0; },
        [](Vec &v, T val) {
          if (N > 2)
            v[2] = val;
        })
      .def_property(
        "w", [](Vec &v) { return N > 3 ? v[3] : 0; },
        [](Vec &v, T val) {
          if (N > 3)
            v[3] = val;
        });
}

PYBIND11_MODULE(_native, m) {
  m.doc() = "Pyg-Engine native module";

  // Engine Bindings
  py::class_<pyg::Engine>(m, "Engine")
      .def(py::init<>())
      .def_property("tick_rate", &pyg::Engine::get_tick_rate,
                    &pyg::Engine::set_tick_rate)
      .def("get_version", &pyg::Engine::get_version)
      .def("update", &pyg::Engine::update)
      .def("render", &pyg::Engine::render)
      .def("on_destroy", &pyg::Engine::on_destroy)
      .def("log",
           static_cast<void (pyg::Engine::*)(std::string)>(&pyg::Engine::log))
      .def("log_type",
           static_cast<void (pyg::Engine::*)(pyg::Logger::Type, std::string)>(
             &pyg::Engine::log_type))
      .def("is_running", &pyg::Engine::is_running)
      .def("start", &pyg::Engine::start)
      .def("stop", &pyg::Engine::stop)
      .def("pause", &pyg::Engine::pause)
      .def("resume", &pyg::Engine::resume)
      .def("restart", &pyg::Engine::restart)
      .def("exit", &pyg::Engine::exit)
      .def("set_window", &pyg::Engine::set_window)
      .def("get_window", &pyg::Engine::get_window);

  // Logger Bindings
  py::class_<pyg::Logger>(m, "Logger")
      .def_static("init", &pyg::Logger::init, py::arg("name") = "pyg_engine",
                  py::arg("logFile") = "")
      .def_static("shutdown", &pyg::Logger::shutdown)
      .def_static("set_level", &pyg::Logger::setLevel)
      .def_static("info",
                  [](const std::string &msg) { pyg::Logger::info(msg); })
      .def_static("debug",
                  [](const std::string &msg) { pyg::Logger::debug(msg); })
      .def_static("warn",
                  [](const std::string &msg) { pyg::Logger::warn(msg); })
      .def_static("error",
                  [](const std::string &msg) { pyg::Logger::error(msg); })
      .def_static("trace",
                  [](const std::string &msg) { pyg::Logger::trace(msg); })
      .def_static("critical",
                  [](const std::string &msg) { pyg::Logger::critical(msg); });

  py::native_enum<pyg::Logger::Type>(m, "LogType", "enum.Enum")
      .value("Trace", pyg::Logger::Type::Trace)
      .value("Debug", pyg::Logger::Type::Debug)
      .value("Info", pyg::Logger::Type::Info)
      .value("Warn", pyg::Logger::Type::Warning)
      .value("Error", pyg::Logger::Type::Error)
      .value("Critical", pyg::Logger::Type::Critical)
      .export_values()
      .finalize();

  py::native_enum<pyg::Input::axis>(m, "Axis", "enum.Enum")
      .value("Horizontal", pyg::Input::axis::horizontal)
      .value("Vertical", pyg::Input::axis::vertical)
      .value("Left", pyg::Input::axis::left)
      .value("Right", pyg::Input::axis::right)
      .value("Jump", pyg::Input::axis::jump)
      .value("Sprint", pyg::Input::axis::sprint)
      .value("Crouch", pyg::Input::axis::crouch)
      .value("Fire1", pyg::Input::axis::fire1)
      .value("Fire2", pyg::Input::axis::fire2)
      .value("Fire3", pyg::Input::axis::fire3)
      .value("Escape", pyg::Input::axis::escape);

  // Window Bindings
  py::class_<pyg::Window>(m, "Window")
      .def("create", &pyg::Window::create)
      .def("close", &pyg::Window::close)
      .def("destroy", &pyg::Window::destroy)
      .def("is_open", &pyg::Window::isOpen)
      .def("poll_events", &pyg::Window::pollEvents)
      .def("display", &pyg::Window::display)
      .def("clear", &pyg::Window::clear)
      .def("set_title", &pyg::Window::setTitle)
      .def("get_title", &pyg::Window::getTitle)
      .def("set_icon", static_cast<void (pyg::Window::*)(const std::string &)>(
             &pyg::Window::setIcon))
      .def("set_icon", static_cast<void (pyg::Window::*)(
             unsigned int, unsigned int, const unsigned char *)>(
             &pyg::Window::setIcon))
      .def("set_size", &pyg::Window::setSize)
      .def("get_size", &pyg::Window::getSize)
      .def("set_position", &pyg::Window::setPosition)
      .def("get_position", &pyg::Window::getPosition)
      .def("set_visible", &pyg::Window::setVisible)
      .def("is_visible", &pyg::Window::isVisible)
      .def("set_framerate_limit", &pyg::Window::setFramerateLimit)
      .def("get_framerate_limit", &pyg::Window::getFramerateLimit)
      .def("set_vertical_sync_enabled", &pyg::Window::setVerticalSyncEnabled)
      .def("is_vertical_sync_enabled", &pyg::Window::isVerticalSyncEnabled)
      .def("set_mouse_cursor_visible", &pyg::Window::setMouseCursorVisible)
      .def("is_mouse_cursor_visible", &pyg::Window::isMouseCursorVisible)
      .def("set_mouse_cursor_grabbed", &pyg::Window::setMouseCursorGrabbed)
      .def("is_mouse_cursor_grabbed", &pyg::Window::isMouseCursorGrabbed)
      .def("set_mouse_cursor_position", &pyg::Window::setMouseCursorPosition)
      .def("get_mouse_cursor_position", &pyg::Window::getMouseCursorPosition)
      .def(py::init<>());

  // Math Bindings
  py::class_<pyg::Math>(m, "Math")
      .def_readonly_static("PI", &pyg::Math::PI)
      .def_readonly_static("EPSILON", &pyg::Math::EPSILON)
      .def_readonly_static("DEG2RAD", &pyg::Math::DEG2RAD)
      .def_readonly_static("RAD2DEG", &pyg::Math::RAD2DEG)
      .def_readonly_static("SQRT2", &pyg::Math::SQRT2)
      .def_readonly_static("SQRT3", &pyg::Math::SQRT3)
      .def_readonly_static("E", &pyg::Math::E)
      .def_readonly_static("GOLDEN_RATIO", &pyg::Math::GOLDEN_RATIO)
      .def_readonly_static("PHI", &pyg::Math::PHI)
      .def_readonly_static("TAU", &pyg::Math::TAU)
      .def_readonly_static("LOG2E", &pyg::Math::LOG2E)
      .def_readonly_static("LOG10E", &pyg::Math::LOG10E)
      .def_readonly_static("LN2", &pyg::Math::LN2)
      .def_readonly_static("LN10", &pyg::Math::LN10)
      .def_readonly_static("INVSQRT2", &pyg::Math::INVSQRT2)
      .def_readonly_static("INVSQRT3", &pyg::Math::INVSQRT3)
      // Comparison functions
      .def_static("is_nan", &pyg::Math::isNaN)
      .def_static("is_infinity", &pyg::Math::isInfinity)
      .def_static("is_finite", &pyg::Math::isFinite)
      .def_static("is_equal",
                  static_cast<bool (*)(float, float)>(&pyg::Math::isEqual))
      .def_static("is_equal", static_cast<bool (*)(float, float, float)>(
                    &pyg::Math::isEqual))
      .def_static("is_greater", &pyg::Math::isGreater)
      .def_static("is_greater_equal", &pyg::Math::isGreaterEqual)
      .def_static("is_less", &pyg::Math::isLess)
      .def_static("is_less_equal", &pyg::Math::isLessEqual)
      .def_static("is_zero", &pyg::Math::isZero)
      .def_static("is_not_zero", &pyg::Math::isNotZero)
      .def_static("is_positive", &pyg::Math::isPositive)
      .def_static("is_negative", &pyg::Math::isNegative)
      // Random functions
      .def_static("random", static_cast<float (*)()>(&pyg::Math::random))
      .def_static("random",
                  static_cast<float (*)(float, float)>(&pyg::Math::random))
      // Basic math functions
      .def_static("abs", &pyg::Math::abs)
      .def_static("sign", &pyg::Math::sign)
      .def_static("floor", &pyg::Math::floor)
      .def_static("ceil", &pyg::Math::ceil)
      .def_static("round", &pyg::Math::round)
      .def_static("clamp_float", &pyg::Math::clamp_float)
      .def_static("clamp_int", &pyg::Math::clamp_int)
      .def_static("frac", &pyg::Math::frac)
      .def_static("mod", &pyg::Math::mod)
      .def_static("min", &pyg::Math::min)
      .def_static("max", &pyg::Math::max)
      // Power and root functions
      .def_static("pow", &pyg::Math::pow)
      .def_static("sqrt", &pyg::Math::sqrt)
      // Trigonometric functions
      .def_static("sin", &pyg::Math::sin)
      .def_static("cos", &pyg::Math::cos)
      .def_static("tan", &pyg::Math::tan)
      .def_static("asin", &pyg::Math::asin)
      .def_static("acos", &pyg::Math::acos)
      .def_static("atan", &pyg::Math::atan)
      .def_static("atan2", &pyg::Math::atan2)
      // Exponential and logarithmic functions
      .def_static("exp", &pyg::Math::exp)
      .def_static("log", &pyg::Math::log)
      .def_static("log2", &pyg::Math::log2)
      .def_static("log10", &pyg::Math::log10)
      // Angle conversion functions
      .def_static("deg2rad", &pyg::Math::deg2rad)
      .def_static("rad2deg", &pyg::Math::rad2deg)
      // Interpolation functions
      .def_static("lerp", &pyg::Math::lerp)
      .def_static("clamp", &pyg::Math::clamp)
      .def_static("smoothstep", &pyg::Math::smoothstep)
      .def_static("smootherstep", &pyg::Math::smootherstep);

  // Vector Bindings
  bind_vector<2, float>(m, "Vector2");
  bind_vector<3, float>(m, "Vector3");
  bind_vector<4, float>(m, "Vector4");

  py::class_<pyg::Color>(m, "Color")
      .def(py::init<>())
      .def(py::init<uint8_t, uint8_t, uint8_t, uint8_t>(),
           py::arg("r"), py::arg("g"), py::arg("b"), py::arg("a") = 255)
      // Arithmetic Operators
      .def(py::self += py::self)
      .def(py::self -= py::self)
      .def(py::self *= py::self)
      .def(py::self *= float())
      .def(
        "__rmul__", [](const pyg::Color &c, const float s) { return c * s; },
        py::is_operator())
      // .def(py::self + py::self)
      // .def(py::self - py::self)
      // Color Space Conversions
      .def("to_srgb", &pyg::Color::SRGB)
      .def("to_hsv", &pyg::Color::HSV)
      .def("to_cmyk", &pyg::Color::CMYK)
      .def("to_hex", &pyg::Color::toHex)
      .def("to_rgb_hex", &pyg::Color::toRGBHex)
      // Static Utility
      .def_static("lerp", &pyg::Color::lerp)
      // String representation
      .def("__repr__", &pyg::Color::toString)
      // Properties
      .def_property("r",
                    [](const pyg::Color &c) { return c.r; },
                    [](pyg::Color &c, const uint8_t val) { c.r = val; })
      .def_property("g",
                    [](const pyg::Color &c) { return c.g; },
                    [](pyg::Color &c, const uint8_t val) { c.g = val; })
      .def_property("b",
                    [](const pyg::Color &c) { return c.b; },
                    [](pyg::Color &c, const uint8_t val) { c.b = val; })
      .def_property("a",
                    [](const pyg::Color &c) { return c.a; },
                    [](pyg::Color &c, const uint8_t val) { c.a = val; })
      .def(py::self + py::self)
      .def(py::self - py::self)
      .def(py::self * py::self)
      .def(py::self / py::self)
      .def(py::self / float())
      .def(py::self * float());


  // Module-level log function
  m.def(
    "log",
    [](const std::string &msg) {
      // Ensure logger is initialized
      if (!pyg::Logger::getCoreLogger()) {
        pyg::Logger::init("pyg_engine");
      }
      pyg::Logger::info(msg);
    },
    "Log a message using the engine's logger");

  m.def(
    "log_type",
    [](const pyg::Logger::Type type, const std::string &msg) {
      // Ensure logger is initialized
      if (!pyg::Logger::getCoreLogger()) {
        pyg::Logger::init("pyg_engine");
      }
      pyg::Logger::print(type, msg);
    },
    "Log a message using the engine's logger with LogType enum");

  py::native_enum<pyg::Input::KB>(m, "KB", "enum.Enum")
      .value("A", pyg::Input::KB::A)
      .value("B", pyg::Input::KB::B)
      .value("C", pyg::Input::KB::C)
      .value("D", pyg::Input::KB::D)
      .value("E", pyg::Input::KB::E)
      .value("F", pyg::Input::KB::F)
      .value("G", pyg::Input::KB::G)
      .value("H", pyg::Input::KB::H)
      .value("I", pyg::Input::KB::I)
      .value("J", pyg::Input::KB::J)
      .value("K", pyg::Input::KB::K)
      .value("L", pyg::Input::KB::L)
      .value("M", pyg::Input::KB::M)
      .value("N", pyg::Input::KB::N)
      .value("O", pyg::Input::KB::O)
      .value("P", pyg::Input::KB::P)
      .value("Q", pyg::Input::KB::Q)
      .value("R", pyg::Input::KB::R)
      .value("S", pyg::Input::KB::S)
      .value("T", pyg::Input::KB::T)
      .value("U", pyg::Input::KB::U)
      .value("V", pyg::Input::KB::V)
      .value("W", pyg::Input::KB::W)
      .value("X", pyg::Input::KB::X)
      .value("Y", pyg::Input::KB::Y)
      .value("Z", pyg::Input::KB::Z)
      .value("ZERO", pyg::Input::KB::ZERO)
      .value("ONE", pyg::Input::KB::ONE)
      .value("TWO", pyg::Input::KB::TWO)
      .value("THREE", pyg::Input::KB::THREE)
      .value("FOUR", pyg::Input::KB::FOUR)
      .value("FIVE", pyg::Input::KB::FIVE)
      .value("SIX", pyg::Input::KB::SIX)
      .value("SEVEN", pyg::Input::KB::SEVEN)
      .value("EIGHT", pyg::Input::KB::EIGHT)
      .value("NINE", pyg::Input::KB::NINE)
      .value("MINUS", pyg::Input::KB::MINUS)
      .value("PLUS", pyg::Input::KB::PLUS)
      .value("L_BRKT", pyg::Input::KB::L_BRKT)
      .value("R_BRKT", pyg::Input::KB::R_BRKT)
      .value("SPACE", pyg::Input::KB::SPACE)
      .value("ENTER", pyg::Input::KB::ENTER)
      .value("BK_SLASH", pyg::Input::KB::BK_SLASH)
      .value("FWD_SLASH", pyg::Input::KB::FWD_SLASH)
      .value("SLASH", pyg::Input::KB::SLASH)
      .value("BK_SPACE", pyg::Input::KB::BK_SPACE)
      .value("SEMI_COLON", pyg::Input::KB::SEMI_COLON)
      .value("QUOTE", pyg::Input::KB::QUOTE)
      .value("LESS_THAN", pyg::Input::KB::LESS_THAN)
      .value("GREATER_THAN", pyg::Input::KB::GREATER_THAN)
      .value("L_CARROT", pyg::Input::KB::L_CARROT)
      .value("R_CARROT", pyg::Input::KB::R_CARROT)
      .value("L_ARROW", pyg::Input::KB::L_ARROW)
      .value("R_ARROW", pyg::Input::KB::R_ARROW)
      .value("UP_ARROW", pyg::Input::KB::UP_ARROW)
      .value("DOWN_ARROW", pyg::Input::KB::DOWN_ARROW)
      .value("L_CTRL", pyg::Input::KB::L_CTRL)
      .value("R_CTRL", pyg::Input::KB::R_CTRL)
      .value("L_ALT", pyg::Input::KB::L_ALT)
      .value("R_ALT", pyg::Input::KB::R_ALT)
      .value("L_SHIFT", pyg::Input::KB::L_SHIFT)
      .value("R_SHIFT", pyg::Input::KB::R_SHIFT)
      .value("LEFT_SHIFT", pyg::Input::KB::LEFT_SHIFT)
      .value("RIGHT_SHIFT", pyg::Input::KB::RIGHT_SHIFT)
      .value("TAB", pyg::Input::KB::TAB)
      .value("ESCAPE", pyg::Input::KB::ESCAPE);

  py::native_enum<pyg::Input::MB>(m, "MB", "enum.Enum")
      .value("LEFT_CLICK", pyg::Input::MB::LEFT_CLICK)
      .value("RIGHT_CLICK", pyg::Input::MB::RIGHT_CLICK)
      .value("MIDDLE_CLICK", pyg::Input::MB::MIDDLE_CLICK)
      .value("L_CLK", pyg::Input::MB::L_CLK)
      .value("R_CLK", pyg::Input::MB::R_CLK)
      .value("M_CLK", pyg::Input::MB::M_CLK);
}
