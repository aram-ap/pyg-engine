use std::borrow::Cow;
use std::cmp::Ordering;
use std::collections::{HashMap, hash_map::DefaultHasher};
use std::f32::consts::TAU;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

use font8x8::{BASIC_FONTS, UnicodeFonts};
use fontdue::Font;
use image::GenericImageView;
use wgpu::{Device, PresentMode, Queue, Surface, SurfaceConfiguration, TextureUsages};
use winit::dpi::PhysicalSize;
use winit::window::Window;

use super::geometry::Vertex;
use super::logging;
use crate::core::draw_manager::{DrawCommand, DrawManager};
use crate::core::object_manager::ObjectManager;
use crate::types::Color;
use crate::types::vector::Vec2;

struct CachedTexture {
    texture: wgpu::Texture,
    _view: wgpu::TextureView,
    _sampler: wgpu::Sampler,
    bind_group: wgpu::BindGroup,
    width: u32,
    height: u32,
}

#[derive(Clone)]
struct DrawItem {
    draw_order: f32,
    texture_path: Option<String>,
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
}

struct PreparedDraw {
    bind_group: wgpu::BindGroup,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    index_count: u32,
}

struct PendingTextureUpload {
    key: String,
    rgba: Arc<[u8]>,
    width: u32,
    height: u32,
}

struct RasterizedText {
    rgba: Vec<u8>,
    width: u32,
    height: u32,
}

struct PooledBuffer {
    buffer: wgpu::Buffer,
    capacity_bytes: usize,
}

const MIN_POOL_BUFFER_BYTES: usize = 256;
// Built-in default font comes from the `font8x8` crate (MIT/Apache-2.0).
const DEFAULT_FONT_NAME: &str = "font8x8-basic";
const DEFAULT_GLYPH_PIXEL_SIZE: f32 = 8.0;

fn hash_f32<H: Hasher>(hasher: &mut H, value: f32) {
    value.to_bits().hash(hasher);
}

fn hash_color<H: Hasher>(hasher: &mut H, color: &Color) {
    hash_f32(hasher, color.r());
    hash_f32(hasher, color.g());
    hash_f32(hasher, color.b());
    hash_f32(hasher, color.a());
}

/// Camera aspect handling policy for world-space rendering.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CameraAspectMode {
    /// Scale x/y independently to always fill the window (may distort).
    #[default]
    Stretch,
    /// Preserve aspect by keeping configured world width fixed.
    MatchHorizontal,
    /// Preserve aspect by keeping configured world height fixed.
    MatchVertical,
    /// Preserve aspect and show full viewport with letter/pillar boxing.
    FitBoth,
    /// Preserve aspect and fill window by cropping one axis.
    FillBoth,
}

impl CameraAspectMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Stretch => "stretch",
            Self::MatchHorizontal => "match_horizontal",
            Self::MatchVertical => "match_vertical",
            Self::FitBoth => "fit_both",
            Self::FillBoth => "fill_both",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct SceneVersion {
    render_state_epoch: u64,
    object_epoch: u64,
    draw_epoch: u64,
}

/// Manages the rendering pipeline using wgpu.
pub struct RenderManager {
    device: Device,
    queue: Queue,
    surface: Surface<'static>,
    surface_config: SurfaceConfiguration,
    vsync_enabled: bool,
    background_color: Color,
    redraw_on_change_only: bool,
    // Keep a reference to the window to ensure it outlives the surface.
    _window: Arc<Window>,
    // Pending resize size - only reconfigure when actually rendering to avoid
    // expensive reconfigurations during rapid resize events.
    pending_resize: Option<PhysicalSize<u32>>,
    requires_redraw: bool,
    last_scene_version: Option<SceneVersion>,
    precomputed_scene_version: Option<SceneVersion>,
    render_state_epoch: u64,
    render_pipeline: wgpu::RenderPipeline,
    texture_bind_group_layout: wgpu::BindGroupLayout,
    default_texture: CachedTexture,
    texture_cache: HashMap<String, Option<CachedTexture>>,
    texture_data_signature_cache: HashMap<String, u64>,
    font_cache: HashMap<String, Option<Font>>,
    vertex_buffer_pool: Vec<PooledBuffer>,
    index_buffer_pool: Vec<PooledBuffer>,
    active_camera_object_id: Option<u32>,
    camera_viewport_size: Option<Vec2>,
    camera_aspect_mode: CameraAspectMode,
}

impl RenderManager {
    /// Create a new RenderManager with the given window reference.
    ///
    /// This is an async function because it needs to request a GPU adapter
    /// and create a device, which are async operations.
    pub async fn new(
        window: Arc<Window>,
        background_color: Option<Color>,
        vsync: bool,
        redraw_on_change_only: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let size = window.inner_size();

        // Create the wgpu instance.
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        // SAFETY: The surface must not outlive the window that created it.
        // We store a clone of the Arc<Window>, ensuring the window lives
        // for as long as the surface. The surface is owned by RenderManager
        // which will be dropped when the engine shuts down.
        let surface = instance.create_surface(Arc::clone(&window))?;

        // Request an adapter (GPU handle).
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await?;

        // Log graphics backend information.
        let adapter_info = adapter.get_info();
        logging::log_info(&format!(
            "Graphics backend: {:?} ({}), device: {}",
            adapter_info.backend, adapter_info.driver_info, adapter_info.name
        ));

        // Request a device and command queue.
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("PyG Engine Device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: Default::default(),
                experimental_features: Default::default(),
                trace: Default::default(),
            })
            .await?;

        // Get surface capabilities.
        let surface_caps = surface.get_capabilities(&adapter);

