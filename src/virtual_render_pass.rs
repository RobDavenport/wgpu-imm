use crate::pipeline::Pipeline;

#[derive(Default)]
pub struct VirtualRenderPass {
    pub commands: Vec<Command>,
    pub last_byte_index: u64,
    pub matrix_count: u64,
    pub light_count: u64,
}

pub enum Command {
    SetPipeline(Pipeline),
    Draw(u32),         //Vertex Count
    SetTexture(usize), // TextureId
    SetModelMatrix,
    DrawStaticMesh(usize),        // Static Mesh ID
    DrawStaticMeshIndexed(usize), // Static Mesh Indexed Id
}

impl VirtualRenderPass {
    pub fn reset(&mut self) {
        self.commands.clear();
        self.last_byte_index = 0;
        self.matrix_count = 0;
        self.light_count = 0;
    }
}
