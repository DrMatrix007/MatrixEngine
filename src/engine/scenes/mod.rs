use tokio::runtime::{Runtime, self};
use winit::{
    event::Event,
    event_loop::{ControlFlow, EventLoopWindowTarget},
};

use crate::engine::events::event_registry::EventRegistry;

pub mod components;
pub mod entities;
pub mod scene_builder;

pub struct Scene {
    events: EventRegistry,
}

impl Scene {
    fn new() -> Self {
        Self {
            events: EventRegistry::default(),
        }
    }

    fn frame(&self,runtime: &Runtime, _target: &EventLoopWindowTarget<()>) -> ControlFlow {
        
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
                *control_flow = self.frame(runtime,target);
            }
            event => {
                self.events.process(event);
            }
        }
    }
}
