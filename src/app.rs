use std::sync::Arc;

use glam::{Mat4, Vec3A};
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

use crate::contexts::Draw3dContext;
use crate::game::Game;
use crate::textures::{self};
use crate::virtual_gpu::VirtualGpu;
use crate::wgpu_setup;

pub struct StateApplication {
    state: Option<State>,
    game: Game,
}

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
                    self.game
                        .draw(&mut self.state.as_mut().unwrap().virtual_gpu);
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
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,

    size: PhysicalSize<u32>,
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
            size,
            window: window_arc,
            camera_delta: Vec3A::ZERO,
            camera_yaw_delta: 0.0,

            virtual_gpu,
        }
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

        self.virtual_gpu.render(view);
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

        self.virtual_gpu.camera.eye += forward * self.camera_delta.z * DT * CAMERA_SPEED;
        self.virtual_gpu.camera.eye -= right * self.camera_delta.x * DT * CAMERA_SPEED;

        self.virtual_gpu.camera.yaw -= self.camera_yaw_delta * DT * CAMERA_ROT_SPEED;

        self.virtual_gpu.push_matrix(Mat4::IDENTITY);
        self.virtual_gpu.set_texture(0);
    }
}
