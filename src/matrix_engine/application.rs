use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, RwLock,
};

use super::{
    ecs::registry::Registry,
    event::Events,
    layer::{Layer, LayerArgs, LayerPool},
    utils::clock::Clock,
};

pub struct Application {
    quitting: Arc<AtomicBool>,
    layers: LayerPool,
    events: Arc<RwLock<Events>>,
    target_frames_per_second: Arc<RwLock<f64>>,
    registry: Arc<RwLock<Registry>>,
}

impl Application {
    const DEFUALT_TARGET_FRAMES_PER_SECOND: f64 = 1.0 / 60.0;

    pub fn new() -> Self {
        Application {
            layers: LayerPool::new(),
            target_frames_per_second: Arc::new(RwLock::new(Self::DEFUALT_TARGET_FRAMES_PER_SECOND)),
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
        self.layers.push_layer(val);
    }

    pub fn run(mut self) {
        let time_clock = Clock::start_new();

        let args = LayerArgs {
            events: self.events.clone(),
            registry: self.registry.clone(),
            time: time_clock,
            quit_ref: self.quitting.clone(),
            target_frames_per_second: self.target_frames_per_second.clone(),
        };
        self.layers.start_all(args);

        while !self.layers.is_done() {}

        for layer in self.layers.iter_mut() {
            layer.clean_up();
        }
    }
}

impl Default for Application {
    fn default() -> Self {
        Self::new()
    }
}
