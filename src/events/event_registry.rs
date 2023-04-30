use std::{
    any::TypeId,
    collections::{HashMap, HashSet, VecDeque},
};

use lazy_static::lazy_static;
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, Event, VirtualKeyCode, WindowEvent},
    window::WindowId,
};

use crate::components::resources::Resource;

use super::matrix_event::{MatrixEvent, MatrixEventReceiver};

#[derive(Default)]
pub struct WindowEventRegistry {
    keys: HashSet<VirtualKeyCode>,
    down_keys: HashSet<VirtualKeyCode>,
    up_keys: HashSet<VirtualKeyCode>,
    new_size: Option<PhysicalSize<u32>>,
    close_requested: bool,
}

lazy_static! {
    static ref EMPTY_WINDOW_EVENTS: WindowEventRegistry = WindowEventRegistry::default();
}

impl WindowEventRegistry {
    pub(crate) fn push(&mut self, event: WindowEvent<'_>) {
        match event {
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
            WindowEvent::Resized(size) => {
                self.new_size = Some(size);
            }
            WindowEvent::CloseRequested => {
                self.close_requested = true;
            }
            _ => {}
        };
    }
    pub(crate) fn update(&mut self) {
        self.down_keys.clear();
        self.up_keys.clear();
        self.new_size.take();
        self.close_requested = false;
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
    pub fn is_resized(&self) -> Option<&PhysicalSize<u32>> {
        self.new_size.as_ref()
    }
    pub fn should_close(&self) -> bool {
        self.close_requested
    }
}

pub struct EventRegistry {
    windows: HashMap<WindowId, WindowEventRegistry>,
    matrix_events: VecDeque<MatrixEvent>,
}

impl EventRegistry {
    pub fn new() -> Self {
        Self {
            windows: Default::default(),
            matrix_events: Default::default(),
        }
    }

    pub(crate) fn update(&mut self, recv: &MatrixEventReceiver) {
        for i in &mut self.windows {
            i.1.update();
        }
        self.matrix_events.clear();
        for i in recv.iter_current() {
            self.matrix_events.push_back(i);
        }
    }

    fn push_window_event(&mut self, id: WindowId, event: WindowEvent<'_>) {
        let events = self.windows.entry(id).or_default();
        events.push(event);
    }
    pub(crate) fn push<T>(&mut self, event: Event<'_, T>) {
        if let Event::WindowEvent { window_id, event } = event {
            self.push_window_event(window_id, event)
        }
    }

    pub fn get_window_events(&self, id: WindowId) -> &WindowEventRegistry {
        self.windows.get(&id).unwrap_or(&EMPTY_WINDOW_EVENTS)
    }

    pub fn is_resource_created<T: Resource + 'static>(&self) -> bool {
        let id = TypeId::of::<T>();
        for i in &self.matrix_events {
            if let MatrixEvent::CreatedResource(other_id) = i {
                if &id == other_id {
                    return true;
                }
            }
        }
        false
    }
}

impl Default for EventRegistry {
    fn default() -> Self {
        Self::new()
    }
}
