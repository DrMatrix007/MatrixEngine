use std::{
    sync::atomic::AtomicUsize,
    time::{Duration, Instant},
};

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
    target_fps: AtomicUsize,
}

impl Engine {
    pub fn new(runtime: impl Runtime<ComponentQueryArgs> + 'static, fps: usize) -> Self {
        Self {
            event_loop: EventLoopBuilder::<EngineEvent>::with_user_event().build(),
            runtime: Box::new(runtime),
            target_fps: fps.into(),
        }
    }

    pub fn run(mut self, builder: &SceneBuilder) -> ! {
        let mut current_scene = builder.build();

        self.runtime
            .use_event_loop_proxy(self.event_loop.create_proxy());

        let mut last_frame_time = Instant::now();

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

            if let Event::MainEventsCleared = &event {
                let frame_duration = Duration::from_secs(1)
                    / self.target_fps.load(std::sync::atomic::Ordering::Relaxed) as u32;
                let elapsed = Instant::now().duration_since(last_frame_time);
                if frame_duration > elapsed {
                    spin_sleep::sleep(frame_duration - elapsed);
                }
                last_frame_time = Instant::now();
            }
        });
    }
}
