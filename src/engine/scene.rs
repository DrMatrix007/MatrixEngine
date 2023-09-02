use winit::{
    event::Event,
    event_loop::{ControlFlow, EventLoopWindowTarget},
};

use super::events::event_registry::EventRegistry;

pub struct SceneBuilder {
    builder: Box<dyn Fn(&mut Scene)>,
}

impl SceneBuilder {
    pub fn new(builder: impl Fn(&mut Scene) + 'static) -> Self {
        Self {
            builder: Box::new(builder),
        }
    }
    pub fn build(&self) -> Scene {
        let mut scene = Scene::new();
        (self.builder)(&mut scene);
        scene
    }
}

pub struct Scene {
    events: EventRegistry,
}

impl Scene {
    fn new() -> Self {
        Self {
            events: EventRegistry::default(),
        }
    }

    fn frame(&self, target: &EventLoopWindowTarget<()>, control_flow: &mut ControlFlow) {}

    pub(crate) fn process(
        &mut self,
        event: Event<()>,
        target: &EventLoopWindowTarget<()>,
        control_flow: &mut ControlFlow,
    ) {
        match event {
            Event::MainEventsCleared => {
                self.frame(target, control_flow);
            }
            event => {
                self.events.process(event);
            }
        }
    }
}
