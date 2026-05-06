use std::collections::VecDeque;

use crate::{
    ecs::commands::{Command, DynCommand},
    prelude::World,
};

pub struct CommandQueue {
    commands: VecDeque<Box<dyn DynCommand>>,
}

impl CommandQueue {
    /// Internally reserve at least 'len' number of **additional** commands in the queu
    pub fn reserve(&mut self, len: usize) {
        self.commands.reserve(len);
    }

    pub fn push<C: Command>(&mut self, command: impl Command) {
        self.commands.push_back(Box::new(command));
    }

    pub fn flush(&mut self, world: &mut World) {
        while let Some(command) = self.commands.pop_front() {
            command.apply(world);
        }
    }

    pub fn clear(&mut self) {
        self.commands.clear();
    }
}
