pub mod engine;
pub mod events;
pub mod scenes;

use tokio::runtime::{self, Runtime};
use winit::event_loop::EventLoop;

use self::scenes::scene_builder::SceneBuilder;

pub struct Engine {
    event_loop: EventLoop<()>,
    runtime: Runtime,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            event_loop: EventLoop::new(),
            runtime: runtime::Builder::new_multi_thread().build().unwrap(),
        }
    }

    pub fn run(self, builder: &SceneBuilder) -> ! {
        let mut current_scene = builder.build();

        self.event_loop.run(move |event, target, control_flow| {
            current_scene.process(event, target, &self.runtime, control_flow);
        });
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}
