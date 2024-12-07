use std::f32::consts::PI;

use glam::{Mat4, Vec3A};
use wgpu::SurfaceConfiguration;

pub struct Camera {
    pub eye: Vec3A,
    pub yaw: f32,
    up: Vec3A,
    aspect: f32,
    fovy: f32,
    z_near: f32,
}

impl Camera {
    pub fn new(config: &SurfaceConfiguration) -> Self {
        Self {
            eye: Vec3A::new(0.0, 1.0, 5.0),
            yaw: PI,
            up: Vec3A::Y,
            aspect: config.width as f32 / config.height as f32,
            fovy: 45.0,
            z_near: 0.1,
        }
    }

    pub fn get_forward(&self) -> Vec3A {
        Vec3A::new(self.yaw.sin(), 0.0, self.yaw.cos())
    }

    pub fn get_view_projection(&self) -> Mat4 {
        let view = Mat4::look_to_rh(self.eye.into(), self.get_forward().into(), self.up.into());

        let proj =
            Mat4::perspective_infinite_reverse_rh(self.fovy.to_radians(), self.aspect, self.z_near);

        proj * view
    }
}
