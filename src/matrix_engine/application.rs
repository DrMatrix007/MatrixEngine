use std::{
    sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}},
    time::Duration,
};

use crate::*;

use super::{ecs::registry::Registry, event::Events, utils::clock::Clock};

pub struct Application {
    quitting: AtomicBool,
    layers: Vec<LayerHolder>,
    events: Events,
    registry: Arc<Mutex<Registry>>,
}

impl Application {
    pub fn new() -> Self {
        Application {
            layers: Vec::new(),
            quitting: AtomicBool::new(false),
            events: Events::new(),
            registry: Arc::new(Mutex::new(Registry::new())),
        }
    }

    pub fn stop(&mut self) {
        self.quitting.store(true, Ordering::Relaxed);
    }

    pub fn push_box(&mut self, layer: Box<dyn Layer>) {
        self.layers.push(LayerHolder::new(layer));
    }

    pub fn push_layer<T: Layer + 'static>(&mut self, val: T) {
        self.push_box(Box::new(val));
    }

    pub fn run(&mut self) {
        let mut delta_clock = Clock::start_new();
        let time_clock = Clock::start_new();
        let mut currnet_time = Duration::ZERO;
        let mut current_delta = Duration::ZERO;
        while !*self.quitting.get_mut() {
            let args = LayerArgs {
                events: &self.events,
                registry: self.registry.clone(),
                delta_time: current_delta,
                time: currnet_time,
                quit_ref: &self.quitting
            };
            for layer in self.layers.iter_mut() {
                layer.start(&args);
            }
            for layer in self.layers.iter_mut() {
                layer.update(&args);            }
            current_delta = delta_clock.restart();
            currnet_time = time_clock.elapsed();
        }
        for layer in self.layers.iter_mut(){
            layer.clean_up();
        }
    }
}
