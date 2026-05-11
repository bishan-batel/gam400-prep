use std::any::TypeId;

use crate::{
    commands::{Command, CommandQueue},
    entity::EntityID,
    prelude::{Bundle, Component, Entity},
    world::World,
};

/// Removes the entity with the given ID from the world
pub struct Remove(pub EntityID);

impl Command for Remove {
    fn apply(self, world: &mut World) {
        world.remove(self.0);
    }
}

/// Adds an empty entity to the world
pub struct Spawn<B: Bundle> {
    pub id: EntityID,
    pub bundle: B,
}

impl<B: Bundle> Command for Spawn<B> {
    fn apply(self, world: &mut World) {
        world.spawn_with_id(self.bundle.into(), self.id);
    }
}

pub struct InsertBundle<B: Bundle> {
    id: EntityID,
    bundle: B,
}

impl<B: Bundle> Command for InsertBundle<B> {
    fn apply(self, world: &mut World) {
        if let Some(entity) = world.get_mut(self.id) {
            self.bundle.insert_into(entity);
        }
    }
}

pub struct RemoveComponent {
    id: EntityID,
    component_uid: TypeId,
}

impl Command for RemoveComponent {
    fn apply(self, world: &mut World) {
        if let Some(entity) = world.get_mut(self.id) {
            entity.remove_by_uid(self.component_uid);
        }
    }
}

pub struct WorldCommands<'q> {
    queue: &'q mut CommandQueue<'static>,
}

impl WorldCommands<'_> {
    pub fn new<'a>(queue: &'a mut CommandQueue<'static>) -> WorldCommands<'a> {
        WorldCommands { queue }
    }

    pub fn submit(&mut self, world: &mut World) {
        self.queue.flush(world);
    }

    pub fn remove(&mut self, id: EntityID) {
        self.queue.push(Remove(id));
    }

    pub fn cmd<C: Command + 'static>(&mut self, command: C) -> &mut Self {
        self.queue.push(command);
        self
    }

    pub fn exec<F>(&mut self, command: F) -> &mut Self
    where
        F: FnOnce(&mut World) + 'static,
    {
        self.queue.push(command);
        self
    }

    pub fn spawn<B: Bundle + 'static>(&mut self, bundle: B) -> EntityCommands<'_> {
        let id = EntityID::unique();
        self.queue.push(Spawn { id, bundle });
        self.entity(id)
    }

    pub fn spawn_empty(&mut self) -> EntityCommands<'_> {
        self.spawn(())
    }

    pub fn entity(&mut self, id: EntityID) -> EntityCommands<'_> {
        EntityCommands {
            queue: self.queue,
            id,
        }
    }
}

pub struct EntityCommands<'q> {
    queue: &'q mut CommandQueue<'static>,
    id: EntityID,
}

impl<'q> EntityCommands<'q> {
    pub fn insert<B: Bundle + 'static>(&mut self, bundle: B) -> &mut Self {
        self.queue.push(InsertBundle {
            id: self.id,
            bundle,
        });
        self
    }

    pub fn submit(&mut self, world: &mut World) {
        self.queue.flush(world);
    }

    pub fn remove<C: Component>(&mut self) -> &mut Self {
        self.remove_by_uid(Entity::uid::<C>());
        self
    }

    pub fn remove_by_uid(&mut self, component_uid: TypeId) -> &mut Self {
        self.queue.push(RemoveComponent {
            id: self.id,
            component_uid,
        });
        self
    }

    pub fn id(&self) -> EntityID {
        self.id
    }
}

#[cfg(test)]
mod tests {

    mod raw_commands {
        use super::super::*;
        use crate::prelude::*;

        #[test]
        fn spawn_empty() {
            let mut world = World::new();

            let e1 = EntityID::unique();

            assert_eq!(world.total_components(), 0);
            assert_eq!(world.len(), 0);

            world.exec(Spawn { id: e1, bundle: () });

            assert_eq!(world.total_components(), 0);
            assert_eq!(world.len(), 1);
        }

