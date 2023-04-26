use std::{
    collections::{HashSet},
};

use winit::event::{ElementState, Event,VirtualKeyCode, WindowEvent};

pub struct Events {
    keys: HashSet<VirtualKeyCode>,
    down_keys: HashSet<VirtualKeyCode>,
}

impl Default for Events {
    fn default() -> Self {
        Self {
            keys: HashSet::new(),
            down_keys: HashSet::new(),
        }
    }
}

impl Events {
    pub(crate) fn push<'a, T>(&mut self, event: Event<'a, T>) {
        match event {
            Event::WindowEvent { window_id:_, event } => match event {
                WindowEvent::KeyboardInput {
                    device_id: _,
                    input,
                    is_synthetic: _,
                } => {
                    if let Some(code) = input.virtual_keycode {
                        match input.state {
                            ElementState::Pressed => {
                                self.keys.insert(code);
                                self.down_keys.insert(code);
                            }
                            ElementState::Released => {
                                self.keys.remove(&code);
                            }
                        };
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }
    pub(crate) fn update(&mut self) {
        self.down_keys.clear();
    }
}
