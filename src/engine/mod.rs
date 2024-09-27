pub mod components;
pub mod entity;
pub mod scene;
pub mod query;
pub mod systems;

use scene::{Scene, SceneManager};
use winit::{error::EventLoopError, event_loop::EventLoop};

pub enum MatrixEvent<Custom> {
    Custom(Custom),
}

pub struct Engine<CustomEvents: 'static = ()> {
    event_loop: EventLoop<MatrixEvent<CustomEvents>>,
    scene: SceneManager<CustomEvents>,
}

impl<CustomEvents: 'static> Engine<CustomEvents> {
    pub fn new() -> Self {
        Self::with_scene(Scene::new())
    }
    pub fn with_scene(scene: Scene<CustomEvents>) -> Self {
        Engine {
            event_loop: EventLoop::with_user_event().build().unwrap(),
            scene: SceneManager::new(scene),
        }
    }

    pub fn run(mut self) -> Result<(), EventLoopError> {
        self.event_loop.run_app(&mut self.scene)
    }
    
    pub fn event_loop(&self) -> &EventLoop<MatrixEvent<CustomEvents>> {
        &self.event_loop
    }
}

impl<CustomEvents: 'static> Default for Engine<CustomEvents> {
    fn default() -> Self {
        Self::new()
    }
}
