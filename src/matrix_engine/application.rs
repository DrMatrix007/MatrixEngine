use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, RwLock, Mutex,
};

use crate::*;

use super::{ecs::registry::Registry, event::Events};

pub struct Application {
    pub(super) quitting: Arc<AtomicBool>,
    pub(super) layers: Arc<RwLock<Vec<LayerHolder>>>,
    pub(super) events: Arc<RwLock<Events>>,
    pub(super) registry: Arc<RwLock<Registry>>,
}

impl Application {
    pub fn new() -> Self {
        Application {
            layers: Arc::new(RwLock::new(Vec::new())),
            quitting: Arc::new(AtomicBool::new(false)),
            events: Arc::new(RwLock::new(Events::new())),
            registry: Arc::new(RwLock::new(Registry::new())),
        }
    }

    pub fn stop(&mut self) {
        self.quitting.store(true, Ordering::Relaxed);
    }

    pub fn push_layer<T: Layer + 'static>(&mut self, val: T) {
        self.layers.write().unwrap().push(LayerHolder::new(Box::new(val)));
    }

    pub fn run(&mut self) {
        let e = self.events.clone();
        let r = self.registry.clone();
        let q = self.quitting.clone();
        let pool = LayerPool::new(self.layers.clone());
        pool.start_execution(e, r, q);

        while !self.quitting.load(Ordering::SeqCst) {}
        // }
        // for layer in self.layers.() {
        //     layer.clean_up();
        // }
    }
}
