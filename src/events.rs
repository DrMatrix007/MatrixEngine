use std::collections::HashSet;

use winit::event::{ElementState, Event, VirtualKeyCode, WindowEvent};

pub struct Events {
    keys: HashSet<VirtualKeyCode>,
    down_keys: HashSet<VirtualKeyCode>,
    up_keys: HashSet<VirtualKeyCode>,
}

impl Default for Events {
    fn default() -> Self {
        Self {
            keys: HashSet::new(),
            down_keys: HashSet::new(),
            up_keys: HashSet::new(),
        }
    }
}

impl Events {
    pub(crate) fn push<'a, T>(&mut self, event: Event<'a, T>) {
        match event {
            Event::WindowEvent {
                window_id: _,
                event,
            } => match event {
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
                                self.up_keys.insert(code);
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
        self.up_keys.clear();
    }

    pub fn is_pressed(&self, k: VirtualKeyCode) -> bool {
        self.keys.contains(&k)
    }
    pub fn is_pressed_down(&self, k: VirtualKeyCode) -> bool {
        self.down_keys.contains(&k)
    }

    pub fn is_released(&self, k: VirtualKeyCode) -> bool {
        self.up_keys.contains(&k)
    }
}
