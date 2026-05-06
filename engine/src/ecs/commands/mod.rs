use std::fmt::Debug;

use crate::prelude::World;

mod queue;
mod world;

pub use queue::*;
pub use world::*;

/// A 'Command' is a packaged callable for mutations to the world
pub trait Command {
    fn apply(self, world: &mut World);
}

impl<'a, F> Command for F
where
    F: FnOnce(&mut World) -> () + 'a,
{
    fn apply(self, world: &mut World) {
        self(world)
    }
}
