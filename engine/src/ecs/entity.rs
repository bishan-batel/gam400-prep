use std::{
    any::TypeId,
    cell::{Ref, RefCell, RefMut},
    ops::Deref,
    sync::atomic::{AtomicU32, Ordering},
};

use hashbrown::HashMap;

use crate::ecs::component::{Bundle, Component};
use std::any::Any;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct EntityID(pub(crate) u32);

static GLOBAL_ID_COUNTER: AtomicU32 = AtomicU32::new(0);

impl EntityID {
    pub(crate) fn unique() -> Self {
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
    components: HashMap<TypeId, Box<RefCell<dyn Component>>>,
}

impl Entity {
    pub fn empty() -> Self {
        Self {
            components: Default::default(),
        }
    }

    /// adds a component
    pub fn add<T: Component>(&mut self, component: T) -> &mut Self {
        self.add_dyn(Box::new(RefCell::<T>::new(component)))
    }

    /// adds a dynamically typed component
    pub fn add_dyn(&mut self, component: Box<RefCell<dyn Component>>) -> &mut Self {
        let id = component.type_id();
        self.components.insert(id, component);
        self
    }

    pub fn num_components(&self) -> usize {
        self.components.len()
    }

    pub fn component<'a, T: Component>(&'a self) -> Option<Ref<'a, T>> {
        let borrowed = self.raw_component::<T>()?.borrow();

        Ref::filter_map(borrowed, |b| (b as &dyn Any).downcast_ref::<T>()).ok()
    }

    pub fn component_mut<'a, T: Component>(&'a self) -> Option<RefMut<'a, T>> {
        let borrowed = self.raw_component::<T>()?.borrow_mut();

        RefMut::filter_map(borrowed, |b| (b as &mut dyn Any).downcast_mut::<T>()).ok()
    }

    fn raw_component<T: Component>(&self) -> Option<&RefCell<dyn Component>> {
        self.components.get(&TypeId::of::<T>()).map(|b| b.deref())
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
