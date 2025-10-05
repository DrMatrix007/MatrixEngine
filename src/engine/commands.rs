pub mod add_entity_command;

use std::collections::VecDeque;

#[derive(Default)]
pub struct CommandBuffer<T> {
    commands: VecDeque<Box<dyn Command<T>>>,
}

impl<T> CommandBuffer<T> {
    pub fn add_command(&mut self, command: impl Command<T> + 'static) {
        self.commands.push_back(Box::new(command));
    }

    pub fn drain(&mut self) -> impl Iterator<Item = Box<dyn Command<T>>> {
        self.commands.drain(..)
    }
}

pub trait Command<Target> {
    fn run(&mut self, data: &mut Target);
}
