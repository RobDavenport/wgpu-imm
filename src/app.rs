use std::sync::Arc;

use bytemuck::cast_slice;
use glam::{Mat4, Vec3A, Vec4Swizzles};
use pollster::FutureExt;
use wgpu::{
    Adapter, Device, Instance, MemoryHints, PresentMode, Queue, Surface, SurfaceCapabilities,
};
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

use crate::game::Game;
use crate::lights::Light;
use crate::pipeline::Pipeline;
use crate::textures::{self};
use crate::virtual_gpu::VirtualGpu;
use crate::virtual_render_pass::{Command, VirtualRenderPass};

pub struct StateApplication {
    state: Option<State>,
    game: Game,
}

const CAMERA_BIND_GROUP_INDEX: u32 = 0;
const TEXTURE_BIND_GROUP_INDEX: u32 = 1;
const LIGHT_BIND_GROUP_INDEX: u32 = 2;

const VERTEX_BUFFER_INDEX: u32 = 0;
const INSTANCE_BUFFER_INDEX: u32 = 1;

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

        self.game.init(&mut state.virtual_gpu);

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
    config: wgpu::SurfaceConfiguration,

    size: PhysicalSize<u32>,
    window: Arc<Window>,

    camera_delta: Vec3A,
    camera_yaw_delta: f32,

    virtual_render_pass: VirtualRenderPass,
    virtual_gpu: VirtualGpu,
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

        let virtual_render_pass = VirtualRenderPass::new(&device, &config);
        let virtual_gpu = VirtualGpu::new(device, queue, &config, &virtual_render_pass);

        Self {
            surface,
            config,
            size,
            window: window_arc,
            camera_delta: Vec3A::ZERO,
            camera_yaw_delta: 0.0,

            virtual_render_pass,
            virtual_gpu,
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

        self.surface
            .configure(&self.virtual_gpu.device, &self.config);

        self.virtual_gpu.textures.depth_texture = textures::DepthTexture::create_depth_texture(
            &self.virtual_gpu.device,
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

        let mut encoder =
            self.virtual_gpu
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

        self.virtual_gpu.queue.write_buffer(
            &self.virtual_render_pass.camera.buffer,
            0,
            bytemuck::cast_slice(&self.virtual_render_pass.camera.get_camera_uniforms()),
        );

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
                    view: &self.virtual_gpu.textures.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(f32::NEG_INFINITY),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_bind_group(
                CAMERA_BIND_GROUP_INDEX,
                &self.virtual_render_pass.camera.bind_group,
                &[],
            );
            render_pass.set_bind_group(
                LIGHT_BIND_GROUP_INDEX,
                &self.virtual_render_pass.lights.bind_group,
                &[],
            );
            render_pass.set_vertex_buffer(
                INSTANCE_BUFFER_INDEX,
                self.virtual_render_pass.instance_buffer.slice(..),
            );
            let mut current_byte_index = 0;
            let mut current_vertex_size = 0;
            let mut current_model_matrix = 0;

            for command in self.virtual_render_pass.commands.iter() {
                match command {
                    Command::SetPipeline(pipeline) => {
                        render_pass.set_pipeline(
                            &self.virtual_gpu.render_pipelines[pipeline.get_shader()],
                        );
                        current_vertex_size = pipeline.get_vertex_size();
                    }
                    Command::Draw(vertex_count) => {
                        render_pass.set_vertex_buffer(
                            VERTEX_BUFFER_INDEX,
                            self.virtual_render_pass
                                .immediate_renderer
                                .buffer
                                .slice(current_byte_index..),
                        );
                        render_pass.draw(
                            0..*vertex_count,
                            current_model_matrix - 1..current_model_matrix,
                        );
                        current_byte_index += *vertex_count as u64 * current_vertex_size as u64;
                    }
                    Command::SetTexture(tex_index) => {
                        let texture = &self.virtual_gpu.textures.textures[*tex_index];
                        render_pass.set_bind_group(
                            TEXTURE_BIND_GROUP_INDEX,
                            &texture.bind_group,
                            &[],
                        );
                    }
                    Command::SetModelMatrix => {
                        current_model_matrix += 1;
                    }
                    Command::DrawStaticMesh(index) => {
                        let mesh = &self.virtual_gpu.preloaded_renderer.meshes[*index];
                        render_pass.set_pipeline(
                            &self.virtual_gpu.render_pipelines[mesh.pipeline.get_shader()],
                        );
                        render_pass
                            .set_vertex_buffer(VERTEX_BUFFER_INDEX, mesh.vertex_buffer.slice(..));
                        render_pass.draw(
                            0..mesh.vertex_count,
                            current_model_matrix - 1..current_model_matrix,
                        );
                    }
                    Command::DrawStaticMeshIndexed(index) => {
                        let mesh = &self.virtual_gpu.preloaded_renderer.indexed_meshes[*index];
                        render_pass.set_pipeline(
                            &self.virtual_gpu.render_pipelines[mesh.pipeline.get_shader()],
                        );
                        render_pass
                            .set_vertex_buffer(VERTEX_BUFFER_INDEX, mesh.vertex_buffer.slice(..));
                        render_pass.set_index_buffer(
                            mesh.index_buffer.slice(..),
                            wgpu::IndexFormat::Uint16,
                        );
                        render_pass.draw_indexed(
                            0..mesh.index_count,
                            0,
                            current_model_matrix - 1..current_model_matrix,
                        );
                    }
                    Command::DrawSprite(sprite_index) => {
                        let texture = &self.virtual_gpu.textures.textures[*sprite_index];
                        render_pass.set_pipeline(
                            &self.virtual_gpu.render_pipelines[Pipeline::Quad2d.get_shader()],
                        );
                        render_pass.set_bind_group(
                            TEXTURE_BIND_GROUP_INDEX,
                            &texture.bind_group,
                            &[],
                        );
                        render_pass.set_index_buffer(
                            self.virtual_gpu.quad_renderer.quad_index_buffer.slice(..),
                            wgpu::IndexFormat::Uint16,
                        );
                        render_pass.set_vertex_buffer(
                            VERTEX_BUFFER_INDEX,
                            self.virtual_gpu.quad_renderer.quad_vertex_buffer.slice(..),
                        );
                        render_pass.draw_indexed(
                            0..6,
                            0,
                            current_model_matrix - 1..current_model_matrix,
                        )
                    }
                }
            }
        }

        self.virtual_gpu
            .queue
            .submit(std::iter::once(encoder.finish()));
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

        self.virtual_gpu.queue.write_buffer(
            &self.virtual_render_pass.immediate_renderer.buffer,
            self.virtual_render_pass.immediate_renderer.last_byte_index,
            bytemuck::cast_slice(data),
        );

        self.virtual_render_pass
            .commands
            .push(Command::SetPipeline(pipeline));
        self.virtual_render_pass
            .commands
            .push(Command::Draw(vertex_count as u32));
        self.virtual_render_pass.immediate_renderer.last_byte_index += total_attributes as u64 * 4;
    }

    pub fn push_light(&mut self, light: &Light) {
        let offset = self.virtual_render_pass.light_count * size_of::<Light>() as u64;
        let mut light = *light;
        let view_position =
            self.virtual_render_pass.camera.get_view() * light.position_range.xyz().extend(1.0);
        let view_direction =
            self.virtual_render_pass.camera.get_view() * light.direction_angle.xyz().extend(0.0);

        light.position_range = view_position.xyz().extend(light.position_range.w);
        light.direction_angle = view_direction.xyz().extend(light.direction_angle.w);

        self.virtual_gpu.queue.write_buffer(
            &self.virtual_render_pass.lights.buffer,
            offset,
            cast_slice(&light.get_light_uniforms()),
        );

        self.virtual_render_pass.light_count += 1;
    }

    pub fn push_matrix(&mut self, matrix: Mat4) {
        let offset = self.virtual_render_pass.inistance_count * size_of::<Mat4>() as u64;
        self.virtual_gpu.queue.write_buffer(
            &self.virtual_render_pass.instance_buffer,
            offset,
            bytemuck::bytes_of(&matrix),
        );
        self.virtual_render_pass
            .commands
            .push(Command::SetModelMatrix);
        self.virtual_render_pass.inistance_count += 1;
    }

    pub fn draw_static_mesh(&mut self, index: usize) {
        self.virtual_render_pass
            .commands
            .push(Command::DrawStaticMesh(index))
    }

    pub fn draw_static_mesh_indexed(&mut self, index: usize) {
        self.virtual_render_pass
            .commands
            .push(Command::DrawStaticMeshIndexed(index))
    }

    pub fn draw_sprite(&mut self, index: usize) {
        self.virtual_render_pass
            .commands
            .push(Command::DrawSprite(index));
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

        let forward = self.virtual_render_pass.camera.get_forward();
        let right = forward.cross(Vec3A::Y);

        self.virtual_render_pass.camera.eye += forward * self.camera_delta.z * DT * CAMERA_SPEED;
        self.virtual_render_pass.camera.eye -= right * self.camera_delta.x * DT * CAMERA_SPEED;

        self.virtual_render_pass.camera.yaw -= self.camera_yaw_delta * DT * CAMERA_ROT_SPEED;

        self.push_matrix(Mat4::IDENTITY);
        self.set_texture(0);
    }
}
