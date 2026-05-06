use std::cell::{Ref, RefMut};

use hashbrown::HashMap;

use crate::ecs::{
    commands::Command,
    component::{Bundle, Component},
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

    pub fn clear(&mut self) {
        self.entities.clear();
        EntityID::reset();
    }

    /// Construct and add a new entity from a bundle
    pub fn spawn<B: Bundle>(&mut self, components: B) -> EntityID {
        let id = EntityID::unique();
        self.spawn_with_id(Entity::from(components), id);
        id
    }

    pub fn spawn_with_id(&mut self, entity: Entity, id: EntityID) {
        self.entities.insert(id, entity);
    }

    /// Remove the entity with the given ID
    pub fn remove(&mut self, id: EntityID) -> Option<Entity> {
        self.entities.remove(&id)
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.entities.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }

    pub fn get(&self, id: EntityID) -> Option<&Entity> {
        self.entities.get(&id)
    }

    pub fn get_mut(&mut self, id: EntityID) -> Option<&mut Entity> {
        self.entities.get_mut(&id)
    }

    pub fn query_component<T: Component>(&self) -> impl Iterator<Item = &(T)> {
        self.entities
            .iter()
            .filter_map(|(_, entity)| entity.component::<T>())
    }

    pub fn query_component_mut<T: Component>(&mut self) -> impl Iterator<Item = &mut T> {
        self.entities
            .iter_mut()
            .filter_map(|(_, entity)| entity.component_mut::<T>())
    }

    pub fn total_components(&self) -> usize {
        self.entities.values().map(|e| e.num_components()).sum()
    }

    /// Executes a command on this world
    pub fn exec<C: Command>(&mut self, command: C) {
        command.apply(self);
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;

    #[derive(Debug, Component)]
    struct Health(usize);

    #[derive(Debug, Component)]
    struct Name(String);

    #[test]
    fn two_entities() {
        let mut world = World::default();

        let e1 = world.spawn(Health(0));

        assert_eq!(world.entities.len(), 1);
        assert_eq!(world.total_components(), 1);

        let e2 = world.spawn((Health(42), Name("hi".into())));

        assert_eq!(world.entities.len(), 2);
        assert_eq!(world.total_components(), 3);

        for mut health in world.query_component_mut::<Health>() {
            health.0 += 1;
        }

        for name in world.query_component::<Name>() {
            assert_eq!(name.0, "hi");
        }

        {
            let health_iter: Vec<_> = world.query_component::<Health>().collect();

            assert_eq!(health_iter[0].0 + health_iter[1].0, 1 + 1 + 42);
        }

        world.remove(e2).unwrap();
        assert_eq!(world.entities.len(), 1);
        assert_eq!(world.total_components(), 1);

        world.remove(e1).unwrap();
        assert_eq!(world.entities.len(), 0);
        assert_eq!(world.total_components(), 0);
    }
}
