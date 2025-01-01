use std::f32::consts::PI;

use crate::{
    contexts::{Draw3dContext, Init3dContext},
    importer::{self},
    lights::Light,
    pipeline::Pipeline,
};
use glam::{Mat4, Vec3, Vec3A, Vec4, Vec4Swizzles};

pub struct Game {
    t: f32,
    tex_index: usize,

    immediate_cube: Vec<f32>,
    immediate_fox: Vec<f32>,

    cube_static_indexed: usize,
    fox_static_raw: usize,
    test_sphere: usize,
    tex_grid: usize,

    pbr_test: usize,
}

impl Game {
    pub fn new() -> Self {
        let immediate_cube =
            importer::import_gltf("assets/BoxVertexColors.glb").import_indexed_to_non_indexed();
        let immediate_fox = importer::import_gltf("assets/Fox.glb").import(Pipeline::Uv);

        Self {
            t: 0.0,
            immediate_cube,
            immediate_fox,
            tex_index: 0,
            cube_static_indexed: 0,
            fox_static_raw: 0,
            test_sphere: 0,
            tex_grid: 0,
            pbr_test: 0,
        }
    }

    pub fn init(&mut self, gpu: &mut impl Init3dContext) {
        self.tex_index = gpu.load_texture("assets/Fox.png");
        self.tex_grid = gpu.load_texture("assets/color grid 128x128.png");
        let (vertices, indices) =
            importer::import_gltf("assets/BoxVertexColors.glb").import_indexed(Pipeline::Color);

        self.cube_static_indexed =
            gpu.load_static_mesh_indexed(&vertices, &indices, Pipeline::Color);

        let (vertices, indices) = importer::import_gltf("assets/test sphere metallic.glb")
            .import_indexed(Pipeline::ColorLit);
        self.test_sphere = gpu.load_static_mesh_indexed(&vertices, &indices, Pipeline::ColorLit);

        let data = importer::import_gltf("assets/Fox.glb").import(Pipeline::Uv);
        self.fox_static_raw = gpu.load_static_mesh(&data, Pipeline::Uv);

        let (sphere, sphere_indices) =
            importer::import_gltf("assets/test sphere base.glb").import_indexed(Pipeline::ColorLit);
        let mut spheres = Vec::new();

        for metallic in 0..5 {
            for roughness in 0..11 {
                for index in sphere_indices.iter() {
                    let start = *index as usize * 12;
                    let end = start + 12;
                    let vertex = &sphere[start..end];
                    let x = vertex[0] + roughness as f32 * 2.0;
                    let y = vertex[1] + metallic as f32 * 2.0 - (3.0);
                    let z = vertex[2];
                    let to_copy = &vertex[3..9];
                    let lighting = &[metallic as f32 / 4.0, roughness as f32 / 10.0, 0.0];
                    spheres.extend_from_slice(&[x, y, z]);
                    spheres.extend_from_slice(to_copy); // Color, Normals
                    spheres.extend_from_slice(lighting);
                }
            }
        }

        self.pbr_test = gpu.load_static_mesh(&spheres, Pipeline::ColorLit);
    }

    pub fn update(&mut self) {
        self.t += 1.0 / 360.0;
    }

    pub fn draw(&self, state: &mut impl Draw3dContext) {
        state.push_matrix(Mat4::IDENTITY);
        state.draw_static_mesh(self.pbr_test);

        // state.push_matrix(Mat4::from_translation(Vec3::new(0.0, 1.0, -2.0)));
        // state.draw_static_mesh_indexed(self.test_sphere);

        // state.draw_tri_list(&self.immediate_cube, Pipeline::Color);

        // state.push_matrix(
        //     Mat4::from_translation(Vec3::new(50.0, 50.0, 1.0))
        //         * Mat4::from_scale(Vec3::splat(128.0)),
        // );
        // state.draw_sprite(self.tex_grid);

        // state.push_matrix(
        //     Mat4::from_translation(Vec3::new(100.0, 150.0, 0.999))
        //         * Mat4::from_scale(Vec3::splat(256.0)),
        // );
        // // state.draw_sprite(self.tex_index);
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
        // state.push_light(&Light {
        //     color_intensity: Vec4::new(1.0, 1.0, 1.0, 0.05),
        //     position_range: Vec4::splat(-1.0),
        //     direction_angle: Vec4::ZERO,
        // });

        // Point Lights
        for n in 0..2 {
            let color_max_angle = if n == 0 {
                Vec4::new(1.0, 0.0, 0.0, 1.0)
            } else {
                Vec4::new(0.0, 1.0, 0.0, 1.0)
            };
            //let light_x = self.t.sin() * 2.0;
            let light_y = self.t.cos() * 2.0 * n as f32;
            let light_z = 1.0;
            let light_offset = Vec4::new((25.0 / 2.0) * n as f32, light_y, light_z, 1.0);
            let modified_position =
                Mat4::from_rotation_y(self.t + (n as f32 * 15.0)) * light_offset;
            state.push_light(&Light {
                color_max_angle,
                position_range: modified_position
                    .xyz()
                    .extend((self.t.sin() * 0.5 + 0.5) * 50.0),
                direction_min_angle: Vec4::ZERO,
            });
        }

        let camera_pos = state.get_camera().eye.clone();
        let forward = state.get_camera().get_forward();

        // Spot Light
        state.push_light(&Light {
            color_max_angle: Vec4::new(1.0, 1.0, 1.0, 15.0_f32.to_radians().cos()),
            position_range: camera_pos.extend(15.0),
            direction_min_angle: forward.extend(12.5_f32.to_radians().cos()),
        });

        // Directional Light, Pointing Left, Down, Forward
        state.push_light(&Light {
            color_max_angle: Vec4::splat(1.0),
            position_range: Vec4::ZERO,
            direction_min_angle: Vec4::new(-1.0, -1.0, -1.0, 0.0),
        });
    }
}
