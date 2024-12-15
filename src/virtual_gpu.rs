use wgpu::{Buffer, Device};

use crate::{
    mesh::{IndexedMesh, Mesh},
    texture::Texture,
    virtual_render_pass::VirtualRenderPass,
};

pub struct VirtualGpu {
    textures: Vec<Texture>,
    meshes: Vec<Mesh>,
    indexed_meshes: Vec<IndexedMesh>,

    bytes_in_use: usize,
    render_pass: VirtualRenderPass,
    // immediate_vertex_buffer: Buffer,
    // device: Device,
}

impl VirtualGpu {
    pub fn new() -> Self {
        Self {
            textures: Vec::new(),
            meshes: Vec::new(),
            indexed_meshes: Vec::new(),
            bytes_in_use: 0,
            render_pass: VirtualRenderPass::default(),
            // device,
            // immediate_vertex_buffer,
        }
    }
}
