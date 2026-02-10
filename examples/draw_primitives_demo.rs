/// Direct Primitive Drawing Demo
///
/// Demonstrates immediate-mode drawing APIs:
/// - pixels
/// - lines
/// - rectangles (filled + outline)
/// - circles (filled + outline)
///
/// To run:
/// `cargo run --example draw_primitives_demo --no-default-features`

use pyg_engine_native::core::{Engine, FullscreenMode, WindowConfig};
use pyg_engine_native::types::Color;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut engine = Engine::new();

    let window_config = WindowConfig::new()
        .with_title("PyG Engine - Direct Draw Demo")
        .with_size(1280, 720)
        .with_fullscreen(FullscreenMode::None)
        .with_background_color(Color::rgb(18, 18, 28))
        .with_redraw_on_change_only(true);

    // Draw a small pixel block in the top-left area.
    for x in 20..50 {
        for y in 20..50 {
            engine.draw_pixel(x, y, Color::WHITE);
        }
    }

    // Lines
    engine.draw_line(80.0, 120.0, 600.0, 180.0, Color::CYAN);
    engine.draw_line(80.0, 180.0, 600.0, 350.0, Color::LIME);

    // Rectangles
    engine.draw_rectangle(120.0, 420.0, 220.0, 120.0, Color::ORANGE);
    engine.draw_rectangle_outline(380.0, 420.0, 220.0, 120.0, 3.0, Color::WHITE);

    // Circles
    engine.draw_circle(860.0, 220.0, 90.0, Color::MAGENTA);
    engine.draw_circle_outline(1060.0, 220.0, 90.0, 4.0, Color::YELLOW);

    engine.run(window_config)?;
    Ok(())
}
