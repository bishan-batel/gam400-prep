use std::{ops::Deref, sync::Arc};

use bevy_ecs::resource::Resource;
use wgpu::Adapter;

#[derive(Debug, Resource, Clone)]
pub struct RenderAdapter(Arc<Adapter>);

impl Deref for RenderAdapter {
    type Target = Adapter;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl From<Adapter> for RenderAdapter {
    fn from(instance: Adapter) -> Self {
        Arc::new(instance).into()
    }
}

impl From<Arc<Adapter>> for RenderAdapter {
    fn from(instance: Arc<Adapter>) -> Self {
        Self(instance)
    }
}
