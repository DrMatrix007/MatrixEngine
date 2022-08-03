use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, RwLock,
    },
    time::Duration,
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
    target_duration: Arc<RwLock<Duration>>,
    registry: Arc<RwLock<Registry>>,
}

impl Application {
    pub fn defualt_duration() -> Duration {
        Duration::from_secs_f64(1.0/60.0)
    }

    pub fn new() -> Self {
        Application {
            layers: LayerPool::new(),
            target_duration: Arc::new(RwLock::new(Self::defualt_duration())),
            quitting: Arc::new(AtomicBool::new(false)),
            events: Arc::new(RwLock::new(Events::new())),
            registry: Arc::new(RwLock::new(Registry::new())),
        }
    }
    pub fn set_target_fps(&mut self, target: Duration) {
        *self.target_duration.write().unwrap() = target;
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
            target_nanos_per_second: self.target_duration.clone(),
        };
        self.layers.start_all(args);

        while !self.layers.is_done() {}
    }
}

impl Default for Application {
    fn default() -> Self {
        Self::new()
    }   
}
