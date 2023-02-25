use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Condvar, Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard,
};

use super::{components::Component, registry::ComponentRegistry};

pub struct SystemArgs {
    quit: Arc<AtomicBool>,
    components: Arc<RwLock<ComponentRegistry>>,
}

impl SystemArgs {
    pub fn new(
        quit: Arc<AtomicBool>,
        components: Arc<RwLock<ComponentRegistry>>,
    ) -> Self {
        
        Self { quit, components }
    }

    pub fn stop(&self) {
        self.quit.store(true, Ordering::Relaxed);
    }
    pub fn read_components(&self) -> Option<RwLockReadGuard<ComponentRegistry>>{
        self.components.read().ok()
    }
    pub fn write_components(&self) -> Option<RwLockWriteGuard<ComponentRegistry>>{
        self.components.write().ok()
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


