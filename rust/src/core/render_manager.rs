use winit::dpi::PhysicalSize;
use winit::window::Window;
use wgpu::{Device, Queue, Surface, SurfaceConfiguration, PresentMode, TextureUsages};
use std::sync::Arc;
use crate::types::Color;
use super::logging;
use crate::core::game_object::GameObject;
use crate::core::object_manager::ObjectManager;
use priority_queue::PriorityQueue;

/// Manages the rendering pipeline using wgpu
pub struct RenderManager {
    device: Device,
    queue: Queue,
    surface: Surface<'static>,
    surface_config: SurfaceConfiguration,
    vsync_enabled: bool,
    background_color: Color,
    // Keep a reference to the window to ensure it outlives the surface
    _window: Arc<Window>,
    // Pending resize size - only reconfigure when actually rendering to avoid
    // expensive reconfigurations during rapid resize events
    pending_resize: Option<PhysicalSize<u32>>,
}

impl RenderManager {
    /// Create a new RenderManager with the given window reference
    /// 
    /// This is an async function because it needs to request a GPU adapter
    /// and create a device, which are async operations.
    pub async fn new(window: Arc<Window>, background_color: Option<Color>, vsync: bool) -> Result<Self, Box<dyn std::error::Error>> {
        let size = window.inner_size();

        // Create the wgpu instance
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        // SAFETY: The surface must not outlive the window that created it.
        // We store a clone of the Arc<Window>, ensuring the window lives
        // for as long as the surface. The surface is owned by RenderManager
        // which will be dropped when the engine shuts down.
        let surface = instance.create_surface(Arc::clone(&window))?;

        // Request an adapter (GPU handle)
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await?;

        // Log graphics backend information
        let adapter_info = adapter.get_info();
        logging::log_info(&format!(
            "Graphics backend: {:?} ({}), device: {}",
            adapter_info.backend,
            adapter_info.driver_info,
            adapter_info.name
        ));

        // Request a device and command queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("PyG Engine Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    memory_hints: Default::default(),
                    experimental_features: Default::default(),
                    trace: Default::default(),
                },
            )
            .await?;

        // Get surface capabilities
        let surface_caps = surface.get_capabilities(&adapter);
        
        // Choose a surface format (prefer sRGB)
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        // Configure the surface
        let present_mode = if vsync {
            PresentMode::Fifo // VSync on
        } else {
            PresentMode::Immediate // VSync off
        };
        
        let surface_config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &surface_config);

        Ok(Self {
            device,
            queue,
            surface,
            surface_config,
            vsync_enabled: vsync,
            background_color: background_color.unwrap_or(Color::BLACK),
            _window: window,
            pending_resize: None,
        })
    }

    /// Render a frame
    /// 
    /// This function acquires a surface texture, renders to it, and presents it.
    /// Returns an error if the surface needs to be reconfigured or if rendering fails.
    pub fn render(&mut self, objects: &Option<ObjectManager>) -> Result<(), wgpu::SurfaceError> {
        // Handle pending resize before rendering to avoid reconfiguring
        // multiple times during rapid resize events
        if let Some(new_size) = self.pending_resize.take() {
            if new_size.width > 0 && new_size.height > 0 {
                self.surface_config.width = new_size.width;
                self.surface_config.height = new_size.height;
                self.surface.configure(&self.device, &self.surface_config);
            }
        }
        
        // Acquire the next frame
        let output = self.surface.get_current_texture()?;
        
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // Create a command encoder
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        // Create a render pass (clear the screen with a color)
        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.background_color.to_wgpu()),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
                multiview_mask: None,
            });

        }

        // let mut sprite_objects = PriorityQueue::::new();

        // Submit the command buffer and present the frame
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    /// Resize the surface to match the new window size
    /// 
    /// Should be called when the window is resized.
    /// The actual reconfiguration is deferred until the next render call
    /// to avoid expensive reconfigurations during rapid resize events.
    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            // Store the pending resize instead of immediately reconfiguring
            // This prevents expensive surface reconfigurations during rapid resize events
            self.pending_resize = Some(new_size);
        }
    }

    /// Configure VSync (vertical synchronization)
    /// 
    /// When enabled, uses Fifo present mode (VSync on).
    /// When disabled, uses Immediate mode (VSync off, may tear).
    pub fn configure_vsync(&mut self, vsync_enabled: bool) {
        self.vsync_enabled = vsync_enabled;
        
        self.surface_config.present_mode = if vsync_enabled {
            PresentMode::Fifo // VSync on
        } else {
            PresentMode::Immediate // VSync off
        };
        
        self.surface.configure(&self.device, &self.surface_config);
    }

    /// Get whether VSync is currently enabled
    pub fn is_vsync_enabled(&self) -> bool {
        self.vsync_enabled
    }

    /// Get the current surface configuration
    pub fn surface_config(&self) -> &SurfaceConfiguration {
        &self.surface_config
    }

    /// Get a reference to the GPU device
    pub fn device(&self) -> &Device {
        &self.device
    }

    /// Get a reference to the command queue
    pub fn queue(&self) -> &Queue {
        &self.queue
    }
}

