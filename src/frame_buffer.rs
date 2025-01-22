use wgpu::{BindGroupLayoutEntry, PushConstantRange, ShaderStages, TextureViewDescriptor};

use crate::{textures, PUSH_CONSTANT_SIZE};

pub const SCALING_BIND_GROUP_INDEX: u32 = 0;
pub const FRAME_BUFFER_BIND_GROUP_INDEX: u32 = 1;

pub struct FrameBuffer {
    pub view: wgpu::TextureView,
    pub texture_bind_group: wgpu::BindGroup,
    pub pipeline: wgpu::RenderPipeline,

    width: u32,
    height: u32,
    pub scaling_buffer: wgpu::Buffer,
    pub scaling: [f32; 2],
    pub scaling_bind_group: wgpu::BindGroup,
}

impl FrameBuffer {
    pub fn adjust_scale(&mut self, surface_width: u32, surface_height: u32) {
        let int_width = surface_width / self.width;
        let int_height = surface_height / self.height;
        let scaling = int_width.min(int_height);

        let width = (self.width * scaling) as f32 / surface_width as f32;
        let height = (self.height * scaling) as f32 / surface_height as f32;
        self.scaling = [width, height];
    }

    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {
        let width = config.width;
        let height = config.height;
        let format = config.format;

        let scaling_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("scaling bind group layout"),
            });

        let scaling_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Scaling Buffer"),
            size: size_of::<[f32; 2]>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let scaling_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &scaling_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: scaling_buffer.as_entire_binding(),
            }],
            label: Some("scaling_bind_group"),
        });

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            label: Some("Frame Buffer Texture"),
            view_formats: &[],
        });

        let sampler = device.create_sampler(&textures::sampler_descriptor());
        let view = texture.create_view(&TextureViewDescriptor::default());
        let texture_bind_group_layout = device.create_bind_group_layout(bind_group_layout_desc());

        let texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
            label: Some("Frame Texture Buffer Bind Group"),
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Frame Buffer Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("frame_buffer.wgsl").into()),
        });

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Frame Buffer Layout"),
            bind_group_layouts: &[&scaling_bind_group_layout, &texture_bind_group_layout],
            push_constant_ranges: &[PushConstantRange {
                stages: ShaderStages::FRAGMENT,
                range: 0..PUSH_CONSTANT_SIZE,
            }],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Frame Buffer Pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        Self {
            width,
            height,
            view,
            texture_bind_group,
            pipeline,
            scaling_buffer,
            scaling: [1.0, 1.0],
            scaling_bind_group,
        }
    }
}

fn bind_group_layout_desc() -> &'static wgpu::BindGroupLayoutDescriptor<'static> {
    &wgpu::BindGroupLayoutDescriptor {
        label: Some("Frame buffer bind group layout descriptor"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: false },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
                count: None,
            },
        ],
    }
}
