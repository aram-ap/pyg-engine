#!/usr/bin/env python3
"""
Test runner script for pyg-engine.

This script runs both Python and C++ tests, providing a unified testing interface.
"""

import argparse
import os
import subprocess
import sys
from pathlib import Path
from typing import List, Optional


def run_command(
    cmd: List[str], cwd: Optional[Path] = None, env: Optional[dict] = None
) -> int:
    """
    Run a command and return its exit code.

    Args:
        cmd: Command to run as list of strings
        cwd: Working directory for command
        env: Environment variables

    Returns:
        int: Exit code of the command
    """
    print(f"\n{'='*70}")
    print(f"Running: {' '.join(cmd)}")
    print(f"{'='*70}\n")

    result = subprocess.run(cmd, cwd=cwd, env=env)
    return result.returncode


def run_python_tests(args: argparse.Namespace, project_root: Path) -> int:
    """
    Run Python tests using pytest.

    Args:
        args: Command line arguments
        project_root: Path to project root

    Returns:
        int: Exit code
    """
    print("\n" + "=" * 70)
    print("RUNNING PYTHON TESTS")
    print("=" * 70)

    pytest_args = ["pytest"]

    if args.verbose:
        pytest_args.append("-vv")
    else:
        pytest_args.append("-v")

    if args.markers:
        pytest_args.extend(["-m", args.markers])

    if args.coverage:
        pytest_args.extend(["--cov=pyg", "--cov-report=html", "--cov-report=term"])

    if args.slow:
        pytest_args.append("--run-slow")

    if args.test_path:
        pytest_args.append(args.test_path)

    # Add any additional pytest args
    if args.pytest_args:
        pytest_args.extend(args.pytest_args.split())

    return run_command(pytest_args, cwd=project_root)


def build_cpp_tests(project_root: Path, build_dir: Path) -> int:
    """
    Build C++ tests using CMake.

    Args:
        project_root: Path to project root
        build_dir: Path to build directory

    Returns:
        int: Exit code
    """
    print("\n" + "=" * 70)
    print("BUILDING C++ TESTS")
    print("=" * 70)

    # Create build directory if it doesn't exist
    build_dir.mkdir(exist_ok=True)

    # Configure CMake
    cmake_config = ["cmake", ".."]
    returncode = run_command(cmake_config, cwd=build_dir)
    if returncode != 0:
        return returncode

    # Build tests
    cmake_build = ["cmake", "--build", ".", "--target", "cpp_tests"]
    return run_command(cmake_build, cwd=build_dir)


def run_cpp_tests(build_dir: Path, args: argparse.Namespace) -> int:
    """
    Run C++ tests using Google Test.

    Args:
        build_dir: Path to build directory
        args: Command line arguments

    Returns:
        int: Exit code
    """
    print("\n" + "=" * 70)
    print("RUNNING C++ TESTS")
    print("=" * 70)

    test_executable = build_dir / "cpp_tests"

    if not test_executable.exists():
        print(f"Error: Test executable not found at {test_executable}")
        print("Please build the tests first using --build-cpp")
        return 1

    gtest_args = [str(test_executable)]

    if args.verbose:
        gtest_args.append("--gtest_print_time=1")

    if args.gtest_filter:
        gtest_args.append(f"--gtest_filter={args.gtest_filter}")

    return run_command(gtest_args)


def main() -> int:
    """
    Main entry point for test runner.

    Returns:
        int: Exit code
    """
    parser = argparse.ArgumentParser(
        description="Run tests for pyg-engine (Python and C++)"
    )

    # General options
    parser.add_argument(
        "-v", "--verbose", action="store_true", help="Verbose test output"
    )

    # Test selection
    parser.add_argument(
        "--python-only", action="store_true", help="Run only Python tests"
    )
    parser.add_argument("--cpp-only", action="store_true", help="Run only C++ tests")
    parser.add_argument(
        "--build-cpp",
        action="store_true",
        help="Build C++ tests before running (required for first run)",
    )

    # Python test options
    parser.add_argument(
        "-m",
        "--markers",
        type=str,
        help="Run only tests matching given mark expression (pytest)",
    )
    parser.add_argument(
        "--coverage", action="store_true", help="Generate coverage report (Python)"
    )
    parser.add_argument(
        "--slow", action="store_true", help="Include slow tests (Python)"
    )
    parser.add_argument(
        "--test-path", type=str, help="Specific test file or directory to run"
    )
    parser.add_argument(
        "--pytest-args", type=str, help="Additional arguments to pass to pytest"
    )

    # C++ test options
    parser.add_argument(
        "--gtest-filter", type=str, help="Filter tests by pattern (Google Test)"
    )
    parser.add_argument(
        "--build-dir",
        type=str,
        default="build_tests",
        help="Build directory for C++ tests (default: build_tests)",
    )

    args = parser.parse_args()

    # Get project root
    project_root = Path(__file__).parent.absolute()
    build_dir = project_root / args.build_dir

    exit_codes = []

    # Run Python tests
    if not args.cpp_only:
        try:
            exit_code = run_python_tests(args, project_root)
            exit_codes.append(("Python tests", exit_code))
        except FileNotFoundError:
            print("\nError: pytest not found. Install it with: pip install pytest pytest-cov")
            exit_codes.append(("Python tests", 1))

    # Build and run C++ tests
    if not args.python_only:
        if args.build_cpp:
            build_code = build_cpp_tests(project_root, build_dir)
            if build_code != 0:
                print("\nError: Failed to build C++ tests")
                exit_codes.append(("C++ build", build_code))
                return build_code

        cpp_code = run_cpp_tests(build_dir, args)
        exit_codes.append(("C++ tests", cpp_code))

    # Print summary
    print("\n" + "=" * 70)
    print("TEST SUMMARY")
    print("=" * 70)

    all_passed = True
    for test_type, code in exit_codes:
        status = "PASSED" if code == 0 else "FAILED"
        symbol = "✓" if code == 0 else "✗"
        print(f"{symbol} {test_type}: {status} (exit code: {code})")
        if code != 0:
            all_passed = False

    print("=" * 70)

    if all_passed:
        print("All tests passed!")
        return 0
    else:
        print("Some tests failed.")
        return 1


if __name__ == "__main__":
    sys.exit(main())

