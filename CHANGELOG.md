# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Added new `Mouse` class to input system for cleaner mouse input handling
  - `get_pos()` returns mouse position as tuple (x, y)
  - `get_rel()` returns relative mouse movement as tuple (x, y)
  - `get_scroll()` returns scroll wheel movement as tuple (x, y)
  - `get_button()`, `get_button_down()`, `get_button_up()` for button state checking
  - `set_pos()` and `set_visible()` for mouse control
  - Integrates with existing event system for button events
- Fixed `get_raw_axis()` for relative mouse movement to return actual values instead of scaled values

### Changed
- Mouse class is now the main interface for mouse operations
- Legacy mouse system remains available as alternative
- Updated documentation to include Mouse class usage examples

## [1.0.0a4] - 2025-01-XX

### Added
- **Event System**: Comprehensive event-driven communication system
  - `Event` class for immutable event data with type, payload, and timestamp
  - `EventManager` for thread-safe event subscription, dispatch, and processing
  - Priority-based event handling with four levels (CRITICAL, HIGH, NORMAL, LOW)
  - Support for both immediate and queued event processing
  - Automatic memory management with weak references to prevent memory leaks
  - Component and script integration for easy event handling
  - Event chaining and filtering capabilities for complex workflows
- **Enhanced Input System**: Modernized input handling with comprehensive features
  - Unified keyboard, mouse, and joystick input through single interface
  - Axis-based input system for combining multiple input sources
  - Event-based input for precise timing (clicks, key presses)
  - Joystick support with automatic detection and mapping
  - Input aliases and string-based access for easy configuration
  - Deadzone support and raw axis values for custom processing
  - Component system for object-specific mouse interactions
- **CORE_SYSTEMS_GUIDE**: Comprehensive documentation system
  - Complete guide covering all core engine systems
  - Detailed class documentation with functions and properties
  - Usage examples for all major systems (input, physics, camera, etc.)
  - Integration examples and best practices
  - Performance optimization guidelines
  - Testing instructions and examples
- **Improved Examples**: Enhanced demonstration and testing capabilities
  - Comprehensive input system examples with keyboard, mouse, and joystick
  - Event system demonstrations with component and script integration
  - Performance analysis and testing tools
  - Visual demonstrations of engine capabilities
  - Snake game example showcasing multiple systems working together
- **Runnable System**: Event-driven function execution with priority management
  - Priority-based execution with four levels (CRITICAL, HIGH, NORMAL, LOW)
  - Event-driven function scheduling and execution
  - Execution limits and error handling capabilities
  - Performance monitoring and statistics
  - Flexible event types and key-based organization
- **Headless Engine Support**: Run engine without display for server-side processing
  - No-display mode for automated testing and server applications
  - Full engine functionality without graphical output
  - Ideal for AI training, automated testing, and server-side game logic
  - Performance optimization for non-visual applications

### Changed
- Enhanced engine architecture with integrated event system
- Updated documentation structure to include Event and EventManager classes
- Improved component and script systems with event-driven capabilities
- Modernized input system with better organization and features
- Comprehensive documentation overhaul with professional, concise style
- Added runnable system for flexible function execution and scheduling

## [1.0.0a3] - 2025-07-30

### Fixed
- Fixed scripting system's lack of start() and on_destroy() calls

### Added
- Added a new testing system in '''examples/test.py''' and '''examples/scripts/test_script.py'''
  - This shows in better detail the component system and scripting system

### Changed
- Updated README to show updated filenames
- Removed ScriptRunner as it was Depricated before first release, and I forgot to remove it.

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
