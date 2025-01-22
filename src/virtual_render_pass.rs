use wgpu::ShaderStages;

use crate::{
    pipeline::Pipeline,
    virtual_gpu::{VirtualGpu, TEXTURE_BIND_GROUP_INDEX, VERTEX_BUFFER_INDEX},
};

pub struct VirtualRenderPass {
    pub commands: Vec<Command>,

    pub immediate_buffer_last_index: u64,
    pub inistance_count: u64,

    pub light_count: u64,
}

pub enum Command {
    SetPipeline(Pipeline),
    Draw(u32),                       //Vertex Count
    SetTexture(usize, usize, usize), // TextureId, Layer Index, Blend Mode
    SetMatcap(usize, usize, usize),  // Matcap Id, Layer Index, Blend Mode
    SetModelMatrix,
    DrawStaticMesh(usize),        // Static Mesh ID
    DrawStaticMeshIndexed(usize), // Static Mesh Indexed Id
    DrawSprite(usize),
}

#[derive(Default)]
struct TextureStates {
    texture_indices: [usize; 4],
    blend_modes: [u8; 4],
    is_matcap: [bool; 4],
}

impl TextureStates {
    fn to_push_constants(&self) -> [u8; 8] {
        let mut out = [0; 8];

        out[..4].copy_from_slice(&self.blend_modes);

        // Pack the is_matcap array into a single byte
        let mut matcap_mask = 0u32;
        for (i, &is_matcap) in self.is_matcap.iter().enumerate() {
            if is_matcap {
                matcap_mask |= 1 << i; // Set the corresponding bit
            }
        }

        // Store the matcap mask in the last byte
        out[4..8].copy_from_slice(bytemuck::bytes_of(&matcap_mask));

        out
    }

    fn set_texture(&mut self, index: usize, layer: usize, blend_mode: usize, is_matcap: bool) {
        self.texture_indices[layer] = index;
        self.blend_modes[layer] = blend_mode as u8;
        self.is_matcap[layer] = is_matcap
    }

    fn create_bind_group(&self, gpu: &VirtualGpu) -> wgpu::BindGroup {
        gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &gpu.textures.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(
                        &gpu.textures.textures[self.texture_indices[0]].view,
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(
                        &gpu.textures.textures[self.texture_indices[1]].view,
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(
                        &gpu.textures.textures[self.texture_indices[2]].view,
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::TextureView(
                        &gpu.textures.textures[self.texture_indices[3]].view,
                    ),
                },
            ],
        })
    }
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

    pub fn execute(&self, rp: &mut wgpu::RenderPass, gpu: &VirtualGpu) {
        let mut current_byte_index = 0;
        let mut current_vertex_size = 0;
        let mut current_model_matrix = 0;

        let mut texture_state = TextureStates::default();

        for command in self.commands.iter() {
            match command {
                Command::SetPipeline(pipeline) => {
                    rp.set_pipeline(&gpu.render_pipelines[pipeline.get_shader()]);
                    rp.set_push_constants(
                        ShaderStages::FRAGMENT,
                        0,
                        &texture_state.to_push_constants(),
                    );
                    current_vertex_size = pipeline.get_vertex_size();
                }
                Command::Draw(vertex_count) => {
                    rp.set_vertex_buffer(
                        VERTEX_BUFFER_INDEX,
                        gpu.immediate_renderer.buffer.slice(current_byte_index..),
                    );
                    rp.draw(
                        0..*vertex_count,
                        current_model_matrix - 1..current_model_matrix,
                    );
                    current_byte_index += *vertex_count as u64 * current_vertex_size as u64;
                }
                Command::SetTexture(tex_index, layer_index, blend_mode) => {
                    texture_state.set_texture(*tex_index, *layer_index, *blend_mode, false);
                    rp.set_bind_group(
                        TEXTURE_BIND_GROUP_INDEX,
                        &texture_state.create_bind_group(gpu),
                        &[],
                    );
                }
                Command::SetMatcap(matcap_index, layer_index, blend_mode) => {
                    texture_state.set_texture(*matcap_index, *layer_index, *blend_mode, true);
                    rp.set_bind_group(
                        TEXTURE_BIND_GROUP_INDEX,
                        &texture_state.create_bind_group(gpu),
                        &[],
                    );
                }
                Command::SetModelMatrix => {
                    current_model_matrix += 1;
                }
                Command::DrawStaticMesh(index) => {
                    let mesh = &gpu.preloaded_renderer.meshes[*index];
                    rp.set_pipeline(&gpu.render_pipelines[mesh.pipeline.get_shader()]);
                    rp.set_push_constants(
                        ShaderStages::FRAGMENT,
                        0,
                        &texture_state.to_push_constants(),
                    );
                    rp.set_vertex_buffer(VERTEX_BUFFER_INDEX, mesh.vertex_buffer.slice(..));
                    rp.draw(
                        0..mesh.vertex_count,
                        current_model_matrix - 1..current_model_matrix,
                    );
                }
                Command::DrawStaticMeshIndexed(index) => {
                    let mesh = &gpu.preloaded_renderer.indexed_meshes[*index];
                    rp.set_pipeline(&gpu.render_pipelines[mesh.pipeline.get_shader()]);
                    rp.set_push_constants(
                        ShaderStages::FRAGMENT,
                        0,
                        &texture_state.to_push_constants(),
                    );
                    rp.set_vertex_buffer(VERTEX_BUFFER_INDEX, mesh.vertex_buffer.slice(..));
                    rp.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                    rp.draw_indexed(
                        0..mesh.index_count,
                        0,
                        current_model_matrix - 1..current_model_matrix,
                    );
                }
                Command::DrawSprite(sprite_index) => {
                    todo!();
                    // let texture = &gpu.textures.textures[*sprite_index];
                    // rp.set_pipeline(&gpu.render_pipelines[Pipeline::Quad2d.get_shader()]);
                    // rp.set_bind_group(TEXTURE_BIND_GROUP_INDEX, &texture.bind_group, &[]);
                    // rp.set_index_buffer(
                    //     gpu.quad_renderer.quad_index_buffer.slice(..),
                    //     wgpu::IndexFormat::Uint16,
                    // );
                    // rp.set_vertex_buffer(
                    //     VERTEX_BUFFER_INDEX,
                    //     gpu.quad_renderer.quad_vertex_buffer.slice(..),
                    // );
                    // rp.draw_indexed(0..6, 0, current_model_matrix - 1..current_model_matrix)
                }
            }
        }
    }
}
