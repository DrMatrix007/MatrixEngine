use std::sync::Arc;

use tokio::sync::Mutex;
use winit::{
    event::Event,
    event_loop::{ControlFlow, EventLoopWindowTarget},
};

use crate::engine::events::event_registry::EventRegistry;

use self::components::component_registry::ComponentRegistry;

use super::{
    runtime::Runtime,
    systems::{query::ComponentQueryArgs, system_registry::SystemRegistry},
};

pub mod components;
pub mod entities;
pub mod scene_builder;

pub struct Scene {
    registry: Arc<Mutex<SceneRegistry>>,
    systems: SystemRegistry<ComponentQueryArgs>,
}

impl Scene {
    fn new(registry: SceneRegistry) -> Self {
        Self {
            registry: Arc::new(Mutex::new(registry)),
            systems: Default::default(),
        }
    }

    fn frame(
        &self,
        runtime: &mut dyn Runtime<ComponentQueryArgs>,
        _target: &EventLoopWindowTarget<()>,
    ) -> ControlFlow {
        let reg = self.registry.clone().try_lock_owned().unwrap();

        for i in self.systems.try_lock_iter_send() {}
        ControlFlow::Poll
    }

    pub fn process(
        &mut self,
        event: Event<()>,
        target: &EventLoopWindowTarget<()>,
        runtime: &mut dyn Runtime<ComponentQueryArgs>,
        control_flow: &mut ControlFlow,
    ) {
        match event {
            Event::MainEventsCleared => {
                *control_flow = self.frame(runtime, target);
            }
            event => {
                self.registry
                    .try_lock()
                    .expect("this registry should not be locked here")
                    .events
                    .process(event);
            }
        }
    }
}

pub struct SceneRegistry {
    events: EventRegistry,
    components: ComponentRegistry,
}

impl SceneRegistry {
    fn new() -> Self {
        Self {
            components: ComponentRegistry::new(),
            events: EventRegistry::default(),
        }
    }

    pub fn events(&self) -> &EventRegistry {
        &self.events
    }

    pub fn events_mut(&mut self) -> &mut EventRegistry {
        &mut self.events
    }

    pub fn components(&self) -> &ComponentRegistry {
        &self.components
    }

    pub fn components_mut(&mut self) -> &mut ComponentRegistry {
        &mut self.components
    }
}
