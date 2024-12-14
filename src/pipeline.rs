use crate::vertex;

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
    pub fn name(&self) -> &'static str {
        match self {
            Pipeline::Color => "color",
            Pipeline::Uv => "uv",
            Pipeline::ColorUv => "color uv",
            Pipeline::ColorLit => "color lit",
            Pipeline::UvLit => "uv lit",
            Pipeline::ColorUvLit => "color uv lit",
        }
    }

    pub fn vertex_shader(&self) -> &'static str {
        match self {
            Pipeline::Color => "vs_color",
            Pipeline::Uv => "vs_uv",
            Pipeline::ColorUv => "vs_color_uv",
            Pipeline::ColorLit => "vs_color_lit",
            Pipeline::UvLit => "vs_uv_lit",
            Pipeline::ColorUvLit => "vs_color_uv_lit",
        }
    }

    pub fn fragment_shader(&self) -> &'static str {
        match self {
            Pipeline::Color => "fs_color",
            Pipeline::Uv => "fs_uv",
            Pipeline::ColorUv => "fs_color_uv",
            Pipeline::ColorLit => "fs_color_lit",
            Pipeline::UvLit => "fs_uv_lit",
            Pipeline::ColorUvLit => "fs_color_uv_lit",
        }
    }

    pub fn get_pipeline_buffers(&self) -> [wgpu::VertexBufferLayout<'static>; 2] {
        [self.get_vertex_descriptor(), vertex::model_matrix()]
    }

    pub fn get_vertex_descriptor(&self) -> wgpu::VertexBufferLayout<'static> {
        match self {
            Pipeline::Color => vertex::color(),
            Pipeline::Uv => vertex::uv(),
            Pipeline::ColorUv => vertex::color_uv(),
            Pipeline::ColorLit => vertex::color_lit(),
            Pipeline::UvLit => vertex::uv_lit(),
            Pipeline::ColorUvLit => vertex::color_uv_lit(),
        }
    }

    pub fn has_color(&self) -> bool {
        match self {
            Pipeline::Color => true,
            Pipeline::Uv => false,
            Pipeline::ColorUv => true,
            Pipeline::ColorLit => true,
            Pipeline::UvLit => false,
            Pipeline::ColorUvLit => true,
        }
    }

    pub fn has_uv(&self) -> bool {
        match self {
            Pipeline::Color => false,
            Pipeline::Uv => true,
            Pipeline::ColorUv => true,
            Pipeline::ColorLit => false,
            Pipeline::UvLit => true,
            Pipeline::ColorUvLit => true,
        }
    }

    pub fn has_lighting(&self) -> bool {
        match self {
            Pipeline::Color => false,
            Pipeline::Uv => false,
            Pipeline::ColorUv => false,
            Pipeline::ColorLit => true,
            Pipeline::UvLit => true,
            Pipeline::ColorUvLit => true,
        }
    }

    pub fn lit(&self) -> Self {
        match self {
            Pipeline::Color => Pipeline::ColorLit,
            Pipeline::Uv => Pipeline::UvLit,
            Pipeline::ColorUv => Pipeline::ColorUvLit,
            Pipeline::ColorLit => Pipeline::ColorLit,
            Pipeline::UvLit => Pipeline::UvLit,
            Pipeline::ColorUvLit => Pipeline::ColorUvLit,
        }
    }

    pub fn get_shader(&self) -> usize {
        match self {
            Pipeline::Color => 0,
            Pipeline::Uv => 1,
            Pipeline::ColorUv => 2,
            Pipeline::ColorLit => 3,
            Pipeline::UvLit => 4,
            Pipeline::ColorUvLit => 5,
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
