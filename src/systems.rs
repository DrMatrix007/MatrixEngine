use std::{collections::VecDeque, future::Future};

use winit::event_loop::{ControlFlow, EventLoopWindowTarget};

use crate::{registry::Registry, resources::ResourceManager};

use super::registry::ComponentRegistry;

pub struct SystemArgs<'a> {
    control_flow: &'a mut ControlFlow,
    registry: &'a mut Registry,
    event_loop: &'a EventLoopWindowTarget<()>,
}

impl<'a> SystemArgs<'a> {
    pub fn new(
        control_flow: &'a mut ControlFlow,
        registry: &'a mut Registry,
        event_loop: &'a EventLoopWindowTarget<()>,
    ) -> Self {
        Self {
            control_flow,
            registry,
            event_loop,
        }
    }

    pub fn stop(&mut self) {
        *self.control_flow = ControlFlow::Exit;
    }
    pub fn components(&mut self) -> &mut ComponentRegistry {
        self.registry.get_component_registry_mut()
    }
    pub fn window_target(&self) -> &EventLoopWindowTarget<()> {
        self.event_loop
    }
    pub fn resources(&mut self) -> &mut ResourceManager {
        self.registry.get_resource_manager_mut()
    }
}

pub trait System {
    fn update(&mut self, args: &mut SystemArgs);
    fn setup(&mut self, _: &mut SystemArgs) {}
}

impl<F: FnMut(&mut SystemArgs)> System for F {
    fn update(&mut self, args: &mut SystemArgs) {
        self(args);
    }
}

#[derive(Default)]
pub struct SystemCollection {
    queue: VecDeque<Box<dyn System>>,
    systems: Vec<Box<dyn System>>,
}

impl SystemCollection {
    pub fn update(&mut self, args: &mut SystemArgs) {
        while let Some(mut s) = self.queue.pop_back() {
            s.setup(args);
            self.systems.push(s);
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Box<dyn System>> {
        self.systems.iter()
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Box<dyn System>> {
        self.systems.iter_mut()
    }

    pub(crate) fn insert_system(&mut self, system: Box<dyn System>) {
        self.queue.push_back(system);
    }
}
