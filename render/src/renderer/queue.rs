use std::{ops::Deref, sync::Arc};

use bevy_ecs::resource::Resource;
use wgpu::Queue;

#[derive(Debug, Resource, Clone)]
pub struct RenderQueue(Arc<Queue>);

impl Deref for RenderQueue {
    type Target = Queue;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl From<Queue> for RenderQueue {
    fn from(instance: Queue) -> Self {
        Arc::new(instance).into()
    }
}

impl From<Arc<Queue>> for RenderQueue {
    fn from(instance: Arc<Queue>) -> Self {
        Self(instance)
    }
}
