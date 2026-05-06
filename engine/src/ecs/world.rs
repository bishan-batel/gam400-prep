use std::cell::{Ref, RefMut};

use hashbrown::HashMap;

use crate::ecs::{
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

    /// Construct and add a new entity
    pub fn spawn<B: Bundle>(&mut self, components: B) -> EntityID {
        self.add(Entity::from(components))
    }

    pub fn add(&mut self, entity: Entity) -> EntityID {
        let id = EntityID::unique();
        self.entities.insert(id, entity);
        id
    }

    /// Remove the entity with the given ID
    pub fn remove(&mut self, id: EntityID) -> Option<Entity> {
        self.entities.remove(&id)
    }

    pub fn get(&self, id: EntityID) -> Option<&Entity> {
        self.entities.get(&id)
    }

    pub fn get_mut(&mut self, id: EntityID) -> Option<&mut Entity> {
        self.entities.get_mut(&id)
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

    pub fn total_components(&self) -> usize {
        self.entities.values().map(|e| e.num_components()).sum()
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
