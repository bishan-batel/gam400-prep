use std::{ops::Deref, sync::Arc};

use wgpu::Device;

#[derive(Debug, Clone)]
pub struct RenderDevice(Arc<Device>);

impl Deref for RenderDevice {
    type Target = Device;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl From<Device> for RenderDevice {
    fn from(instance: Device) -> Self {
        Arc::new(instance).into()
    }
}

impl From<Arc<Device>> for RenderDevice {
    fn from(instance: Arc<Device>) -> Self {
        Self(instance)
    }
}