        #[test]
        fn remove() {
            let mut world = World::new();

            let e1 = EntityID::unique();

            world.exec(Spawn { id: e1, bundle: () });

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
                id: e1,
                bundle: Name::from("Hello"),
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

        #[derive(Component, Debug)]
        struct DummyComponent;

        #[test]
        fn insert() {
            let mut world = World::new();

            let e1 = EntityID::unique();

            // empty entity
            world.exec(Spawn {
                id: e1,
                bundle: Name::from("Hello"),
            });

            assert_eq!(world.len(), 1);
            assert_eq!(world.total_components(), 1);

            world.exec(InsertBundle {
                id: e1,
                bundle: DummyComponent,
            });

            assert_eq!(world.len(), 1);
            assert_eq!(world.total_components(), 2);

            assert!(world.entity_has_component::<DummyComponent>(e1));
            assert!(world.entity_has_component::<Name>(e1));
        }
    }

    mod commands {
        use super::super::*;
        use crate::prelude::*;

        #[test]
        fn spawn_empty() {
            let mut world = World::new();
            let mut queue = CommandQueue::<'static>::default();
            let mut commands = WorldCommands::new(&mut queue);

            commands.exec(|world| {
                assert_eq!(world.total_components(), 0);
                assert_eq!(world.len(), 0);
            });

            commands.spawn_empty();

            commands.exec(|world| {
                assert_eq!(world.total_components(), 0);
                assert_eq!(world.len(), 1);
            });

            commands.submit(&mut world);
        }

        #[test]
        fn remove() {
            let mut world = World::new();
            let mut queue = CommandQueue::<'static>::default();
            let mut commands = WorldCommands::new(&mut queue);

            commands.exec(|world| {
                assert!(world.is_empty());
            });

            let e1 = commands.spawn(()).id();

            commands.exec(|world| {
                assert_eq!(world.len(), 1);
            });

            commands.remove(e1);

            commands.exec(|world| {
                assert!(world.is_empty());
            });

            commands.submit(&mut world);
        }

        #[test]
        fn spawn_with_bundle_empty() {
            let mut world = World::new();
            let mut queue = CommandQueue::<'static>::default();
            let mut commands = WorldCommands::new(&mut queue);

            // empty entity
            let e1 = commands.spawn(()).id();

            commands.exec(move |world| {
                assert_eq!(world.total_components(), 0);
                assert_eq!(world.len(), 1);

                world.get(e1).unwrap();
            });

            commands.submit(&mut world);
        }

        #[test]
        fn spawn_with_bundle() {
            let mut world = World::new();
            let mut queue = CommandQueue::<'static>::default();
            let mut commands = WorldCommands::new(&mut queue);

            let e1 = commands.spawn(Name::from("Hello")).id();

            commands.exec(|world| {
                assert_eq!(world.total_components(), 1);
            });

            // should not fail
            commands.exec(move |world| {
                assert_eq!(
                    world.get(e1).unwrap().component::<Name>().unwrap().0,
                    "Hello"
                );
            });

            let e2 = commands.spawn(Name::from("Hello3")).id();

            commands.exec(move |world| {
                assert_eq!(
                    world.get(e2).unwrap().component::<Name>().unwrap().0,
                    "Hello3"
                );
                assert_eq!(world.len(), 2);
                assert_eq!(world.total_components(), 2);
            });

            commands.submit(&mut world);
        }

        #[derive(Component, Debug)]
        struct DummyComponent;

        #[test]
        fn insert() {
            let mut world = World::new();

            let e1 = EntityID::unique();

            // empty entity
            world.exec(Spawn {
                id: e1,
                bundle: Name::from("Hello"),
            });

            assert_eq!(world.len(), 1);
            assert_eq!(world.total_components(), 1);

            world.exec(InsertBundle {
                id: e1,
                bundle: DummyComponent,
            });

            assert_eq!(world.len(), 1);
            assert_eq!(world.total_components(), 2);

            assert!(world.entity_has_component::<DummyComponent>(e1));
            assert!(world.entity_has_component::<Name>(e1));
        }
    }
}
