use super::geometry::{Mesh, Rect, Vertex};
use super::texture::Texture;
use std::rc::Rc;
use wgpu::util::DeviceExt;

pub struct Entity {
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
    pub scale: f32,
    pub z_index: i32,
    pub mesh: Mesh,
    pub texture: Option<Rc<Texture>>,

    // We keep raw vertices to apply transform updates
    raw_vertices: Vec<Vertex>,
}

impl Entity {
    pub fn new_rect(
        device: &wgpu::Device,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        color: [f32; 4],
        texture: Option<Rc<Texture>>,
    ) -> Self {
        let rect = Rect { w, h, color };
        let (vertices, indices) = rect.generate_vertices();

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Entity VB"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Entity IB"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self {
            x,
            y,
            rotation: 0.0,
            scale: 1.0,
            z_index: 0,
            mesh: Mesh {
                vertex_buffer,
                index_buffer,
                num_indices: indices.len() as u32,
            },
            texture,
            raw_vertices: vertices,
        }
    }

    pub fn update_gpu_buffer(&self, queue: &wgpu::Queue) {
        // Simple 2D Transform: Scale -> Rotate -> Translate
        let cos_t = self.rotation.cos();
        let sin_t = self.rotation.sin();

        let transformed: Vec<Vertex> = self
            .raw_vertices
            .iter()
            .map(|v| {
                let mut p = v.position;
                // 1. Scale
                p[0] *= self.scale;
                p[1] *= self.scale;
                // 2. Rotate
                let rx = p[0] * cos_t - p[1] * sin_t;
                let ry = p[0] * sin_t + p[1] * cos_t;
                // 3. Translate
                Vertex {
                    position: [rx + self.x, ry + self.y, 0.0],
                    color: v.color,
                    tex_coords: v.tex_coords,
                }
            })
            .collect();

        queue.write_buffer(
            &self.mesh.vertex_buffer,
            0,
            bytemuck::cast_slice(&transformed),
        );
    }
}
