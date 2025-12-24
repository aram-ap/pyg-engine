use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event_loop::ActiveEventLoop;
use winit::window::{Fullscreen, Icon, Window};
use std::sync::Arc;
use super::logging;
use crate::types::Color;

/// Fullscreen mode options for the window
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FullscreenMode {
    /// Window mode (not fullscreen)
    None,
    /// Borderless fullscreen (recommended for most games)
    Borderless,
    /// Exclusive fullscreen (may change display mode)
    Exclusive,
}

/// Configuration for creating a window
pub struct WindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub resizable: bool,
    pub fullscreen: FullscreenMode,
    pub min_width: Option<u32>,
    pub min_height: Option<u32>,
    pub max_width: Option<u32>,
    pub max_height: Option<u32>,
    pub icon: Option<Icon>,
    pub background_color: Option<Color>,
    pub vsync: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: "PyG Engine".to_string(),
            width: 1280,
            height: 720,
            resizable: true,
            fullscreen: FullscreenMode::None,
            min_width: None,
            min_height: None,
            max_width: None,
            max_height: None,
            icon: None,
            background_color: None,
            vsync: true,
        }
    }
}

impl WindowConfig {
    /// Create a new window configuration builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the window title
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Set the window dimensions
    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Set whether the window is resizable
    pub fn with_resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    /// Set the fullscreen mode
    pub fn with_fullscreen(mut self, mode: FullscreenMode) -> Self {
        self.fullscreen = mode;
        self
    }

    /// Set minimum window size constraints
    pub fn with_min_size(mut self, min_width: u32, min_height: u32) -> Self {
        self.min_width = Some(min_width);
        self.min_height = Some(min_height);
        self
    }

    /// Set maximum window size constraints
    pub fn with_max_size(mut self, max_width: u32, max_height: u32) -> Self {
        self.max_width = Some(max_width);
        self.max_height = Some(max_height);
        self
    }

    /// Set the window icon
    pub fn with_icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Set the background color
    pub fn with_background_color(mut self, color: Color) -> Self {
        self.background_color = Some(color);
        self
    }

    /// Set VSync (vertical synchronization)
    /// 
    /// When enabled (default), the frame rate will be limited to the display's refresh rate.
    /// When disabled, frames will be presented immediately, which may cause screen tearing.
    pub fn with_vsync(mut self, vsync: bool) -> Self {
        self.vsync = vsync;
        self
    }
}

/// Manages the game window using winit
pub struct WindowManager {
    window: Arc<Window>,
    current_size: PhysicalSize<u32>,
}

impl WindowManager {
    /// Create a new WindowManager with the given configuration
    pub fn new(event_loop: &ActiveEventLoop, config: WindowConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let mut window_attrs = Window::default_attributes()
            .with_title(&config.title)
            .with_inner_size(LogicalSize::new(config.width, config.height))
            .with_resizable(config.resizable);

        // Apply min/max size constraints
        if let (Some(min_w), Some(min_h)) = (config.min_width, config.min_height) {
            window_attrs = window_attrs.with_min_inner_size(LogicalSize::new(min_w, min_h));
        }
        if let (Some(max_w), Some(max_h)) = (config.max_width, config.max_height) {
            window_attrs = window_attrs.with_max_inner_size(LogicalSize::new(max_w, max_h));
        }

        // Apply icon if provided
        if let Some(icon) = config.icon {
            window_attrs = window_attrs.with_window_icon(Some(icon));
        }

        // Apply fullscreen mode
        let fullscreen = match config.fullscreen {
            FullscreenMode::None => None,
            FullscreenMode::Borderless => {
                Some(Fullscreen::Borderless(None))
            }
            FullscreenMode::Exclusive => {
                // For exclusive fullscreen, we'd need to select a video mode
                // For now, default to borderless if exclusive is requested
                Some(Fullscreen::Borderless(None))
            }
        };
        if let Some(fs) = fullscreen {
            window_attrs = window_attrs.with_fullscreen(Some(fs));
        }

        let window = event_loop.create_window(window_attrs)?;
        let current_size = window.inner_size();

        // Log window creation details
        logging::log_info(&format!(
            "Window created: title='{}', size={}x{}, fullscreen={:?}",
            config.title,
            current_size.width,
            current_size.height,
            config.fullscreen
        ));

        Ok(Self {
            window: Arc::new(window),
            current_size,
        })
    }

    /// Get a reference to the underlying winit Window
    pub fn window(&self) -> &Window {
        &self.window
    }

    /// Get an Arc clone of the window (useful for sharing across threads)
    pub fn window_arc(&self) -> Arc<Window> {
        Arc::clone(&self.window)
    }

    /// Get the current window size
    pub fn size(&self) -> PhysicalSize<u32> {
        self.current_size
    }

    /// Update the stored window size (should be called when resize events occur)
    pub fn update_size(&mut self, new_size: PhysicalSize<u32>) {
        self.current_size = new_size;
    }

    /// Set the window title
    pub fn set_title(&self, title: &str) {
        self.window.set_title(title);
    }

    /// Set the window size
    pub fn set_size(&self, width: u32, height: u32) {
        let _ = self.window.request_inner_size(LogicalSize::new(width, height));
    }

    /// Set the fullscreen mode
    pub fn set_fullscreen(&self, mode: FullscreenMode) {
        let fullscreen = match mode {
            FullscreenMode::None => None,
            FullscreenMode::Borderless => Some(Fullscreen::Borderless(None)),
            FullscreenMode::Exclusive => {
                // Default to borderless for now
                Some(Fullscreen::Borderless(None))
            }
        };
        self.window.set_fullscreen(fullscreen);
    }

    /// Request a redraw of the window
    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    /// Set the window's resizable state
    pub fn set_resizable(&self, resizable: bool) {
        self.window.set_resizable(resizable);
    }

    /// Set minimum window size
    pub fn set_min_size(&self, min_width: Option<u32>, min_height: Option<u32>) {
        let min_size = match (min_width, min_height) {
            (Some(w), Some(h)) => Some(LogicalSize::new(w, h)),
            _ => None,
        };
        self.window.set_min_inner_size(min_size);
    }

    /// Set maximum window size
    pub fn set_max_size(&self, max_width: Option<u32>, max_height: Option<u32>) {
        let max_size = match (max_width, max_height) {
            (Some(w), Some(h)) => Some(LogicalSize::new(w, h)),
            _ => None,
        };
        self.window.set_max_inner_size(max_size);
    }

    /// Get the window's scale factor
    pub fn scale_factor(&self) -> f64 {
        self.window.scale_factor()
    }

    /// Check if the window is currently focused
    pub fn has_focus(&self) -> bool {
        self.window.has_focus()
    }
}

