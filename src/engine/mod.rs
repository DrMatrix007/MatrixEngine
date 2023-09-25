use winit::event_loop::EventLoop;

use self::{
    runtime::Runtime, scenes::scene_builder::SceneBuilder, systems::query::ComponentQueryArgs,
};

pub mod events;
pub mod runtime;
pub mod scenes;
pub mod systems;

pub struct Engine {
    event_loop: EventLoop<()>,
    runtime: Box<dyn Runtime<ComponentQueryArgs>>,
}

impl Engine {
    pub fn new(runtime: impl Runtime<ComponentQueryArgs> + 'static) -> Self {
        Self {
            event_loop: EventLoop::new(),
            runtime: Box::new(runtime),
        }
    }

    pub fn run(mut self, builder: &SceneBuilder) -> ! {
        let mut current_scene = builder.build();

        self.event_loop.run(move |event, target, control_flow| {
            current_scene.process(event, target, self.runtime.as_mut(), control_flow);
        });
    }
}
