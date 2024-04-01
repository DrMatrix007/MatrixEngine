use winit::event_loop::EventLoop;

use super::scene::Scene;

pub struct Engine {
    event_loop: EventLoop<()>,
    scene: Scene
}

impl Engine {
    pub fn new(scene: Scene) -> Self {
        Self {
            event_loop: EventLoop::new().unwrap(),
            scene,
        }
    }

    pub fn run(mut self) {
        self.event_loop.run(|_event, _target| {
            self.scene.update();
        }).unwrap();
    }

    pub fn event_loop(&self) -> &EventLoop<()> {
        &self.event_loop
    }
}

