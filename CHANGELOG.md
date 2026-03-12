# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

No changes yet.

## [1.3.0] - 2026-03-12

### Added
- Introduced a new font system with named font family registration, direct `font_path` support, and shared text styling across `draw_text`, `pyg.Text`, `TextMeshComponent`, `Label`, and `Button`.
- Added support for font variants such as regular, bold, italic, and bold-italic through family registration and automatic variant resolution.
- Added text measurement APIs to help align and size proportional text more accurately in UI and game scenes.

### Changed
- Upgraded text rendering to use cached font faces, cached text layouts, cached glyph rasters, and cached text textures for better performance with repeated and dynamic text.
- Enabled kerning-aware text rendering by default when supported by the active font backend, improving spacing and overall text aesthetics.
- Relative font, texture, image, and window icon paths now resolve from the calling script's source directory by default, while absolute paths continue to work unchanged.
- UI text rendering now honors configured font styling instead of always falling back to the default text path.

### Fixed
- Fixed button and label text layout behavior so proportional fonts can be centered and styled more consistently.
- Fixed source-root asset resolution so generated in-memory text texture keys are no longer treated like filesystem paths.

**Full Changelog**: https://github.com/aram-ap/pyg-engine/compare/v1.2.7...v1.3.0

## [1.2.7] - 2026-03-12

**Full Changelog**: https://github.com/aram-ap/pyg-engine/compare/v1.2.6...v1.2.7

## [1.2.6] - 2026-03-12

**Full Changelog**: https://github.com/aram-ap/pyg-engine/compare/v1.2.5...v1.2.6

## [1.2.5] - 2026-03-12

**Full Changelog**: https://github.com/aram-ap/pyg-engine/compare/v1.2.4...v1.2.5

## [1.2.4] - 2026-02-12

### Fixed
- Fixed issue related to build system caching.

## [1.2.3] - 2026-02-12

### Fixed
- Fixed issues related to scaling and rotation of meshes.

**Full Changelog**: https://github.com/aram-ap/pyg-engine/compare/v1.2.2...v1.2.3

## [1.2.2] - 2026-02-12

### Fixed
- Build system fixes.

## [1.2.1] - 2026-02-12

**Full Changelog**: https://github.com/aram-ap/pyg-engine/compare/v1.2.0a7...v1.2.1

## [1.2.0a7] - 2026-02-12

### Fixed
- Fixed issue that caused errors installing from PyPI.

**Full Changelog**: https://github.com/aram-ap/pyg-engine/compare/v1.2.0a6...v1.2.0a7

## [1.2.0a6] - 2026-02-12

**Full Changelog**: https://github.com/aram-ap/pyg-engine/compare/v1.0.0a6...v1.2.0a6

## [1.2.0a5] - 2026-02-12

**Full Changelog**: https://github.com/aram-ap/pyg-engine/compare/v1.0.0a6...v1.2.0a5

## [1.2.0a4] - 2026-02-12

**Full Changelog**: https://github.com/aram-ap/pyg-engine/compare/v1.0.0a6...v1.2.0a4

## [1.2.0a3] - 2026-02-12

**Full Changelog**: https://github.com/aram-ap/pyg-engine/compare/v1.0.0a6...v1.2.0a3

## [1.2.0a2] - 2026-02-12

### Changed
- Made changes to allow building for PyPI.

**Full Changelog**: https://github.com/aram-ap/pyg-engine/compare/v1.0.0a6...v1.2.0a2

## [1.2.0a1] - 2026-02-12

Made a few changes for publishing to PyPI.
This is the first release of the Rust-backend Pyg-Engine system. Better documents, more performant systems, better Pythonic styling. Get ready for more.

**Full Changelog**: https://github.com/aram-ap/pyg-engine/compare/v1.0.0a6...v1.2.0a

## [1.2.0a] - 2026-02-12

This is the first release of the Rust-backend Pyg-Engine system. Better documents, more performant systems, better Pythonic styling. Get ready for more.

**Full Changelog**: https://github.com/aram-ap/pyg-engine/compare/v1.0.0a6...v1.2.0a

## [1.0.0a6] - 2025-12-07

This release makes the engine actually usable, adding sprite systems, audio, a physics wrapper (no more using Pymunk directly), and other wrappers to reduce the need for importing pygame.

> ### NOTE
>
> This is likely the last release using pure Python.
>
> Currently, the engine is being rewritten in C++ using PyBind11 and SFML.
> It will bring a lot of changes in writing conventions and syntax but will be
> much more efficient, easier to write, and still compatible with Python.
>
> You can check out progress at the `pyg-engine@cpp` branch.

