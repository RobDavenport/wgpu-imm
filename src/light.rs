use bytemuck::{Pod, Zeroable};
use glam::{Vec3, Vec4, Vec4Swizzles};

pub type LightUniformType = [f32; 16];
pub const MAX_LIGHTS: u64 = 16;

pub enum LightKind {
    Ambient,
    Directional,
    Point,
    Spot,
}

#[derive(Pod, Zeroable, Clone, Copy)]
#[repr(C)]
pub struct Light {
    color_intensity: Vec4,
    position_range: Vec4,
    direction_angle: Vec4,
}

impl Light {
    pub fn get_kind(&self) -> LightKind {
        let is_global = self.position_range.w <= 0.0;
        let is_directional = self.direction_angle.xyz() != Vec3::ZERO;

        match (is_global, is_directional) {
            // Global, Directional
            (true, true) => LightKind::Directional,

            // Global, Omnidirectional
            (true, false) => LightKind::Ambient,

            // Local, Omnidirectional
            (false, true) => LightKind::Point,

            // Local, Directional
            (false, false) => LightKind::Spot,
        }
    }

    pub fn get_light_uniforms(&self) -> LightUniformType {
        bytemuck::cast(*self)
    }
}

pub fn get_vertex_buffer_layout() -> wgpu::VertexBufferLayout<'static> {
    wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<LightUniformType>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &[
            wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x4,
            },
            wgpu::VertexAttribute {
                offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                shader_location: 1,
                format: wgpu::VertexFormat::Float32x4,
            },
            wgpu::VertexAttribute {
                offset: std::mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                shader_location: 2,
                format: wgpu::VertexFormat::Float32x4,
            },
        ],
    }
}
