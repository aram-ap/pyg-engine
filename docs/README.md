# PyG Engine Examples

This directory contains Rust and Python examples demonstrating features of the PyG Engine.

## Running Rust Examples

All examples must be run with the `--no-default-features` flag to disable Python bindings:

```bash
cargo run --example window_demo --no-default-features
```

This is required because the library uses pyo3's `extension-module` feature by default, which expects to be loaded by Python. When running standalone Rust examples, we need to disable this feature.

## Running Python Examples

Install the package in editable mode first:

```bash
pip install -e .
```

Then run a Python demo from the project root:

```bash
python examples/python_direct_draw_demo.py
python examples/python_mesh_demo.py
```

## Available Examples

### window_demo

Demonstrates the window manager and render manager:
- Creates a resizable 1280x720 window
- Renders a blue clear color
- Supports window resizing
- Min/max size constraints
- Proper shutdown on close

**Run:**
```bash
cargo run --example window_demo --no-default-features
```

**Controls:**
- Close the window or press Alt+F4 to exit
- Resize the window by dragging edges
- The window enforces min size (640x480) and max size (1920x1080)

### mesh_demo

Demonstrates mesh rendering through Rust-side GameObject + MeshComponent:
- Color-filled mesh rendering
- Textured mesh rendering
- Transform (position, scale, rotation)
- Single draw-order value

**Run:**
```bash
cargo run --example mesh_demo --no-default-features
```

### draw_primitives_demo

Demonstrates direct drawing primitives on the Rust backend:
- Pixels
- Lines
- Rectangles (filled + outline)
- Circles (filled + outline)

**Run:**
```bash
cargo run --example draw_primitives_demo --no-default-features
```

### python_direct_draw_demo.py

Demonstrates immediate-mode direct draw from Python:
- `engine.draw_pixel(...)`
- `engine.draw_line(...)`
- `engine.draw_rectangle(...)`
- `engine.draw_circle(...)`

**Run:**
```bash
python examples/python_direct_draw_demo.py
```

### python_mesh_demo.py

Demonstrates Python-side scene setup with GameObject + MeshComponent:
- Build mesh components in Python
- Set transform and render ordering
- Add objects to runtime engine scene

**Run:**
```bash
python examples/python_mesh_demo.py
```

See `examples/PYTHON_RENDERING_GUIDE.md` for API details.

## Creating New Examples

1. Create a new `.rs` file in this directory
2. Use the following template:

```rust
use pyg_engine_native::core::{Engine, WindowConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let engine = Engine::new();
    let config = WindowConfig::new()
        .with_title("My Example")
        .with_size(800, 600);
    engine.run(config)?;
    Ok(())
}
```

3. Run it with:
```bash
cargo run --example your_example_name --no-default-features
```

## Troubleshooting

**Error: STATUS_DLL_NOT_FOUND (0xc0000135)**
- Solution: Add `--no-default-features` flag when running examples

**Error: unresolved import or module not found**
- Make sure `Cargo.toml` has `crate-type = ["cdylib", "rlib"]` in `[lib]` section
- Ensure `core` and `types` modules are marked as `pub` in `rust/src/lib.rs`

**Window doesn't open or crashes immediately**
- Check GPU drivers are up to date
- Verify wgpu can find a compatible adapter (check console output)
- Try running in release mode: `cargo run --example window_demo --no-default-features --release`

