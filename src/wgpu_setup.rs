use pollster::FutureExt;
use wgpu::{
    Adapter, Device, Instance, MemoryHints, PresentMode, Queue, Surface, SurfaceCapabilities,
};
use winit::dpi::PhysicalSize;

pub fn create_surface_config(
    size: PhysicalSize<u32>,
    capabilities: SurfaceCapabilities,
) -> wgpu::SurfaceConfiguration {
    let surface_format = capabilities
        .formats
        .iter()
        .find(|f| f.is_srgb())
        .copied()
        .unwrap_or(capabilities.formats[0]);

    wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: size.width,
        height: size.height,
        present_mode: PresentMode::AutoVsync,
        alpha_mode: capabilities.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    }
}

pub fn create_device(adapter: &Adapter) -> (Device, Queue) {
    adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                label: None,
                memory_hints: MemoryHints::default(),
            },
            None,
        )
        .block_on()
        .unwrap()
}

pub fn create_adapter(instance: Instance, surface: &Surface) -> Adapter {
    instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(surface),
            force_fallback_adapter: false,
        })
        .block_on()
        .unwrap()
}

pub fn create_gpu_instance() -> Instance {
    Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::PRIMARY,
        ..Default::default()
    })
}
