use crate::pipeline::Pipeline;

pub struct VirtualRenderPass {
    pub commands: Vec<Command>,

    pub immediate_buffer_last_index: u64,
    pub inistance_count: u64,

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
            inistance_count: 0,
            light_count: 0,
            immediate_buffer_last_index: 0,
        }
    }

    pub fn reset(&mut self) {
        self.commands.clear();
        self.inistance_count = 0;
        self.light_count = 0;
        self.immediate_buffer_last_index = 0;
    }
}
