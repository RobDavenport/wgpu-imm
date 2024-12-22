use crate::mesh;

// TODO: Could do something for immediate Textures

pub struct ImmediateRenderer {
    pub buffer: wgpu::Buffer,
    pub last_byte_index: u64,
}

impl ImmediateRenderer {
    pub fn new(device: &wgpu::Device) -> Self {
        let buffer = device.create_buffer(&mesh::vertex_buffer_descriptor(
            1024 * 1024 * 8, // 8mb
            Some("Immediate Vertex Buffer"),
        ));

        Self {
            buffer,
            last_byte_index: 0,
        }
    }

    pub fn reset(&mut self) {
        self.last_byte_index = 0;
    }
}
