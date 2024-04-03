use super::{scene::Scene, window::Window};

pub struct Engine {
    scene: Scene,
}

impl Engine {
    pub fn new(scene: Scene) -> Self {
        Self {
            scene,
        }
    }

    pub fn run(mut self) {
    }

    pub fn update(&mut self, event:glfw::WindowEvent) {
    }

}
