use std::collections::VecDeque;

use super::Command;
use crate::world::World;

/// Structure for encoding deferred commands to be applied to a World
#[derive(Default)]
pub struct CommandQueue<'a> {
    /// Internal circular buffer of commands
    commands: VecDeque<Box<dyn DynCommand + 'a>>,
}

/// Internal use for getting around the restriction that dyn traitobjects cannot call
/// methods that consume self due to unknown compiletime size
trait DynCommand {
    fn apply(self: Box<Self>, world: &mut World);
}

/// Every command is a valid dyncommand, just dereference
impl<C: Command> DynCommand for C {
    fn apply(self: Box<Self>, world: &mut World) {
        C::apply(*self, world)
    }
}

impl<'a> CommandQueue<'a> {
    /// Constructs a new CommandQueue, this is just an alias for Self::default()
    pub fn new() -> Self {
        Self::default()
    }

    /// Internally reserve at least 'len' number of **additional** commands in the queu
    pub fn reserve(&mut self, len: usize) {
        self.commands.reserve(len);
    }

    /// Enqueue a new command
    pub fn push(&mut self, command: impl Command + 'a) {
        self.commands.push_back(Box::new(command));
    }

    /// Execute all deferred commands on the given world
    pub fn flush(&mut self, world: &mut World) {
        while let Some(command) = self.commands.pop_front() {
            command.apply(world);
        }
    }

    /// Number of commands enqueued
    #[must_use]
    pub fn len(&self) -> usize {
        self.commands.len()
    }

    /// Is the command queue empty / has nothing to do
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }

    /// Removes all commands from the queue
    pub fn clear(&mut self) {
        self.commands.clear();
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use crate::{
        commands::{Command, CommandQueue},
        prelude::World,
    };

    struct DummyCommand;
    impl Command for DummyCommand {
        fn apply(self, _world: &mut World) {}
    }

    struct SetValue<'a, T> {
        loc: &'a Cell<T>,
        value: T,
    }

    impl<T> Command for SetValue<'_, T> {
        fn apply(self, _world: &mut World) {
            self.loc.set(self.value);
        }
    }

    #[test]
    fn simple() {
        let mut queue = CommandQueue::default();
        assert_eq!(queue.len(), 0);
        queue.clear();
        assert_eq!(queue.len(), 0);

        queue.push(DummyCommand);

        assert_eq!(queue.len(), 1);

        let mut world = World::new();
        queue.flush(&mut world);

        assert!(queue.is_empty());
    }

    #[test]
    fn lifetime() {
        // ECS world
        let mut world = World::new();

        const VALUE: i32 = 10;

        // Commands lifetime must be constricted to the command queue length

        // we use a cell for interior mutability so its easier to give a command an adress to write
        // to, rusts lifetime rules and guarentees means the type-solver would not be able to
        // ensure correctnesss if we gave push a command containing a mutable reference to a local.
        // It would get confused afterwards on whether or not after such action is the use of
        // 'queue' valid.
        let value = Cell::new(0);

        let mut queue = CommandQueue::default();

        // enqueue action that sets value to 10
        queue.push(SetValue {
            loc: &value,
            value: VALUE,
        });

        // queue should've book-kept the deferred event
        assert_eq!(queue.len(), 1);

        // flush queue
        queue.flush(&mut world);

        // make sure the value was actually set
        assert_eq!(value.get(), VALUE);

        // queue should be empty again
        assert_eq!(queue.len(), 0);
    }
}
