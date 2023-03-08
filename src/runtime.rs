use winit::event_loop::EventLoop;

use crate::systems::SystemCollection;

use super::{
    registry::Registry,
    systems::{System, SystemArgs},
};

#[derive(Default)]
pub struct Runtime {
    event_loop: EventLoop<()>,
    registry: Registry,
    systems: SystemCollection,
}

impl Runtime {
    pub fn new(r: Registry) -> Self {
        Self {
            event_loop: EventLoop::default(),
            registry: r,
            systems: SystemCollection::default(),
        }
    }
    pub fn window_target(&self) -> &EventLoop<()> {
        &self.event_loop
    }
    pub fn run(mut self) -> ! {
        self.event_loop.run(move |_, target, control_flow| {
            let mut args = SystemArgs::new(control_flow, &mut self.registry, target);
            self.systems.update(&mut args);

            for i in self.systems.iter_mut() {
                i.update(&mut args)
            }
        });
    }
    pub fn registry(&self) -> &Registry {
        &self.registry
    }
    pub fn registry_mut(&mut self) -> &mut Registry {
        &mut self.registry
    }
    pub fn insert_system<T: System + 'static>(&mut self, system: T) {
        self.systems.insert_system(Box::new(system));
    }
}
