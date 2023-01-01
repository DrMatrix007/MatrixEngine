use std::sync::{atomic::AtomicBool, RwLockWriteGuard, Arc, RwLock, RwLockReadGuard};

use super::registry::{ComponentRegistry};

pub struct SystemArgs {
    running: Arc< AtomicBool>,
    components: Arc<RwLock<ComponentRegistry>>,
}

impl SystemArgs {
    pub fn new(
        running: Arc<AtomicBool>,
        components: Arc<RwLock<ComponentRegistry>>,
    ) -> Self {
        Self {
            running,
            components,
        }
    }
    pub fn stop(&self) {
        self.running.store(false, std::sync::atomic::Ordering::Release);
    }
    pub fn read_component_registry(&self) -> Option<RwLockReadGuard<ComponentRegistry>> {
        self.components.read().ok()
    }
    pub fn write_component_registry(&self) -> Option<RwLockWriteGuard<ComponentRegistry>> {
        self.components.write().ok()
    }
}

pub trait System: Send  {
    fn update(&mut self, args: SystemArgs);
}
