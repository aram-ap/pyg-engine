#!/usr/bin/env python3
"""
System dependency installer for pyg-engine.
Run this script before installing pyg-engine to install required system libraries.

Note: Some dependencies (SFML, spdlog, ImGui) are automatically downloaded
and built via CMake's FetchContent during the build process. This script
only installs system-level dependencies that cannot be auto-fetched.

Usage:
    python install_deps.py
    
Or to check dependencies without installing:
    python install_deps.py --check
"""

import platform
import subprocess
import sys
import shutil

def run_command(cmd, shell=False):
    """Run a command and return success status."""
    try:
        subprocess.check_call(cmd, shell=shell, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        return True
    except (subprocess.CalledProcessError, FileNotFoundError):
        return False

def check_command_exists(cmd):
    """Check if a command exists on the system."""
    return shutil.which(cmd) is not None

def get_platform():
    """Detect the operating system."""
    system = platform.system()
    if system == "Linux":
        # Try to detect Linux distribution
        try:
            with open("/etc/os-release") as f:
                os_release = f.read().lower()
                if "ubuntu" in os_release or "debian" in os_release:
                    return "ubuntu"
                elif "fedora" in os_release or "rhel" in os_release or "centos" in os_release:
                    return "fedora"
                elif "arch" in os_release:
                    return "arch"
        except FileNotFoundError:
            pass
        return "linux"
    elif system == "Darwin":
        return "macos"
    elif system == "Windows":
        return "windows"
    return "unknown"

def check_dependencies():
    """Check if required dependencies are installed."""
    platform_type = get_platform()
    missing = []
    
    # Check CMake
    if not check_command_exists("cmake"):
        missing.append("cmake")
    
    if platform_type in ["ubuntu", "linux"]:
        # Check for X11 libraries by trying to find pkg-config
        if check_command_exists("pkg-config"):
            libs_to_check = ["x11", "xcursor", "xrandr", "xi", "gl", "flac", "vorbis", "openal"]
            for lib in libs_to_check:
                if not run_command(["pkg-config", "--exists", lib]):
                    missing.append(f"lib{lib}-dev")
    elif platform_type == "macos":
        if not check_command_exists("brew"):
            print("WARNING: Homebrew not found. Install from: https://brew.sh")
            return False
        # Check for SFML
        result = subprocess.run(["brew", "list", "sfml"], capture_output=True)
        if result.returncode != 0:
            missing.append("sfml")
    
    if missing:
        print("ERROR: Missing dependencies:", ", ".join(missing))
        return False
    else:
        print("SUCCESS: All system dependencies are installed!")
        return True

def install_dependencies_ubuntu():
    """Install dependencies on Ubuntu/Debian."""
    print("Installing dependencies for Ubuntu/Debian...")
    packages = [
        "libxcursor-dev",
        "libxrandr-dev",
        "libxinerama-dev",
        "libxi-dev",
        "libudev-dev",
        "libgl1-mesa-dev",
        "libflac-dev",
        "libogg-dev",
        "libvorbis-dev",
        "libopenal-dev",
        "libfreetype6-dev",
        "cmake",
    ]
    
    try:
        # Update package list
        print("Updating package list...")
        subprocess.check_call(["sudo", "apt-get", "update", "-qq"])
        
        # Install packages
        print(f"Installing {len(packages)} packages...")
        subprocess.check_call(["sudo", "apt-get", "install", "-y"] + packages)
        
        print("SUCCESS: Dependencies installed successfully!")
        return True
    except subprocess.CalledProcessError as e:
        print(f"ERROR: Failed to install dependencies: {e}")
        return False

def install_dependencies_fedora():
    """Install dependencies on Fedora/RHEL/CentOS."""
    print("Installing dependencies for Fedora/RHEL...")
    packages = [
        "libXcursor-devel",
        "libXrandr-devel",
        "libXinerama-devel",
        "libXi-devel",
        "systemd-devel",
        "mesa-libGL-devel",
        "flac-devel",
        "libogg-devel",
        "libvorbis-devel",
        "openal-soft-devel",
        "freetype-devel",
        "cmake",
    ]
    
    try:
        subprocess.check_call(["sudo", "dnf", "install", "-y"] + packages)
        print("SUCCESS: Dependencies installed successfully!")
        return True
    except subprocess.CalledProcessError as e:
        print(f"ERROR: Failed to install dependencies: {e}")
        return False

def install_dependencies_arch():
    """Install dependencies on Arch Linux."""
    print("Installing dependencies for Arch Linux...")
    packages = [
        "libxcursor",
        "libxrandr",
        "libxinerama",
        "libxi",
        "systemd",
        "mesa",
        "flac",
        "libvorbis",
        "openal",
        "freetype2",
        "cmake",
    ]
    
    try:
        subprocess.check_call(["sudo", "pacman", "-S", "--noconfirm"] + packages)
        print("SUCCESS: Dependencies installed successfully!")
        return True
    except subprocess.CalledProcessError as e:
        print(f"ERROR: Failed to install dependencies: {e}")
        return False

def install_dependencies_macos():
    """Install dependencies on macOS."""
    print("Installing dependencies for macOS...")
    
    if not check_command_exists("brew"):
        print("ERROR: Homebrew is not installed.")
        print("Please install Homebrew from: https://brew.sh")
        print("Then run this script again.")
        return False
    
    try:
        subprocess.check_call(["brew", "install", "sfml", "cmake"])
        print("SUCCESS: Dependencies installed successfully!")
        return True
    except subprocess.CalledProcessError as e:
        print(f"ERROR: Failed to install dependencies: {e}")
        return False

def install_dependencies_windows():
    """Provide instructions for Windows."""
    print("Windows Installation Instructions:")
    print()
    print("1. Install Visual Studio with C++ support:")
    print("   https://visualstudio.microsoft.com/downloads/")
    print()
    print("2. Install CMake:")
    print("   https://cmake.org/download/")
    print()
    print("3. (Optional) Install SFML manually if needed:")
    print("   https://www.sfml-dev.org/download.php")
    print()
    print("After installing these, you can install pyg-engine with pip.")
    return False

def main():
    """Main entry point."""
    import argparse
    
    parser = argparse.ArgumentParser(description="Install system dependencies for pyg-engine")
    parser.add_argument("--check", action="store_true", help="Only check dependencies without installing")
    args = parser.parse_args()
    
    print("=" * 60)
    print("Pyg-Engine Dependency Installer")
    print("=" * 60)
    print()
    
    platform_type = get_platform()
    print(f"Detected platform: {platform_type}")
    print()
    
    if args.check:
        sys.exit(0 if check_dependencies() else 1)
    
    # Ask for confirmation
    print("This script will install system dependencies required by pyg-engine.")
    response = input("Continue? [Y/n]: ").strip().lower()
    if response and response not in ["y", "yes"]:
        print("Installation cancelled.")
        sys.exit(1)
    
    print()
    
    # Install based on platform
    if platform_type == "ubuntu":
        success = install_dependencies_ubuntu()
    elif platform_type == "fedora":
        success = install_dependencies_fedora()
    elif platform_type == "arch":
        success = install_dependencies_arch()
    elif platform_type == "macos":
        success = install_dependencies_macos()
    elif platform_type == "windows":
        success = install_dependencies_windows()
    else:
        print(f"ERROR: Unsupported platform: {platform_type}")
        print("Please install dependencies manually.")
        success = False
    
    print()
    if success:
        print("=" * 60)
        print("Setup complete! You can now install pyg-engine:")
        print("   pip install pyg-engine")
        print("   OR")
        print("   pip install git+https://github.com/aram-ap/pyg-engine.git")
        print("=" * 60)
        sys.exit(0)
    else:
        sys.exit(1)

if __name__ == "__main__":
    main()

