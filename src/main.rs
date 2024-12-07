use pollster::block_on;
use winit::event_loop::EventLoop;

mod app;
mod camera;
mod game;
mod vertex;

use app::StateApplication;

fn main() {
    env_logger::init();

    block_on(run());
}

pub async fn run() {
    let event_loop = EventLoop::new().unwrap();

    let mut window_state = StateApplication::new();
    let _ = event_loop.run_app(&mut window_state);
}
