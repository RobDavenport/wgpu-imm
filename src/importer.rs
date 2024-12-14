use bytemuck::{cast_slice, from_bytes};

use crate::pipeline::Pipeline;

pub struct Importer {
    positions: Vec<f32>,
    indices: Vec<u16>,
    colors: Vec<f32>,
    uvs: Vec<f32>,
    normals: Vec<f32>,
    lighting: Vec<f32>,
}

impl Importer {
    fn has_colors(&self) -> bool {
        !self.colors.is_empty()
    }

    fn has_uvs(&self) -> bool {
        !self.uvs.is_empty()
    }

    fn has_normals(&self) -> bool {
        !self.normals.is_empty()
    }

    fn has_lighting(&self) -> bool {
        !self.lighting.is_empty()
    }

    // Some exporters might automatically export COLOR_0
    // and exporting COLOR_1
    fn clean_up_buffers(&mut self) {
        if !self.has_normals() {
            self.lighting.clear();
            // No normals = no lighting data needed at all
            return;
        }

        if self.has_lighting() {
            // We have normals and lighting, so we can operate as normal
            return;
        }

        // We aren't sure if we should use Color for lighting or vertex colors
        // Must check for an alternative color provider, in this case UVs
        if self.has_uvs() {
            // We have UVs, so we want to use COLOR for LIGHTING
            std::mem::swap(&mut self.colors, &mut self.lighting);
        } else {
            // We don't have UVs, so normals are extra unneeded data
            self.normals.clear();
        }
    }

    fn get_pipeline(&self) -> Option<Pipeline> {
        let color_pipeline = match (self.has_colors(), self.has_uvs()) {
            (true, true) => Pipeline::ColorUv,
            (true, false) => Pipeline::Color,
            (false, true) => Pipeline::Uv,
            (false, false) => return None,
        };

        if self.has_lighting() && self.has_normals() {
            Some(color_pipeline.lit())
        } else {
            Some(color_pipeline)
        }
    }

    fn import(mut self) -> Vec<f32> {
        let mut out = Vec::new();

        self.clean_up_buffers();

        let pipeline = if let Some(pipeline) = self.get_pipeline() {
            println!("Importing pipeline: {:?}", pipeline);
            pipeline
        } else {
            println!("Invalid import.");
            return out;
        };

        let import_color = pipeline.has_color();
        let import_uv = pipeline.has_uv();
        let import_lighting = pipeline.has_lighting();

        for index in self.indices.iter().copied() {
            let start = index as usize * 3;
            let end = start + 3;

            out.extend_from_slice(&self.positions[start..end]);

            if import_color {
                out.extend_from_slice(&self.colors[start..end]);
            }

            if import_uv {
                let uv_start = index as usize * 2;
                let uv_end = uv_start + 2;
                out.extend_from_slice(&self.uvs[uv_start..uv_end]);
            }

            if import_lighting {
                out.extend_from_slice(&self.normals[start..end]);
                out.extend_from_slice(&self.lighting[start..end]);
            }
        }

        out
    }
}

pub fn import_gltf(path: &str) -> Vec<f32> {
    let (document, buffers, images) = gltf::import(path).unwrap();

    let blob = &buffers[0].0;

    let mut indices = Vec::new();
    let mut positions = Vec::new();
    let mut colors = Vec::new();
    let mut uvs = Vec::new();
    let mut normals = Vec::new();
    let mut lighting = Vec::new();

    for mesh in document.meshes() {
        let primitive = mesh.primitives().next().unwrap();
        let primitive_count = mesh.primitives().count();

        if primitive_count > 1 {
            println!(
                "Primitive count > 1 ({primitive_count}), mesh may not be exported correctly..."
            )
        }

        for (kind, attribute) in primitive.attributes() {
            if attribute.view().unwrap().buffer().index() != 0 {
                panic!("wrong buffer index");
            }
            println!(
                "Found {kind:?}: {:?} x {:?}",
                attribute.data_type(),
                attribute.dimensions()
            );
            let view = attribute.view().unwrap();
            let start = attribute.offset() + view.offset();
            let end = start + (attribute.count() * attribute.size());
            let view = &blob[start..end];

            match kind {
                gltf::Semantic::Positions => {
                    let view: &[f32] = cast_slice(view);

                    for p in view {
                        positions.push(*p);
                    }
                }
                gltf::Semantic::Colors(0) => {
                    let view: &[f32] = cast_slice(view);

                    for c in view {
                        colors.push(*c);
                    }
                }
                gltf::Semantic::Colors(1) => {
                    let view: &[f32] = cast_slice(view);

                    for l in view {
                        lighting.push(*l);
                    }
                }
                gltf::Semantic::TexCoords(_) => {
                    let view: &[f32] = cast_slice(view);

                    for t in view {
                        uvs.push(*t);
                    }
                }
                gltf::Semantic::Normals => {
                    let view: &[f32] = cast_slice(view);

                    for n in view {
                        normals.push(*n);
                    }
                }

                _ => {}
            };
        }

        if let Some(indices_accessor) = primitive.indices() {
            let size = indices_accessor.size();
            let start = indices_accessor.offset() + indices_accessor.view().unwrap().offset();
            let count = indices_accessor.count();
            let end = start + (count * size);

            for index in blob[start..end].chunks_exact(size) {
                let i = if size == 2 {
                    *from_bytes::<u16>(&index[0..2]) as u16
                } else if size == 4 {
                    *from_bytes::<u32>(&index[0..4]) as u16
                } else {
                    panic!("Unhandled byte size for mesh");
                };
                indices.push(i as u16);
            }
            println!("Triangles found: {}", indices.len() / 3);
        } else {
            indices.clear();
            for n in 0..positions.len() / 3 {
                let n = n as u16;
                indices.push(n)
            }
            println!("Autogenerated {} triangles.", indices.len() / 3);
        }
    }

    Importer {
        positions,
        indices,
        colors,
        uvs,
        normals,
        lighting,
    }
    .import()
}
