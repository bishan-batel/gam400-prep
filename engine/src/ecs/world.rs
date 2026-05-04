use std::cell::{Ref, RefMut};

use hashbrown::HashMap;

use crate::ecs::{
    component::{Component, ComponentList},
    entity::{Entity, EntityID},
};

#[derive(Default)]
pub struct World {
    entities: HashMap<EntityID, Entity>,
}

impl World {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn add(&mut self, components: impl ComponentList) {
        self.entities
            .insert(unsafe { EntityID::unique() }, components.into());
    }

    pub fn query_component<'a, T: Component>(&'a self) -> impl Iterator<Item = Ref<'a, T>> {
        self.entities
            .iter()
            .map(|(_, entity)| entity)
            .filter_map(|entity| entity.component::<T>())
    }

    pub fn query_component_mut<'a, T: Component>(
        &'a mut self,
    ) -> impl Iterator<Item = RefMut<'a, T>> {
        self.entities
            .iter()
            .map(|(_, entity)| entity)
            .filter_map(|entity| entity.component_mut::<T>())
    }
}
