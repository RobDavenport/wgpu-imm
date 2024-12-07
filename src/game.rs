use crate::{app::State, vertex::Vertex};

pub struct Game {
    t: f32,
}

impl Game {
    pub fn new() -> Self {
        Self { t: 0.0 }
    }

    pub fn update(&mut self) {
        self.t += 1.0 / 60.0;
    }

    pub fn draw(&self, state: &mut State) {
        let sin = self.t.sin();
        let cos = self.t.cos();
        let data = [
            // X Y Z, R G B
            1.0,
            1.0,
            0.0,
            1.0 + sin * 0.1,
            0.0 + cos * 0.2,
            0.0 + sin * 0.5, // V1
            -1.0,
            1.0,
            0.0,
            0.0 + cos * 0.3,
            1.0 + sin * 0.25,
            0.0 + cos * 0.2, // V2
            -1.0,
            -1.0,
            0.0,
            0.0 + sin * 0.2,
            0.0 + cos * 0.1,
            1.0 + sin * 0.3, // V3
        ];
        state.draw_tri_list(&data);
    }
}
