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
    z_far: f32,
}

impl Camera {
    pub fn new(config: &SurfaceConfiguration) -> Self {
        Self {
            eye: Vec3A::new(0.0, 0.0, 5.0),
            yaw: PI,
            up: Vec3A::Y,
            aspect: config.width as f32 / config.height as f32,
            fovy: 45.0,
            z_near: 0.1,
            z_far: 100.0,
        }
    }

    pub fn get_forward(&self) -> Vec3A {
        Vec3A::new(self.yaw.sin(), 0.0, self.yaw.cos())
    }

    pub fn get_view_projection(&self) -> Mat4 {
        let view = Mat4::look_to_rh(self.eye.into(), self.get_forward().into(), self.up.into());
        let proj =
            Mat4::perspective_rh(self.fovy.to_radians(), self.aspect, self.z_near, self.z_far);

        proj * view
    }
}
