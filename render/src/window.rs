use std::{ops::Deref, sync::Arc};

use bevy_ecs::resource::Resource;
use glam::{UVec2, uvec2};
use wgpu::{Adapter, Queue, Surface, SurfaceConfiguration, TextureFormat};

use crate::renderer::{adapter::RenderAdapter, device::RenderDevice, queue::RenderQueue};

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

    pub fn render(&mut self, queue: &RenderQueue) {
        if !self.is_surface_ready() {
            let size = self.window.inner_size();
            self.resize(UVec2::new(size.width, size.height));
            log::info!("Resizing Window (Init)");
            return;
        }

        let output = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(surface_texture) => surface_texture,
            wgpu::CurrentSurfaceTexture::Suboptimal(surface_texture) => surface_texture,
            wgpu::CurrentSurfaceTexture::Timeout
            | wgpu::CurrentSurfaceTexture::Occluded
            | wgpu::CurrentSurfaceTexture::Validation => {
                // Skip this frame
                return;
            }
            wgpu::CurrentSurfaceTexture::Outdated => {
                self.surface.configure(&self.device, &self.config);
                return;
            }
            wgpu::CurrentSurfaceTexture::Lost => {
                panic!("Lost device");
            }
        };

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
                multiview_mask: None,
            });
        }

        queue.submit(std::iter::once(encoder.finish()));
        queue.present(output);
    }
}

impl Deref for Window {
    type Target = winit::window::Window;

    fn deref(&self) -> &Self::Target {
        &self.window
    }
}
