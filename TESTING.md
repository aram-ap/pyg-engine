# Testing Guide for Pyg-Engine

This document provides info about testing pyg-engine.

## Table of Contents

- [Overview](#overview)
- [Setup](#setup)
- [Running Tests](#running-tests)
- [Writing Tests](#writing-tests)
- [Test Organization](#test-organization)
- [Continuous Integration](#continuous-integration)
- [Best Practices](#best-practices)
- [Troubleshooting](#troubleshooting)

## Overview

There's two main tests performed:

1. **Python Tests**: Using pytest for Python code and bindings
2. **C++ Tests**: Using Google Test for C++ core components

Both test suites are integrated and can be run together or separately.

## Setup

### Install Testing Dependencies

**For Python tests:**

```bash
pip install pytest pytest-cov pytest-mock
```

Or install all dev dependencies:

```bash
pip install -e ".[dev]"
```

**For C++ tests:**

C++ testing dependencies (Google Test) are automatically fetched by CMake.

### System Dependencies

Ensure you have the required system dependencies installed:

**Ubuntu/Debian:**
```bash
sudo apt-get install cmake build-essential libsfml-dev
```

**macOS:**
```bash
brew install cmake sfml
```

## Running Tests

### Quick Start

Run all tests with a single command:

```bash
python run_tests.py --build-cpp
```

> **Note:** If you're running to any errors where '_native' is not found, 
> ```bash
> pip install .
> ```

### Python Tests

**Run all Python tests:**

```bash
pytest tests/
```

**Run with verbose output:**

```bash
pytest tests/ -v
```

**Run with extra verbosity:**

```bash
pytest tests/ -vv
```

**Run specific test file:**

```bash
pytest tests/test_color.py
```

**Run specific test class:**

```bash
pytest tests/test_color.py::TestColorCreation
```

**Run specific test method:**

```bash
pytest tests/test_color.py::TestColorCreation::test_create_color_with_rgb
```

**Run with markers:**

```bash
# Run only unit tests
pytest tests/ -m unit

# Run only integration tests
pytest tests/ -m integration

# Run unit OR integration tests
pytest tests/ -m "unit or integration"

# Run unit tests but NOT slow tests
pytest tests/ -m "unit and not slow"
```

**Generate coverage report:**

```bash
# Terminal output
pytest tests/ --cov=pyg --cov-report=term

# HTML report
pytest tests/ --cov=pyg --cov-report=html

# XML report (for CI)
pytest tests/ --cov=pyg --cov-report=xml

# All formats
pytest tests/ --cov=pyg --cov-report=term --cov-report=html --cov-report=xml
```

**Run tests in parallel (requires pytest-xdist):**

```bash
pip install pytest-xdist
pytest tests/ -n auto
```

**Stop on first failure:**

```bash
pytest tests/ -x
```

**Stop after N failures:**

```bash
pytest tests/ --maxfail=3
```

**Show local variables in tracebacks:**

```bash
pytest tests/ -l
```

**Rerun only failed tests from last run:**

```bash
pytest tests/ --lf
```

**Run failed tests first, then others:**

```bash
pytest tests/ --ff
```

### C++ Tests

**Build C++ tests:**

```bash
mkdir -p build_tests
cd build_tests
cmake ..
cmake --build . --target cpp_tests -j 4
```

**Run all C++ tests:**

```bash
./build_tests/cpp_tests
```

**Run specific test suite:**

```bash
./build_tests/cpp_tests --gtest_filter="MathTest.*"
```

**Run specific test:**

```bash
./build_tests/cpp_tests --gtest_filter="MathTest.TestClampFloat"
```

**Run with verbose output:**

```bash
./build_tests/cpp_tests --gtest_print_time=1
```

**List all available tests:**

```bash
./build_tests/cpp_tests --gtest_list_tests
```

**Repeat tests:**

```bash
./build_tests/cpp_tests --gtest_repeat=10
```

**Run tests in random order:**

```bash
./build_tests/cpp_tests --gtest_shuffle
```

**Generate XML output:**

```bash
./build_tests/cpp_tests --gtest_output=xml:test_results.xml
```

### Using the Test Runner Script

The `run_tests.py` script provides a unified interface:

**Basic commands:**

```bash
# Run all tests
python run_tests.py --build-cpp

# Python only
python run_tests.py --python-only

# C++ only
python run_tests.py --cpp-only

# Verbose mode
python run_tests.py -v

# With coverage
python run_tests.py --coverage
```

**Advanced options:**

```bash
# Run specific markers
python run_tests.py -m unit

# Include slow tests
python run_tests.py --slow

# Run specific test path
python run_tests.py --test-path tests/test_color.py

# Filter C++ tests
python run_tests.py --cpp-only --gtest-filter="ColorTest.*"

# Additional pytest arguments
python run_tests.py --pytest-args="-x --maxfail=1"

# Custom build directory
python run_tests.py --build-dir my_build
```

## Writing Tests

### Python Tests

**Test file structure:**

```python
"""
Module docstring describing what's being tested.
"""

from typing import TYPE_CHECKING

import pytest

from pyg.types.color import Color

if TYPE_CHECKING:
    from _pytest.fixtures import FixtureRequest


@pytest.mark.unit
class TestColorClass:
    """Test class for Color functionality."""

    def test_color_creation(self) -> None:
        """Test creating a color with RGB values."""
        # Arrange
        expected_r = 255
        expected_g = 128
        expected_b = 64

        # Act
        color = Color(expected_r, expected_g, expected_b)

        # Assert
        assert color.r == expected_r
        assert color.g == expected_g
        assert color.b == expected_b

    def test_color_addition(self) -> None:
        """Test adding two colors together."""
        # Arrange
        color1 = Color(100, 50, 25)
        color2 = Color(50, 25, 10)

        # Act
        result = color1 + color2

        # Assert
        assert result.r == 150
        assert result.g == 75
        assert result.b == 35

    def test_invalid_color_raises_error(self) -> None:
        """Test that invalid color values raise appropriate errors."""
        # Should clamp instead of raising
        color = Color(300, 400, 500)
        assert color.r == 255  # Clamped to max
```

**Best practices:**

1. Use descriptive test names that explain what's being tested
2. Follow Arrange-Act-Assert pattern
3. Add type hints to all test functions
4. Include docstrings for test methods
5. Use markers to categorize tests
6. Test both success and failure cases
7. Test edge cases and boundary conditions

### C++ Tests

**Test file structure:**

```cpp
/**
 * @file test_color.cpp
 * @brief Unit tests for Color class
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
        // Setup code runs before each test
        testColor = Color(255, 128, 64);
    }

    void TearDown() override {
        // Cleanup code runs after each test
    }

    Color testColor;
};

TEST_F(ColorTest, TestColorCreation) {
    Color c(255, 128, 64);
    EXPECT_EQ(c.r, 255);
    EXPECT_EQ(c.g, 128);
    EXPECT_EQ(c.b, 64);
}

TEST_F(ColorTest, TestColorAddition) {
    Color c1(100, 50, 25);
    Color c2(50, 25, 10);
    Color result = c1 + c2;

    EXPECT_EQ(result.r, 150);
    EXPECT_EQ(result.g, 75);
    EXPECT_EQ(result.b, 35);
}
```

**Google Test Assertions:**

- `EXPECT_EQ(a, b)` - Expects a == b
- `EXPECT_NE(a, b)` - Expects a != b
- `EXPECT_LT(a, b)` - Expects a < b
- `EXPECT_LE(a, b)` - Expects a <= b
- `EXPECT_GT(a, b)` - Expects a > b
- `EXPECT_GE(a, b)` - Expects a >= b
- `EXPECT_TRUE(condition)` - Expects condition is true
- `EXPECT_FALSE(condition)` - Expects condition is false
- `EXPECT_FLOAT_EQ(a, b)` - Expects floats are approximately equal
- `EXPECT_NEAR(a, b, tolerance)` - Expects |a - b| <= tolerance
- `EXPECT_THROW(statement, exception)` - Expects statement throws exception

Use `ASSERT_*` instead of `EXPECT_*` to abort the test on failure.

### Test Markers

Available pytest markers:

- `@pytest.mark.unit` - Unit tests for individual components
- `@pytest.mark.integration` - Integration tests for module interactions
- `@pytest.mark.slow` - Tests that take a long time to run
- `@pytest.mark.cpp_binding` - Tests for C++ bindings
- `@pytest.mark.core` - Tests for core engine functionality
- `@pytest.mark.rendering` - Tests for rendering components
- `@pytest.mark.physics` - Tests for physics components
- `@pytest.mark.input` - Tests for input handling

### Fixtures

Common fixtures are defined in `tests/conftest.py`:

```python
@pytest.fixture
def temp_game_object() -> dict:
    """Create a temporary game object for testing."""
    return {"id": "test", "position": (0, 0)}
```

## Continuous Integration

### GitHub Actions Workflows

**tests.yml**: Main test workflow
- Runs on Ubuntu, macOS, and Windows
- Tests Python 3.8 through 3.12
- Runs both Python and C++ tests
- Generates coverage reports
- Uploads test results

**codeql.yml**: Security analysis
- Performs static analysis on Python and C++ code
- Runs weekly and on pull requests
- Identifies security vulnerabilities

### Running Locally

To run tests exactly as CI does:

```bash
# Build extension
python setup.py build_ext --inplace -j 4

# Run Python tests with coverage
pytest tests/ -v --cov=pyg --cov-report=xml

# Build and run C++ tests
mkdir -p build_tests
cd build_tests
cmake .. -DCMAKE_BUILD_TYPE=Release
cmake --build . --target cpp_tests -j 4
./cpp_tests --gtest_output=xml:test_results.xml
```

## Best Practices

### General Guidelines

1. **Write tests first (TDD)** - Consider writing tests before implementation
2. **One assertion per test** - Keep tests focused and simple
3. **Descriptive names** - Test names should describe what's being tested
4. **Independent tests** - Tests should not depend on each other
5. **Fast tests** - Keep tests fast; mark slow tests appropriately
6. **Deterministic** - Tests should always produce the same result
7. **Clean up** - Always clean up resources in teardown/fixtures

### Python-Specific

1. Use type hints for all test functions
2. Add docstrings to test methods
3. Use fixtures for common setup
4. Mark tests appropriately
5. Test both Python wrapper and C++ binding

### C++-Specific

1. Use test fixtures for common setup/teardown
2. Add Doxygen comments to test files
3. Use appropriate assertions (EXPECT vs ASSERT)
4. Test edge cases and boundary conditions
5. Check for memory leaks in complex tests

## Troubleshooting

### Common Issues

**Import errors when running Python tests:**

```bash
# Make sure extension is built
python setup.py build_ext --inplace -j 4

# Or install in development mode
pip install -e .
```

**C++ tests won't compile:**

```bash
# Clean build directory
rm -rf build_tests
mkdir build_tests
cd build_tests
cmake ..
cmake --build . --target cpp_tests -j 4
```

**Tests pass locally but fail in CI:**

- Check Python version compatibility (CI tests 3.8-3.12)
- Ensure all dependencies are in requirements/pyproject.toml
- Check for platform-specific issues (Windows, macOS, Linux)
- Look at CI logs for detailed error messages

**Coverage not generating:**

```bash
# Install coverage dependencies
pip install pytest-cov

# Make sure you're using the --cov flag
pytest tests/ --cov=pyg --cov-report=html
```

**Google Test not found:**

CMake automatically fetches Google Test. If it fails:

```bash
# Check CMake version (needs >= 3.15)
cmake --version

# Try cleaning CMake cache
rm -rf build_tests/CMakeCache.txt build_tests/_deps
```

### Getting Help

- Check test output for error messages
- Review test documentation in this file
- Look at example tests in the repository
- Check GitHub Actions logs for CI failures
- Open an issue on GitHub with test output

## Performance Testing

For performance-critical code, consider:

```python
import pytest
import time

@pytest.mark.slow
def test_performance():
    """Test that operation completes within acceptable time."""
    start = time.time()
    
    # Perform operation
    result = expensive_operation()
    
    duration = time.time() - start
    assert duration < 0.1, f"Operation took {duration}s, expected < 0.1s"
```

## Code Coverage Goals

- Target: 80% overall coverage
- Core modules: 90%+ coverage
- New code: 100% coverage for new features

View coverage report:

```bash
pytest tests/ --cov=pyg --cov-report=html
open htmlcov/index.html
```

## Contributing Tests

When contributing to pyg-engine:

1. Write tests for all new features
2. Update existing tests when modifying features
3. Ensure all tests pass before submitting PR
4. Aim for high code coverage
5. Follow existing test patterns and conventions
6. Add appropriate test markers
7. Update this documentation if adding new test types

