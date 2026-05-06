use crate::ecs::{commands::Command, entity::EntityID, world::World};

pub struct Remove(EntityID);

impl Command for Remove {
    fn apply(self, world: &mut World) {
        world.remove(self.0);
    }
}
