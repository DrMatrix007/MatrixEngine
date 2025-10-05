use std::{collections::VecDeque, mem::swap};

use crate::engine::systems::System;

#[derive(Debug)]
pub enum SystemRegistryNextFrameError {
    NotAllSystemExhausted,
}

type BoxedSystem<Registry> = Box<dyn System<Registry = Registry>>;

pub struct SystemRegistry<Registry> {
    systems: VecDeque<BoxedSystem<Registry>>,
    systems_done: VecDeque<BoxedSystem<Registry>>,
}

impl<Registry> Default for SystemRegistry<Registry> {
    fn default() -> Self {
        Self {
            systems: Default::default(),
            systems_done: Default::default(),
        }
    }
}

impl<Registry> SystemRegistry<Registry> {
    pub fn add_system(&mut self, system: impl System<Registry = Registry> + 'static) {
        self.systems.push_back(Box::new(system));
    }

    pub fn take_out_system(&mut self) -> Option<BoxedSystem<Registry>> {
        self.systems.pop_front()
    }

    pub fn take_back_system(&mut self, b: BoxedSystem<Registry>) {
        self.systems_done.push_back(b);
    }

    pub fn prepare_next_frame(&mut self) -> Result<(), SystemRegistryNextFrameError> {
        if !self.systems.is_empty() {
            return Err(SystemRegistryNextFrameError::NotAllSystemExhausted);
        }
        swap(&mut self.systems, &mut self.systems_done);
        Ok(())
    }
}
