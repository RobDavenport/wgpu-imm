use crate::{
    mesh::{IndexedMesh, Mesh},
    texture::Texture,
};

#[derive(Default)]
pub struct VirtualGpu {
    textures: Vec<Texture>,
    meshes: Vec<Mesh>,
    indexed_meshes: Vec<IndexedMesh>,

    bytes_in_use: usize,
}
