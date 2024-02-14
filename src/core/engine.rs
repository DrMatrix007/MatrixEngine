use winit::event_loop::{self, EventLoop};

pub struct Engine {
    event_loop: EventLoop<()>,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            event_loop: EventLoop::new().unwrap(),
        }
    }

    pub fn run(self) {
        self.event_loop.run(|_event, _target| {}).unwrap();
    }

    pub fn event_loop(&self) -> &EventLoop<()> {
        &self.event_loop
    }
}
