use std::sync::Arc;
use std::time::{Duration, Instant};

use glam::{Mat4, Vec3A};
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

use crate::contexts::Draw3dContext;
use crate::game::Game;
use crate::resolution::Resolution;
use crate::virtual_gpu::VirtualGpu;
use crate::wgpu_setup;

pub struct StateApplication {
    state: Option<State>,
    game: Game,
    last_frame: Instant,
    frame_time: Duration,
}

impl StateApplication {
    pub fn new(fps: f32) -> Self {
        Self {
            state: None,
            game: Game::new(),
            last_frame: Instant::now(),
            frame_time: Duration::from_secs_f32(1.0 / fps),
        }
    }
}

impl ApplicationHandler for StateApplication {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let resolution = Resolution::Full;
        let (width, height) = resolution.dimensions();

        let inner_size = PhysicalSize { width, height };

        let window = event_loop
            .create_window(
                Window::default_attributes()
                    .with_title("wgpu-imm")
                    .with_inner_size(inner_size)
                    .with_min_inner_size(inner_size),
            )
            .unwrap();

        let mut state = State::new(window);

        self.game.init(&mut state.virtual_gpu);

        self.state = Some(state);
        self.last_frame = Instant::now();
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
                    let now = Instant::now();

                    let diff = now.duration_since(self.last_frame);
                    if diff >= self.frame_time {
                        self.last_frame = now;
                        self.state.as_mut().unwrap().update();
                        self.game.update();
                        self.game
                            .draw(&mut self.state.as_mut().unwrap().virtual_gpu);
                        self.state.as_mut().unwrap().render().unwrap();
                    }
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
                            PhysicalKey::Code(KeyCode::KeyR) => state.camera_delta.y += 1.0,
                            PhysicalKey::Code(KeyCode::KeyF) => state.camera_delta.y -= 1.0,
                            PhysicalKey::Code(KeyCode::KeyA) => state.camera_yaw_delta = 1.0,
                            PhysicalKey::Code(KeyCode::KeyD) => state.camera_yaw_delta = -1.0,
                            PhysicalKey::Code(KeyCode::KeyO) => {
                                state
                                    .virtual_gpu
                                    .environment_map
                                    .uniforms
                                    .environment_color_strength
                                    .w += 0.1
                            }
                            PhysicalKey::Code(KeyCode::KeyL) => {
                                state
                                    .virtual_gpu
                                    .environment_map
                                    .uniforms
                                    .environment_color_strength
                                    .w -= 0.1
                            }
                            _ => {}
                        },
                        false => {
                            match event.physical_key {
                                PhysicalKey::Code(KeyCode::KeyW)
                                | PhysicalKey::Code(KeyCode::KeyS) => state.camera_delta.z = 0.0,
                                PhysicalKey::Code(KeyCode::KeyQ)
                                | PhysicalKey::Code(KeyCode::KeyE) => state.camera_delta.x = 0.0,
                                PhysicalKey::Code(KeyCode::KeyR)
                                | PhysicalKey::Code(KeyCode::KeyF) => state.camera_delta.y = 0.0,
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
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,

    window: Arc<Window>,

    camera_delta: Vec3A,
    camera_yaw_delta: f32,

    pub virtual_gpu: VirtualGpu,
}

impl State {
    pub fn new(window: Window) -> Self {
        let window_arc = Arc::new(window);
        let size = window_arc.inner_size();
        let instance = wgpu_setup::create_gpu_instance();
        let surface = instance.create_surface(window_arc.clone()).unwrap();
        let adapter = wgpu_setup::create_adapter(instance, &surface);
        let (device, queue) = wgpu_setup::create_device(&adapter);
        let surface_caps = surface.get_capabilities(&adapter);
        let config = wgpu_setup::create_surface_config(size, surface_caps);
        surface.configure(&device, &config);

        let virtual_gpu = VirtualGpu::new(device, queue, &config);

        Self {
            surface,
            config,
            window: window_arc,
            camera_delta: Vec3A::ZERO,
            camera_yaw_delta: 0.0,

            virtual_gpu,
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.config.width = new_size.width;
        self.config.height = new_size.height;

        self.virtual_gpu
            .frame_buffer
            .adjust_scale(new_size.width, new_size.height);

        self.surface
            .configure(&self.virtual_gpu.device, &self.config);
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture().unwrap();
        let surface_view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        self.virtual_gpu.render(&surface_view);
        output.present();

        Ok(())
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    fn update(&mut self) {
        const DT: f32 = 1.0 / 60.0;
        const CAMERA_SPEED: f32 = 2.5;
        const CAMERA_ROT_SPEED: f32 = 0.75;

        let forward = self.virtual_gpu.camera.get_forward();
        let right = forward.cross(Vec3A::Y);
        let up = Vec3A::Y;

        self.virtual_gpu.camera.eye += forward * self.camera_delta.z * DT * CAMERA_SPEED;
        self.virtual_gpu.camera.eye -= right * self.camera_delta.x * DT * CAMERA_SPEED;
        self.virtual_gpu.camera.eye += up * self.camera_delta.y * DT * CAMERA_SPEED;

        self.virtual_gpu.camera.yaw -= self.camera_yaw_delta * DT * CAMERA_ROT_SPEED;

        self.virtual_gpu.push_matrix(Mat4::IDENTITY);
        self.virtual_gpu.set_texture(0);
    }
}
