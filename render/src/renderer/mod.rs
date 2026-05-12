use std::sync::Arc;

use bevy_ecs::resource::Resource;

use crate::{
    renderer::{device::RenderDevice, instance::RenderInstance},
    window::Window,
};

pub mod device;
pub mod instance;

#[derive(Resource, Clone)]
pub struct RenderState {
    window: Arc<Window>,
}

impl RenderState {
    pub async fn new(window: Arc<winit::window::Window>) -> eyre::Result<Self> {
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

        // get default configuration
        let Some(config) = surface.get_default_config(&adapter, 800, 800) else {
            eyre::bail!("Surface is not supported by the given adapter");
        };

        surface.configure(&device, &config);

        let window = Window::new(device.clone(), &adapter, surface, config, window)?.into();

        Ok(Self { window })
    }

    pub fn window(&self) -> &Arc<Window> {
        &self.window
    }
}
