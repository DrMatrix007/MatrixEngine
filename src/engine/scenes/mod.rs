use std::sync::Arc;

use tokio::{runtime::Runtime, sync::Mutex};
use winit::{
    event::Event,
    event_loop::{ControlFlow, EventLoopWindowTarget},
};

use crate::engine::events::event_registry::EventRegistry;

use self::components::component_registry::ComponentRegistry;

pub mod components;
pub mod entities;
pub mod scene_builder;

pub struct Scene {
    registry: Arc<Mutex<SceneRegistry>>,
}

impl Scene {
    fn new(registry: SceneRegistry) -> Self {
        Self {
            registry: Arc::new(Mutex::new(registry)),
        }
    }

    fn frame(&self, runtime: &Runtime, _target: &EventLoopWindowTarget<()>) -> ControlFlow {
        ControlFlow::Poll
    }

    pub fn process(
        &mut self,
        event: Event<()>,
        target: &EventLoopWindowTarget<()>,
        runtime: &Runtime,
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
}
