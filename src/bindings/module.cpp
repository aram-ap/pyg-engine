#include <pybind11/native_enum.h>
#include <pybind11/operators.h>
#include <pybind11/pybind11.h>
#include <pybind11/stl.h>
#include <stdlib.h>

#include "core/Engine.h"
#include "core/Math.h"
#include "datatypes/Vector.h"
#include "logging/Logger.h"
#include "rendering/Window.h"

namespace py = pybind11;

// --- Helper to bind Vector<N, T> instantiations ---
template <size_t N, typename T>
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
      .def(
          "__rmul__", [](const Vec &v, T s) { return v * s; },
          py::is_operator())

      // Common Vector Functions
      .def("dot", &Vec::dot)
      .def("length", &Vec::length)

      // Array Access
      .def("__getitem__", [](const Vec &v, size_t i) { return v[i]; })
      .def("__setitem__", [](Vec &v, size_t i, T val) { v[i] = val; })

      // String representation
      .def("__repr__", &Vec::toString)

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
      .def_property("tick_rate", &pyg::Engine::getTickRate,
                    &pyg::Engine::setTickRate)
      .def("get_version", &pyg::Engine::getVersion)
      .def("update", &pyg::Engine::update)
      .def("render", &pyg::Engine::render)
      .def("on_destroy", &pyg::Engine::on_destroy)
      .def("log",
           static_cast<void (pyg::Engine::*)(std::string)>(&pyg::Engine::log))
      .def("log_type",
           static_cast<void (pyg::Engine::*)(pyg::Logger::Type, std::string)>(
               &pyg::Engine::logType))
      .def("is_running", &pyg::Engine::isRunning)
      .def("start", &pyg::Engine::start)
      .def("stop", &pyg::Engine::stop)
      .def("pause", &pyg::Engine::pause)
      .def("resume", &pyg::Engine::resume)
      .def("restart", &pyg::Engine::restart)
      .def("exit", &pyg::Engine::exit)
      .def("set_window", &pyg::Engine::setWindow)
      .def("get_window", &pyg::Engine::getWindow);

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
  bind_vector<2, float>(m, "Vec2");
  bind_vector<3, float>(m, "Vec3");
  bind_vector<4, float>(m, "Vec4");

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
}
