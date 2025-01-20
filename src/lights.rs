use bytemuck::{Pod, Zeroable};
use glam::Vec4;

pub type LightUniformType = [f32; 12];
pub const MAX_LIGHTS: u64 = 4;

#[derive(Pod, Zeroable, Clone, Copy)]
#[repr(C)]
pub struct Light {
    pub color_max_angle: Vec4,
    pub position_range: Vec4,
    pub direction_min_angle: Vec4,
}

impl Light {
    pub fn get_light_uniforms(&self) -> LightUniformType {
        bytemuck::cast(*self)
    }
}

pub struct Lights {
    pub buffer: wgpu::Buffer,
}

impl Lights {
    pub fn new(device: &wgpu::Device) -> Self {
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("LightBuffer"),
            size: size_of::<Light>() as u64 * MAX_LIGHTS,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            buffer,
        }
    }
}
