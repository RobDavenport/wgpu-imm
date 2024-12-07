#[derive(Clone, Copy, Debug)]
pub enum Pipeline {
    Color,
    Uv,
    ColorUv,
    ColorLit,
    UvLit,
    ColorUvLit,
}

impl Pipeline {
    pub fn get_shader(&self) -> usize {
        match self {
            Pipeline::Color => 0,
            Pipeline::Uv => 1,
            Pipeline::ColorUv => todo!(),
            Pipeline::ColorLit => todo!(),
            Pipeline::UvLit => todo!(),
            Pipeline::ColorUvLit => todo!(),
        }
    }

    pub fn get_attribute_count(&self) -> usize {
        3 + match self {
            Pipeline::Color => 3,
            Pipeline::Uv => 2,
            Pipeline::ColorUv => 5,
            Pipeline::ColorLit => 9,
            Pipeline::UvLit => 8,
            Pipeline::ColorUvLit => 11,
        }
    }

    pub fn get_vertex_size(&self) -> usize {
        self.get_attribute_count() * 4
    }
}
