# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.0a2] - 2025-07-30

### Fixed
- Fixed GameObject constructor crash when using tuples for position and size parameters
  - **The Bug:** When creating a GameObject with `position=(400, 300)` or `size=(50, 50)`, it would throw `AttributeError: 'tuple' object has no attribute 'x'` because the code expected Vector2 objects.
  - **The Fix:** Added automatic conversion of tuples and lists to Vector2 objects in the GameObject constructor.

### Changed
- Updated README with clearer examples showing both tuple and Vector2 usage
- Enhanced PyPI publishing workflow for more reliable distribution

### Compatibility
- This release is fully backward compatible - existing code using Vector2 objects continues to work unchanged

## [1.0.0a1] - 2025-07-30

### Added
- Initial alpha release of Pyg Engine
- Complete 2D game engine with Pygame integration
- Component-based GameObject system
- Physics system with Pymunk integration
- Mouse and keyboard input handling
- Camera system with multiple scaling modes
- Script system for dynamic code loading
- Basic shapes (Rectangle, Circle) rendering
- Collision detection and response
- Physics materials system
- PyPI publishing workflow

---

**Pyg Engine** - Making 2D game development in Python easier and more flexible! :) 