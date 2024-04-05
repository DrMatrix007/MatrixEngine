use super::{
    scene::Scene,
    window::{Window, WindowRegistry},
};

pub struct Engine {
    scene: Scene,
    running: bool,
    windows: WindowRegistry,
}

impl Engine {
    pub fn new(scene: Scene) -> Self {
        Self {
            scene,
            running: true,
            windows: WindowRegistry::new(),
        }
    }

    pub fn run(mut self, mut glfw: glfw::Glfw) {
        while self.running {
            for window in self.windows.values_mut() {
                window.swap_buffers()
            }

            glfw.poll_events();

            self.scene.update();

            for window in self.windows.values_mut() {
                for (_, event) in window.flush_events_iter() {
                    println!("{event:?}");
                }
            }
        }
    }

    pub fn add_window(&mut self, window: Window) {
        self.windows.add_window(window);
    }

    // pub fn update(&mut self, event: glfw::WindowEvent) {}
}
