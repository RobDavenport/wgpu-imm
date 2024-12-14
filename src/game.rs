use crate::{
    app::State,
    importer::{self},
    pipeline::Pipeline,
};
use glam::{Mat4, Vec3};

pub struct Game {
    t: f32,
    tex_index: usize,

    cube: Vec<f32>,
    fox: Vec<f32>,
}

impl Game {
    pub fn new() -> Self {
        let cube = importer::import_gltf("assets/BoxVertexColors.glb");
        let fox = importer::import_gltf("assets/Fox.glb");

        Self {
            t: 0.0,
            cube,
            fox,
            tex_index: 0,
        }
    }

    pub fn init(&mut self, state: &mut State) {
        self.tex_index = state.load_texture("assets/Fox.png");
    }

    pub fn update(&mut self) {
        self.t += 1.0 / 60.0;
    }

    pub fn draw(&self, state: &mut State) {
        state.push_matrix(Mat4::IDENTITY);
        state.draw_tri_list(&self.cube, Pipeline::Color);
        state.set_texture(self.tex_index);
        state.push_matrix(Mat4::from_scale(Vec3::splat(0.025)));
        state.draw_tri_list(&self.fox, Pipeline::Uv);
        state.push_matrix(Mat4::from_translation(Vec3::new(-2.0, 0.0, 0.0)));
        state.draw_tri_list(&self.cube, Pipeline::Color);
        state.push_matrix(Mat4::from_translation(Vec3::new(2.0, 0.0, 0.0)));
        state.draw_tri_list(&self.cube, Pipeline::Color);
    }
}
