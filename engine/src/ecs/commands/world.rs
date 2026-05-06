use crate::{
    ecs::{commands::Command, entity::EntityID, world::World},
    prelude::{Bundle, Component},
};

/// Removes the entity with the given ID from the world
pub struct Remove(pub EntityID);

impl Command for Remove {
    fn apply(self, world: &mut World) {
        world.remove(self.0);
    }
}

/// Adds an empty entity to the world
pub struct SpawnEmpty(pub EntityID);

impl Command for SpawnEmpty {
    fn apply(self, world: &mut World) {
        world.spawn_with_id(().into(), self.0);
    }
}

/// Adds an empty entity to the world
pub struct Spawn<B: Bundle> {
    pub bundle: B,
    pub id: EntityID,
}

impl<B: Bundle> Command for Spawn<B> {
    fn apply(self, world: &mut World) {
        world.spawn_with_id(self.bundle.into(), self.id);
    }
}

pub struct InsertComponent<C: Component> {
    id: EntityID,
    component: C,
}

impl<C: Component> Command for InsertComponent<C> {
    fn apply(self, world: &mut World) {
        if let Some(entity) = world.get_mut(self.id) {
            entity.add(self.component);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn spawn_empty() {
        let mut world = World::new();

        let e1 = EntityID::unique();

        assert_eq!(world.total_components(), 0);
        assert_eq!(world.len(), 0);

        world.exec(SpawnEmpty(e1));

        assert_eq!(world.total_components(), 0);
        assert_eq!(world.len(), 1);
    }

    #[test]
    fn remove() {
        let mut world = World::new();

        let e1 = EntityID::unique();

        world.exec(SpawnEmpty(e1));

        assert_eq!(world.len(), 1);

        world.exec(Remove(e1));

        assert!(world.is_empty());
    }

    #[test]
    fn spawn_with_bundle_empty() {
        let mut world = World::new();

        let e1 = EntityID::unique();
        // empty entity
        world.exec(Spawn { bundle: (), id: e1 });

        assert_eq!(world.total_components(), 0);
        assert_eq!(world.len(), 1);
    }

    #[test]
    fn spawn_with_bundle() {
        let mut world = World::new();

        let e1 = EntityID::unique();
        // empty entity
        world.exec(Spawn {
            bundle: Name::from("Hello"),
            id: e1,
        });

        assert_eq!(world.total_components(), 1);

        // duplicate test
        world.exec(Spawn {
            bundle: Name::from("Hello2"),
            id: e1,
        });

        assert_eq!(world.total_components(), 1);

        // should not fail
        assert_eq!(
            world.get(e1).unwrap().component::<Name>().unwrap().0,
            "Hello2"
        );

        let e2 = EntityID::unique();
        world.exec(Spawn {
            bundle: Name::from("Hello3"),
            id: e2,
        });

        assert_eq!(
            world.get(e2).unwrap().component::<Name>().unwrap().0,
            "Hello3"
        );
        assert_eq!(world.len(), 2);
        assert_eq!(world.total_components(), 2);
    }
}
