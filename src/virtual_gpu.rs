use bytemuck::cast_slice;
use glam::{Mat4, Vec4Swizzles};
use wgpu::{PushConstantRange, RenderPipeline, ShaderStages, TextureView};

use crate::{
    camera::Camera,
    contexts,
    environment_map::EnvironmentMap,
    frame_buffer::{FrameBuffer, FRAME_BUFFER_BIND_GROUP_INDEX, SCALING_BIND_GROUP_INDEX},
    immediate_renderer::ImmediateRenderer,
    lights::{Light, Lights},
    pipeline::Pipeline,
    preloaded_renderer::PreloadedRenderer,
    quad_renderer::QuadRenderer,
    textures::{self, Textures},
    virtual_render_pass::{Command, VirtualRenderPass},
    PUSH_CONSTANT_SIZE,
};

pub const PER_FRAME_BIND_GROUP_INDEX: u32 = 0;
pub const TEXTURE_BIND_GROUP_INDEX: u32 = 1;
pub const MATCAP_BIND_GROUP_INDEX: u32 = 2;

pub const VERTEX_BUFFER_INDEX: u32 = 0;
pub const INSTANCE_BUFFER_INDEX: u32 = 1;

pub struct VirtualGpu {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,

    pub render_pipelines: [RenderPipeline; 11],
    pub textures: Textures,
    pub quad_renderer: QuadRenderer,
    pub preloaded_renderer: PreloadedRenderer,
    pub immediate_renderer: ImmediateRenderer,

    pub camera: Camera,
    pub lights: Lights,

    pub instance_buffer: wgpu::Buffer,
    pub virtual_render_pass: VirtualRenderPass,

    pub frame_buffer: FrameBuffer,
    pub environment_map: EnvironmentMap,

    pub per_frame_bind_group: wgpu::BindGroup,
}

impl VirtualGpu {
    pub fn new(
        device: wgpu::Device,
        queue: wgpu::Queue,
        config: &wgpu::SurfaceConfiguration,
    ) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Master Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let camera = Camera::new(&device, config);
        let mut textures = Textures::new(&device, config);
        let lights = Lights::new(&device);
        let environment_map = EnvironmentMap::new(&device, &queue);

