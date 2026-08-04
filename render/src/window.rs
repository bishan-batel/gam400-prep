use std::{ops::Deref, sync::Arc};

use bevy_ecs::resource::Resource;
use glam::{UVec2, uvec2};
use wgpu::{Adapter, Surface, SurfaceConfiguration, TextureFormat};

use crate::renderer::{adapter::RenderAdapter, device::RenderDevice};

#[derive(Debug, Resource)]
pub struct Window {
    device: RenderDevice,
    size: UVec2,
    window: Arc<winit::window::Window>,
    surface: Surface<'static>,
    config: SurfaceConfiguration,
    is_surface_configured: bool,
    swapchain_format: TextureFormat,
}

impl Window {
    pub fn new(
        device: RenderDevice,
        adapter: RenderAdapter,
        surface: Surface<'static>,
        config: SurfaceConfiguration,
        window: Arc<winit::window::Window>,
    ) -> eyre::Result<Self> {
        // initial window size
        let size = uvec2(config.width, config.height);

        let swapchain_capabilities = surface.get_capabilities(&adapter);

        let Some(swapchain_format) = swapchain_capabilities.formats.first().cloned() else {
            eyre::bail!("Swapchain / Window Surface is incompatable with this device's adapter");
        };

        Ok(Self {
            size,
            device,
            window,
            is_surface_configured: false,
            config,
            surface,
            swapchain_format,
        })
    }

    pub fn window(&self) -> &Arc<winit::window::Window> {
        &self.window
    }

    pub fn size(&self) -> UVec2 {
        self.size
    }

    pub fn surface(&self) -> &Surface<'_> {
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

impl Deref for Window {
    type Target = winit::window::Window;

    fn deref(&self) -> &Self::Target {
        &self.window
    }
}
