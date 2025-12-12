cmake_minimum_required(VERSION 4.1)          # matches the requirement
project(MyGLFWApp LANGUAGES C CXX)

# ------------------------------------------------------------------
# C++23
# ------------------------------------------------------------------
set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# ------------------------------------------------------------------
# Find GLFW 3.4 (the config package provides the imported target “glfw”)
# ------------------------------------------------------------------
find_package(glfw3 3.4 REQUIRED)

# ------------------------------------------------------------------
# Create the executable – this must come **before** any target_* commands
# ------------------------------------------------------------------
add_executable(MyGLFWApp
        src/main.cpp          # ← add all your source files here
        src/gl.c
        # src/other.cpp
)

# ------------------------------------------------------------------
# GLAD – header‑only loader (glad.h lives in include/glad/)
# ------------------------------------------------------------------
# Add the directory that contains the GLAD header to the target’s include path
target_include_directories(MyGLFWApp PRIVATE "${CMAKE_CURRENT_SOURCE_DIR}/include")

# If you also have a compiled glad.c, you can build it as a static library:
#   add_library(glad STATIC src/glad.c)
#   target_include_directories(glad PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}/include")
#   target_link_libraries(MyGLFWApp PRIVATE glad)

# ------------------------------------------------------------------
# Link libraries
# ------------------------------------------------------------------
target_link_libraries(MyGLFWApp PRIVATE glfw)   # “glfw” is the imported target from find_package

# ------------------------------------------------------------------
# Optional: make the include directory visible for IDE code‑completion
# ------------------------------------------------------------------
target_include_directories(MyGLFWApp PRIVATE "${CMAKE_CURRENT_SOURCE_DIR}/include")
