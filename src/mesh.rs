use wgpu::{Buffer, BufferDescriptor};

use crate::pipeline::Pipeline;

pub struct Mesh {
    pub vertex_buffer: Buffer,
    pub vertex_count: u32,
    pub pipeline: Pipeline,
}

pub struct IndexedMesh {
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub vertex_count: u32,
    pub pipeline: Pipeline,
}

pub fn vertex_buffer_descriptor(
    size: u64,
    label: Option<&'static str>,
) -> wgpu::BufferDescriptor<'static> {
    wgpu::BufferDescriptor {
        label,
        size, // 8mb
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    }
}
