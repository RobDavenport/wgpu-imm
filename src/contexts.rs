use glam::Mat4;

use crate::{lights::Light, pipeline::Pipeline};

pub trait Init3dContext {
    fn load_texture(&mut self, path: &str) -> usize;

    fn load_static_mesh(&mut self, data: &[f32], pipeline: Pipeline) -> usize;

    fn load_static_mesh_indexed(
        &mut self,
        data: &[f32],
        indices: &[u16],
        pipeline: Pipeline,
    ) -> usize;
}

pub trait Draw3dContext {
    fn draw_tri_list(&mut self, data: &[f32], pipeline: Pipeline);
    fn push_light(&mut self, light: &Light);
    fn push_matrix(&mut self, matrix: Mat4);
    fn draw_static_mesh(&mut self, index: usize);
    fn draw_static_mesh_indexed(&mut self, index: usize);
    fn draw_sprite(&mut self, index: usize);
    fn set_texture(&mut self, tex_id: usize);
}
