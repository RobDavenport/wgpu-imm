use glam::{Mat4, Vec3A};
use wgpu::SurfaceConfiguration;

pub type CameraUniformType = [f32; 48];

pub struct Camera {
    pub eye: Vec3A,
    pub yaw: f32,
    up: Vec3A,
    aspect: f32,
    fovy: f32,
    z_near: f32,
    width: u32,
    height: u32,
}

impl Camera {
    pub fn new(config: &SurfaceConfiguration) -> Self {
        Self {
            eye: Vec3A::new(0.0, 1.0, 5.0),
            yaw: 0.0,
            up: Vec3A::Y,
            aspect: config.width as f32 / config.height as f32,
            fovy: 45.0,
            z_near: 0.1,
            width: config.width,
            height: config.height,
        }
    }

    pub fn get_forward(&self) -> Vec3A {
        Vec3A::new(self.yaw.sin(), 0.0, -self.yaw.cos())
    }

    pub fn get_view(&self) -> Mat4 {
        Mat4::look_to_rh(self.eye.into(), self.get_forward().into(), self.up.into())
    }

    fn get_projection_3d(&self) -> Mat4 {
        Mat4::perspective_infinite_reverse_rh(self.fovy.to_radians(), self.aspect, self.z_near)
    }

    pub fn get_camera_uniforms(&self) -> CameraUniformType {
        let mut out = [0.0; 48];

        let view = self.get_view();
        let projection = self.get_projection_3d();
        let ortho = self.get_projection_2d();

        view.write_cols_to_slice(&mut out[0..16]);
        projection.write_cols_to_slice(&mut out[16..32]);
        ortho.write_cols_to_slice(&mut out[32..48]);
        out
    }

    pub fn get_projection_2d(&self) -> Mat4 {
        Mat4::orthographic_rh(0.0, self.width as f32, self.height as f32, 0.0, 1.0, -1.0)
    }
}
