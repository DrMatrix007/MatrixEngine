pub mod add_entity_command;
pub mod add_window_resource_command;

use std::collections::VecDeque;

use crate::{engine::{
    query::{Query, QueryError}, EngineState, Scene
}, lockable::LockableError};

#[derive(Default)]
pub struct CommandBuffer<T = EngineState> {
    commands: VecDeque<Box<dyn Command<Args = T>>>,
}

impl<T> CommandBuffer<T> {
    pub fn new() -> Self {
        Self {
            commands: VecDeque::with_capacity(0),
        }
    }
    pub fn add_command(&mut self, command: impl Command<Args = T> + 'static) {
        self.commands.push_back(Box::new(command));
    }

    pub fn drain(&mut self) -> impl Iterator<Item = Box<dyn Command<Args = T>>> {
        self.commands.drain(..)
    }
}

pub trait Command: Send {
    type Args;
    fn run(&mut self, data: &mut Self::Args) -> Result<(), CommandError>;
}

pub struct CommandArgs<'a> {
    pub event_loop: &'a winit::event_loop::ActiveEventLoop,
    pub scene: &'a mut Scene,
}

#[derive(Debug, Clone)]
pub enum CommandError {
    LockableError(LockableError),
    UnknownError
}

impl<T> Query<T> for CommandBuffer<T> {
    fn prepare(_: &mut T) -> Result<Self, super::query::QueryError> {
        Ok(Self::new())
    }

    fn consume(mut self, registry: &mut T) -> Result<(), super::query::QueryError> {
        for mut command in self.drain() {
            command
                .run(registry)
                .map_err(QueryError::CommandError)?;
        }
        Ok(())
    }
}
