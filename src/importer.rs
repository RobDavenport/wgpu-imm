use crate::pipeline::Pipeline;

#[derive(PartialEq, Clone, Debug)]
pub enum ColorSource {
    Invalid,
    Uvs,
    Colors,
    Both,
}

pub struct Importer {
    positions: Vec<f32>,
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

    pub fn import(mut self) -> Vec<f32> {
        let mut out = Vec::new();

        self.clean_up_buffers();

        let pipeline = if let Some(pipeline) = self.get_pipeline() {
            pipeline
        } else {
            println!("Invalid import.");
            return out;
        };

        let import_color = pipeline.has_color();
        let import_uv = pipeline.has_uv();
        let import_lighting = pipeline.has_lighting();

        // TODO: Write this
        todo!();

        out
    }
}
