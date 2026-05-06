use crate::prelude::World;

mod queue;
mod world;

pub use queue::*;
pub use world::*;

/// A 'Command' is a packaged callable for mutations to the world
pub trait Command: Send + 'static {
    fn apply(self, world: &mut World);
}

/// Internal use for getting around the restriction that dyn traitobjects cannot call
/// methods that consume self due to unknown compiletime size
pub(crate) trait DynCommand {
    fn apply(self: Box<Self>, world: &mut World);
}

/// Every command is a valid dyncommand, just dereference
impl<C: Command> DynCommand for C {
    fn apply(self: Box<Self>, world: &mut World) {
        C::apply(*self, world);
    }
}
