use pollster::block_on;
use winit::event_loop::EventLoop;

mod app;
mod camera;
mod contexts;
mod environment_map;
mod frame_buffer;
mod game;
mod immediate_renderer;
mod importer;
mod lights;
mod mesh;
mod pipeline;
mod preloaded_renderer;
mod quad_renderer;
mod resolution;
mod spec_tex;
mod textures;
mod vertex;
mod virtual_gpu;
mod virtual_render_pass;
mod wgpu_setup;

use app::StateApplication;

pub const PUSH_CONSTANT_SIZE: u32 = 128;

fn main() {
    env_logger::init();

    //spec_tex::generate_texture();

    block_on(run());
}

pub async fn run() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

    let mut window_state = StateApplication::new(120.0);
    let _ = event_loop.run_app(&mut window_state);
}
