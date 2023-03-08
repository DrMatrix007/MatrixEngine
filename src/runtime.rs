use winit::event_loop::EventLoop;

use crate::systems::SystemCollection;

use super::{
    registry::Registry,
    systems::{System, SystemArgs},
};

#[derive(Default)]
pub struct Runtime {
    registry: Registry,
    systems: SystemCollection,
}

impl Runtime {
    pub fn new(r: Registry) -> Self {
        Self {
            registry: r,
            systems: SystemCollection::default(),
        }
    }

    pub fn run(mut self) -> ! {
        let event_loop = EventLoop::new();

        event_loop.run(move |_, target, control_flow| {
            let mut args = SystemArgs::new(
                control_flow,
                self.registry.get_component_registry_mut(),
                target,
            );
            self.systems.update(&mut args);

            for i in self.systems.iter_mut() {
                i.update(&mut args)
            }
        });
    }

    pub fn insert_system<T: System + 'static>(&mut self, system: T) {
        self.systems.insert_system(Box::new(system));
    }
}
