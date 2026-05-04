use std::{
    any::TypeId,
    cell::{Ref, RefCell, RefMut},
    ops::Deref,
};

use hashbrown::HashMap;

use crate::ecs::component::Component;
use std::any::Any;

#[derive(displaydoc::Display, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[displaydoc("{0}")]
pub struct EntityID(pub(crate) u32);

impl EntityID {
    #[must_use]
    pub fn from_index(idx: usize) -> Self {
        Self(idx as u32)
    }

    #[must_use]
    pub fn index(&self) -> usize {
        self.0 as usize
    }
}

pub struct Entity {
    components: HashMap<TypeId, Box<RefCell<dyn Component>>>,
}

impl Entity {
    fn empty() -> Self {
        Self {
            components: Default::default(),
        }
    }

    pub fn add<T: Component>(mut self, component: T) -> Self {
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

// impl<T: Component> From<T> for Entity {
//     fn from(value: T) -> Self {
//         Self::empty().add(value)
//     }
// }

impl<T1: Component> From<(T1,)> for Entity {
    fn from(value: (T1,)) -> Self {
        Self::empty().add(value.0)
    }
}

impl<T1, T2> From<(T1, T2)> for Entity
where
    T1: Component,
    T2: Component,
{
    fn from(value: (T1, T2)) -> Self {
        Self::empty().add(value.0).add(value.1)
    }
}

impl<T1, T2, T3> From<(T1, T2, T3)> for Entity
where
    T1: Component,
    T2: Component,
    T3: Component,
{
    fn from(value: (T1, T2, T3)) -> Self {
        Self::empty().add(value.0).add(value.1).add(value.2)
    }
}

impl<T1, T2, T3, T4> From<(T1, T2, T3, T4)> for Entity
where
    T1: Component,
    T2: Component,
    T3: Component,
    T4: Component,
{
    fn from(value: (T1, T2, T3, T4)) -> Self {
        Self::empty()
            .add(value.0)
            .add(value.1)
            .add(value.2)
            .add(value.3)
    }
}
