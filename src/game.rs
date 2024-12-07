use crate::{app::State, vertex::Vertex};


pub struct Game {

}

impl Game {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw(&self, state: &mut State) {
        let data = [
            1.0, 1.0, 0.0, 1.0, 0.0, 0.0,
            -1.0, 1.0, 0.0, 0.0, 1.0, 0.0,
            -1.0, -1.0, 0.0, 0.0, 0.0, 1.0
        ];
        state.draw_tri_list(&data);
    }
}