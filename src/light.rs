use glam::{Vec3, Vec4, Vec4Swizzles};

pub enum LightKind {
    Ambient,
    Directional,
    Point,
    Spot,
}

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
}