        let per_frame_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Per Frame Bind Group Layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    // Views
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    // Positions
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    // Projections
                    wgpu::BindGroupLayoutEntry {
                        binding: 3,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    // Lights
                    wgpu::BindGroupLayoutEntry {
                        binding: 4,
                        visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    // Env Map
                    wgpu::BindGroupLayoutEntry {
                        binding: 5,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::Cube,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 6,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 7,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
            });

        let per_frame_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Per Frame Bind Group"),
            layout: &per_frame_bind_group_layout,
            entries: &[
                // Camera Bindings
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera.buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: camera.views_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: camera.positions_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: camera.projections_buffer.as_entire_binding(),
                },
                // Lights Bindings
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: lights.buffer.as_entire_binding(),
                },
                // Env Map
                wgpu::BindGroupEntry {
                    binding: 5,
                    resource: wgpu::BindingResource::TextureView(&environment_map.view),
                },
                wgpu::BindGroupEntry {
                    binding: 6,
                    resource: wgpu::BindingResource::Sampler(&environment_map.sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 7,
                    resource: environment_map.uniforms_buffer.as_entire_binding(),
                },
            ],
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    &per_frame_bind_group_layout,
                    &textures.bind_group_layout,
                    &textures.matcap_bind_group_layout, //Matcap Layout
                ],
                push_constant_ranges: &[PushConstantRange {
                    stages: ShaderStages::FRAGMENT,
                    range: 0..PUSH_CONSTANT_SIZE,
                }],
            });

        textures.load_texture(&device, &queue, "assets/default texture.png", false);

        let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Instance Buffer"),
            size: 8 * 1024 * 1024, // 8mb
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let frame_buffer = FrameBuffer::new(&device, config);

        Self {
            render_pipelines: generate_render_pipelines(
                &device,
                &shader,
                &render_pipeline_layout,
                config.format,
            ),
            textures,
            quad_renderer: QuadRenderer::new(&device, &queue),
            preloaded_renderer: PreloadedRenderer::new(),
            immediate_renderer: ImmediateRenderer::new(&device),
            camera,
            lights,
            device,
            queue,
            instance_buffer,
            virtual_render_pass: VirtualRenderPass::new(),
            frame_buffer,
            environment_map,
            per_frame_bind_group,
        }
    }

    pub fn render(&mut self, surface_view: &TextureView) {
        let view = &self.frame_buffer.view;

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Main Render Encoder"),
            });

        // Game Render Pass
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Main Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.textures.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(f32::NEG_INFINITY),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            self.queue.write_buffer(
                &self.camera.buffer,
                0,
                bytemuck::cast_slice(&self.camera.get_camera_uniforms()),
            );

            self.queue.write_buffer(
                &self.camera.views_buffer,
                0,
                bytemuck::bytes_of(&self.camera.get_view()),
            );
            self.queue.write_buffer(
                &self.camera.positions_buffer,
                0,
                bytemuck::bytes_of(&self.camera.eye.extend(1.0)),
            );
            self.queue.write_buffer(
                &self.camera.projections_buffer,
                0,
                bytemuck::bytes_of(&self.camera.get_projection_3d()),
            );

            render_pass.set_bind_group(PER_FRAME_BIND_GROUP_INDEX, &self.per_frame_bind_group, &[]);
            self.queue.write_buffer(
                &self.environment_map.uniforms_buffer,
                0,
                bytemuck::cast_slice(&self.environment_map.uniforms.get_uniforms()),
            );
            render_pass.set_vertex_buffer(INSTANCE_BUFFER_INDEX, self.instance_buffer.slice(..));

            self.virtual_render_pass.execute(&mut render_pass, self);
        }

        // Frame Buffer Render Pass
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Frame Buffer Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: surface_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.frame_buffer.pipeline);
            self.queue.write_buffer(
                &self.frame_buffer.scaling_buffer,
                0,
                bytemuck::cast_slice(&self.frame_buffer.scaling),
            );
            render_pass.set_bind_group(
                SCALING_BIND_GROUP_INDEX,
                &self.frame_buffer.scaling_bind_group,
                &[],
            );
            render_pass.set_bind_group(
                FRAME_BUFFER_BIND_GROUP_INDEX,
                &self.frame_buffer.texture_bind_group,
                &[],
            );
            render_pass.draw(0..4, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        self.virtual_render_pass.reset();
    }
}

fn generate_render_pipelines(
    device: &wgpu::Device,
    shader: &wgpu::ShaderModule,
    layout: &wgpu::PipelineLayout,
    format: wgpu::TextureFormat,
) -> [RenderPipeline; 11] {
    const PIPELINES: [Pipeline; 11] = [
        Pipeline::Color,
        Pipeline::Uv,
        Pipeline::ColorUv,
        Pipeline::ColorLit,
        Pipeline::UvLit,
        Pipeline::ColorUvLit,
        Pipeline::Quad2d,
        Pipeline::Matcap,
        Pipeline::MatcapColor,
        Pipeline::MatcapUv,
        Pipeline::MatcapColorUv,
    ];

    std::array::from_fn(|i| {
        let pipeline = PIPELINES[i];

        create_render_pipeline(device, shader, layout, format, pipeline)
    })
}

