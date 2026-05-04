use crate::prelude::EntityID;

#[derive(Debug)]
pub enum Commands {
    Remove(EntityID),
    Spawn(),
}
