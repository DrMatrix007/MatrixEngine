use winit::event_loop::EventLoop;

use super::scenes::scene_builder::SceneBuilder;


pub struct Engine {
    event_loop: EventLoop<()>,
    runtime: tokio::runtime::Runtime
}

impl Engine {
    pub fn new() -> Self {
        Self {
            event_loop: EventLoop::new(),
            runtime: tokio::runtime::Builder::new_multi_thread().build().unwrap()
        }
    }

    pub fn run(self, builder: &SceneBuilder) -> ! {
        let mut current_scene = builder.build();
        
        self.event_loop.run(move |event, target, control_flow| {
            current_scene.process(event, target, &self.runtime,control_flow);
        });
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}
