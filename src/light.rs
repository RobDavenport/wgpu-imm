use bytemuck::{Pod, Zeroable};
use glam::{Vec3, Vec4, Vec4Swizzles};

pub type LightUniformType = [f32; 12];
pub const MAX_LIGHTS: u64 = 8;

pub enum LightKind {
    Ambient,
    Directional,
    Point,
    Spot,
}

#[derive(Pod, Zeroable, Clone, Copy)]
#[repr(C)]
pub struct Light {
    pub color_intensity: Vec4,
    pub position_range: Vec4,
    pub direction_angle: Vec4,
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
