use crate::{
    app::State,
    importer::{self},
    light::Light,
    pipeline::Pipeline,
};
use glam::{Mat4, Vec3, Vec4};

pub struct Game {
    t: f32,
    tex_index: usize,

    immediate_cube: Vec<f32>,
    immediate_fox: Vec<f32>,

    cube_static_indexed: usize,
    fox_static_raw: usize,
}

impl Game {
    pub fn new() -> Self {
        let immediate_cube =
            importer::import_gltf("assets/BoxVertexColors.glb").import_indexed_to_non_indexed();
        let immediate_fox = importer::import_gltf("assets/Fox.glb").import();

        Self {
            t: 0.0,
            immediate_cube,
            immediate_fox,
            tex_index: 0,
            cube_static_indexed: 0,
            fox_static_raw: 0,
        }
    }

    pub fn init(&mut self, state: &mut State) {
        self.tex_index = state.load_texture("assets/Fox.png");
        let (vertices, indices) =
            importer::import_gltf("assets/BoxVertexColors.glb").import_indexed();

        self.cube_static_indexed =
            state.load_static_mesh_indexed(&vertices, &indices, Pipeline::ColorLit);

        let data = importer::import_gltf("assets/Fox.glb").import();
        self.fox_static_raw = state.load_static_mesh(&data, Pipeline::Uv);
    }

    pub fn update(&mut self) {
        self.t += 1.0 / 360.0;
    }

    pub fn draw(&self, state: &mut State) {
        state.push_matrix(Mat4::IDENTITY);
        state.draw_tri_list(&self.immediate_cube, Pipeline::ColorLit);

        state.set_texture(self.tex_index);
        state.push_matrix(Mat4::from_scale(Vec3::splat(0.025)));
        state.draw_tri_list(&self.immediate_fox, Pipeline::Uv);

        let cube_transform =
            Mat4::from_translation(Vec3::new(-3.0, 0.0, 0.0)) * Mat4::from_rotation_y(self.t);
        state.push_matrix(cube_transform);
        state.draw_static_mesh_indexed(self.cube_static_indexed);

        let fox_transform = Mat4::from_translation(Vec3::new(3.0, 3.0, 0.0))
            * Mat4::from_rotation_y(self.t)
            * Mat4::from_scale(Vec3::splat(0.025));
        state.push_matrix(fox_transform);
        state.draw_static_mesh(self.fox_static_raw);

        // Ambient Light
        state.push_light(&Light {
            color_intensity: Vec4::new(1.0, 1.0, 1.0, 0.01),
            position_range: Vec4::splat(-1.0),
            direction_angle: Vec4::ZERO,
        });

        // Point Light
        state.push_light(&Light {
            color_intensity: Vec4::new(1.0, 1.0, 1.0, 1.0),
            position_range: Vec4::new(0.0, 0.0, 0.0, 999.999),
            direction_angle: Vec4::ZERO,
        });

    }
}
