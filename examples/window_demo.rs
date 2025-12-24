/// Window Manager Demo
/// 
/// This example demonstrates how to create and run a window using the
/// PyG Engine's window manager and render manager.
/// 
/// To run this example:
/// ```
/// cargo run --example window_demo --no-default-features
/// ```
/// 
/// Note: The --no-default-features flag is required to disable Python bindings
/// when running standalone Rust examples.

use pyg_engine_native::core::{Engine, WindowConfig, FullscreenMode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create engine instance with custom logging
    let engine = Engine::with_logging(
        true,                       // Enable file logging
        Some("logs".to_string()),   // Log directory
        Some("INFO".to_string()),   // Log level
    );
    
    // Create window configuration using the builder pattern
    let window_config = WindowConfig::new()
        .with_title("PyG Engine - Window Demo")
        .with_size(1280, 720)
        .with_resizable(true)
        .with_fullscreen(FullscreenMode::None)
        .with_min_size(640, 480)
        .with_max_size(1920, 1080);
    
    // Run the engine with the window
    // This will:
    // 1. Create the window with the specified configuration
    // 2. Initialize the render manager with wgpu
    // 3. Start the event loop
    // 4. Render frames continuously (blue clear color)
    // 5. Handle window events (resize, close, etc.)
    println!("Starting engine...");
    println!("Press ESC or close the window to exit.");
    
    engine.run(window_config)?;
    
    println!("Engine shut down successfully.");
    Ok(())
}

