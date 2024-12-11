use std::sync::Arc;

use glam::{Mat4, Vec3A};
use image::ImageReader;
use pollster::FutureExt;
use wgpu::{
    Adapter, BindGroup, BindGroupLayout, Device, Instance, MemoryHints, PresentMode, Queue,
    RenderPipeline, Surface, SurfaceCapabilities,
};
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

use crate::camera::{Camera, CameraUniformType};
use crate::game::Game;
use crate::pipeline::Pipeline;
use crate::texture::{self, DepthTexture, Texture};
use crate::vertex::{self, *};
use crate::virtual_render_pass::{Command, VirtualRenderPass};

pub struct StateApplication {
    state: Option<State>,
    game: Game,
}

const CAMERA_BIND_GROUP_INDEX: u32 = 0;
const TEXTURE_BIND_GROUP_INDEX: u32 = 1;

const IMMEDIATE_VERTEX_BUFFER_INDEX: u32 = 0;
const MODEL_MATRIX_VERTEX_BUFFER_INDEX: u32 = 1;

impl StateApplication {
    pub fn new() -> Self {
        Self {
            state: None,
            game: Game::new(),
        }
    }
}

impl ApplicationHandler for StateApplication {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes().with_title("Hello!"))
            .unwrap();

        let mut state = State::new(window);

        self.game.init(&mut state);

        self.state = Some(state);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let window = self.state.as_ref().unwrap().window();

        if window.id() == window_id {
            match event {
                WindowEvent::CloseRequested => {
                    event_loop.exit();
                }
                WindowEvent::Resized(physical_size) => {
                    self.state.as_mut().unwrap().resize(physical_size);
                }
                WindowEvent::RedrawRequested => {
                    self.state.as_mut().unwrap().update();
                    self.game.update();
                    self.game.draw(self.state.as_mut().unwrap());
                    self.state.as_mut().unwrap().render().unwrap();
                }
                WindowEvent::CursorMoved { position, .. } => {
                    let state = self.state.as_mut().unwrap();
                    state.mx = (position.x as f32) / state.size.width as f32;
                    state.my = 1.0 - ((position.y as f32) / state.size.height as f32);
                }
                WindowEvent::MouseInput { button, state, .. } => {
                    // if button == MouseButton::Left && state.is_pressed() {
                    //     let state = self.state.as_mut().unwrap();

                    //     let vertex = Vertex {
                    //         position: [(state.mx * 2.0) - 1.0, (state.my * 2.0) - 1.0, 1.0],
                    //         color: [fastrand::f32(), fastrand::f32(), fastrand::f32()],
                    //     };

                    //     println!("Pushed: {vertex:?}");

                    //     state.vertices.push(vertex);
                    // }
                }
                WindowEvent::KeyboardInput { event, .. } => {
                    let state = &mut self.state.as_mut().unwrap();
                    // Check the key event state and handle accordingly
                    match event.state.is_pressed() {
                        true => match event.physical_key {
                            PhysicalKey::Code(KeyCode::KeyW) => state.camera_delta.z += 1.0,
                            PhysicalKey::Code(KeyCode::KeyS) => state.camera_delta.z -= 1.0,
                            PhysicalKey::Code(KeyCode::KeyQ) => state.camera_delta.x += 1.0,
                            PhysicalKey::Code(KeyCode::KeyE) => state.camera_delta.x -= 1.0,
                            PhysicalKey::Code(KeyCode::KeyA) => state.camera_yaw_delta = 1.0,
                            PhysicalKey::Code(KeyCode::KeyD) => state.camera_yaw_delta = -1.0,
                            _ => {}
                        },
                        false => {
                            match event.physical_key {
                                PhysicalKey::Code(KeyCode::KeyW)
                                | PhysicalKey::Code(KeyCode::KeyS) => state.camera_delta.z = 0.0,
                                PhysicalKey::Code(KeyCode::KeyQ)
                                | PhysicalKey::Code(KeyCode::KeyE) => state.camera_delta.x = 0.0,
                                PhysicalKey::Code(KeyCode::KeyA)
                                | PhysicalKey::Code(KeyCode::KeyD) => state.camera_yaw_delta = 0.0,
                                _ => {}
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        let window = self.state.as_ref().unwrap().window();
        window.request_redraw();
    }
}

pub struct State {
    surface: Surface<'static>,
    device: Device,
    queue: Queue,
    config: wgpu::SurfaceConfiguration,

    size: PhysicalSize<u32>,
    window: Arc<Window>,

    render_pipelines: [RenderPipeline; 2],
    vertex_buffer: wgpu::Buffer,
    depth_texture: DepthTexture,

    mx: f32,
    my: f32,

    texture_bind_group_layout: BindGroupLayout,
    textures: Vec<Texture>,

    camera: Camera,
    camera_buffer: wgpu::Buffer,
    model_matrix_buffer: wgpu::Buffer,
    camera_bind_group: BindGroup,
    camera_delta: Vec3A,
    camera_yaw_delta: f32,

    virtual_render_pass: VirtualRenderPass,
}

impl State {
    pub fn new(window: Window) -> Self {
        let window_arc = Arc::new(window);
        let size = window_arc.inner_size();
        let instance = Self::create_gpu_instance();
        let surface = instance.create_surface(window_arc.clone()).unwrap();
        let adapter = Self::create_adapter(instance, &surface);
        let (device, queue) = Self::create_device(&adapter);
        let surface_caps = surface.get_capabilities(&adapter);
        let config = Self::create_surface_config(size, surface_caps);
        surface.configure(&device, &config);

        let shader_color = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader Color"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader_color.wgsl").into()),
        });

        let shader_texture = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader Texture"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader_uv.wgsl").into()),
        });

        let depth_texture =
            texture::DepthTexture::create_depth_texture(&device, &config, "depth_texture");

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: false },
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
                label: Some("texture_bind_group_layout"),
            });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    // View/Proj
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("camera_bind_group_layout"),
            });

        let model_matrix_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Model Matrix Buffer"),
            size: 8 * 1024 * 1024, // 8mb
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let camera_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("View Matrix Buffer"),
            size: size_of::<CameraUniformType>() as u64, //20 floats
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        let render_pipeline_layout_colored =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout Colored"),
                bind_group_layouts: &[&camera_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline_layout_uvs =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout Uvs"),
                bind_group_layouts: &[&camera_bind_group_layout, &texture_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline_color =
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline Color"),
                layout: Some(&render_pipeline_layout_colored),
                vertex: wgpu::VertexState {
                    module: &shader_color,
                    entry_point: Some("vs_main"), // 1.
                    buffers: &[vertex::color(), vertex::model_matrix()], // 2.
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader_color,
                    entry_point: Some("fs_main"),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: config.format,
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
                    format: texture::DepthTexture::DEPTH_FORMAT,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::Greater, // 1.
                    stencil: wgpu::StencilState::default(),        // 2.
                    bias: wgpu::DepthBiasState::default(),
                }),
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
                cache: None,
            });

        let render_pipeline_uvs = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline UVs"),
            layout: Some(&render_pipeline_layout_uvs),
            vertex: wgpu::VertexState {
                module: &shader_texture,
                entry_point: Some("vs_main"),
                buffers: &[vertex::textured(), vertex::model_matrix()],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader_texture,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
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
                format: texture::DepthTexture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Greater, // 1.
                stencil: wgpu::StencilState::default(),        // 2.
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Vertex Buffer"),
            size: 8 * 1024 * 1024, // 8mb
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let camera = Camera::new(&config);

        Self {
            surface,
            device,
            queue,
            config,
            size,
            window: window_arc,
            render_pipelines: [render_pipeline_color, render_pipeline_uvs],
            vertex_buffer,
            mx: 0.0,
            my: 0.0,
            camera,
            camera_buffer,
            camera_bind_group,
            camera_delta: Vec3A::ZERO,
            camera_yaw_delta: 0.0,

            textures: Vec::new(),
            texture_bind_group_layout,

            virtual_render_pass: VirtualRenderPass::default(),
            model_matrix_buffer,
            depth_texture,
        }
    }

    fn create_surface_config(
        size: PhysicalSize<u32>,
        capabilities: SurfaceCapabilities,
    ) -> wgpu::SurfaceConfiguration {
        let surface_format = capabilities
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(capabilities.formats[0]);

        wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: PresentMode::AutoVsync,
            alpha_mode: capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        }
    }

    fn create_device(adapter: &Adapter) -> (Device, Queue) {
        adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    label: None,
                    memory_hints: MemoryHints::default(),
                },
                None,
            )
            .block_on()
            .unwrap()
    }

    fn create_adapter(instance: Instance, surface: &Surface) -> Adapter {
        instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(surface),
                force_fallback_adapter: false,
            })
            .block_on()
            .unwrap()
    }

    fn create_gpu_instance() -> Instance {
        Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        })
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.size = new_size;

        self.config.width = new_size.width;
        self.config.height = new_size.height;

        self.surface.configure(&self.device, &self.config);

        self.depth_texture = texture::DepthTexture::create_depth_texture(
            &self.device,
            &self.config,
            "depth_texture",
        );

        println!("Resized to {:?} from state!", new_size);
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&self.camera.get_camera_uniforms()),
        );

        self.queue.submit([]);

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
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
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(0.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_bind_group(CAMERA_BIND_GROUP_INDEX, &self.camera_bind_group, &[]);
            render_pass.set_vertex_buffer(
                MODEL_MATRIX_VERTEX_BUFFER_INDEX,
                self.model_matrix_buffer.slice(..),
            );
            let mut current_byte_index = 0;
            let mut current_vertex_size = 0;
            let mut current_model_matrix = 0;

            for command in self.virtual_render_pass.commands.iter() {
                match command {
                    Command::SetPipeline(pipeline) => {
                        render_pass.set_pipeline(&self.render_pipelines[pipeline.get_shader()]);
                        current_vertex_size = pipeline.get_vertex_size();
                    }
                    Command::Draw(vertex_count) => {
                        render_pass.set_vertex_buffer(
                            IMMEDIATE_VERTEX_BUFFER_INDEX,
                            self.vertex_buffer.slice(current_byte_index..),
                        );
                        render_pass.draw(
                            0..*vertex_count,
                            current_model_matrix - 1..current_model_matrix,
                        );
                        current_byte_index += *vertex_count as u64 * current_vertex_size as u64;
                    }
                    Command::SetTexture(tex_index) => {
                        let texture = &self.textures[*tex_index];
                        render_pass.set_bind_group(
                            TEXTURE_BIND_GROUP_INDEX,
                            &texture.bind_group,
                            &[],
                        );
                    }
                    Command::SetModelMatrix => {
                        current_model_matrix += 1;
                    }
                }
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        self.virtual_render_pass.reset();

        Ok(())
    }

    pub fn draw_tri_list(&mut self, data: &[f32], pipeline: Pipeline) {
        let attribute_count = pipeline.get_attribute_count();
        let total_attributes = data.len();
        let vertex_count = total_attributes / attribute_count;

        if total_attributes % attribute_count != 0 {
            println!("Invalid triangle list, size mismatch");
            return;
        }

        self.queue.write_buffer(
            &self.vertex_buffer,
            self.virtual_render_pass.last_byte_index,
            bytemuck::cast_slice(data),
        );

        self.virtual_render_pass
            .commands
            .push(Command::SetPipeline(pipeline));
        self.virtual_render_pass
            .commands
            .push(Command::Draw(vertex_count as u32));
        self.virtual_render_pass.last_byte_index += total_attributes as u64 * 4;
    }

    pub fn push_matrix(&mut self, matrix: Mat4) {
        let offset = self.virtual_render_pass.matrix_count * size_of::<Mat4>() as u64;
        self.queue.write_buffer(
            &self.model_matrix_buffer,
            offset,
            bytemuck::bytes_of(&matrix),
        );
        self.virtual_render_pass
            .commands
            .push(Command::SetModelMatrix);
        self.virtual_render_pass.matrix_count += 1;
    }

    pub fn set_texture(&mut self, tex_id: usize) {
        self.virtual_render_pass
            .commands
            .push(Command::SetTexture(tex_id));
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    fn update(&mut self) {
        const DT: f32 = 1.0 / 60.0;
        const CAMERA_SPEED: f32 = 2.5;
        const CAMERA_ROT_SPEED: f32 = 0.75;

        let forward = self.camera.get_forward();
        let right = Vec3A::Y.cross(forward);

        self.camera.eye += forward * self.camera_delta.z * DT * CAMERA_SPEED;
        self.camera.eye += right * self.camera_delta.x * DT * CAMERA_SPEED;

        self.camera.yaw += self.camera_yaw_delta * DT * CAMERA_ROT_SPEED;
    }

    pub fn load_texture(&mut self, path: &str) -> usize {
        let image = ImageReader::open(path).unwrap().decode().unwrap();
        let image = image.to_rgba8();
        let dimensions = image.dimensions();
        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            size,
            mip_level_count: 1, // We'll talk about this a little later
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            label: Some("diffuse_texture"),
            view_formats: &[],
        });

        let sampler = self.device.create_sampler(&texture::sampler_descriptor());
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.texture_bind_group_layout,
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
            label: Some(path),
        });

        self.queue.write_texture(
            // Tells wgpu where to copy the pixel data
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            // The actual pixel data
            &image,
            // The layout of the texture
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            size,
        );

        self.queue.submit([]);

        let texture = Texture {
            texture,
            view,
            sampler,
            bind_group,
        };

        self.textures.push(texture);
        self.textures.len() - 1
    }
}
