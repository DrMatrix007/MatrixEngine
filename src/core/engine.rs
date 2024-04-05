use super::scene::Scene;

pub struct Engine {
    scene: Scene,
    running: bool,
}

impl Engine {
    pub fn new(scene: Scene) -> Self {
        Self {
            scene,
            running: true,
        }
    }

    pub fn run(mut self) {
        while self.running {
            self.scene.update();
        }
    }

    // pub fn update(&mut self, event: glfw::WindowEvent) {}
}