### Added
- Added the sprite class to the Pyg Engine.
- Added audio manager.
- Added a new debugging system (WIP) that can be used to see `GameObject` variables in real time (built with PyQt6).
- Added a variety of new examples.
- Added a new event system.

### Changed
- Updated the documentation.
- Updated the examples.
- Updated the structure of the engine.

**Full Changelog**: https://github.com/aram-ap/pyg-engine/compare/v1.0.0a5...v1.0.0a6

## [1.0.0a5] - 2025-08-02

## What's Changed
- Create `jekyll-gh-pages.yml` by @aram-ap in https://github.com/aram-ap/pyg-engine/pull/1

**Full Changelog**: https://github.com/aram-ap/pyg-engine/compare/v1.0.0a4_r1...v1.0.0a5

## [1.0.0a4_r1] - 2025-07-31

Forgot to change version number :P

**Full Changelog**: https://github.com/aram-ap/pyg-engine/compare/v1.0.0a4...v1.0.0a4_r1

## [1.0.0a4] - 2025-07-31

# PyG Engine v1.0.0a4 Release Notes

## Big Update: Event System & Better Docs

This is a big update - added a proper event system, added a unified input system, and significantly improved the documentation.

### Event System
- Event class with type, data, and timestamps.
- `EventManager` for thread-safe event handling with priority levels (CRITICAL, HIGH, NORMAL, LOW).
- Flexible processing with immediate or queued events.
- Memory management via weak references.
- Component integration with the existing component system.
- Advanced features like event chaining, filtering, and broadcasting.

### Input System Overhaul
- Unified interface for keyboard, mouse, and joystick.
- Axis-based input (e.g. WASD + arrow keys).
- Event-based input with better timing for button presses.
- Joystick support with auto-detection and mapping.
- Input aliases using string-based mappings.
- Component-based mouse interaction.

### Documentation
- Added a `CORE_SYSTEMS_GUIDE` covering all engine systems with examples.
- Detailed class docs with functions and properties.
- Integration patterns and best practices.
- Performance tips.
- Testing instructions.

### Runnable System
- Priority-based execution with four levels.
- Performance monitoring.
- Flexible event types and error handling.

### Headless Mode
- Run the engine without a display for automated testing, server-side logic, and AI training.

### Examples
- New examples for input, events, performance tools, and a snake game.

### Technical
- Improved thread safety and memory cleanup.
- Optimized event processing.
- Better overall architecture.

**Full Changelog**: https://github.com/aram-ap/pyg-engine/compare/v1.0.0a3...v1.0.0a4

## [1.0.0a3] - 2025-07-30

### Fixed
- Fixed scripting system's lack of `start()` and `on_destroy()` calls.

### Added
- Added a new testing system in `examples/test.py` and `examples/scripts/test_script.py` to better demonstrate the component and scripting systems.

### Changed
- Updated README to show updated filenames.
- Removed `ScriptRunner` as it was deprecated before first release.

**Full Changelog**: https://github.com/aram-ap/pyg-engine/compare/v1.0.0a2...v1.0.0a3

## [1.0.0a2] - 2025-07-30

### Fixed
- Fixed a bug where `GameObject` constructor would crash when using tuples for `position` and `size` parameters by automatically converting tuples and lists to `Vector2`.

This release is fully backward compatible; existing code using `Vector2` continues to work unchanged.

## [1.0.0a1] - 2025-07-30

### Features
- Initial Pyg Engine implementation.
- 2D physics system with Pymunk integration.
- Mouse and keyboard input handling.
- Component-based game object system.
- Camera system with multiple scaling modes.
- Collision detection and response.

### Installation
```bash
pip install --pre pyg-engine
```

### Documentation
See the `README.md` for usage examples and documentation.

### Note
This is an alpha release. Features may change in future versions.

## [1.2.0] - 2025-12-22

### The switch to Rust
This is it, we're switching to Rust. Moving ahead from version 1.1.0,
which brought pyg-engine a C++ backing.
Why am I doing this?
- The C++ backing wasn't all that developed yet, meaning there's not too much work being lost.
- Rust is personally a really compelling option. I haven't learned it thoroughly yet, so this is my opportunity.
- F#@K dependency management with C++.

---

**Pyg Engine** - Making 2D game development in Python easier and more flexible! :)
