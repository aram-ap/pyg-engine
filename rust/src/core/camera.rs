use cgmath::SquareMatrix;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

pub struct Camera {
    pub uniform: CameraUniform,
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
}

impl Camera {
    pub fn new(device: &wgpu::Device, width: u32, height: u32) -> Self {
        let uniform = CameraUniform {
            view_proj: cgmath::Matrix4::identity().into(),
        };

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("camera_layout"),
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        let mut camera = Self {
            uniform,
            buffer,
            bind_group,
            bind_group_layout,
        };

        // camera.resize(queue_is_not_needed_here_conceptually_but_passed_later(width, height));
        camera
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        let aspect = width as f32 / height as f32;
        let proj = cgmath::ortho(-aspect, aspect, -1.0, 1.0, -1.0, 1.0);
        self.uniform.view_proj = proj.into();
    }

    pub fn update(&self, queue: &wgpu::Queue) {
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[self.uniform]));
    }

    // Helper to fix the mock logic above
    fn resize_internal(width: u32, height: u32) -> [[f32; 4]; 4] {
        let aspect = width as f32 / height as f32;
        cgmath::ortho(-aspect, aspect, -1.0, 1.0, -1.0, 1.0).into()
    }
}
