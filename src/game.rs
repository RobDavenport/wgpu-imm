use crate::{
    app::State,
    importer::{self},
    light::Light,
    pipeline::Pipeline,
};
use glam::{Mat4, Vec3, Vec4, Vec4Swizzles};

pub struct Game {
    t: f32,
    tex_index: usize,

    immediate_cube: Vec<f32>,
    immediate_fox: Vec<f32>,

    cube_static_indexed: usize,
    fox_static_raw: usize,
    test_sphere: usize,
}

impl Game {
    pub fn new() -> Self {
        // let immediate_cube =
        //     importer::import_gltf("assets/BoxVertexColors.glb").import_indexed_to_non_indexed();
        // let immediate_fox = importer::import_gltf("assets/Fox.glb").import();

        Self {
            t: 0.0,
            immediate_cube: Vec::new(),
            immediate_fox: Vec::new(),
            tex_index: 0,
            cube_static_indexed: 0,
            fox_static_raw: 0,
            test_sphere: 0,
        }
    }

    pub fn init(&mut self, state: &mut State) {
        // self.tex_index = state.load_texture("assets/Fox.png");
        // let (vertices, indices) =
        //     importer::import_gltf("assets/BoxVertexColors.glb").import_indexed();

        // self.cube_static_indexed =
        //     state.load_static_mesh_indexed(&vertices, &indices, Pipeline::ColorLit);

        let (vertices, indices) = importer::import_gltf("assets/test sphere metallic.glb")
            .import_indexed(Pipeline::ColorLit);
        self.test_sphere = state.load_static_mesh_indexed(&vertices, &indices, Pipeline::ColorLit);

        // let data = importer::import_gltf("assets/Fox.glb").import();
        // self.fox_static_raw = state.load_static_mesh(&data, Pipeline::Uv);
    }

    pub fn update(&mut self) {
        self.t += 1.0 / 360.0;
    }

    pub fn draw(&self, state: &mut State) {
        state.push_matrix(Mat4::IDENTITY);

        state.push_matrix(Mat4::from_translation(Vec3::new(0.0, 1.0, -2.0)));
        state.draw_static_mesh_indexed(self.test_sphere);

        // state.draw_tri_list(&self.immediate_cube, Pipeline::ColorLit);

        // state.set_texture(self.tex_index);
        // state.push_matrix(Mat4::from_scale(Vec3::splat(0.025)));
        // state.draw_tri_list(&self.immediate_fox, Pipeline::Uv);

        // let cube_transform =
        //     Mat4::from_translation(Vec3::new(-3.0, 0.0, 0.0)) * Mat4::from_rotation_y(self.t);
        // state.push_matrix(cube_transform);
        // state.draw_static_mesh_indexed(self.cube_static_indexed);

        // let fox_transform = Mat4::from_translation(Vec3::new(3.0, 3.0, 0.0))
        //     * Mat4::from_rotation_y(self.t)
        //     * Mat4::from_scale(Vec3::splat(0.025));
        // state.push_matrix(fox_transform);
        // state.draw_static_mesh(self.fox_static_raw);

        // Ambient Light
        state.push_light(&Light {
            color_intensity: Vec4::new(1.0, 1.0, 1.0, 0.05),
            position_range: Vec4::splat(-1.0),
            direction_angle: Vec4::ZERO,
        });

        // state.push_light(&Light {
        //     color_intensity: Vec4::splat(1.0),
        //     position_range: Vec4::splat(1.0),
        //     direction_angle: Vec4::ZERO,
        // });

        // Point Light
        for n in 0..6 {
            let light_x = self.t.sin() * 2.0;
            let light_y = self.t.cos() * 2.0 * n as f32;
            let light_offset = Vec4::new(light_x, light_y, 1.0, 1.0);
            let modified_position =
                Mat4::from_rotation_y(self.t + (n as f32 * 15.0)) * light_offset;
            state.push_light(&Light {
                color_intensity: Vec4::new(1.0, 1.0, 1.0, 1.0),
                position_range: modified_position.xyz().extend(1.0),
                direction_angle: Vec4::ZERO,
            });
        }
    }
}
