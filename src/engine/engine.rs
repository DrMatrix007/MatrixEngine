use winit::event_loop::EventLoop;

use super::scene::SceneBuilder;

pub struct Engine {
    event_loop: EventLoop<()>,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            event_loop: EventLoop::new(),
        }
    }

    pub fn run(self, builder: &SceneBuilder) -> ! {
        let mut current_scene = builder.build();

        self.event_loop.run(move |event, target, control_flow| {
            current_scene.process(event, target, control_flow);
        });
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}
