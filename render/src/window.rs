use std::sync::Arc;

use bevy_ecs::resource::Resource;
use eyre::bail;
use glam::{UVec2, uvec2};
use wgpu::SurfaceConfiguration;

use crate::renderer::{device::RenderDevice, instance::RenderInstance};

#[derive(Resource, Debug)]
pub struct Window {
    instance: RenderInstance,
    device: RenderDevice,
    size: UVec2,
    window: Arc<winit::window::Window>,
    surface: wgpu::Surface<'static>,
    config: SurfaceConfiguration,
    is_surface_configured: bool,
}

impl Window {
    pub fn new(
        instance: RenderInstance,
        device: RenderDevice,
        adapter: wgpu::Adapter,
        window: Arc<winit::window::Window>,
    ) -> eyre::Result<Self> {
        // initial window size
        let size = window.inner_size();
        let size = uvec2(size.width, size.height);

        // create the window surface
        let surface = instance.create_surface(window.clone())?;

        // get default configuration
        let Some(config) = surface.get_default_config(&adapter, size.x, size.y) else {
            eyre::bail!("Surface is not supported by the given adapter");
        };

        surface.configure(&device, &config);

        Ok(Self {
            instance,
            device,
            size,
            window,
            is_surface_configured: false,
            config,
            surface,
        })
    }

    pub fn window(&self) -> &Arc<winit::window::Window> {
        &self.window
    }

    pub fn size(&self) -> UVec2 {
        self.size
    }

    pub fn surface(&self) -> &wgpu::Surface<'_> {
        &self.surface
    }

    pub fn is_surface_ready(&self) -> bool {
        self.is_surface_configured
    }

    pub fn resize(&mut self, size: UVec2) {
        if size == UVec2::ZERO {
            return;
        }

        self.config.width = size.x;
        self.config.height = size.y;
        self.surface.configure(&self.device, &self.config);
        self.is_surface_configured = true;
    }
}
