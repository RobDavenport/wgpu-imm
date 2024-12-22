use bytemuck::{Pod, Zeroable};
use glam::Vec4;

pub type LightUniformType = [f32; 12];
pub const MAX_LIGHTS: u64 = 8;

#[derive(Pod, Zeroable, Clone, Copy)]
#[repr(C)]
pub struct Light {
    pub color_intensity: Vec4,
    pub position_range: Vec4,
    pub direction_angle: Vec4,
}

impl Light {
    pub fn get_light_uniforms(&self) -> LightUniformType {
        bytemuck::cast(*self)
    }
}
