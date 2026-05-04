use std::cell::{Ref, RefMut};

use hashbrown::HashMap;

use crate::ecs::{
    component::Component,
    entity::{Entity, EntityID},
};

#[derive(Debug)]
pub enum WorldCommands {
    Push {},
}

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

    pub fn add(&mut self, entity: impl Into<Entity>) {
        let entity = entity.into();
        let id = self.new_uid();

        self.entities.insert(id, entity);
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

    fn new_uid(&self) -> EntityID {
        let mut id = EntityID::from_index(self.entities.len());

        while self.entities.contains_key(&id) {
            id.0 = id.0.saturating_mul(11);
        }

        id
    }
}