        // Choose a surface format (prefer sRGB).
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        // Configure the surface.
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

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("mesh_texture_bind_group_layout"),
            });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("mesh_shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("mesh_render_pipeline_layout"),
                bind_group_layouts: &[&texture_bind_group_layout],
                immediate_size: 0,
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("mesh_render_pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

        let default_texture = Self::create_cached_texture(
            &device,
            &queue,
            &texture_bind_group_layout,
            &[255, 255, 255, 255],
            1,
            1,
            "default_white_texture",
        );

        Ok(Self {
            device,
            queue,
            surface,
            surface_config,
            vsync_enabled: vsync,
            background_color: background_color.unwrap_or(Color::BLACK),
            redraw_on_change_only,
            _window: window,
            pending_resize: None,
            requires_redraw: true,
            last_scene_version: None,
            precomputed_scene_version: None,
            render_state_epoch: 0,
            render_pipeline,
            texture_bind_group_layout,
            default_texture,
            texture_cache: HashMap::new(),
            texture_data_signature_cache: HashMap::new(),
            font_cache: HashMap::new(),
            vertex_buffer_pool: Vec::new(),
            index_buffer_pool: Vec::new(),
            active_camera_object_id: None,
            camera_viewport_size: None,
            camera_aspect_mode: CameraAspectMode::default(),
        })
    }

    fn create_cached_texture(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        layout: &wgpu::BindGroupLayout,
        rgba: &[u8],
        width: u32,
        height: u32,
        label: &str,
    ) -> CachedTexture {
        let size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some(label),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            rgba,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
            label: Some("mesh_texture_bind_group"),
        });

        CachedTexture {
            texture,
            _view: view,
            _sampler: sampler,
            bind_group,
            width,
            height,
        }
    }

    fn load_texture_from_path(&self, texture_path: &str) -> Result<CachedTexture, String> {
        let bytes = std::fs::read(texture_path)
            .map_err(|e| format!("failed to read texture '{}': {e}", texture_path))?;

        let img = image::load_from_memory(&bytes)
            .map_err(|e| format!("failed to decode texture '{}': {e}", texture_path))?;
        let rgba = img.to_rgba8();
        let (width, height) = img.dimensions();

        Ok(Self::create_cached_texture(
            &self.device,
            &self.queue,
            &self.texture_bind_group_layout,
            rgba.as_raw(),
            width,
            height,
            texture_path,
        ))
    }

    fn cache_texture_from_rgba(
        &mut self,
        texture_key: &str,
        rgba: &[u8],
        width: u32,
        height: u32,
    ) -> Result<(), String> {
        let expected_size = (width as usize)
            .checked_mul(height as usize)
            .and_then(|value| value.checked_mul(4))
            .ok_or_else(|| {
                format!("texture size overflow while caching texture '{texture_key}'")
            })?;

        if rgba.len() != expected_size {
            return Err(format!(
                "failed to cache texture '{texture_key}': expected {expected_size} bytes ({}x{} RGBA), got {} bytes",
                width,
                height,
                rgba.len()
            ));
        }

        let mut hasher = DefaultHasher::new();
        texture_key.hash(&mut hasher);
        width.hash(&mut hasher);
        height.hash(&mut hasher);
        rgba.hash(&mut hasher);
        let signature = hasher.finish();

        if self
            .texture_data_signature_cache
            .get(texture_key)
            .is_some_and(|prev| *prev == signature)
        {
            return Ok(());
        }

        if let Some(Some(cached_texture)) = self.texture_cache.get_mut(texture_key) {
            if cached_texture.width == width && cached_texture.height == height {
                Self::write_rgba_to_texture(
                    &self.queue,
                    &cached_texture.texture,
                    rgba,
                    width,
                    height,
                );
                self.texture_data_signature_cache
                    .insert(texture_key.to_string(), signature);
                return Ok(());
            }
        }

        let cached = Self::create_cached_texture(
            &self.device,
            &self.queue,
            &self.texture_bind_group_layout,
            rgba,
            width,
            height,
            texture_key,
        );
        self.texture_cache
            .insert(texture_key.to_string(), Some(cached));
        self.texture_data_signature_cache
            .insert(texture_key.to_string(), signature);

        Ok(())
    }

    fn write_rgba_to_texture(
        queue: &wgpu::Queue,
        texture: &wgpu::Texture,
        rgba: &[u8],
        width: u32,
        height: u32,
    ) {
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            rgba,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            },
            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );
    }

    fn pooled_buffer_capacity(required_bytes: usize) -> usize {
        required_bytes
            .max(MIN_POOL_BUFFER_BYTES)
            .next_power_of_two()
    }

    fn write_to_pooled_buffer(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        pool: &mut Vec<PooledBuffer>,
        slot: usize,
        bytes: &[u8],
        usage: wgpu::BufferUsages,
        label: &str,
    ) -> wgpu::Buffer {
        debug_assert!(!bytes.is_empty(), "pooled buffer writes must not be empty");
        let required_bytes = bytes.len();

        if pool.len() <= slot {
            let capacity = Self::pooled_buffer_capacity(required_bytes);
            let buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(label),
                size: capacity as u64,
                usage: usage | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            pool.push(PooledBuffer {
                buffer,
                capacity_bytes: capacity,
            });
        }

        if pool[slot].capacity_bytes < required_bytes {
            let capacity = Self::pooled_buffer_capacity(required_bytes);
            pool[slot].buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(label),
                size: capacity as u64,
                usage: usage | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            pool[slot].capacity_bytes = capacity;
        }

        queue.write_buffer(&pool[slot].buffer, 0, bytes);
        pool[slot].buffer.clone()
    }

    fn texture_bind_group_for(&mut self, texture_path: Option<&str>) -> wgpu::BindGroup {
        if let Some(path) = texture_path {
            if !self.texture_cache.contains_key(path) {
                let loaded = match self.load_texture_from_path(path) {
                    Ok(texture) => Some(texture),
                    Err(err) => {
                        logging::log_warn(&format!("Texture load failed: {err}"));
                        None
                    }
                };
                self.texture_cache.insert(path.to_string(), loaded);
            }

            if let Some(Some(texture)) = self.texture_cache.get(path) {
                return texture.bind_group.clone();
            }
        }

        self.default_texture.bind_group.clone()
    }

    fn color_to_array(color: Color) -> [f32; 4] {
        [color.r(), color.g(), color.b(), color.a()]
    }

    fn default_camera_viewport_size(&self) -> Vec2 {
        let width = self.surface_config.width.max(1) as f32;
        let height = self.surface_config.height.max(1) as f32;
        let aspect = width / height;
        Vec2::new(2.0 * aspect, 2.0)
    }

    fn display_aspect_ratio(&self) -> f32 {
        let width = self.surface_config.width.max(1) as f32;
        let height = self.surface_config.height.max(1) as f32;
        width / height
    }

    fn effective_camera_viewport_size(&self) -> Vec2 {
        self.camera_viewport_size
            .unwrap_or_else(|| self.default_camera_viewport_size())
    }

    fn effective_world_viewport_size(&self, viewport: Vec2) -> Vec2 {
        let safe_display_aspect = self.display_aspect_ratio().max(f32::EPSILON);
        match self.camera_aspect_mode {
            CameraAspectMode::MatchHorizontal => Vec2::new(
                viewport.x().max(f32::EPSILON),
                (viewport.x() / safe_display_aspect).max(f32::EPSILON),
            ),
            CameraAspectMode::MatchVertical => Vec2::new(
                (viewport.y() * safe_display_aspect).max(f32::EPSILON),
                viewport.y().max(f32::EPSILON),
            ),
            _ => viewport,
        }
    }

    fn world_clip_scale(&self, viewport: Vec2) -> (f32, f32) {
        let safe_display_aspect = self.display_aspect_ratio().max(f32::EPSILON);
        let safe_viewport_aspect =
            (viewport.x().max(f32::EPSILON) / viewport.y().max(f32::EPSILON)).max(f32::EPSILON);
        match self.camera_aspect_mode {
            CameraAspectMode::FitBoth => {
                if safe_display_aspect >= safe_viewport_aspect {
                    (safe_viewport_aspect / safe_display_aspect, 1.0)
                } else {
                    (1.0, safe_display_aspect / safe_viewport_aspect)
                }
            }
            CameraAspectMode::FillBoth => {
                if safe_display_aspect >= safe_viewport_aspect {
                    (1.0, safe_display_aspect / safe_viewport_aspect)
                } else {
                    (safe_viewport_aspect / safe_display_aspect, 1.0)
                }
            }
            _ => (1.0, 1.0),
        }
    }

    fn world_to_clip(&self, world_x: f32, world_y: f32, camera_position: Vec2) -> [f32; 2] {
        let viewport = self.effective_camera_viewport_size();
        let effective_viewport = self.effective_world_viewport_size(viewport);
        let half_width = (effective_viewport.x() * 0.5).max(f32::EPSILON);
        let half_height = (effective_viewport.y() * 0.5).max(f32::EPSILON);

        let relative_x = world_x - camera_position.x();
        let relative_y = world_y - camera_position.y();
        let normalized_x = relative_x / half_width;
        let normalized_y = relative_y / half_height;
        let (clip_scale_x, clip_scale_y) = self.world_clip_scale(viewport);

        [normalized_x * clip_scale_x, normalized_y * clip_scale_y]
    }

    fn active_camera_position(&self, objects: &Option<ObjectManager>) -> Vec2 {
        let Some(camera_id) = self.active_camera_object_id else {
            return Vec2::new(0.0, 0.0);
        };

        objects
            .as_ref()
            .and_then(|object_manager| object_manager.get_object_by_id(camera_id))
            .map(|camera| *camera.transform().position())
            .unwrap_or_else(|| Vec2::new(0.0, 0.0))
    }

    pub fn camera_viewport_size(&self) -> (f32, f32) {
        let viewport = self.effective_camera_viewport_size();
        (viewport.x(), viewport.y())
    }

    pub fn set_camera_viewport_size(&mut self, width: f32, height: f32) {
        let safe_width = width.max(f32::EPSILON);
        let safe_height = height.max(f32::EPSILON);
        self.camera_viewport_size = Some(Vec2::new(safe_width, safe_height));
        self.requires_redraw = true;
        self.precomputed_scene_version = None;
        self.bump_render_state_epoch();
    }

    pub fn camera_aspect_mode(&self) -> CameraAspectMode {
        self.camera_aspect_mode
    }

    pub fn set_camera_aspect_mode(&mut self, mode: CameraAspectMode) {
        if self.camera_aspect_mode == mode {
            return;
        }

        self.camera_aspect_mode = mode;
        self.requires_redraw = true;
        self.precomputed_scene_version = None;
        self.bump_render_state_epoch();
    }

    pub fn set_active_camera_object_id(&mut self, camera_object_id: Option<u32>) {
        if self.active_camera_object_id == camera_object_id {
            return;
        }

        self.active_camera_object_id = camera_object_id;
        self.requires_redraw = true;
        self.precomputed_scene_version = None;
        self.bump_render_state_epoch();
    }

    pub fn world_to_screen(&self, world_position: Vec2, camera_position: Vec2) -> (f32, f32) {
        let clip = self.world_to_clip(world_position.x(), world_position.y(), camera_position);
        let width = self.surface_config.width.max(1) as f32;
        let height = self.surface_config.height.max(1) as f32;
        let screen_x = (clip[0] + 1.0) * 0.5 * width;
        let screen_y = (1.0 - clip[1]) * 0.5 * height;
        (screen_x, screen_y)
    }

    pub fn screen_to_world(&self, screen_x: f32, screen_y: f32, camera_position: Vec2) -> Vec2 {
        let width = self.surface_config.width.max(1) as f32;
        let height = self.surface_config.height.max(1) as f32;
        let clip_x = (screen_x / width) * 2.0 - 1.0;
        let clip_y = 1.0 - (screen_y / height) * 2.0;

        let viewport = self.effective_camera_viewport_size();
        let (clip_scale_x, clip_scale_y) = self.world_clip_scale(viewport);
        let normalized_x = clip_x / clip_scale_x.max(f32::EPSILON);
        let normalized_y = clip_y / clip_scale_y.max(f32::EPSILON);
        let effective_viewport = self.effective_world_viewport_size(viewport);
        let world_x = camera_position.x() + normalized_x * (effective_viewport.x() * 0.5);
        let world_y = camera_position.y() + normalized_y * (effective_viewport.y() * 0.5);
        Vec2::new(world_x, world_y)
    }

    fn pixel_to_clip(&self, x: f32, y: f32) -> [f32; 2] {
        let width = self.surface_config.width.max(1) as f32;
        let height = self.surface_config.height.max(1) as f32;
        let clip_x = (x / width) * 2.0 - 1.0;
        let clip_y = 1.0 - (y / height) * 2.0;
        [clip_x, clip_y]
    }

    fn build_quad_draw_item(
        &self,
        p0: [f32; 2],
        p1: [f32; 2],
        p2: [f32; 2],
        p3: [f32; 2],
        color: [f32; 4],
        draw_order: f32,
    ) -> DrawItem {
        self.build_quad_draw_item_with_options(
            p0,
            p1,
            p2,
            p3,
            [color, color, color, color],
            [[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]],
            None,
            draw_order,
        )
    }

    fn build_quad_draw_item_with_options(
        &self,
        p0: [f32; 2],
        p1: [f32; 2],
        p2: [f32; 2],
        p3: [f32; 2],
        colors: [[f32; 4]; 4],
        tex_coords: [[f32; 2]; 4],
        texture_path: Option<String>,
        draw_order: f32,
    ) -> DrawItem {
        DrawItem {
            draw_order,
            texture_path,
            vertices: vec![
                Vertex {
                    position: [p0[0], p0[1], 0.0],
                    color: colors[0],
                    tex_coords: tex_coords[0],
                },
                Vertex {
                    position: [p1[0], p1[1], 0.0],
                    color: colors[1],
                    tex_coords: tex_coords[1],
                },
                Vertex {
                    position: [p2[0], p2[1], 0.0],
                    color: colors[2],
                    tex_coords: tex_coords[2],
                },
                Vertex {
                    position: [p3[0], p3[1], 0.0],
                    color: colors[3],
                    tex_coords: tex_coords[3],
                },
            ],
            indices: vec![0, 1, 2, 0, 2, 3],
        }
    }

    fn build_filled_rect_draw_item(
        &self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color: Color,
        draw_order: f32,
    ) -> DrawItem {
        let x0 = x.min(x + width);
        let x1 = x.max(x + width);
        let y0 = y.min(y + height);
        let y1 = y.max(y + height);

        let p0 = self.pixel_to_clip(x0, y0);
        let p1 = self.pixel_to_clip(x0, y1);
        let p2 = self.pixel_to_clip(x1, y1);
        let p3 = self.pixel_to_clip(x1, y0);

        self.build_quad_draw_item(p0, p1, p2, p3, Self::color_to_array(color), draw_order)
    }

    fn build_gradient_rect_draw_item(
        &self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        top_left: Color,
        bottom_left: Color,
        bottom_right: Color,
        top_right: Color,
        draw_order: f32,
    ) -> DrawItem {
        let x0 = x.min(x + width);
        let x1 = x.max(x + width);
        let y0 = y.min(y + height);
        let y1 = y.max(y + height);

        let p0 = self.pixel_to_clip(x0, y0);
        let p1 = self.pixel_to_clip(x0, y1);
        let p2 = self.pixel_to_clip(x1, y1);
        let p3 = self.pixel_to_clip(x1, y0);

        self.build_quad_draw_item_with_options(
            p0,
            p1,
            p2,
            p3,
            [
                Self::color_to_array(top_left),
                Self::color_to_array(bottom_left),
                Self::color_to_array(bottom_right),
                Self::color_to_array(top_right),
            ],
            [[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]],
            None,
            draw_order,
        )
    }

    fn build_image_rect_draw_item(
        &self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        texture_path: String,
        draw_order: f32,
    ) -> DrawItem {
        let x0 = x.min(x + width);
        let x1 = x.max(x + width);
        let y0 = y.min(y + height);
        let y1 = y.max(y + height);

        let p0 = self.pixel_to_clip(x0, y0);
        let p1 = self.pixel_to_clip(x0, y1);
        let p2 = self.pixel_to_clip(x1, y1);
        let p3 = self.pixel_to_clip(x1, y0);
        let white = Self::color_to_array(Color::WHITE);

        self.build_quad_draw_item_with_options(
            p0,
            p1,
            p2,
            p3,
            [white, white, white, white],
            [[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]],
            Some(texture_path),
            draw_order,
        )
    }

    fn color_component_to_u8(value: f32) -> u8 {
        (value.clamp(0.0, 1.0) * 255.0).round() as u8
    }

    fn build_text_texture_key(
        text: &str,
        font_path: Option<&str>,
        font_size: f32,
        color: Color,
        letter_spacing: f32,
        line_spacing: f32,
    ) -> String {
        let mut hasher = DefaultHasher::new();
        text.hash(&mut hasher);
        font_path.unwrap_or(DEFAULT_FONT_NAME).hash(&mut hasher);
        hash_f32(&mut hasher, font_size);
        hash_color(&mut hasher, &color);
        hash_f32(&mut hasher, letter_spacing);
        hash_f32(&mut hasher, line_spacing);
        format!("__pyg_text_{:016x}", hasher.finish())
    }

    fn load_font_from_path(&mut self, font_path: &str) -> Option<&Font> {
        if !self.font_cache.contains_key(font_path) {
            let loaded = match std::fs::read(font_path) {
                Ok(bytes) => match Font::from_bytes(bytes, fontdue::FontSettings::default()) {
                    Ok(font) => Some(font),
                    Err(err) => {
                        logging::log_warn(&format!(
                            "Failed to decode font '{font_path}': {err}. Falling back to built-in font."
                        ));
                        None
                    }
                },
                Err(err) => {
                    logging::log_warn(&format!(
                        "Failed to read font '{font_path}': {err}. Falling back to built-in font."
                    ));
                    None
                }
            };
            self.font_cache.insert(font_path.to_string(), loaded);
        }

        self.font_cache
            .get(font_path)
            .and_then(|font| font.as_ref())
    }

    fn rasterize_text_font8x8(
        text: &str,
        font_size: f32,
        color: Color,
        letter_spacing: f32,
        line_spacing: f32,
    ) -> Option<RasterizedText> {
        if text.is_empty() {
            return None;
        }

        let scale = (font_size.max(1.0) / DEFAULT_GLYPH_PIXEL_SIZE)
            .max(1.0)
            .round() as i32;
        let glyph_width = (DEFAULT_GLYPH_PIXEL_SIZE as i32) * scale;
        let glyph_height = (DEFAULT_GLYPH_PIXEL_SIZE as i32) * scale;
        let spacing_x = (letter_spacing.round() as i32).max(-(glyph_width - 1));
        let spacing_y = (line_spacing.round() as i32).max(-(glyph_height - 1));

        let lines: Vec<&str> = text.split('\n').collect();
        if lines.is_empty() {
            return None;
        }

        let mut max_width = 0i32;
        for line in &lines {
            let glyph_count = line.chars().count() as i32;
            let line_width = if glyph_count == 0 {
                0
            } else {
                glyph_count * glyph_width + (glyph_count - 1) * spacing_x
            };
            max_width = max_width.max(line_width);
        }

        let line_count = lines.len() as i32;
        let total_height = if line_count == 0 {
            0
        } else {
            line_count * glyph_height + (line_count - 1) * spacing_y
        };

        let width = max_width.max(1) as u32;
        let height = total_height.max(1) as u32;
        let mut rgba = vec![0u8; (width as usize) * (height as usize) * 4];

        let r = Self::color_component_to_u8(color.r());
        let g = Self::color_component_to_u8(color.g());
        let b = Self::color_component_to_u8(color.b());
        let a = Self::color_component_to_u8(color.a());

        for (line_index, line) in lines.iter().enumerate() {
            let mut pen_x = 0i32;
            let base_y = line_index as i32 * (glyph_height + spacing_y);

            for ch in line.chars() {
                let glyph = BASIC_FONTS.get(ch).or_else(|| BASIC_FONTS.get('?'));
                if let Some(bitmap) = glyph {
                    for (row, bits) in bitmap.iter().enumerate() {
                        for col in 0..8usize {
                            if ((bits >> col) & 1) == 0 {
                                continue;
                            }

                            let pixel_x = pen_x + (col as i32) * scale;
                            let pixel_y = base_y + (row as i32) * scale;

                            for sy in 0..scale {
                                for sx in 0..scale {
                                    let x = pixel_x + sx;
                                    let y = pixel_y + sy;
                                    if x < 0 || y < 0 {
                                        continue;
                                    }
                                    if x >= width as i32 || y >= height as i32 {
                                        continue;
                                    }

                                    let idx = ((y as usize) * (width as usize) + (x as usize)) * 4;
                                    rgba[idx] = r;
                                    rgba[idx + 1] = g;
                                    rgba[idx + 2] = b;
                                    rgba[idx + 3] = a;
                                }
                            }
                        }
                    }
                }

                pen_x += glyph_width + spacing_x;
            }
        }

        Some(RasterizedText {
            rgba,
            width,
            height,
        })
    }

    fn rasterize_text_fontdue(
        font: &Font,
        text: &str,
        font_size: f32,
        color: Color,
        letter_spacing: f32,
        line_spacing: f32,
    ) -> Option<RasterizedText> {
        if text.is_empty() {
            return None;
        }

        let font_size = font_size.max(1.0);
        let letter_spacing = letter_spacing.max(-(font_size * 0.95));
        let line_spacing = line_spacing.max(-(font_size * 0.95));
        let line_metrics = font.horizontal_line_metrics(font_size);
        let ascent = line_metrics
            .map(|metrics| metrics.ascent)
            .unwrap_or(font_size * 0.8);
        let base_line_height = line_metrics
            .map(|metrics| metrics.new_line_size)
            .unwrap_or(font_size * 1.2)
            .max(1.0);
        let line_stride = (base_line_height + line_spacing).max(1.0);
        let lines: Vec<&str> = text.split('\n').collect();
        let (space_metrics, _) = font.rasterize(' ', font_size);
        let tab_advance = (space_metrics.advance_width.max(font_size * 0.25)) * 4.0;

        struct GlyphBitmap {
            x: i32,
            y: i32,
            width: usize,
            height: usize,
            bitmap: Vec<u8>,
        }

        let mut glyphs: Vec<GlyphBitmap> = Vec::new();
        let mut min_x = 0i32;
        let mut min_y = 0i32;
        let mut max_x = 0i32;
        let mut max_y = 0i32;
        let mut has_visible_glyph = false;
        let mut measured_width = 0.0f32;

        for (line_index, line) in lines.iter().enumerate() {
            let baseline_y = ascent + line_index as f32 * line_stride;
            let chars: Vec<char> = line.chars().collect();
            let mut pen_x = 0.0f32;

            for (char_index, ch) in chars.iter().enumerate() {
                if *ch == '\t' {
                    pen_x += tab_advance;
                } else {
                    let (metrics, bitmap) = font.rasterize(*ch, font_size);

                    if metrics.width > 0 && metrics.height > 0 {
                        let glyph_x = (pen_x + metrics.xmin as f32).floor() as i32;
                        let glyph_y = (baseline_y - metrics.ymin as f32 - metrics.height as f32)
                            .floor() as i32;
                        let glyph_right = glyph_x + metrics.width as i32;
                        let glyph_bottom = glyph_y + metrics.height as i32;

                        if !has_visible_glyph {
                            min_x = glyph_x;
                            min_y = glyph_y;
                            max_x = glyph_right;
                            max_y = glyph_bottom;
                            has_visible_glyph = true;
                        } else {
                            min_x = min_x.min(glyph_x);
                            min_y = min_y.min(glyph_y);
                            max_x = max_x.max(glyph_right);
                            max_y = max_y.max(glyph_bottom);
                        }

                        glyphs.push(GlyphBitmap {
                            x: glyph_x,
                            y: glyph_y,
                            width: metrics.width,
                            height: metrics.height,
                            bitmap,
                        });
                    }

                    pen_x += metrics.advance_width.max(font_size * 0.25);
                }

                if char_index + 1 < chars.len() {
                    pen_x += letter_spacing;
                }
            }

            measured_width = measured_width.max(pen_x.max(0.0));
        }

        let (width, height) = if has_visible_glyph {
            ((max_x - min_x).max(1) as u32, (max_y - min_y).max(1) as u32)
        } else {
            let text_height = if lines.is_empty() {
                1.0
            } else {
                (lines.len().saturating_sub(1) as f32 * line_stride) + base_line_height
            };
            (
                measured_width.ceil().max(1.0) as u32,
                text_height.ceil().max(1.0) as u32,
            )
        };

        let mut rgba = vec![0u8; (width as usize) * (height as usize) * 4];

        let r = Self::color_component_to_u8(color.r());
        let g = Self::color_component_to_u8(color.g());
        let b = Self::color_component_to_u8(color.b());
        let alpha_scale = color.a().clamp(0.0, 1.0);

        for glyph in glyphs {
            let origin_x = glyph.x - min_x;
            let origin_y = glyph.y - min_y;

            for gy in 0..glyph.height {
                for gx in 0..glyph.width {
                    let x = origin_x + gx as i32;
                    let y = origin_y + gy as i32;
                    if x < 0 || y < 0 || x >= width as i32 || y >= height as i32 {
                        continue;
                    }

                    let coverage = glyph.bitmap[gy * glyph.width + gx] as f32 / 255.0;
                    let alpha = (coverage * alpha_scale * 255.0).round() as u8;
                    if alpha == 0 {
                        continue;
                    }

                    let idx = ((y as usize) * (width as usize) + (x as usize)) * 4;
                    rgba[idx] = r;
                    rgba[idx + 1] = g;
                    rgba[idx + 2] = b;
                    rgba[idx + 3] = alpha;
                }
            }
        }

        Some(RasterizedText {
            rgba,
            width,
            height,
        })
    }

    fn rasterize_text(
        &mut self,
        text: &str,
        font_size: f32,
        color: Color,
        font_path: Option<&str>,
        letter_spacing: f32,
        line_spacing: f32,
    ) -> Option<RasterizedText> {
        let font_size = font_size.max(1.0);
        if let Some(path) = font_path
            && let Some(font) = self.load_font_from_path(path)
        {
            if let Some(rasterized) = Self::rasterize_text_fontdue(
                font,
                text,
                font_size,
                color,
                letter_spacing,
                line_spacing,
            ) {
                return Some(rasterized);
            }
        }

        Self::rasterize_text_font8x8(text, font_size, color, letter_spacing, line_spacing)
    }

    #[allow(clippy::too_many_arguments)]
    fn build_text_draw_item(
        &mut self,
        text: &str,
        x: f32,
        y: f32,
        font_size: f32,
        color: Color,
        font_path: Option<&str>,
        letter_spacing: f32,
        line_spacing: f32,
        draw_order: f32,
    ) -> Option<(DrawItem, Option<PendingTextureUpload>)> {
        if text.is_empty() {
            return None;
        }

        let font_size = font_size.max(1.0);
        let texture_key = Self::build_text_texture_key(
            text,
            font_path,
            font_size,
            color,
            letter_spacing,
            line_spacing,
        );

        // Fast path: skip CPU rasterization when this text texture is already cached.
        if let Some(Some(cached_texture)) = self.texture_cache.get(&texture_key) {
            let item = self.build_image_rect_draw_item(
                x,
                y,
                cached_texture.width as f32,
                cached_texture.height as f32,
                texture_key,
                draw_order,
            );
            return Some((item, None));
        }

        let rasterized = self.rasterize_text(
            text,
            font_size,
            color,
            font_path,
            letter_spacing,
            line_spacing,
        )?;
        let item = self.build_image_rect_draw_item(
            x,
            y,
            rasterized.width as f32,
            rasterized.height as f32,
            texture_key.clone(),
            draw_order,
        );
        let upload = PendingTextureUpload {
            key: texture_key,
            rgba: Arc::from(rasterized.rgba),
            width: rasterized.width,
            height: rasterized.height,
        };

        Some((item, Some(upload)))
    }

    fn build_line_draw_item(
        &self,
        start_x: f32,
        start_y: f32,
        end_x: f32,
        end_y: f32,
        thickness: f32,
        color: Color,
        draw_order: f32,
    ) -> DrawItem {
        let thickness = thickness.max(1.0);
        let dx = end_x - start_x;
        let dy = end_y - start_y;
        let length = (dx * dx + dy * dy).sqrt();

        if length <= f32::EPSILON {
            return self.build_filled_rect_draw_item(
                start_x, start_y, thickness, thickness, color, draw_order,
            );
        }

        let nx = -dy / length * (thickness * 0.5);
        let ny = dx / length * (thickness * 0.5);

        let a = self.pixel_to_clip(start_x + nx, start_y + ny);
        let b = self.pixel_to_clip(start_x - nx, start_y - ny);
        let c = self.pixel_to_clip(end_x - nx, end_y - ny);
        let d = self.pixel_to_clip(end_x + nx, end_y + ny);

        self.build_quad_draw_item(a, b, c, d, Self::color_to_array(color), draw_order)
    }

    fn build_filled_circle_draw_item(
        &self,
        center_x: f32,
        center_y: f32,
        radius: f32,
        segments: u32,
        color: Color,
        draw_order: f32,
    ) -> Option<DrawItem> {
        if radius <= 0.0 {
            return None;
        }

        let segments = segments.max(8);
        let mut vertices = Vec::with_capacity((segments + 2) as usize);
        let mut indices = Vec::with_capacity((segments * 3) as usize);
        let color = Self::color_to_array(color);

        let center = self.pixel_to_clip(center_x, center_y);
        vertices.push(Vertex {
            position: [center[0], center[1], 0.0],
            color,
            tex_coords: [0.5, 0.5],
        });

        for i in 0..=segments {
            let angle = (i as f32 / segments as f32) * TAU;
            let px = center_x + radius * angle.cos();
            let py = center_y + radius * angle.sin();
            let clip = self.pixel_to_clip(px, py);
            vertices.push(Vertex {
                position: [clip[0], clip[1], 0.0],
                color,
                tex_coords: [(angle.cos() + 1.0) * 0.5, (angle.sin() + 1.0) * 0.5],
            });
        }

        for i in 1..=segments {
            indices.extend_from_slice(&[0, i, i + 1]);
        }

        Some(DrawItem {
            draw_order,
            texture_path: None,
            vertices,
            indices,
        })
    }

    fn build_circle_outline_draw_item(
        &self,
        center_x: f32,
        center_y: f32,
        radius: f32,
        thickness: f32,
        segments: u32,
        color: Color,
        draw_order: f32,
    ) -> Option<DrawItem> {
        if radius <= 0.0 {
            return None;
        }

        let segments = segments.max(8);
        let thickness = thickness.max(1.0);
        let inner = (radius - thickness * 0.5).max(0.0);
        let outer = radius + thickness * 0.5;

        let mut vertices = Vec::with_capacity(((segments + 1) * 2) as usize);
        let mut indices = Vec::with_capacity((segments * 6) as usize);
        let color = Self::color_to_array(color);

        for i in 0..=segments {
            let angle = (i as f32 / segments as f32) * TAU;
            let cos_a = angle.cos();
            let sin_a = angle.sin();

            let outer_clip = self.pixel_to_clip(center_x + outer * cos_a, center_y + outer * sin_a);
            let inner_clip = self.pixel_to_clip(center_x + inner * cos_a, center_y + inner * sin_a);

            vertices.push(Vertex {
                position: [outer_clip[0], outer_clip[1], 0.0],
                color,
                tex_coords: [1.0, 0.0],
            });
            vertices.push(Vertex {
                position: [inner_clip[0], inner_clip[1], 0.0],
                color,
                tex_coords: [0.0, 1.0],
            });
        }

        for i in 0..segments {
            let base = i * 2;
            indices.extend_from_slice(&[base, base + 1, base + 2, base + 1, base + 3, base + 2]);
        }

        Some(DrawItem {
            draw_order,
            texture_path: None,
            vertices,
            indices,
        })
    }

    fn collect_direct_draw_items(
        &mut self,
        draw_manager: Option<&DrawManager>,
    ) -> (Vec<DrawItem>, Vec<PendingTextureUpload>) {
        let mut items = Vec::new();
        let mut texture_uploads = Vec::new();
        let Some(draw_manager) = draw_manager else {
            return (items, texture_uploads);
        };

        for command in draw_manager.commands() {
            match command {
                DrawCommand::Pixel {
                    x,
                    y,
                    color,
                    draw_order,
                } => {
                    items.push(self.build_filled_rect_draw_item(
                        *x,
                        *y,
                        1.0,
                        1.0,
                        *color,
                        *draw_order,
                    ));
                }
                DrawCommand::Line {
                    start_x,
                    start_y,
                    end_x,
                    end_y,
                    thickness,
                    color,
                    draw_order,
                } => {
                    items.push(self.build_line_draw_item(
                        *start_x,
                        *start_y,
                        *end_x,
                        *end_y,
                        *thickness,
                        *color,
                        *draw_order,
                    ));
                }
                DrawCommand::Rectangle {
                    x,
                    y,
                    width,
                    height,
                    color,
                    filled,
                    thickness,
                    draw_order,
                } => {
                    if *filled {
                        items.push(self.build_filled_rect_draw_item(
                            *x,
                            *y,
                            *width,
                            *height,
                            *color,
                            *draw_order,
                        ));
                    } else {
                        items.push(self.build_line_draw_item(
                            *x,
                            *y,
                            *x + *width,
                            *y,
                            *thickness,
                            *color,
                            *draw_order,
                        ));
                        items.push(self.build_line_draw_item(
                            *x + *width,
                            *y,
                            *x + *width,
                            *y + *height,
                            *thickness,
                            *color,
                            *draw_order,
                        ));
                        items.push(self.build_line_draw_item(
                            *x + *width,
                            *y + *height,
                            *x,
                            *y + *height,
                            *thickness,
                            *color,
                            *draw_order,
                        ));
                        items.push(self.build_line_draw_item(
                            *x,
                            *y + *height,
                            *x,
                            *y,
                            *thickness,
                            *color,
                            *draw_order,
                        ));
                    }
                }
                DrawCommand::Circle {
                    center_x,
                    center_y,
                    radius,
                    color,
                    filled,
                    thickness,
                    segments,
                    draw_order,
                } => {
                    let item = if *filled {
                        self.build_filled_circle_draw_item(
                            *center_x,
                            *center_y,
                            *radius,
                            *segments,
                            *color,
                            *draw_order,
                        )
                    } else {
                        self.build_circle_outline_draw_item(
                            *center_x,
                            *center_y,
                            *radius,
                            *thickness,
                            *segments,
                            *color,
                            *draw_order,
                        )
                    };

                    if let Some(item) = item {
                        items.push(item);
                    }
                }
                DrawCommand::GradientRect {
                    x,
                    y,
                    width,
                    height,
                    top_left,
                    bottom_left,
                    bottom_right,
                    top_right,
                    draw_order,
                } => {
                    items.push(self.build_gradient_rect_draw_item(
                        *x,
                        *y,
                        *width,
                        *height,
                        *top_left,
                        *bottom_left,
                        *bottom_right,
                        *top_right,
                        *draw_order,
                    ));
                }
                DrawCommand::Image {
                    x,
                    y,
                    width,
                    height,
                    texture_path,
                    draw_order,
                } => {
                    items.push(self.build_image_rect_draw_item(
                        *x,
                        *y,
                        *width,
                        *height,
                        texture_path.clone(),
                        *draw_order,
                    ));
                }
                DrawCommand::ImageBytes {
                    x,
                    y,
                    width,
                    height,
                    texture_key,
                    rgba,
                    texture_width,
                    texture_height,
                    draw_order,
                } => {
                    items.push(self.build_image_rect_draw_item(
                        *x,
                        *y,
                        *width,
                        *height,
                        texture_key.clone(),
                        *draw_order,
                    ));
                    texture_uploads.push(PendingTextureUpload {
                        key: texture_key.clone(),
                        rgba: Arc::clone(rgba),
                        width: *texture_width,
                        height: *texture_height,
                    });
                }
                DrawCommand::Text {
                    text,
                    x,
                    y,
                    font_size,
                    color,
                    font_path,
                    letter_spacing,
                    line_spacing,
                    draw_order,
                } => {
                    if let Some((item, upload)) = self.build_text_draw_item(
                        text,
                        *x,
                        *y,
                        *font_size,
                        *color,
                        font_path.as_deref(),
                        *letter_spacing,
                        *line_spacing,
                        *draw_order,
                    ) {
                        items.push(item);
                        if let Some(upload) = upload {
                            texture_uploads.push(upload);
                        }
                    }
                }
            }
        }

        (items, texture_uploads)
    }

    fn collect_mesh_draw_items(
        &self,
        objects: &Option<ObjectManager>,
        camera_position: Vec2,
    ) -> Vec<DrawItem> {
        let mut items = Vec::new();

        let Some(object_manager) = objects else {
            return items;
        };

        let keys = object_manager.get_sorted_keys();

        for &id in keys {
            if self.active_camera_object_id == Some(id) {
                continue;
            }

            let Some(object) = object_manager.get_object_by_id(id) else {
                continue;
            };

            if !object.is_active() {
                continue;
            }

            let Some(mesh) = object.mesh_component() else {
                continue;
            };

            if !mesh.visible() || !mesh.geometry().is_valid() {
                continue;
            }

            let transform = object.transform();
            let fill_color = mesh.fill_color().copied().unwrap_or(Color::WHITE);
            let color = [
                fill_color.r(),
                fill_color.g(),
                fill_color.b(),
                fill_color.a(),
            ];

            let cos_t = transform.rotation().cos();
            let sin_t = transform.rotation().sin();
            let scale_x = transform.scale().x();
            let scale_y = transform.scale().y();
            let pos_x = transform.position().x();
            let pos_y = transform.position().y();

            let mut vertices = Vec::with_capacity(mesh.geometry().vertices().len());
            for vertex in mesh.geometry().vertices() {
                let local_x = vertex.position().x() * scale_x;
                let local_y = vertex.position().y() * scale_y;

                let rotated_x = local_x * cos_t - local_y * sin_t;
                let rotated_y = local_x * sin_t + local_y * cos_t;
                let world_x = pos_x + rotated_x;
                let world_y = pos_y + rotated_y;
                let clip = self.world_to_clip(world_x, world_y, camera_position);

                vertices.push(Vertex {
                    position: [clip[0], clip[1], 0.0],
                    color,
                    tex_coords: [vertex.uv().x(), vertex.uv().y()],
                });
            }

            items.push(DrawItem {
                draw_order: mesh.draw_order(),
                texture_path: mesh.image_path().map(|p| p.to_string()),
                vertices,
                indices: mesh.geometry().indices().to_vec(),
            });
        }

        items
    }

    fn collect_draw_items(
        &mut self,
        objects: &Option<ObjectManager>,
        draw_manager: Option<&DrawManager>,
    ) -> (Vec<DrawItem>, Vec<PendingTextureUpload>) {
        let camera_position = self.active_camera_position(objects);
        let mut items = self.collect_mesh_draw_items(objects, camera_position);
        let (direct_draw_items, texture_uploads) = self.collect_direct_draw_items(draw_manager);
        items.extend(direct_draw_items);

        items.sort_by(|a, b| {
            a.draw_order
                .partial_cmp(&b.draw_order)
                .unwrap_or(Ordering::Equal)
        });

        (items, texture_uploads)
    }

    fn compute_scene_version(
        &self,
        objects: &Option<ObjectManager>,
        draw_manager: Option<&DrawManager>,
    ) -> SceneVersion {
        SceneVersion {
            render_state_epoch: self.render_state_epoch,
            object_epoch: objects.as_ref().map_or(0, ObjectManager::scene_version),
            draw_epoch: draw_manager.map_or(0, DrawManager::scene_version),
        }
    }

    /// Returns whether the window should request another redraw.
    pub fn should_request_redraw(
        &mut self,
        objects: &Option<ObjectManager>,
        draw_manager: Option<&DrawManager>,
    ) -> bool {
        if !self.redraw_on_change_only {
            self.precomputed_scene_version = None;
            return true;
        }

        if self.requires_redraw || self.pending_resize.is_some() {
            self.precomputed_scene_version = None;
            return true;
        }

        let scene_version = self.compute_scene_version(objects, draw_manager);
        self.precomputed_scene_version = Some(scene_version);
        self.last_scene_version != Some(scene_version)
    }

    /// Mark the renderer as needing a redraw.
    pub fn request_redraw(&mut self) {
        self.requires_redraw = true;
        self.precomputed_scene_version = None;
    }

    fn bump_render_state_epoch(&mut self) {
        self.render_state_epoch = self.render_state_epoch.wrapping_add(1);
    }

    /// Invalidate any scene version precomputed before a simulation update.
    pub fn invalidate_precomputed_scene_signature(&mut self) {
        self.precomputed_scene_version = None;
    }

    /// Render a frame.
    ///
    /// This function acquires a surface texture, renders to it, and presents it.
    /// Returns an error if the surface needs to be reconfigured or if rendering fails.
    pub fn render(
        &mut self,
        objects: &Option<ObjectManager>,
        draw_manager: Option<&DrawManager>,
    ) -> Result<(), wgpu::SurfaceError> {
        let scene_version = self
            .precomputed_scene_version
            .take()
            .unwrap_or_else(|| self.compute_scene_version(objects, draw_manager));
        let scene_changed = self.last_scene_version != Some(scene_version);

        if self.redraw_on_change_only
            && !self.requires_redraw
            && self.pending_resize.is_none()
            && !scene_changed
        {
            return Ok(());
        }

        // Handle pending resize before rendering to avoid reconfiguring
        // multiple times during rapid resize events.
        if let Some(new_size) = self.pending_resize.take() {
            if new_size.width > 0 && new_size.height > 0 {
                self.surface_config.width = new_size.width;
                self.surface_config.height = new_size.height;
                self.surface.configure(&self.device, &self.surface_config);
            }
        }

        let (draw_items, pending_texture_uploads) = self.collect_draw_items(objects, draw_manager);
        for upload in pending_texture_uploads {
            if let Err(err) = self.cache_texture_from_rgba(
                &upload.key,
                upload.rgba.as_ref(),
                upload.width,
                upload.height,
            ) {
                logging::log_warn(&err);
            }
        }
        let mut prepared_draws = Vec::new();
        let mut batch_slot = 0usize;

        // Batching State
        let mut batch_vertices: Vec<Vertex> = Vec::new();
        let mut batch_indices: Vec<u32> = Vec::new();
        let mut batch_texture_path: Option<String> = None;

        // Iteration and Merging
        for item in draw_items {
            // Determine if we need to switch batches
            let texture_changed = item.texture_path != batch_texture_path;
            let is_first_item = batch_vertices.is_empty() && batch_indices.is_empty();

            if texture_changed && !is_first_item {
                // 1. Resolve the bind group for the COMPLETED batch
                let bind_group = self.texture_bind_group_for(batch_texture_path.as_deref());

                // 2. Flush
                if !batch_vertices.is_empty() {
                    let vertex_buffer = Self::write_to_pooled_buffer(
                        &self.device,
                        &self.queue,
                        &mut self.vertex_buffer_pool,
                        batch_slot,
                        bytemuck::cast_slice(&batch_vertices),
                        wgpu::BufferUsages::VERTEX,
                        "batch_vertex_buffer_pool",
                    );
                    let index_buffer = Self::write_to_pooled_buffer(
                        &self.device,
                        &self.queue,
                        &mut self.index_buffer_pool,
                        batch_slot,
                        bytemuck::cast_slice(&batch_indices),
                        wgpu::BufferUsages::INDEX,
                        "batch_index_buffer_pool",
                    );

                    prepared_draws.push(PreparedDraw {
                        bind_group,
                        vertex_buffer,
                        index_buffer,
                        index_count: batch_indices.len() as u32,
                    });
                    batch_slot += 1;

                    batch_vertices.clear();
                    batch_indices.clear();
                }
            }

            // Update current batch tracker
            if batch_vertices.is_empty() {
                batch_texture_path = item.texture_path.clone();
            }

            // MERGE: Append geometry
            let index_offset = batch_vertices.len() as u32;
            batch_vertices.extend(item.vertices);
            // Crucial: Offset indices so they point to the correct vertices in the combined buffer
            batch_indices.extend(item.indices.iter().map(|i| i + index_offset));
        }

        // Final Flush
        if !batch_vertices.is_empty() {
            let bind_group = self.texture_bind_group_for(batch_texture_path.as_deref());

            let vertex_buffer = Self::write_to_pooled_buffer(
                &self.device,
                &self.queue,
                &mut self.vertex_buffer_pool,
                batch_slot,
                bytemuck::cast_slice(&batch_vertices),
                wgpu::BufferUsages::VERTEX,
                "batch_vertex_buffer_pool",
            );

            let index_buffer = Self::write_to_pooled_buffer(
                &self.device,
                &self.queue,
                &mut self.index_buffer_pool,
                batch_slot,
                bytemuck::cast_slice(&batch_indices),
                wgpu::BufferUsages::INDEX,
                "batch_index_buffer_pool",
            );

            prepared_draws.push(PreparedDraw {
                bind_group,
                vertex_buffer,
                index_buffer,
                index_count: batch_indices.len() as u32,
            });
        }

        // Acquire the next frame.
        let output = self.surface.get_current_texture()?;

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // Create a command encoder.
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        // Create a render pass.
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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

            render_pass.set_pipeline(&self.render_pipeline);
            for draw in &prepared_draws {
                render_pass.set_bind_group(0, &draw.bind_group, &[]);
                render_pass.set_vertex_buffer(0, draw.vertex_buffer.slice(..));
                render_pass
                    .set_index_buffer(draw.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                render_pass.draw_indexed(0..draw.index_count, 0, 0..1);
            }
        }

        // Submit the command buffer and present the frame.
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        self.requires_redraw = false;
        self.last_scene_version = Some(scene_version);

        Ok(())
    }

    /// Resize the surface to match the new window size.
    ///
    /// Should be called when the window is resized.
    /// The actual reconfiguration is deferred until the next render call
    /// to avoid expensive reconfigurations during rapid resize events.
    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.pending_resize = Some(new_size);
            self.requires_redraw = true;
            self.precomputed_scene_version = None;
            self.bump_render_state_epoch();
        }
    }

    /// Configure VSync (vertical synchronization).
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
        self.requires_redraw = true;
        self.precomputed_scene_version = None;
        self.bump_render_state_epoch();
    }

    /// Configure redraw-on-change behavior.
    ///
    /// When enabled, frames are only rendered when scene state changes.
    pub fn configure_redraw_on_change_only(&mut self, enabled: bool) {
        self.redraw_on_change_only = enabled;
        self.requires_redraw = true;
        self.precomputed_scene_version = None;
        self.bump_render_state_epoch();
    }

    /// Get whether VSync is currently enabled.
    pub fn is_vsync_enabled(&self) -> bool {
        self.vsync_enabled
    }

    /// Get whether redraw-on-change mode is currently enabled.
    pub fn is_redraw_on_change_only(&self) -> bool {
        self.redraw_on_change_only
    }

    /// Set the background clear color.
    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
        self.requires_redraw = true;
        self.precomputed_scene_version = None;
        self.bump_render_state_epoch();
    }

    /// Get the background clear color.
    pub fn background_color(&self) -> Color {
        self.background_color
    }

    /// Get the current surface configuration.
    pub fn surface_config(&self) -> &SurfaceConfiguration {
        &self.surface_config
    }

    /// Get a reference to the GPU device.
    pub fn device(&self) -> &Device {
        &self.device
    }

    /// Get a reference to the command queue.
    pub fn queue(&self) -> &Queue {
        &self.queue
    }
}
