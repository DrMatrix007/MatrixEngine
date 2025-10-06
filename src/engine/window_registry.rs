use std::collections::HashMap;

use winit::window::{Window, WindowId};

#[derive(Debug, Default)]
pub struct WindowRegistry {
    windows: HashMap<WindowId, Window>,
}

impl WindowRegistry {
    pub fn get(&self, k: &WindowId) -> Option<&Window> {
        self.windows.get(k)
    }
    pub fn get_mut(&mut self, k: &WindowId) -> Option<&mut Window> {
        self.windows.get_mut(k)
    }

    pub fn add_window(&mut self, window: Window) {
        self.windows.insert(window.id(), window);
    }
}
