use std::{
    any::TypeId,
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Arc,
    },
    thread,
    time::{Duration, Instant},
};

use winit::event_loop::EventLoop;

use super::{
    registry::Registry,
    systems::{System, SystemArgs, SystemCreator},
};

#[derive(Default)]
pub struct Runtime {
    registry: Registry,
    systems: Vec<Box<dyn System>>,
    target_fps: u64,
}

impl Runtime {
    pub fn new(r: Registry, target_fps: u64) -> Self {
        Self {
            registry: r,
            target_fps,
            systems: vec![],
        }
    }

    pub fn run(mut self) -> ! {
        let event_loop = EventLoop::new();

        event_loop.run(move |event, _, control_flow| {
            for i in self.systems.iter_mut() {
                let args =
                    SystemArgs::new(control_flow, self.registry.get_component_registry_mut());

                i.update(args)
            }
        });
    }

    pub fn insert_system<T: System + 'static>(&mut self, system: T) {
        self.systems.push(Box::new(system));
    }
}
