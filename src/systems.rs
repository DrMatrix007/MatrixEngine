use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Condvar, Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard,
};

use winit::event_loop::ControlFlow;

use super::{components::Component, registry::ComponentRegistry};

pub struct SystemArgs<'a> {
    control_flow: &'a mut ControlFlow,
    components: &'a mut ComponentRegistry,
}

impl<'a> SystemArgs<'a> {
    pub fn new(control_flow: &'a mut ControlFlow, components: &'a mut ComponentRegistry) -> Self {
        Self {
            control_flow,
            components,
        }
    }

    pub fn stop(&mut self) {
        *self.control_flow = ControlFlow::Exit;
    }
    pub fn components(&mut self) -> &mut ComponentRegistry {
        self.components
    }
}

pub trait System {
    fn update(&mut self, args: SystemArgs);
}

pub struct SystemCreator {
    creator: Box<dyn FnOnce() -> Box<dyn System> + Send + Sync>,
}

impl SystemCreator {
    pub fn default_function<T: System + Default + 'static>() -> Self {
        Self {
            creator: Box::new(|| Box::<T>::default()),
        }
    }
    pub fn with_function(f: impl FnOnce() -> Box<dyn System> + Send + Sync + 'static) -> Self {
        Self {
            creator: Box::new(f),
        }
    }

    pub fn create(self) -> Box<dyn System> {
        (self.creator)()
    }
}
