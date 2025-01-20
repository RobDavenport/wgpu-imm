use crate::vertex;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Pipeline {
    Color,
    Uv,
    ColorUv,
    ColorLit,
    UvLit,
    ColorUvLit,
    Quad2d,
    Matcap,
    MatcapColor,
    MatcapUv,
    MatcapColorUv,
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
            Pipeline::Quad2d => "quad 2d",
            Pipeline::Matcap => "matcap",
            Pipeline::MatcapColor => "matcap color",
            Pipeline::MatcapUv => "matcap uv",
            Pipeline::MatcapColorUv => "matcap color uv",
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
            Pipeline::Quad2d => "vs_quad_2d",
            Pipeline::Matcap => "vs_matcap",
            Pipeline::MatcapColor => "vs_matcap_color",
            Pipeline::MatcapUv => "vs_matcap_uv",
            Pipeline::MatcapColorUv => "vs_matcap_color_uv",
        }
    }

    pub fn fragment_shader(&self) -> &'static str {
        match self {
            Pipeline::Color => "fs_color",
            Pipeline::Uv | Pipeline::Quad2d => "fs_uv",
            Pipeline::ColorUv => "fs_color_uv",
            Pipeline::ColorLit => "fs_color_lit",
            Pipeline::UvLit => "fs_uv_lit",
            Pipeline::ColorUvLit => "fs_color_uv_lit",
            Pipeline::Matcap => "fs_matcap",
            Pipeline::MatcapColor => "fs_matcap_color",
            Pipeline::MatcapUv => "fs_matcap_uv",
            Pipeline::MatcapColorUv => "fs_matcap_color_uv",
        }
    }

    pub fn can_reduce(&self, into: Self) -> bool {
        let color = !into.has_color() || self.has_color();
        let uv = !into.has_uv() || self.has_uv();
        let lighting = !into.has_lighting() || self.has_lighting();

        color && uv && lighting
    }

    pub fn get_pipeline_buffers(&self) -> [wgpu::VertexBufferLayout<'static>; 2] {
        [self.get_vertex_buffer_layout(), vertex::model_matrix()]
    }

    pub fn get_vertex_buffer_layout(&self) -> wgpu::VertexBufferLayout<'static> {
        match self {
            Pipeline::Color => vertex::color(),
            Pipeline::Uv => vertex::uv(),
            Pipeline::ColorUv => vertex::color_uv(),
            Pipeline::ColorLit => vertex::color_lit(),
            Pipeline::UvLit => vertex::uv_lit(),
            Pipeline::ColorUvLit => vertex::color_uv_lit(),
            Pipeline::Quad2d => vertex::uv(),
            Pipeline::Matcap => vertex::matcap(),
            Pipeline::MatcapColor => vertex::matcap_color(),
            Pipeline::MatcapUv => vertex::matcap_uv(),
            Pipeline::MatcapColorUv => vertex::matcap_color_uv(),
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
            Pipeline::Quad2d => true,
            Pipeline::Matcap => false,
            Pipeline::MatcapColor => true,
            Pipeline::MatcapUv => false,
            Pipeline::MatcapColorUv => true,
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
            Pipeline::Quad2d => true,
            Pipeline::Matcap => false,
            Pipeline::MatcapColor => false,
            Pipeline::MatcapUv => true,
            Pipeline::MatcapColorUv => true,
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
            Pipeline::Quad2d => false,
            Pipeline::Matcap => false,
            Pipeline::MatcapColor => false,
            Pipeline::MatcapUv => false,
            Pipeline::MatcapColorUv => false,
        }
    }

    pub fn has_normals(&self) -> bool {
        match self {
            Pipeline::Color => false,
            Pipeline::Uv => false,
            Pipeline::ColorUv => false,
            Pipeline::ColorLit => true,
            Pipeline::UvLit => true,
            Pipeline::ColorUvLit => true,
            Pipeline::Quad2d => false,
            Pipeline::Matcap => true,
            Pipeline::MatcapColor => true,
            Pipeline::MatcapUv => true,
            Pipeline::MatcapColorUv => true,
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
            Pipeline::Quad2d => panic!("Quad2d can't be lit"),
            Pipeline::Matcap => panic!("Matcap can't be lit"),
            Pipeline::MatcapColor => panic!("Matcap Color can't be lit"),
            Pipeline::MatcapUv => panic!("Matcap Uv can't be lit"),
            Pipeline::MatcapColorUv => panic!("Matcap Color Uv can't be lit"),
        }
    }

    pub fn matcap(&self) -> Self {
        match self {
            Pipeline::Color => Pipeline::MatcapColor,
            Pipeline::Uv => Pipeline::MatcapUv,
            Pipeline::ColorUv => Pipeline::MatcapColorUv,
            Pipeline::ColorLit => panic!("Color Lit can't be a matcap."),
            Pipeline::UvLit => panic!("Uv Lit can't be a matcap."),
            Pipeline::ColorUvLit => panic!("Color Uv Lit can't be a matcap."),
            Pipeline::Quad2d => panic!("Quad2d can't be a matcap"),
            Pipeline::Matcap => *self,
            Pipeline::MatcapColor => *self,
            Pipeline::MatcapUv => *self,
            Pipeline::MatcapColorUv => *self,
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
            Pipeline::Quad2d => 6,
            Pipeline::Matcap => 7,
            Pipeline::MatcapColor => 8,
            Pipeline::MatcapUv => 9,
            Pipeline::MatcapColorUv => 10,
        }
    }

    pub fn get_attribute_count(&self) -> usize {
        3 + match self {
            Pipeline::Color => 3,
            Pipeline::Uv => 2,
            Pipeline::ColorUv | Pipeline::Quad2d => 5,
            Pipeline::ColorLit => 9,
            Pipeline::UvLit => 8,
            Pipeline::ColorUvLit => 11,
            Pipeline::Matcap => 3,
            Pipeline::MatcapColor => 6,
            Pipeline::MatcapUv => 5,
            Pipeline::MatcapColorUv => 8,
        }
    }

    pub fn get_vertex_size(&self) -> usize {
        self.get_attribute_count() * 4
    }
}
