use std::{
    any::TypeId,
    cell::{Ref, RefCell, RefMut},
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicU32, Ordering},
};

use hashbrown::HashMap;

use crate::{
    ecs::component::{Bundle, Component},
    prelude::ComponentUIDKeyable,
};
use std::any::Any;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct EntityID(pub(crate) u32);

static GLOBAL_ID_COUNTER: AtomicU32 = AtomicU32::new(0);

impl EntityID {
    pub fn unique() -> Self {
        EntityID(GLOBAL_ID_COUNTER.fetch_add(1, Ordering::Relaxed))
    }

    pub(crate) fn reset() {
        GLOBAL_ID_COUNTER.store(0, Ordering::Release);
    }

    #[must_use]
    pub fn index(&self) -> usize {
        self.0 as usize
    }
}

#[derive(Debug)]
pub struct Entity {
    components: HashMap<TypeId, Box<dyn Component>>,
}

impl Entity {
    pub fn empty() -> Self {
        Self {
            components: Default::default(),
        }
    }

    /// adds a component
    pub fn add<C: Component>(&mut self, component: C) -> &mut Self {
        self.components
            .insert(component.component_uid(), Box::new(component));
        self
    }

    /// adds a dynamically typed component
    pub fn add_dyn(&mut self, component: Box<dyn Component>) -> &mut Self {
        self.components
            .insert(component.deref().component_uid(), component);
        self
    }

    #[must_use]
    pub fn num_components(&self) -> usize {
        self.components.len()
    }

    pub fn component<C: Component>(&self) -> Option<&C> {
        self.components
            .get(&C::uid())
            .and_then(|c| (c.deref() as &dyn Any).downcast_ref::<C>())
    }

    pub fn component_mut<C: Component>(&mut self) -> Option<&mut C> {
        self.components
            .get_mut(&C::uid())
            .and_then(|c| (c.deref_mut() as &mut dyn Any).downcast_mut::<C>())
    }
}

impl<T> From<T> for Entity
where
    T: Bundle,
{
    fn from(value: T) -> Self {
        let mut entity = Entity::empty();
        value.insert(&mut entity);
        entity
    }
}
