use std::{
    any::TypeId,
    cell::{Ref, RefCell, RefMut},
    ops::Deref,
    sync::atomic::{AtomicU32, AtomicUsize, Ordering},
};

use hashbrown::HashMap;

use crate::ecs::component::{Component, ComponentList};
use std::any::Any;

#[derive(displaydoc::Display, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[displaydoc("{0}")]
pub struct EntityID(pub(crate) u32);

static GLOBAL_ID_COUNTER: AtomicU32 = AtomicU32::new(0);

impl EntityID {
    pub unsafe fn unique() -> Self {
        EntityID(GLOBAL_ID_COUNTER.fetch_add(1, Ordering::Relaxed))
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
        let id = component.type_id();
        let component = Box::new(RefCell::<T>::new(component));
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

    fn raw_component<T: Component>(&self) -> Option<&Box<RefCell<dyn Component>>> {
        self.components.get(&TypeId::of::<T>())
    }
}

impl<T> From<T> for Entity
where
    T: ComponentList,
{
    fn from(value: T) -> Self {
        let mut entity = Entity::empty();
        value.insert(&mut entity);
        entity
    }
}
