use crate::{
    mesh::{IndexedMesh, Mesh},
    textures::Texture,
    virtual_render_pass::VirtualRenderPass,
};

pub struct VirtualGpu {
    textures: Vec<Texture>,
    meshes: Vec<Mesh>,
    indexed_meshes: Vec<IndexedMesh>,

    bytes_in_use: usize,
    //virtual_render_pass: VirtualRenderPass,
    width: usize,
    height: usize,
}

impl VirtualGpu {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            textures: Vec::new(),
            meshes: Vec::new(),
            indexed_meshes: Vec::new(),
            bytes_in_use: 0,
            //virtual_render_pass: VirtualRenderPass::default(),
            width,
            height,
        }
    }
}
