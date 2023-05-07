use std::{
    any::TypeId,
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
    time::{Duration, Instant},
};

use lazy_static::lazy_static;
use winit::{
    dpi::PhysicalSize,
    event::{DeviceEvent, ElementState, Event, MouseButton, VirtualKeyCode, WindowEvent},
    window::WindowId,
};

use crate::components::resources::Resource;

use super::matrix_event::{MatrixEvent, MatrixEventReceiver};

struct ButtonEventGroup<T: Hash + Eq + Clone> {
    keys: HashSet<T>,
    down_keys: HashSet<T>,
    up_keys: HashSet<T>,
}
impl<T: Hash + Eq + Clone> ButtonEventGroup<T> {
    fn insert(&mut self, code: T) {
        self.keys.insert(code.clone());
        self.down_keys.insert(code);
    }
    fn remove(&mut self, code: T) {
        self.keys.remove(&code);
        self.up_keys.insert(code);
    }
    fn update(&mut self) {
        self.down_keys.clear();
        self.up_keys.clear();
    }

    fn contains(&self, k: &T) -> bool {
        self.keys.contains(k)
    }

    fn contains_down(&self, k: &T) -> bool {
        self.down_keys.contains(k)
    }

    fn contains_up(&self, k: &T) -> bool {
        self.up_keys.contains(k)
    }
}

impl<T: Hash + Eq + Clone> Default for ButtonEventGroup<T> {
    fn default() -> Self {
        Self {
            keys: Default::default(),
            down_keys: Default::default(),
            up_keys: Default::default(),
        }
    }
}

#[derive(Default)]
pub struct WindowEventRegistry {
    keybaord: ButtonEventGroup<VirtualKeyCode>,
    mouse: ButtonEventGroup<MouseButton>,
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
                        ElementState::Pressed => self.keybaord.insert(code),
                        ElementState::Released => self.keybaord.remove(code),
                    };
                }
            }
            WindowEvent::Resized(size) => {
                self.new_size = Some(size);
            }
            WindowEvent::CloseRequested => {
                self.close_requested = true;
            }
            WindowEvent::MouseInput { state, button, .. } => match state {
                ElementState::Pressed => self.mouse.insert(button),
                ElementState::Released => self.mouse.remove(button),
            },
            _ => {}
        };
    }
    pub(crate) fn update(&mut self) {
        self.new_size.take();
        self.close_requested = false;
        self.keybaord.update();
        self.mouse.update();
    }

    pub fn is_pressed(&self, k: VirtualKeyCode) -> bool {
        self.keybaord.contains(&k)
    }
    pub fn is_pressed_down(&self, k: VirtualKeyCode) -> bool {
        self.keybaord.contains_down(&k)
    }

    pub fn is_released(&self, k: VirtualKeyCode) -> bool {
        self.keybaord.contains_up(&k)
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
    start: Instant,
    mouse_delta: (f64, f64),
}

impl EventRegistry {
    pub fn new() -> Self {
        Self {
            windows: Default::default(),
            matrix_events: Default::default(),
            start: Instant::now(),
            mouse_delta: (0., 0.),
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
        self.start = Instant::now();
        self.mouse_delta = (0.0, 0.0);
    }

    fn push_window_event(&mut self, id: WindowId, event: WindowEvent<'_>) {
        let events = self.windows.entry(id).or_default();
        events.push(event);
    }
    pub(crate) fn push<T>(&mut self, event: Event<'_, T>) {
        match event {
            Event::WindowEvent { window_id, event } => self.push_window_event(window_id, event),
            Event::DeviceEvent { event, .. } => {
                self.push_device_event(event);
            }
            _ => {}
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
    pub fn calculate_delta_time(&self) -> Duration {
        Instant::now() - self.start
    }

    fn push_device_event(&mut self, event: DeviceEvent) {
        if let DeviceEvent::MouseMotion { delta } = event {
            self.mouse_delta = delta;
        }
    }

    pub fn mouse_delta(&self) -> (f64, f64) {
        self.mouse_delta
    }
}

impl Default for EventRegistry {
    fn default() -> Self {
        Self::new()
    }
}
