use std::{f32::consts::PI, fs};

use crate::{
    contexts::{Draw3dContext, Init3dContext},
    importer::{self},
    lights::Light,
    pipeline::Pipeline,
};
use glam::{Mat4, Vec3, Vec3A, Vec4, Vec4Swizzles};

pub struct Game {
    t: f32,
    fox_tex: usize,

    immediate_cube: Vec<f32>,
    immediate_fox: Vec<f32>,

    cube_static_indexed: usize,
    fox_static_raw: usize,
    test_sphere: usize,
    tex_grid: usize,

    pbr_test: usize,

    matcaps: Vec<usize>,
    monkey_index: usize,
    dog_matcap_mesh: usize,
    dog_tex: usize,
    dog_static: usize,

    ship_tex: usize,
    ship_mesh: usize,
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
            fox_tex: 0,
            cube_static_indexed: 0,
            fox_static_raw: 0,
            test_sphere: 0,
            tex_grid: 0,
            pbr_test: 0,
            monkey_index: 0,
            matcaps: Vec::new(),
            dog_matcap_mesh: 0,
            dog_tex: 0,
            dog_static: 0,
            ship_tex: 0,
            ship_mesh: 0,
        }
    }

    pub fn init(&mut self, gpu: &mut impl Init3dContext) {
        self.fox_tex = gpu.load_texture("assets/Fox.png");
        self.dog_tex = gpu.load_texture("assets/dog tex.png");
        self.tex_grid = gpu.load_texture("assets/color grid 128x128.png");
        self.ship_tex = gpu.load_texture("assets/ship tex.png");

        let (vertices, indices) =
            importer::import_gltf("assets/BoxVertexColors.glb").import_indexed(Pipeline::Color);

        self.cube_static_indexed =
            gpu.load_static_mesh_indexed(&vertices, &indices, Pipeline::Color);

        let (vertices, indices) = importer::import_gltf("assets/test sphere metallic.glb")
            .import_indexed(Pipeline::ColorLit);
        self.test_sphere = gpu.load_static_mesh_indexed(&vertices, &indices, Pipeline::ColorLit);

        let data = importer::import_gltf("assets/Fox.glb").import(Pipeline::Uv);
        self.fox_static_raw = gpu.load_static_mesh(&data, Pipeline::Uv);

        let (data, indices) =
            importer::import_gltf("assets/dog.glb").import_indexed(Pipeline::MatcapUv);
        self.dog_matcap_mesh = gpu.load_static_mesh_indexed(&data, &indices, Pipeline::MatcapUv);

        let (data, indices) = importer::import_gltf("assets/dog.glb").import_indexed(Pipeline::Uv);
        self.dog_static = gpu.load_static_mesh_indexed(&data, &indices, Pipeline::Uv);

        let (data, indices) =
            importer::import_gltf("assets/ship.glb").import_indexed(Pipeline::MatcapUv);
        self.ship_mesh = gpu.load_static_mesh_indexed(&data, &indices, Pipeline::MatcapUv);

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

        for file in fs::read_dir("assets/matcaps").unwrap() {
            let file = file.unwrap();
            println!("Loading matcap: {:?}", file.file_name());
            let id = gpu.load_texture(file.path().to_str().unwrap());
            self.matcaps.push(id);
        }

        let (monkey, monkey_indices) =
            importer::import_gltf("assets/monkey1.glb").import_indexed(Pipeline::Matcap);
        self.monkey_index =
            gpu.load_static_mesh_indexed(&monkey, &monkey_indices, Pipeline::Matcap);

        self.pbr_test = gpu.load_static_mesh(&spheres, Pipeline::ColorLit);
    }

    pub fn update(&mut self) {
        self.t += 1.0 / 360.0;
    }

    pub fn draw(&self, state: &mut impl Draw3dContext) {
        self.draw_matcaps(state);
        // self.draw_pbr_test(state);
    }

    fn draw_matcaps(&self, state: &mut impl Draw3dContext) {
        let max = self.matcaps.len() as f32;
        let offset = -(max / 2.0);
        let distance = 2.5;
        let rotation = Mat4::from_rotation_y(self.t * 0.5);

        let scale = Mat4::from_scale(Vec3::splat(0.25));

        for (i, matcap_id) in self.matcaps.iter().enumerate() {
            let translation = Vec3::new(offset + distance * i as f32, 0.0, 0.0);
            state.push_matrix(Mat4::from_translation(translation) * rotation);
            state.set_matcap(*matcap_id, 0, 8);
            state.set_matcap(
                (*matcap_id * 2) % self.matcaps.len() + self.matcaps[0],
                1,
                2,
            );
            state.set_matcap(
                (*matcap_id * 3) % self.matcaps.len() + self.matcaps[0],
                2,
                6,
            );
            state.set_matcap(
                (*matcap_id * 4) % self.matcaps.len() + self.matcaps[0],
                3,
                8,
            );
            state.draw_static_mesh_indexed(self.monkey_index);

            state.push_matrix(
                Mat4::from_translation(translation + Vec3::new(0.0, 2.0, 0.0)) * rotation,
            );
            state.set_texture(self.dog_tex, 0, 0);
            state.set_matcap(*matcap_id, 1, 3);
            state.set_texture(self.ship_tex, 2, 8);
            state.set_matcap(
                (*matcap_id * 2) % self.matcaps.len() + self.matcaps[0],
                3,
                8,
            );
            state.draw_static_mesh_indexed(self.dog_matcap_mesh);

            state.push_matrix(
                Mat4::from_translation(translation + Vec3::new(0.0, -2.0, 0.0)) * rotation * scale,
            );
            state.set_texture(self.ship_tex, 0, 0);
            state.set_texture(0, 2, 0);
            state.draw_static_mesh_indexed(self.ship_mesh);
        }

        state.set_texture(self.dog_tex, 0, 1);
        state.push_matrix(Mat4::IDENTITY);
        state.draw_static_mesh_indexed(self.dog_static);
    }

    fn draw_pbr_test(&self, state: &mut impl Draw3dContext) {
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
