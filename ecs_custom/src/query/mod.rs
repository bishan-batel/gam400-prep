use std::{any::TypeId, marker::PhantomData};

use crate::prelude::{Component, ComponentUIDKeyable, Entity, World};

pub trait View {
    type Item<'a>;

    fn matches(entity: &Entity) -> bool;

    fn get<'a>(entity: &'a mut Entity) -> Option<Self::Item<'a>>;
}

pub struct Query<'a, V: View> {
    world: &'a mut World,
    _phantom: PhantomData<V>,
}

trait ComponentView<C: Component>: View {
    fn uid() -> TypeId {
        <C as ComponentUIDKeyable>::uid()
    }
}

impl<C: Component> View for &'_ C {
    type Item<'a> = &'a C;

    fn matches(entity: &Entity) -> bool {
        entity.has_component::<C>()
    }

    fn get<'a>(entity: &'a mut Entity) -> Option<Self::Item<'a>> {
        entity.component::<C>()
    }
}

impl<C: Component> View for &'_ mut C {
    type Item<'a> = &'a C;

    fn matches(entity: &Entity) -> bool {
        entity.has_component::<C>()
    }

    fn get<'a>(entity: &'a mut Entity) -> Option<Self::Item<'a>> {
        entity.component::<C>()
    }
}

impl<'w, V: View> From<&'w mut World> for Query<'w, V> {
    fn from(world: &'w mut World) -> Self {
        Self {
            world,
            _phantom: Default::default(),
        }
    }
}

impl<V: View> Query<'_, V> {
    pub fn new(world: &mut World) -> Query<'_, V> {
        world.into()
    }

    pub fn iter(&mut self) -> impl Iterator<Item = V::Item<'_>> {
        self.world.entities_mut().flat_map(V::get)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[derive(Debug, Component)]
    struct Health(usize);

    #[test]
    fn single_component() {
        let mut world = World::new();

        fn q(mut query: Query<&Health>) {
            for q in query.iter() {
                assert_eq!(q.0, 50);
            }
        }

        world.spawn(Health(50));

        q(Query::from(&mut world));
    }
}
