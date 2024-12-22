use crate::{
    camera::Camera, immediate_renderer::ImmediateRenderer, lights::Lights, pipeline::Pipeline,
};

pub struct VirtualRenderPass {
    pub commands: Vec<Command>,

    pub camera: Camera,

    pub instance_buffer: wgpu::Buffer,
    pub inistance_count: u64,

    pub lights: Lights,
    pub light_count: u64,

    pub immediate_renderer: ImmediateRenderer,
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
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {
        let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Instance Buffer"),
            size: 8 * 1024 * 1024, // 8mb
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            commands: Vec::new(),
            camera: Camera::new(device, config),
            instance_buffer,
            inistance_count: 0,
            light_count: 0,
            immediate_renderer: ImmediateRenderer::new(device),
            lights: Lights::new(device),
        }
    }

    pub fn reset(&mut self) {
        self.commands.clear();
        self.inistance_count = 0;
        self.light_count = 0;

        self.immediate_renderer.reset();
    }
}
