# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

## [1.2.0] - 2025-12-22

### The switch to Rust
This is it, we're switching to Rust. Moving ahead from version 1.1.0,
which brought pyg-engine a c++ backing. 
Why am I doing this? 
- The c++ backing wasn't all that developed yet, meaning there's not too much work being lost.
- Rust is personally a really compelling option. I haven't learned it thoroughly yet, so this is my opportunity. 
- F#@K dependency management with c++.

---

**Pyg Engine** - Making 2D game development in Python easier and more flexible! :)
