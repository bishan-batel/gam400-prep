use std::{
    any::TypeId,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicU32, Ordering},
};

use hashbrown::HashMap;

use crate::{
    ecs::component::{self, Bundle, Component},
    prelude::ComponentUIDKeyable,
};
use std::any::Any;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct EntityID(u32);

static GLOBAL_ID_COUNTER: AtomicU32 = AtomicU32::new(0);

impl EntityID {
    /// # Safety
    pub unsafe fn from_unchecked(id: u32) -> Self {
        Self(id)
    }

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
        self.components.insert(component.uid(), Box::new(component));
        self
    }

    /// adds a dynamically typed component
    pub fn add_dyn(&mut self, component: Box<dyn Component>) -> &mut Self {
        self.components.insert(component.deref().uid(), component);
        self
    }

    pub fn remove<C: Component>(&mut self) -> Option<Box<C>> {
        let component: Box<dyn Any> = self.remove_by_uid(Self::uid::<C>())?;
        component.downcast().ok()
    }

    pub fn remove_by_uid(&mut self, uid: TypeId) -> Option<Box<dyn Component>> {
        self.components.remove(&uid)
    }

    #[must_use]
    pub fn num_components(&self) -> usize {
        self.components.len()
    }

    pub fn has_component<C: Component>(&self) -> bool {
        self.components.contains_key(&Self::uid::<C>())
    }

    pub(crate) fn uid<C: ComponentUIDKeyable>() -> TypeId {
        C::uid()
    }

    pub fn component<C: Component>(&self) -> Option<&C> {
        self.components
            .get(&Self::uid::<C>())
            .and_then(|c| (c.deref() as &dyn Any).downcast_ref::<C>())
    }

    pub fn component_mut<C: Component>(&mut self) -> Option<&mut C> {
        self.components
            .get_mut(&Self::uid::<C>())
            .and_then(|c| (c.deref_mut() as &mut dyn Any).downcast_mut::<C>())
    }
}

impl<T> From<T> for Entity
where
    T: Bundle,
{
    fn from(value: T) -> Self {
        let mut entity = Entity::empty();
        value.insert_into(&mut entity);
        entity
    }
}
