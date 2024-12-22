use wgpu::RenderPipeline;

use crate::{
    camera::Camera,
    immediate_renderer::ImmediateRenderer,
    lights::Lights,
    pipeline::Pipeline,
    preloaded_renderer::PreloadedRenderer,
    quad_renderer::QuadRenderer,
    textures::{self, Textures},
};

pub struct VirtualGpu {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,

    pub render_pipelines: [RenderPipeline; 7],
    pub textures: Textures,
    pub quad_renderer: QuadRenderer,
    pub preloaded_renderer: PreloadedRenderer,
    pub immediate_renderer: ImmediateRenderer,

    pub camera: Camera,
    pub lights: Lights,

    pub instance_buffer: wgpu::Buffer,
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

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    &camera.bind_group_layout,
                    &textures.bind_group_layout,
                    &lights.bind_group_layout,
                ],
                push_constant_ranges: &[],
            });

        textures.load_texture(&device, &queue, "assets/default texture.png");

        let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Instance Buffer"),
            size: 8 * 1024 * 1024, // 8mb
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

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
        }
    }

    pub fn load_texture(&mut self, path: &str) -> usize {
        self.textures.load_texture(&self.device, &self.queue, path)
    }

    pub fn load_static_mesh(&mut self, data: &[f32], pipeline: Pipeline) -> usize {
        self.preloaded_renderer
            .load_static_mesh(&self.device, &self.queue, data, pipeline)
    }

    pub fn load_static_mesh_indexed(
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

fn generate_render_pipelines(
    device: &wgpu::Device,
    shader: &wgpu::ShaderModule,
    layout: &wgpu::PipelineLayout,
    format: wgpu::TextureFormat,
) -> [RenderPipeline; 7] {
    const PIPELINES: [Pipeline; 7] = [
        Pipeline::Color,
        Pipeline::Uv,
        Pipeline::ColorUv,
        Pipeline::ColorLit,
        Pipeline::UvLit,
        Pipeline::ColorUvLit,
        Pipeline::Quad2d,
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