fn create_render_pipeline(
    device: &wgpu::Device,
    shader: &wgpu::ShaderModule,
    layout: &wgpu::PipelineLayout,
    format: wgpu::TextureFormat,
    pipeline: Pipeline,
) -> RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(pipeline.name()),
        layout: Some(layout),
        vertex: wgpu::VertexState {
            module: shader,
            entry_point: Some(pipeline.vertex_shader()),
            buffers: &pipeline.get_pipeline_buffers(),
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: shader,
            entry_point: Some(pipeline.fragment_shader()),
            targets: &[Some(wgpu::ColorTargetState {
                format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: textures::DepthTexture::DEPTH_FORMAT,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::GreaterEqual,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
        cache: None,
    })
}

impl contexts::Init3dContext for VirtualGpu {
    fn load_texture(&mut self, path: &str, is_matcap: bool) -> usize {
        self.textures
            .load_texture(&self.device, &self.queue, path, is_matcap)
    }

    fn load_static_mesh(&mut self, data: &[f32], pipeline: Pipeline) -> usize {
        self.preloaded_renderer
            .load_static_mesh(&self.device, &self.queue, data, pipeline)
    }

    fn load_static_mesh_indexed(
        &mut self,
        data: &[f32],
        indices: &[u16],
        pipeline: Pipeline,
    ) -> usize {
        self.preloaded_renderer.load_static_mesh_indexed(
            &self.device,
            &self.queue,
            data,
            indices,
            pipeline,
        )
    }
}

impl contexts::Draw3dContext for VirtualGpu {
    fn get_camera(&self) -> &Camera {
        &self.camera
    }

    fn draw_tri_list(&mut self, data: &[f32], pipeline: Pipeline) {
        let attribute_count = pipeline.get_attribute_count();
        let total_attributes = data.len();
        let vertex_count = total_attributes / attribute_count;

        if total_attributes % attribute_count != 0 {
            println!("Invalid triangle list, size mismatch");
            return;
        }

        self.queue.write_buffer(
            &self.immediate_renderer.buffer,
            self.virtual_render_pass.immediate_buffer_last_index,
            bytemuck::cast_slice(data),
        );

        self.virtual_render_pass
            .commands
            .push(Command::SetPipeline(pipeline));
        self.virtual_render_pass
            .commands
            .push(Command::Draw(vertex_count as u32));
        self.virtual_render_pass.immediate_buffer_last_index += total_attributes as u64 * 4;
    }

    fn push_light(&mut self, light: &Light) {
        let offset = self.virtual_render_pass.light_count * size_of::<Light>() as u64;
        let mut light = *light;
        let view_position = self.camera.get_view() * light.position_range.xyz().extend(1.0);
        let view_direction = self.camera.get_view() * light.direction_min_angle.xyz().extend(0.0);

        light.position_range = view_position.xyz().extend(light.position_range.w);
        light.direction_min_angle = view_direction.xyz().extend(light.direction_min_angle.w);

        self.queue.write_buffer(
            &self.lights.buffer,
            offset,
            cast_slice(&light.get_light_uniforms()),
        );

        self.virtual_render_pass.light_count += 1;
    }

    fn push_matrix(&mut self, matrix: Mat4) {
        let offset = self.virtual_render_pass.inistance_count * size_of::<Mat4>() as u64;
        self.queue
            .write_buffer(&self.instance_buffer, offset, bytemuck::bytes_of(&matrix));
        self.virtual_render_pass
            .commands
            .push(Command::SetModelMatrix);
        self.virtual_render_pass.inistance_count += 1;
    }

    fn draw_static_mesh(&mut self, index: usize) {
        self.virtual_render_pass
            .commands
            .push(Command::DrawStaticMesh(index))
    }

    fn draw_static_mesh_indexed(&mut self, index: usize) {
        self.virtual_render_pass
            .commands
            .push(Command::DrawStaticMeshIndexed(index))
    }

    fn draw_sprite(&mut self, index: usize) {
        self.virtual_render_pass
            .commands
            .push(Command::DrawSprite(index));
    }

    fn set_texture(&mut self, tex_id: usize) {
        self.virtual_render_pass
            .commands
            .push(Command::SetTexture(tex_id));
    }

    fn set_matcap(&mut self, matcap_id: usize, layer_index: usize, blend_mode: usize) {
        self.virtual_render_pass.commands.push(Command::SetMatcap(
            matcap_id,
            layer_index,
            blend_mode,
        ))
    }
}
