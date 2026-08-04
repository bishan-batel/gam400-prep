use std::{ops::Deref, sync::Arc};

use bevy_ecs::resource::Resource;

#[derive(Resource, Debug, Clone)]
pub struct RenderInstance(Arc<wgpu::Instance>);

impl Deref for RenderInstance {
    type Target = wgpu::Instance;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl From<wgpu::Instance> for RenderInstance {
    fn from(instance: wgpu::Instance) -> Self {
        Arc::new(instance).into()
    }
}

impl From<Arc<wgpu::Instance>> for RenderInstance {
    fn from(instance: Arc<wgpu::Instance>) -> Self {
        Self(instance)
    }
}
