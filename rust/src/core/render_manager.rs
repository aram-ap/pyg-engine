use std::borrow::Cow;
use std::cmp::Ordering;
use std::collections::{HashMap, hash_map::DefaultHasher};
use std::f32::consts::TAU;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use font8x8::{BASIC_FONTS, UnicodeFonts};
use fontdue::Font;
use image::GenericImageView;
use wgpu::{Device, PresentMode, Queue, Surface, SurfaceConfiguration, TextureUsages};
use winit::dpi::PhysicalSize;
use winit::window::Window;

use super::geometry::Vertex;
use super::logging;
use super::text::{
    FontDescriptor, FontFamilyDefinition, TextAlign, TextLayoutOptions, TextStyle,
    VerticalTextAlign, normalize_font_family_key, normalize_font_path,
};
use crate::core::component::ComponentTrait;
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

struct CachedTextureEntry {
    cached_texture: CachedTexture,
    last_used_frame: u64,
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct GlyphCacheKey {
    font_cache_key: String,
    glyph: char,
    font_size_bits: u32,
}

#[derive(Clone)]
struct CachedGlyph {
    metrics: fontdue::Metrics,
    bitmap: Arc<[u8]>,
    glyph_index: u16,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct TextLayoutCacheKey {
    font_cache_key: String,
    text: String,
    font_size_bits: u32,
    letter_spacing_bits: u32,
    line_spacing_bits: u32,
    kerning: bool,
}

#[derive(Clone)]
struct PositionedGlyph {
    x: i32,
    y: i32,
    glyph_key: GlyphCacheKey,
}

#[derive(Clone)]
struct CachedTextLayout {
    width: u32,
    height: u32,
    glyphs: Vec<PositionedGlyph>,
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
    surface_present_modes: Vec<PresentMode>,
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
    texture_cache: HashMap<String, Option<CachedTextureEntry>>,
    texture_data_signature_cache: HashMap<String, u64>,
    font_registry: HashMap<String, FontFamilyDefinition>,
    font_cache: HashMap<String, Option<Font>>,
    glyph_cache: HashMap<GlyphCacheKey, Option<CachedGlyph>>,
    layout_cache: HashMap<TextLayoutCacheKey, CachedTextLayout>,
    vertex_buffer_pool: Vec<PooledBuffer>,
    index_buffer_pool: Vec<PooledBuffer>,
    active_camera_object_id: Option<u32>,
    camera_viewport_size: Option<Vec2>,
    camera_aspect_mode: CameraAspectMode,
    source_root: Option<PathBuf>,
    current_frame: u64,
    texture_ttl_frames: u64,
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
        // Choose present mode based on vsync setting and platform capabilities.
        // Fifo (vsync) is always supported. For non-vsync, prefer Mailbox > Immediate.
        let present_mode = if vsync {
            PresentMode::Fifo // VSync on (always supported)
        } else {
            // Try to use Mailbox (low-latency without tearing) if available,
            // otherwise fall back to Immediate (may tear but lowest latency).
            // Mailbox is better supported on macOS/Metal than Immediate.
            if surface_caps.present_modes.contains(&PresentMode::Mailbox) {
                PresentMode::Mailbox
            } else if surface_caps.present_modes.contains(&PresentMode::Immediate) {
                PresentMode::Immediate
            } else {
                // If neither is available, fall back to Fifo
                logging::log_warn("VSync off requested but Mailbox/Immediate modes not available; using Fifo");
                PresentMode::Fifo
            }
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
            surface_present_modes: surface_caps.present_modes,
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
            font_registry: HashMap::new(),
            font_cache: HashMap::new(),
            glyph_cache: HashMap::new(),
            layout_cache: HashMap::new(),
            vertex_buffer_pool: Vec::new(),
            index_buffer_pool: Vec::new(),
            active_camera_object_id: None,
            camera_viewport_size: None,
            camera_aspect_mode: CameraAspectMode::default(),
            source_root: None,
            current_frame: 0,
            texture_ttl_frames: 180, // Clean up textures unused for 180 frames (~3 seconds at 60fps)
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
        let resolved_path = self.resolve_source_path(texture_path);
        let bytes = std::fs::read(&resolved_path)
            .map_err(|e| format!("failed to read texture '{}': {e}", resolved_path))?;

        let img = image::load_from_memory(&bytes)
            .map_err(|e| format!("failed to decode texture '{}': {e}", resolved_path))?;
        let rgba = img.to_rgba8();
        let (width, height) = img.dimensions();

        Ok(Self::create_cached_texture(
            &self.device,
            &self.queue,
            &self.texture_bind_group_layout,
            rgba.as_raw(),
            width,
            height,
            &resolved_path,
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
            // Update last used frame even if data hasn't changed
            if let Some(Some(entry)) = self.texture_cache.get_mut(texture_key) {
                entry.last_used_frame = self.current_frame;
            }
            return Ok(());
        }

        if let Some(Some(entry)) = self.texture_cache.get_mut(texture_key) {
            if entry.cached_texture.width == width && entry.cached_texture.height == height {
                Self::write_rgba_to_texture(
                    &self.queue,
                    &entry.cached_texture.texture,
                    rgba,
                    width,
                    height,
                );
                entry.last_used_frame = self.current_frame;
                self.texture_data_signature_cache
                    .insert(texture_key.to_string(), signature);
                return Ok(());
            }
        }

        let cached_texture = Self::create_cached_texture(
            &self.device,
            &self.queue,
            &self.texture_bind_group_layout,
            rgba,
            width,
            height,
            texture_key,
        );
        let entry = CachedTextureEntry {
            cached_texture,
            last_used_frame: self.current_frame,
        };
        self.texture_cache
            .insert(texture_key.to_string(), Some(entry));
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
            if let Some(Some(entry)) = self.texture_cache.get_mut(path) {
                entry.last_used_frame = self.current_frame;
                return entry.cached_texture.bind_group.clone();
            }

            let resolved_path = self.resolve_source_path(path);
            if !self.texture_cache.contains_key(&resolved_path) {
                let loaded = match self.load_texture_from_path(path) {
                    Ok(cached_texture) => Some(CachedTextureEntry {
                        cached_texture,
                        last_used_frame: self.current_frame,
                    }),
                    Err(err) => {
                        logging::log_warn(&format!("Texture load failed: {err}"));
                        None
                    }
                };
                self.texture_cache.insert(resolved_path.clone(), loaded);
            }

            if let Some(Some(entry)) = self.texture_cache.get_mut(&resolved_path) {
                entry.last_used_frame = self.current_frame;
                return entry.cached_texture.bind_group.clone();
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

    fn active_camera_position(&self, objects: &ObjectManager) -> Vec2 {
        let Some(camera_id) = self.active_camera_object_id else {
            return Vec2::new(0.0, 0.0);
        };

        objects
            .world_position(camera_id)
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

    fn build_text_texture_key(text: &str, style: &TextStyle, color: Color) -> String {
        let mut hasher = DefaultHasher::new();
        text.hash(&mut hasher);
        style.font.cache_key().hash(&mut hasher);
        hash_f32(&mut hasher, style.font_size);
        hash_color(&mut hasher, &color);
        hash_f32(&mut hasher, style.letter_spacing);
        hash_f32(&mut hasher, style.line_spacing);
        style.kerning.hash(&mut hasher);
        format!("__pyg_text_{:016x}", hasher.finish())
    }

    fn build_text_layout_cache_key(
        text: &str,
        style: &TextStyle,
        font_cache_key: &str,
    ) -> TextLayoutCacheKey {
        TextLayoutCacheKey {
            font_cache_key: font_cache_key.to_string(),
            text: text.to_string(),
            font_size_bits: style.font_size.to_bits(),
            letter_spacing_bits: style.letter_spacing.to_bits(),
            line_spacing_bits: style.line_spacing.to_bits(),
            kerning: style.kerning,
        }
    }

    fn clear_resolved_asset_caches(&mut self) {
        self.font_cache.clear();
        self.glyph_cache.clear();
        self.layout_cache.clear();
        self.texture_cache.clear();
        self.texture_data_signature_cache.clear();
    }

    pub fn set_source_root(&mut self, source_root: Option<PathBuf>) {
        let normalized = source_root.map(|path| {
            if path.is_absolute() {
                path
            } else {
                std::env::current_dir()
                    .unwrap_or_else(|_| PathBuf::from("."))
                    .join(path)
            }
        });
        if self.source_root != normalized {
            self.source_root = normalized;
            self.clear_resolved_asset_caches();
            self.request_redraw();
        }
    }

    pub fn register_font_family(
        &mut self,
        family: impl Into<String>,
        definition: FontFamilyDefinition,
    ) -> bool {
        if definition.is_empty() {
            return false;
        }

        let key = normalize_font_family_key(&family.into());
        self.font_registry.insert(key, definition);
        self.layout_cache.clear();
        self.request_redraw();
        true
    }

    pub fn font_family_definition(&self, family: &str) -> Option<&FontFamilyDefinition> {
        self.font_registry.get(&normalize_font_family_key(family))
    }

    fn is_supported_font_path(font_path: &str) -> bool {
        let extension = std::path::Path::new(font_path)
            .extension()
            .and_then(|extension| extension.to_str())
            .map(|extension| extension.to_ascii_lowercase());
        matches!(extension.as_deref(), Some("ttf") | Some("otf"))
    }

    fn resolve_source_path(&self, path: &str) -> String {
        let input = Path::new(path);
        if input.is_absolute() {
            return normalize_font_path(path);
        }
        if let Some(source_root) = &self.source_root {
            return normalize_font_path(&source_root.join(input).to_string_lossy());
        }
        normalize_font_path(path)
    }

    fn load_font_from_path(&mut self, font_path: &str) -> Option<&Font> {
        let resolved_path = self.resolve_source_path(font_path);
        if !self.font_cache.contains_key(&resolved_path) {
            let loaded = if !Self::is_supported_font_path(font_path) {
                logging::log_warn(&format!(
                    "Unsupported font '{font_path}'. Expected a .ttf or .otf file. Falling back to built-in font."
                ));
                None
            } else {
                match std::fs::read(&resolved_path) {
                    Ok(bytes) => match Font::from_bytes(bytes, fontdue::FontSettings::default()) {
                        Ok(font) => Some(font),
                        Err(err) => {
                            logging::log_warn(&format!(
                                "Failed to decode font '{resolved_path}': {err}. Falling back to built-in font."
                            ));
                            None
                        }
                    },
                    Err(err) => {
                        logging::log_warn(&format!(
                            "Failed to read font '{resolved_path}': {err}. Falling back to built-in font."
                        ));
                        None
                    }
                }
            };
            self.font_cache.insert(resolved_path.clone(), loaded);
        }

        self.font_cache
            .get(&resolved_path)
            .and_then(|font| font.as_ref())
    }

    fn resolve_font_path(&self, descriptor: &FontDescriptor) -> Option<String> {
        if let Some(path) = descriptor.path() {
            return Some(self.resolve_source_path(path));
        }

        let family_key = descriptor.family_key()?;
        let family = self.font_registry.get(&family_key)?;
        family
            .resolve(descriptor.weight(), descriptor.style())
            .map(|path| self.resolve_source_path(path))
    }

    fn resolved_font_cache_key(&self, descriptor: &FontDescriptor, resolved_path: &str) -> String {
        if descriptor.path().is_some() {
            format!("path:{}", normalize_font_path(resolved_path))
        } else if let Some(family) = descriptor.family_key() {
            format!(
                "family:{}:{}:{}",
                family,
                descriptor.weight().as_str(),
                descriptor.style().as_str()
            )
        } else {
            DEFAULT_FONT_NAME.to_string()
        }
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

    fn load_cached_glyph(
        &mut self,
        font_path: &str,
        font_cache_key: &str,
        glyph: char,
        font_size: f32,
    ) -> Option<CachedGlyph> {
        let key = GlyphCacheKey {
            font_cache_key: font_cache_key.to_string(),
            glyph,
            font_size_bits: font_size.to_bits(),
        };
        if !self.glyph_cache.contains_key(&key) {
            let loaded = self.load_font_from_path(font_path).map(|font| CachedGlyph {
                metrics: font.metrics(glyph, font_size),
                bitmap: Arc::from(font.rasterize(glyph, font_size).1),
                glyph_index: font.lookup_glyph_index(glyph),
            });
            self.glyph_cache.insert(key.clone(), loaded);
        }
        self.glyph_cache.get(&key).and_then(|glyph| glyph.clone())
    }

    fn build_fontdue_text_layout(
        &mut self,
        font_path: &str,
        font_cache_key: &str,
        text: &str,
        style: &TextStyle,
    ) -> Option<CachedTextLayout> {
        if text.is_empty() {
            return None;
        }

        let font_size = style.font_size.max(1.0);
        let letter_spacing = style.letter_spacing.max(-(font_size * 0.95));
        let line_spacing = style.line_spacing.max(-(font_size * 0.95));
        let (ascent, base_line_height, tab_advance) = {
            let font = self.load_font_from_path(font_path)?;
            let line_metrics = font.horizontal_line_metrics(font_size);
            let ascent = line_metrics
                .map(|metrics| metrics.ascent)
                .unwrap_or(font_size * 0.8);
            let base_line_height = line_metrics
                .map(|metrics| metrics.new_line_size)
                .unwrap_or(font_size * 1.2)
                .max(1.0);
            let (space_metrics, _) = font.rasterize(' ', font_size);
            let tab_advance = (space_metrics.advance_width.max(font_size * 0.25)) * 4.0;
            (ascent, base_line_height, tab_advance)
        };
        let line_stride = (base_line_height + line_spacing).max(1.0);
        let lines: Vec<&str> = text.split('\n').collect();

        let mut glyphs = Vec::new();
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
            let mut previous_char: Option<char> = None;

            for (char_index, ch) in chars.iter().enumerate() {
                if *ch == '\t' {
                    pen_x += tab_advance;
                    previous_char = None;
                } else {
                    let glyph =
                        self.load_cached_glyph(font_path, font_cache_key, *ch, font_size)?;
                    if style.kerning
                        && let Some(previous) = previous_char
                        && let Some(font) = self.load_font_from_path(font_path)
                    {
                        pen_x += font
                            .horizontal_kern(previous, *ch, font_size)
                            .unwrap_or(0.0);
                    }

                    if glyph.metrics.width > 0 && glyph.metrics.height > 0 {
                        let glyph_x = (pen_x + glyph.metrics.xmin as f32).floor() as i32;
                        let glyph_y = (baseline_y
                            - glyph.metrics.ymin as f32
                            - glyph.metrics.height as f32)
                            .floor() as i32;
                        let glyph_right = glyph_x + glyph.metrics.width as i32;
                        let glyph_bottom = glyph_y + glyph.metrics.height as i32;

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

                        glyphs.push(PositionedGlyph {
                            x: glyph_x,
                            y: glyph_y,
                            glyph_key: GlyphCacheKey {
                                font_cache_key: font_cache_key.to_string(),
                                glyph: *ch,
                                font_size_bits: font_size.to_bits(),
                            },
                        });
                    }

                    pen_x += glyph.metrics.advance_width.max(font_size * 0.25);
                    previous_char = Some(*ch);
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

        if has_visible_glyph && (min_x != 0 || min_y != 0) {
            for glyph in &mut glyphs {
                glyph.x -= min_x;
                glyph.y -= min_y;
            }
        }

        Some(CachedTextLayout {
            width,
            height,
            glyphs,
        })
    }

    fn cached_text_layout(
        &mut self,
        text: &str,
        style: &TextStyle,
        font_path: &str,
        font_cache_key: &str,
    ) -> Option<CachedTextLayout> {
        let key = Self::build_text_layout_cache_key(text, style, font_cache_key);
        if !self.layout_cache.contains_key(&key) {
            let layout = self.build_fontdue_text_layout(font_path, font_cache_key, text, style)?;
            self.layout_cache.insert(key.clone(), layout);
        }
        self.layout_cache.get(&key).cloned()
    }

    fn rasterize_text_fontdue(
        &mut self,
        font_path: &str,
        font_cache_key: &str,
        text: &str,
        style: &TextStyle,
        color: Color,
    ) -> Option<RasterizedText> {
        let layout = self.cached_text_layout(text, style, font_path, font_cache_key)?;
        let mut rgba = vec![0u8; (layout.width as usize) * (layout.height as usize) * 4];

        let r = Self::color_component_to_u8(color.r());
        let g = Self::color_component_to_u8(color.g());
        let b = Self::color_component_to_u8(color.b());
        let alpha_scale = color.a().clamp(0.0, 1.0);

        for positioned in &layout.glyphs {
            let Some(glyph) = self
                .glyph_cache
                .get(&positioned.glyph_key)
                .and_then(|entry| entry.clone())
            else {
                continue;
            };

            for gy in 0..glyph.metrics.height {
                for gx in 0..glyph.metrics.width {
                    let x = positioned.x + gx as i32;
                    let y = positioned.y + gy as i32;
                    if x < 0 || y < 0 || x >= layout.width as i32 || y >= layout.height as i32 {
                        continue;
                    }

                    let coverage = glyph.bitmap[gy * glyph.metrics.width + gx] as f32 / 255.0;
                    let alpha = (coverage * alpha_scale * 255.0).round() as u8;
                    if alpha == 0 {
                        continue;
                    }

                    let idx = ((y as usize) * (layout.width as usize) + (x as usize)) * 4;
                    rgba[idx] = r;
                    rgba[idx + 1] = g;
                    rgba[idx + 2] = b;
                    rgba[idx + 3] = alpha;
                }
            }
        }

        Some(RasterizedText {
            rgba,
            width: layout.width,
            height: layout.height,
        })
    }

    fn text_dimensions_from_style(
        &mut self,
        text: &str,
        style: &TextStyle,
    ) -> Option<(u32, u32)> {
        if let Some(font_path) = self.resolve_font_path(&style.font) {
            let font_cache_key = self.resolved_font_cache_key(&style.font, &font_path);
            if let Some(layout) = self.cached_text_layout(text, style, &font_path, &font_cache_key) {
                return Some((layout.width, layout.height));
            }
        }

        Self::rasterize_text_font8x8(
            text,
            style.font_size.max(1.0),
            Color::WHITE,
            style.letter_spacing,
            style.line_spacing,
        )
        .map(|rasterized| (rasterized.width, rasterized.height))
    }

    pub fn measure_text(&mut self, text: &str, style: &TextStyle) -> (f32, f32) {
        self.text_dimensions_from_style(text, style)
            .map(|(width, height)| (width as f32, height as f32))
            .unwrap_or((0.0, 0.0))
    }

    fn rasterize_text(
        &mut self,
        text: &str,
        style: &TextStyle,
        color: Color,
    ) -> Option<RasterizedText> {
        if let Some(font_path) = self.resolve_font_path(&style.font) {
            let font_cache_key = self.resolved_font_cache_key(&style.font, &font_path);
            if self.load_font_from_path(&font_path).is_some()
                && let Some(rasterized) = self.rasterize_text_fontdue(
                    &font_path,
                    &font_cache_key,
                    text,
                    style,
                    color,
                )
            {
                return Some(rasterized);
            }
        }

        Self::rasterize_text_font8x8(
            text,
            style.font_size.max(1.0),
            color,
            style.letter_spacing,
            style.line_spacing,
        )
    }

    fn aligned_text_position(
        x: f32,
        y: f32,
        text_width: f32,
        text_height: f32,
        layout: &TextLayoutOptions,
    ) -> (f32, f32) {
        let offset_x = match layout.horizontal_align {
            TextAlign::Left => 0.0,
            TextAlign::Center => layout
                .width
                .map(|width| (width - text_width) * 0.5)
                .unwrap_or(0.0),
            TextAlign::Right => layout.width.map(|width| width - text_width).unwrap_or(0.0),
        };
        let offset_y = match layout.vertical_align {
            VerticalTextAlign::Top => 0.0,
            VerticalTextAlign::Center => layout
                .height
                .map(|height| (height - text_height) * 0.5)
                .unwrap_or(0.0),
            VerticalTextAlign::Bottom => layout
                .height
                .map(|height| height - text_height)
                .unwrap_or(0.0),
        };
        (x + offset_x, y + offset_y)
    }

    #[allow(clippy::too_many_arguments)]
    fn build_text_draw_item(
        &mut self,
        text: &str,
        x: f32,
        y: f32,
        style: &TextStyle,
        color: Color,
        layout: &TextLayoutOptions,
        draw_order: f32,
    ) -> Option<(DrawItem, Option<PendingTextureUpload>)> {
        if text.is_empty() {
            return None;
        }

        let texture_key = Self::build_text_texture_key(text, style, color);

        // Fast path: skip CPU rasterization when this text texture is already cached.
        let cached_dimensions = if let Some(Some(entry)) = self.texture_cache.get_mut(&texture_key) {
            entry.last_used_frame = self.current_frame;
            Some((entry.cached_texture.width, entry.cached_texture.height))
        } else {
            None
        };

        if let Some((width, height)) = cached_dimensions {
            let (text_x, text_y) =
                Self::aligned_text_position(x, y, width as f32, height as f32, layout);
            let item = self.build_image_rect_draw_item(
                text_x,
                text_y,
                width as f32,
                height as f32,
                texture_key,
                draw_order,
            );
            return Some((item, None));
        }

        let rasterized = self.rasterize_text(text, style, color)?;
        let (text_x, text_y) = Self::aligned_text_position(
            x,
            y,
            rasterized.width as f32,
            rasterized.height as f32,
            layout,
        );
        let item = self.build_image_rect_draw_item(
            text_x,
            text_y,
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

    #[allow(clippy::too_many_arguments)]
    fn build_world_text_draw_item(
        &mut self,
        text: &str,
        position: Vec2,
        rotation: f32,
        scale: Vec2,
        style: &TextStyle,
        color: Color,
        camera_position: Vec2,
        draw_order: f32,
    ) -> Option<(DrawItem, Option<PendingTextureUpload>)> {
        if text.is_empty() {
            return None;
        }

        let texture_key = Self::build_text_texture_key(text, style, color);

        let cached_dimensions = if let Some(Some(entry)) = self.texture_cache.get_mut(&texture_key) {
            entry.last_used_frame = self.current_frame;
            Some((entry.cached_texture.width, entry.cached_texture.height))
        } else {
            None
        };

        let (width, height, upload) = if let Some((width, height)) = cached_dimensions {
            (width as f32, height as f32, None)
        } else {
            let rasterized = self.rasterize_text(text, style, color)?;
            let upload = PendingTextureUpload {
                key: texture_key.clone(),
                rgba: Arc::from(rasterized.rgba),
                width: rasterized.width,
                height: rasterized.height,
            };
            (rasterized.width as f32, rasterized.height as f32, Some(upload))
        };

        let half_w = width * 0.5 * scale.x();
        let half_h = height * 0.5 * scale.y();
        let cos_t = rotation.cos();
        let sin_t = rotation.sin();
        let corners = [
            Vec2::new(-half_w, half_h),
            Vec2::new(-half_w, -half_h),
            Vec2::new(half_w, -half_h),
            Vec2::new(half_w, half_h),
        ];

        let transform_corner = |corner: Vec2| {
            let rotated_x = corner.x() * cos_t - corner.y() * sin_t;
            let rotated_y = corner.x() * sin_t + corner.y() * cos_t;
            self.world_to_clip(
                position.x() + rotated_x,
                position.y() + rotated_y,
                camera_position,
            )
        };

        let white = Self::color_to_array(Color::WHITE);
        let item = self.build_quad_draw_item_with_options(
            transform_corner(corners[0]),
            transform_corner(corners[1]),
            transform_corner(corners[2]),
            transform_corner(corners[3]),
            [white, white, white, white],
            [[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]],
            Some(texture_key),
            draw_order,
        );

        Some((item, upload))
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

    fn build_filled_arc_draw_item(
        &self,
        center_x: f32,
        center_y: f32,
        radius: f32,
        start_angle: f32,
        end_angle: f32,
        segments: u32,
        color: Color,
        draw_order: f32,
    ) -> Option<DrawItem> {
        if radius <= 0.0 {
            return None;
        }

        let sweep = end_angle - start_angle;
        if sweep.abs() <= f32::EPSILON {
            return None;
        }

        let segments = segments.max(3);
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
            let t = i as f32 / segments as f32;
            let angle = start_angle + sweep * t;
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

    fn build_arc_outline_draw_item(
        &self,
        center_x: f32,
        center_y: f32,
        radius: f32,
        start_angle: f32,
        end_angle: f32,
        thickness: f32,
        segments: u32,
        color: Color,
        draw_order: f32,
    ) -> Option<DrawItem> {
        if radius <= 0.0 {
            return None;
        }

        let sweep = end_angle - start_angle;
        if sweep.abs() <= f32::EPSILON {
            return None;
        }

        let segments = segments.max(3);
        let thickness = thickness.max(1.0);
        let inner = (radius - thickness * 0.5).max(0.0);
        let outer = radius + thickness * 0.5;
        let mut vertices = Vec::with_capacity(((segments + 1) * 2) as usize);
        let mut indices = Vec::with_capacity((segments * 6) as usize);
        let color = Self::color_to_array(color);

        for i in 0..=segments {
            let t = i as f32 / segments as f32;
            let angle = start_angle + sweep * t;
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

    fn build_filled_polygon_draw_item(
        &self,
        points: &[Vec2],
        color: Color,
        draw_order: f32,
    ) -> Option<DrawItem> {
        if points.len() < 3 {
            return None;
        }

        let mut vertices = Vec::with_capacity(points.len());
        let mut indices = Vec::with_capacity((points.len().saturating_sub(2)) * 3);
        let color = Self::color_to_array(color);

        for point in points {
            let clip = self.pixel_to_clip(point.x(), point.y());
            vertices.push(Vertex {
                position: [clip[0], clip[1], 0.0],
                color,
                tex_coords: [0.0, 0.0],
            });
        }

        for i in 1..(points.len() - 1) {
            indices.extend_from_slice(&[0, i as u32, (i + 1) as u32]);
        }

        Some(DrawItem {
            draw_order,
            texture_path: None,
            vertices,
            indices,
        })
    }

    fn build_mesh_draw_item(
        &self,
        vertices: &[crate::core::component::MeshVertex],
        indices: &[u32],
        color: Color,
        texture_path: Option<String>,
        draw_order: f32,
    ) -> Option<DrawItem> {
        if vertices.is_empty() || indices.is_empty() {
            return None;
        }

        let vertex_count = vertices.len() as u32;
        if indices.iter().any(|index| *index >= vertex_count) {
            return None;
        }

        let color = Self::color_to_array(color);
        let mut draw_vertices = Vec::with_capacity(vertices.len());

        for vertex in vertices {
            let clip = self.pixel_to_clip(vertex.position().x(), vertex.position().y());
            draw_vertices.push(Vertex {
                position: [clip[0], clip[1], 0.0],
                color,
                tex_coords: [vertex.uv().x(), vertex.uv().y()],
            });
        }

        Some(DrawItem {
            draw_order,
            texture_path: texture_path.map(|path| self.resolve_source_path(&path)),
            vertices: draw_vertices,
            indices: indices.to_vec(),
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
                DrawCommand::Arc {
                    center_x,
                    center_y,
                    radius,
                    start_angle,
                    end_angle,
                    color,
                    filled,
                    thickness,
                    segments,
                    draw_order,
                } => {
                    let item = if *filled {
                        self.build_filled_arc_draw_item(
                            *center_x,
                            *center_y,
                            *radius,
                            *start_angle,
                            *end_angle,
                            *segments,
                            *color,
                            *draw_order,
                        )
                    } else {
                        self.build_arc_outline_draw_item(
                            *center_x,
                            *center_y,
                            *radius,
                            *start_angle,
                            *end_angle,
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
                DrawCommand::Polygon {
                    points,
                    color,
                    filled,
                    thickness,
                    draw_order,
                } => {
                    if *filled {
                        if let Some(item) =
                            self.build_filled_polygon_draw_item(points, *color, *draw_order)
                        {
                            items.push(item);
                        }
                    } else if points.len() >= 2 {
                        for i in 0..points.len() {
                            let start = points[i];
                            let end = points[(i + 1) % points.len()];
                            items.push(self.build_line_draw_item(
                                start.x(),
                                start.y(),
                                end.x(),
                                end.y(),
                                *thickness,
                                *color,
                                *draw_order,
                            ));
                        }
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
                        self.resolve_source_path(texture_path),
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
                DrawCommand::Mesh {
                    vertices,
                    indices,
                    color,
                    texture_path,
                    draw_order,
                } => {
                    if let Some(item) = self.build_mesh_draw_item(
                        vertices,
                        indices,
                        *color,
                        texture_path.clone(),
                        *draw_order,
                    ) {
                        items.push(item);
                    }
                }
                DrawCommand::Text {
                    text,
                    x,
                    y,
                    style,
                    color,
                    layout,
                    draw_order,
                } => {
                    if let Some((item, upload)) = self.build_text_draw_item(
                        text,
                        *x,
                        *y,
                        style,
                        *color,
                        layout,
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
        objects: &ObjectManager,
        camera_position: Vec2,
    ) -> Vec<DrawItem> {
        let mut items = Vec::new();
        let keys = objects.get_sorted_keys();

        for &id in keys {
            if self.active_camera_object_id == Some(id) {
                continue;
            }

            let Some(object) = objects.get_object_by_id(id) else {
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

            let Some(world_transform) = objects.world_transform(id) else {
                continue;
            };
            let fill_color = mesh.fill_color().copied().unwrap_or(Color::WHITE);
            let color = [
                fill_color.r(),
                fill_color.g(),
                fill_color.b(),
                fill_color.a(),
            ];

            if !mesh.is_effectively_enabled() {
                continue;
            }

            let cos_t = world_transform.rotation.cos();
            let sin_t = world_transform.rotation.sin();
            let scale_x = world_transform.scale.x();
            let scale_y = world_transform.scale.y();
            let pos_x = world_transform.position.x();
            let pos_y = world_transform.position.y();

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
                texture_path: mesh.image_path().map(|p| self.resolve_source_path(p)),
                vertices,
                indices: mesh.geometry().indices().to_vec(),
            });
        }

        items
    }

    fn collect_text_mesh_draw_items(
        &mut self,
        objects: &ObjectManager,
        camera_position: Vec2,
    ) -> (Vec<DrawItem>, Vec<PendingTextureUpload>) {
        let mut items = Vec::new();
        let mut texture_uploads = Vec::new();
        let keys = objects.get_sorted_keys();

        for &id in keys {
            if self.active_camera_object_id == Some(id) {
                continue;
            }

            let Some(object) = objects.get_object_by_id(id) else {
                continue;
            };

            if !object.is_active() {
                continue;
            }

            let Some(world_transform) = objects.world_transform(id) else {
                continue;
            };

            for component in object.all_components() {
                let Some(text_mesh) = component
                    .as_any()
                    .downcast_ref::<crate::core::component::TextMeshComponent>()
                else {
                    continue;
                };

                if !text_mesh.is_effectively_enabled() || !text_mesh.visible() {
                    continue;
                }

                if let Some((item, upload)) = self.build_world_text_draw_item(
                    text_mesh.text(),
                    world_transform.position,
                    world_transform.rotation,
                    world_transform.scale,
                    text_mesh.text_style(),
                    text_mesh.color(),
                    camera_position,
                    text_mesh.draw_order(),
                ) {
                    items.push(item);
                    if let Some(upload) = upload {
                        texture_uploads.push(upload);
                    }
                }
            }
        }

        (items, texture_uploads)
    }

    fn collect_draw_items(
        &mut self,
        objects: &ObjectManager,
        draw_manager: Option<&DrawManager>,
    ) -> (Vec<DrawItem>, Vec<PendingTextureUpload>) {
        let camera_position = self.active_camera_position(objects);
        let mut items = self.collect_mesh_draw_items(objects, camera_position);
        let (mut text_mesh_items, mut text_mesh_uploads) =
            self.collect_text_mesh_draw_items(objects, camera_position);
        let (direct_draw_items, mut texture_uploads) = self.collect_direct_draw_items(draw_manager);
        items.append(&mut text_mesh_items);
        items.extend(direct_draw_items);
        texture_uploads.append(&mut text_mesh_uploads);

        items.sort_by(|a, b| {
            a.draw_order
                .partial_cmp(&b.draw_order)
                .unwrap_or(Ordering::Equal)
        });

        (items, texture_uploads)
    }

    fn compute_scene_version(
        &self,
        objects: &ObjectManager,
        draw_manager: Option<&DrawManager>,
    ) -> SceneVersion {
        SceneVersion {
            render_state_epoch: self.render_state_epoch,
            object_epoch: objects.scene_version(),
            draw_epoch: draw_manager.map_or(0, DrawManager::scene_version),
        }
    }

    /// Returns whether the window should request another redraw.
    pub fn should_request_redraw(
        &mut self,
        objects: &ObjectManager,
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

    /// Clean up textures that haven't been used recently.
    ///
    /// Removes cached textures (typically text textures) that haven't been
    /// accessed for more than `texture_ttl_frames` frames. This prevents
    /// memory leaks from dynamically generated text that changes every frame.
    fn cleanup_unused_textures(&mut self) {
        if self.current_frame < self.texture_ttl_frames {
            return; // Not enough frames have passed yet
        }

        let cutoff_frame = self.current_frame - self.texture_ttl_frames;
        let keys_to_remove: Vec<String> = self
            .texture_cache
            .iter()
            .filter_map(|(key, entry)| {
                // Only clean up text textures (they have the __pyg_text_ prefix)
                if key.starts_with("__pyg_text_") {
                    if let Some(cached_entry) = entry {
                        if cached_entry.last_used_frame < cutoff_frame {
                            return Some(key.clone());
                        }
                    }
                }
                None
            })
            .collect();

        if !keys_to_remove.is_empty() {
            logging::log_debug(&format!(
                "Cleaning up {} unused text textures (older than {} frames)",
                keys_to_remove.len(),
                self.texture_ttl_frames
            ));

            for key in keys_to_remove {
                self.texture_cache.remove(&key);
                self.texture_data_signature_cache.remove(&key);
            }
        }
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
        objects: &ObjectManager,
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

        // Increment frame counter and periodically clean up unused textures
        self.current_frame = self.current_frame.wrapping_add(1);
        if self.current_frame % 60 == 0 {
            // Clean up every 60 frames to avoid overhead
            self.cleanup_unused_textures();
        }

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
    /// When disabled, prefers Mailbox (low-latency, tear-free) or Immediate mode.
    pub fn configure_vsync(&mut self, vsync_enabled: bool) {
        self.vsync_enabled = vsync_enabled;

        self.surface_config.present_mode = if vsync_enabled {
            PresentMode::Fifo // VSync on (always supported)
        } else {
            // Try to use Mailbox (low-latency without tearing) if available,
            // otherwise fall back to Immediate (may tear but lowest latency).
            // Mailbox is better supported on macOS/Metal than Immediate.
            if self.surface_present_modes.contains(&PresentMode::Mailbox) {
                PresentMode::Mailbox
            } else if self.surface_present_modes.contains(&PresentMode::Immediate) {
                PresentMode::Immediate
            } else {
                // If neither is available, fall back to Fifo
                logging::log_warn("VSync off requested but Mailbox/Immediate modes not available; using Fifo");
                PresentMode::Fifo
            }
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
