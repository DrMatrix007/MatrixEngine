use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, RwLock, Mutex,
};

use crate::*;

use super::{ecs::registry::Registry, event::Events, layer::{LayerHolder, Layer, LayerPool}};

pub struct Application {
    pub(super) quitting: Arc<AtomicBool>,
    pub(super) layers: Arc<RwLock<Vec<LayerHolder>>>,
    pub(super) events: Arc<RwLock<Events>>,
    pub(super) registry: Arc<RwLock<Registry>>,
    target_frames_per_second: Mutex<f64>
}

impl Application {
    const DEFUALT_TARGET_FRAMES_PER_SECOND: f64 = 1.0 / 60.0;

    pub fn new() -> Self {
        Application {
            layers: Arc::new(RwLock::new(Vec::new())),
            quitting: Arc::new(AtomicBool::new(false)),
            events: Arc::new(RwLock::new(Events::new())),
            registry: Arc::new(RwLock::new(Registry::new())),
            
        }
    }
    pub fn set_target_fps(&mut self, target: u64) {
        *self.target_frames_per_second.write().unwrap() = 1.0 / target as f64;
    }
    pub fn get_target_fps(&self) -> u64 {
        (1.0 / *self.target_frames_per_second.read().unwrap() as f64) as u64
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

impl Default for Application {
    fn default() -> Self {
        Self::new()
    }
}
