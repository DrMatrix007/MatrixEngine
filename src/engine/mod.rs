use winit::{
    event::Event,
    event_loop::{EventLoop, EventLoopBuilder},
};

use self::{
    events::engine_event::EngineEvent, runtime::Runtime, scenes::scene_builder::SceneBuilder,
    systems::query::ComponentQueryArgs,
};

pub mod events;
pub mod runtime;
pub mod scenes;
pub mod systems;

pub struct Engine {
    event_loop: EventLoop<EngineEvent>,
    runtime: Box<dyn Runtime<ComponentQueryArgs>>,
}

impl Engine {
    pub fn new(runtime: impl Runtime<ComponentQueryArgs> + 'static) -> Self {
        Self {
            event_loop: EventLoopBuilder::<EngineEvent>::with_user_event().build(),
            runtime: Box::new(runtime),
        }
    }

    pub fn run(mut self, builder: &SceneBuilder) -> ! {
        let mut current_scene = builder.build();

        self.event_loop.run(move |event, target, control_flow| {
            current_scene.process(&event, target, self.runtime.as_mut(), control_flow);
            if let Event::UserEvent(event) = &event {
                self.runtime.process_engine_event(
                    event,
                    &mut ComponentQueryArgs::new(
                        current_scene
                            .registry()
                            .clone()
                            .try_lock_owned()
                            .expect("the value should not be locked here"),
                    ),
                );
            }
        });
    }
}
