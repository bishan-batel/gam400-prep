use std::sync::Arc;

use crate::renderer::{
    adapter::RenderAdapter, device::RenderDevice, instance::RenderInstance, queue::RenderQueue,
};

use super::window::Window;

pub mod adapter;
pub mod device;
pub mod instance;
pub mod queue;

pub(crate) async fn setup_init_render_resources(
    window: Arc<winit::window::Window>,
) -> eyre::Result<(
    RenderDevice,
    RenderQueue,
    RenderAdapter,
    RenderInstance,
    Window,
)> {
    let instance: RenderInstance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::PRIMARY,
        display: None,
        flags: Default::default(),
        memory_budget_thresholds: Default::default(),
        backend_options: Default::default(),
    })
    .into();

    // create the window surface
    let surface = instance.create_surface(window.clone())?;

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            // Request an adapter which can render to our surface
            compatible_surface: Some(&surface),
            apply_limit_buckets: false,
        })
        .await
        // hard crash if we can't render anything
        .expect("Failed to find an appropriate adapter");

    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor {
            label: None,

            required_features: wgpu::Features::POLYGON_MODE_LINE,

            required_limits: wgpu::Limits::default(),

            // value memory usage over raw performance
            memory_hints: wgpu::MemoryHints::MemoryUsage,
            experimental_features: wgpu::ExperimentalFeatures::default(),
            trace: wgpu::Trace::Off,
        })
        .await
        .expect("Failed to create device");

    let device = RenderDevice::from(device);
    let queue = RenderQueue::from(queue);
    let adapter = RenderAdapter::from(adapter);

    // get default configuration
    let Some(config) = surface.get_default_config(&adapter, 800, 800) else {
        eyre::bail!("Surface is not supported by the given adapter");
    };

    surface.configure(&device, &config);

    let window = Window::new(device.clone(), adapter.clone(), surface, config, window)?;

    Ok((device, queue, adapter, instance, window))
}
