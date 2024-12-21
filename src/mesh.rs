use crate::pipeline::Pipeline;

pub struct Mesh {
    pub vertex_buffer: wgpu::Buffer,
    pub vertex_count: u32,
    pub pipeline: Pipeline,
}

pub struct IndexedMesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: u32,
    pub pipeline: Pipeline,
}

pub fn quad_vertex_buffer_descriptor() -> wgpu::BufferDescriptor<'static> {
    wgpu::BufferDescriptor {
        label: Some("Quad Vertex Buffer"),
        size: size_of::<[f32; 5 * 4]>() as u64,
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    }
}

pub const fn quad_vertices() -> &'static [f32; 5 * 4] {
    // 0--3
    // |\ |
    // | \|
    // 1--2
    &[
        -0.5, 0.5, 0.0, 0.0, 1.0, // Top-left vertex
        -0.5, -0.5, 0.0, 0.0, 0.0, // Bottom-left vertex
        0.5, -0.5, 0.0, 1.0, 0.0, // Bottom-right vertex
        0.5, 0.5, 0.0, 1.0, 1.0, // Top-right vertex
    ]
}

pub const fn quad_indices() -> &'static [u16; 6] {
    &[
        0, 1, 2, // First triangle (bottom-left to top-right)
        2, 3, 0, // Second triangle (top-right to top-left)
    ]
}

pub fn vertex_buffer_descriptor(
    size: u64,
    label: Option<&'static str>,
) -> wgpu::BufferDescriptor<'static> {
    wgpu::BufferDescriptor {
        label,
        size,
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    }
}

pub fn index_buffer_descriptor(
    size: u64,
    label: Option<&'static str>,
) -> wgpu::BufferDescriptor<'static> {
    wgpu::BufferDescriptor {
        label,
        size,
        usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    }
}
