use std::{
    any::{Any, TypeId},
    fmt::Debug,
};

use crate::ecs::entity::Entity;

mod common;
pub use common::*;

/// A struct that can be attached to any entity
pub trait Component: Debug + Any {
    fn uid(&self) -> TypeId {
        TypeId::of::<Self>()
    }
}

pub(crate) trait ComponentUIDKeyable {
    fn uid() -> TypeId;
}

impl<C: Component> ComponentUIDKeyable for C {
    fn uid() -> TypeId {
        TypeId::of::<C>()
    }
}

/// A bundle of components that can make an entity
pub trait Bundle: Debug {
    fn insert_into(self, entity: &mut Entity);
}

impl<T: Component> Bundle for T {
    fn insert_into(self, entity: &mut Entity) {
        entity.add(self);
    }
}

/// A unit bundle just leads to an empty entity
impl Bundle for () {
    fn insert_into(self, _entity: &mut Entity) {}
}

impl<T1: Component> Bundle for (T1,) {
    fn insert_into(self, entity: &mut Entity) {
        entity.add(self.0);
    }
}

impl<T1, T2> Bundle for (T1, T2)
where
    T1: Component,
    T2: Component,
{
    fn insert_into(self, entity: &mut Entity) {
        entity.add(self.0).add(self.1);
    }
}

impl<T1, T2, T3> Bundle for (T1, T2, T3)
where
    T1: Component,
    T2: Component,
    T3: Component,
{
    fn insert_into(self, entity: &mut Entity) {
        entity.add(self.0).add(self.1).add(self.2);
    }
}

impl<T1, T2, T3, T4> Bundle for (T1, T2, T3, T4)
where
    T1: Component,
    T2: Component,
    T3: Component,
    T4: Component,
{
    fn insert_into(self, entity: &mut Entity) {
        entity.add(self.0).add(self.1).add(self.2).add(self.3);
    }
}

impl<T1, T2, T3, T4, T5> Bundle for (T1, T2, T3, T4, T5)
where
    T1: Component,
    T2: Component,
    T3: Component,
    T4: Component,
    T5: Component,
{
    fn insert_into(self, entity: &mut Entity) {
        entity
            .add(self.0)
            .add(self.1)
            .add(self.2)
            .add(self.3)
            .add(self.4);
    }
}
