cmake_minimum_required(VERSION 3.15)
project(pyg_engine C CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# This is for OpenGL (If we decide to move towards it)
set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)

set(CMAKE_POSITION_INDEPENDENT_CODE ON)

# Detect platform
if(WIN32)
    set(PLATFORM_WINDOWS ON)
elseif(APPLE)
    set(PLATFORM_MACOS ON)
else()
    set(PLATFORM_LINUX ON)
endif()

# Python and Pybind11
find_package(Python 3.8 COMPONENTS Interpreter Development REQUIRED)

# Try to find pybind11 using Python (works with pip-installed pybind11)
execute_process(
    COMMAND "${Python_EXECUTABLE}" -m pybind11 --cmakedir
    OUTPUT_VARIABLE pybind11_DIR
    OUTPUT_STRIP_TRAILING_WHITESPACE
    RESULT_VARIABLE pybind11_NOTFOUND
)

if(pybind11_NOTFOUND)
    message(FATAL_ERROR "pybind11 not found. Please install it via: pip install pybind11")
endif()

find_package(pybind11 REQUIRED)

# SFML
include(FetchContent)

FetchContent_Declare(
    SFML
    GIT_REPOSITORY https://github.com/SFML/SFML.git
    GIT_TAG 2.6.1
)

set(SFML_BUILD_AUDIO ON CACHE BOOL "" FORCE)
set(SFML_BUILD_GRAPHICS ON CACHE BOOL "" FORCE)
set(SFML_BUILD_WINDOW ON CACHE BOOL "" FORCE)
set(SFML_BUILD_NETWORK OFF CACHE BOOL "" FORCE)
set(BUILD_SHARED_LIBS OFF CACHE BOOL "" FORCE)

FetchContent_MakeAvailable(SFML)

# GLFW
#find_package(OpenGL REQUIRED)
#
#FetchContent_Declare(
#    glfw
#    GIT_REPOSITORY https://github.com/glfw/glfw.git
#    GIT_TAG 3.4
#)
#
#set(GLFW_BUILD_DOCS OFF CACHE BOOL "" FORCE)
#set(GLFW_BUILD_TESTS OFF CACHE BOOL "" FORCE)
#set(GLFW_BUILD_EXAMPLES OFF CACHE BOOL "" FORCE)
#set(GLFW_INSTALL OFF CACHE BOOL "" FORCE)
#
#FetchContent_MakeAvailable(glfw)

# ImGui - keeping declared but not used for this minimal example to preserve build setup
#FetchContent_Declare(
#    imgui
#    GIT_REPOSITORY https://github.com/ocornut/imgui.git
#    GIT_TAG v1.89.9
#)
#FetchContent_MakeAvailable(imgui)
#
#set(IMGUI_DIR ${imgui_SOURCE_DIR} CACHE PATH "Path to ImGui source" FORCE)
#set(IMGUI_SFML_FIND_SFML OFF CACHE BOOL "" FORCE)
#set(IMGUI_SFML_IMGUI_DEMO OFF CACHE BOOL "" FORCE)
#
#FetchContent_Declare(
#    imgui-sfml
#    GIT_REPOSITORY https://github.com/SFML/imgui-sfml.git
#    GIT_TAG v2.6
#)
#FetchContent_MakeAvailable(imgui-sfml)

# spdlog
FetchContent_Declare(
    spdlog
    GIT_REPOSITORY https://github.com/gabime/spdlog.git
    GIT_TAG v1.12.0
)
FetchContent_MakeAvailable(spdlog)

# Google Test for C++ unit testing
FetchContent_Declare(
    googletest
    GIT_REPOSITORY https://github.com/google/googletest.git
    GIT_TAG v1.14.0
)
# For Windows: Prevent overriding the parent project's compiler/linker settings
set(gtest_force_shared_crt ON CACHE BOOL "" FORCE)
FetchContent_MakeAvailable(googletest)

# Source definitions
set(CORE_SOURCES
        src/core/Engine.cpp
        src/rendering/Window.cpp
        src/core/Math.cpp
    src/logging/Logger.cpp
)

set(BINDING_SOURCES
    src/bindings/module.cpp
)

# Create the extension module
pybind11_add_module(_native
    ${CORE_SOURCES}
    ${BINDING_SOURCES}
)

# Note: pybind11_add_module automatically sets the correct naming with ABI tags
# e.g., _native.cp312-win_amd64.pyd on Windows, _native.cpython-312-x86_64-linux-gnu.so on Linux
# Do not override PREFIX, OUTPUT_NAME, or SUFFIX unless you have a specific reason

# Include directories
target_include_directories(_native PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src
    ${CMAKE_CURRENT_SOURCE_DIR}/include
#    ${imgui_SOURCE_DIR}
#    ${imgui-sfml_SOURCE_DIR}
)

# Link libraries
target_link_libraries(_native PRIVATE
    sfml-graphics
    sfml-window
    sfml-audio
    sfml-system
    spdlog::spdlog
    # ImGui-SFML::ImGui-SFML # Not linking for minimal example but available
#    glfw               # GLFW library
#    OpenGL::GL         # OpenGL (Linux/macOS) or OpenGL::GLU if needed
)

# Platform-specific settings
# Note: pybind11_add_module automatically handles platform-specific extensions
# (.pyd on Windows, .so on Linux/macOS)

# Copy the extension to the pyg package directory after build (CMake post-build)
# NOTE: Disabled when using pip install, as pip manages file placement via CMAKE_LIBRARY_OUTPUT_DIRECTORY
# For manual builds with cmake directly, you may need to manually copy the built .pyd/.so file
# add_custom_command(TARGET _native POST_BUILD
#     COMMAND ${CMAKE_COMMAND} -E copy $<TARGET_FILE:_native> ${CMAKE_CURRENT_SOURCE_DIR}/pyg/
#     COMMENT "Copying _native module to pyg directory"
# )

install(TARGETS _native DESTINATION pyg)
