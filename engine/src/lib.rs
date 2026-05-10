pub mod core;
pub mod ecs;
pub mod resource;

pub mod prelude {
    pub use crate::{
        ecs::{component::*, entity::*, world::*},
        resource::*,
    };
    pub use g4_derive::Component;
}

pub struct Engine {}
