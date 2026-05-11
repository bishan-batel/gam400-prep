pub mod commands;
pub mod component;
pub mod entity;
pub mod query;
pub mod world;

pub mod prelude {
    pub use crate::{component::*, entity::*, world::*};
    pub use ecs_custom_macros::Component;
}

pub struct Engine {}
