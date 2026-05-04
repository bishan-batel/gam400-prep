use std::{any::Any, fmt::Debug};

use crate::ecs::entity::Entity;

/// A struct that can be attached to any entity
pub trait Component: Debug + Any {}

pub trait ComponentList {
    fn insert(self, entity: &mut Entity);
}

impl<T: Component> ComponentList for T {
    fn insert(self, entity: &mut Entity) {
        entity.add(self);
    }
}

impl<T1: Component> ComponentList for (T1,) {
    fn insert(self, entity: &mut Entity) {
        entity.add(self.0);
    }
}

impl<T1, T2> ComponentList for (T1, T2)
where
    T1: Component,
    T2: Component,
{
    fn insert(self, entity: &mut Entity) {
        entity.add(self.0).add(self.1);
    }
}

impl<T1, T2, T3> ComponentList for (T1, T2, T3)
where
    T1: Component,
    T2: Component,
    T3: Component,
{
    fn insert(self, entity: &mut Entity) {
        entity.add(self.0).add(self.1).add(self.2);
    }
}

impl<T1, T2, T3, T4> ComponentList for (T1, T2, T3, T4)
where
    T1: Component,
    T2: Component,
    T3: Component,
    T4: Component,
{
    fn insert(self, entity: &mut Entity) {
        entity.add(self.0).add(self.1).add(self.2).add(self.3);
    }
}

impl<T1, T2, T3, T4, T5> ComponentList for (T1, T2, T3, T4, T5)
where
    T1: Component,
    T2: Component,
    T3: Component,
    T4: Component,
    T5: Component,
{
    fn insert(self, entity: &mut Entity) {
        entity
            .add(self.0)
            .add(self.1)
            .add(self.2)
            .add(self.3)
            .add(self.4);
    }
}
