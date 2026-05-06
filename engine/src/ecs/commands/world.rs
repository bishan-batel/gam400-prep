use crate::ecs::{commands::Command, entity::EntityID, world::World};

#[derive(Debug)]
pub struct Remove(EntityID);
