use crate::{immediate_renderer::ImmediateRenderer, pipeline::Pipeline};

pub struct VirtualRenderPass {
    pub commands: Vec<Command>,
    pub last_byte_index: u64,
    pub matrix_count_3d: u64,
    pub light_count: u64,
}

pub enum Command {
    SetPipeline(Pipeline),
    Draw(u32),         //Vertex Count
    SetTexture(usize), // TextureId
    SetModelMatrix,
    DrawStaticMesh(usize),        // Static Mesh ID
    DrawStaticMeshIndexed(usize), // Static Mesh Indexed Id
    DrawSprite(usize),
}

impl VirtualRenderPass {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            last_byte_index: 0,
            matrix_count_3d: 0,
            light_count: 0,
        }
    }

    pub fn reset(&mut self) {
        self.commands.clear();
        self.last_byte_index = 0;
        self.matrix_count_3d = 0;
        self.light_count = 0;
    }
}
