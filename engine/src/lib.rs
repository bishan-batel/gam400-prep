pub mod core;
pub mod ecs;

pub mod prelude {
    pub use crate::ecs::{component::*, entity::*, world::*};
    pub use g4_derive::Component;
}

pub struct Engine {}
