use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
    pub tex_coords: [f32; 2],
}

impl Vertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute { offset: 0, shader_location: 0, format: wgpu::VertexFormat::Float32x3 }, // Pos
                wgpu::VertexAttribute { offset: 12, shader_location: 1, format: wgpu::VertexFormat::Float32x4 }, // Color
                wgpu::VertexAttribute { offset: 28, shader_location: 2, format: wgpu::VertexFormat::Float32x2 }, // UV
            ],
        }
    }
}

pub struct Mesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
}

// --- Shape Logic ---

pub struct Rect {
    pub w: f32, pub h: f32,
    pub color: [f32; 4],
}

impl Rect {
    pub fn generate_vertices(&self) -> (Vec<Vertex>, Vec<u16>) {
        let (w, h) = (self.w / 2.0, self.h / 2.0);
        let vertices = vec![
            Vertex { position: [-w, h, 0.0], color: self.color, tex_coords: [0.0, 0.0] }, // TL
            Vertex { position: [-w, -h, 0.0], color: self.color, tex_coords: [0.0, 1.0] }, // BL
            Vertex { position: [w, -h, 0.0], color: self.color, tex_coords: [1.0, 1.0] }, // BR
            Vertex { position: [w, h, 0.0], color: self.color, tex_coords: [1.0, 0.0] }, // TR
        ];
        let indices = vec![0, 1, 2, 0, 2, 3];
        (vertices, indices)
    }
}