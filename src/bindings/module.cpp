#include <pybind11/pybind11.h>
#include <pybind11/native_enum.h>
#include "core/Engine.h"
#include "../rendering/Window.h"
#include "logging/Logger.h"

namespace py = pybind11;

PYBIND11_MODULE(_native, m) {
    m.doc() = "Pyg-Engine native module";

    py::class_<pyg::Engine>(m, "Engine")
        .def(py::init<>())
        .def_property("tick_rate", &pyg::Engine::getTickRate, &pyg::Engine::setTickRate)
        .def("get_version", &pyg::Engine::getVersion)
        .def("update", &pyg::Engine::update)
        .def("render", &pyg::Engine::render)
        .def("on_destroy", &pyg::Engine::on_destroy)
        .def("log", static_cast<void (pyg::Engine::*)(std::string)>(&pyg::Engine::log))
        .def("log_type", static_cast<void (pyg::Engine::*)(pyg::Logger::Type, std::string)>(&pyg::Engine::logType))
        .def("is_running", &pyg::Engine::isRunning)
        .def("start", &pyg::Engine::start)
        .def("stop", &pyg::Engine::stop)
        .def("pause", &pyg::Engine::pause)
        .def("resume", &pyg::Engine::resume)
        .def("restart", &pyg::Engine::restart)
        .def("exit", &pyg::Engine::exit)
        .def("set_window", &pyg::Engine::setWindow)
        .def("get_window", &pyg::Engine::getWindow);


    py::class_<pyg::Logger>(m, "Logger")
        .def_static("init", &pyg::Logger::init, py::arg("name") = "pyg_engine", py::arg("logFile") = "")
        .def_static("shutdown", &pyg::Logger::shutdown)
        .def_static("set_level", &pyg::Logger::setLevel);

    py::native_enum<pyg::Logger::Type>(m, "LogType", "enum.Enum")
        .value("Trace", pyg::Logger::Type::Trace)
        .value("Debug", pyg::Logger::Type::Debug)
        .value("Info", pyg::Logger::Type::Info)
        .value("Warn", pyg::Logger::Type::Warning)
        .value("Error", pyg::Logger::Type::Error)
        .value("Critical", pyg::Logger::Type::Critical)
        .export_values()
        .finalize();

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
        .def("set_icon", &pyg::Window::setIcon)
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
}
