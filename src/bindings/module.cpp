#include <pybind11/pybind11.h>
#include "core/Core.h"

namespace py = pybind11;

PYBIND11_MODULE(_native, m) {
    m.doc() = "Pyg-Engine native module";

    py::class_<pyg::Core>(m, "Core")
        .def(py::init<>())
        .def("get_version", &pyg::Core::getVersion)
        .def("update", &pyg::Core::update)
        .def("render", &pyg::Core::render)
        .def("on_destroy", &pyg::Core::on_destroy);
}
