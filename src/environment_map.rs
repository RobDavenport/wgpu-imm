use glam::Vec4;
use image::ImageReader;

pub struct EnvironmentMap {
    pub uniforms_buffer: wgpu::Buffer,
    pub uniforms: EnvironmentUniforms,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

pub struct EnvironmentUniforms {
    pub environment_color_strength: Vec4,
}

impl EnvironmentUniforms {
    pub fn new() -> Self {
        Self {
            environment_color_strength: Vec4::ONE,
        }
    }

    pub fn get_uniforms(&self) -> [f32; 4] {
        self.environment_color_strength.into()
    }
}

// Right
// Left
// Top
// Bottom
// Front
// Back

impl EnvironmentMap {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        const IMAGES: [&str; 6] = [
            "assets/skybox3/right.png",
            "assets/skybox3/left.png",
            "assets/skybox3/top.png",
            "assets/skybox3/bottom.png",
            "assets/skybox3/front.png",
            "assets/skybox3/back.png",
        ];

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Environment Map Texture"),
            size: wgpu::Extent3d {
                width: 128,
                height: 128,
                // A cube has 6 sides, so we need 6 layers
                depth_or_array_layers: 6,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some("Environment Map View"),
            dimension: Some(wgpu::TextureViewDimension::Cube),
            array_layer_count: Some(6),
            ..Default::default()
        });

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Environment Map Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let uniforms_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Environment Map Uniforms Buffer"),
            size: size_of::<EnvironmentUniforms>() as u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
            mapped_at_creation: false,
        });

        let uniforms = EnvironmentUniforms::new();

        for (index, path) in IMAGES.iter().enumerate() {
            let image = ImageReader::open(path).unwrap().decode().unwrap();
            let image = image.to_rgba8();
            let dimensions = image.dimensions();
            let size = wgpu::Extent3d {
                width: dimensions.0,
                height: dimensions.1,
                depth_or_array_layers: 1,
            };

            queue.write_texture(
                wgpu::ImageCopyTexture {
                    texture: &texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d {
                        x: 0,
                        y: 0,
                        z: index as u32,
                    },
                    aspect: wgpu::TextureAspect::All,
                },
                &image,
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(4 * dimensions.0),
                    rows_per_image: Some(dimensions.1),
                },
                size,
            );
        }

        Self {
            uniforms_buffer,
            uniforms,
            view,
            sampler,
        }
    }
}

