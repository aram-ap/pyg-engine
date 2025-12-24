/// Input Demo
///
/// This example demonstrates how to create and run a window using the
/// PyG Engine's window manager while feeding input events into the
/// `InputManager` and logging them.
///
/// To run this example:
/// ```bash
/// cargo run --example input_demo --no-default-features
/// ```
///
/// Note: The `--no-default-features` flag is required to disable Python
/// bindings when running standalone Rust examples.

use pyg_engine_native::core::{
    logging, FullscreenMode, InputManager, MouseButtonType, WindowConfig, WindowManager,
};
use pyg_engine_native::types::Color;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::WindowId;

struct InputApp {
    window_config: Option<WindowConfig>,
    window_manager: Option<WindowManager>,
    input_manager: InputManager,
}

impl ApplicationHandler for InputApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window_manager.is_none() {
            if let Some(config) = self.window_config.take() {
                match WindowManager::new(event_loop, config) {
                    Ok(window_manager) => {
                        logging::log_info("Input demo window created successfully");
                        self.window_manager = Some(window_manager);

                        if let Some(wm) = &self.window_manager {
                            wm.request_redraw();
                        }
                    }
                    Err(e) => {
                        logging::log_error(&format!("Failed to create window: {}", e));
                        event_loop.exit();
                    }
                }
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        // First, feed the event into the InputManager so its internal state is updated.
        self.input_manager.handle_window_event(&event);

        // Log high-level information about the input event.
        match &event {
            WindowEvent::CloseRequested => {
                logging::log_info("Close requested, shutting down input demo");
                event_loop.exit();
            }
            WindowEvent::Resized(physical_size) => {
                logging::log_debug(&format!(
                    "Window resized to: {}x{}",
                    physical_size.width, physical_size.height
                ));

                if let Some(window_manager) = &mut self.window_manager {
                    window_manager.update_size(*physical_size);
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                let key = &event.logical_key;
                match event.state {
                    ElementState::Pressed => {
                        logging::log_info(&format!("Key pressed: {:?}", key));
                    }
                    ElementState::Released => {
                        logging::log_info(&format!("Key released: {:?}", key));
                    }
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                logging::log_info(&format!("Mouse button {:?} {:?}", button, state));
            }
            WindowEvent::CursorMoved { position, .. } => {
                logging::log_info(&format!("Mouse moved to: ({}, {})", position.x, position.y));
            }
            WindowEvent::MouseWheel { delta, .. } => {
                logging::log_info(&format!("Mouse wheel: {:?}", delta));
            }
            WindowEvent::RedrawRequested => {
                // Update logical axes and derived input state
                self.input_manager.update();

                // Example: log current axis values and some button states
                let horizontal = self.input_manager.axis("Horizontal");
                let vertical = self.input_manager.axis("Vertical");
                let mouse_pos = self.input_manager.mouse_position();
                let left_mouse = self
                    .input_manager
                    .mouse_button_down(MouseButtonType::Left);

                logging::log_debug(&format!(
                    "Axes - Horizontal: {:.2}, Vertical: {:.2}, Mouse: ({:.1}, {:.1}), LMB: {}",
                    horizontal, vertical, mouse_pos.0, mouse_pos.1, left_mouse
                ));

                // Request next frame for continuous input sampling
                if let Some(window_manager) = &self.window_manager {
                    window_manager.request_redraw();
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window_manager) = &self.window_manager {
            window_manager.request_redraw();
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging to console
    logging::init_default();

    // Create the event loop
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);

    // Configure the window
    let window_config = WindowConfig::new()
        .with_title("PyG Engine - Input Demo")
        .with_size(1280, 720)
        .with_resizable(true)
        .with_fullscreen(FullscreenMode::None)
        .with_min_size(640, 480)
        .with_background_color(Color::DARK_GRAY)
        .with_vsync(false);

    // Create the application handler
    let mut app = InputApp {
        window_config: Some(window_config),
        window_manager: None,
        input_manager: InputManager::new(),
    };

    logging::log_info("Starting input demo...");
    logging::log_info("Move the mouse or press keys to see input logs.");

    // Run the event loop
    event_loop.run_app(&mut app)?;

    logging::log_info("Input demo shut down successfully.");
    Ok(())
}


