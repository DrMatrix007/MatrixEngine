use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    sync::mpsc::{channel, Receiver, Sender},
    time::{Duration, Instant},
};

use lazy_static::lazy_static;
use winit::{
    dpi::PhysicalSize,
    event::{DeviceEvent, ElementState, Event, MouseButton, VirtualKeyCode, WindowEvent},
    window::WindowId,
};

use super::engine_event::EngineEvent;

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
    size: PhysicalSize<u32>,
    close_requested: bool,
}

lazy_static! {
    static ref EMPTY_WINDOW_EVENTS: WindowEventRegistry = WindowEventRegistry::default();
}

impl WindowEventRegistry {
    pub(crate) fn push(&mut self, event: &WindowEvent<'_>) {
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
                self.size = *size;
            }
            WindowEvent::CloseRequested => {
                self.close_requested = true;
            }
            WindowEvent::MouseInput { state, button, .. } => match state {
                ElementState::Pressed => self.mouse.insert(*button),
                ElementState::Released => self.mouse.remove(*button),
            },
            _ => {}
        };
    }
    pub(crate) fn update(&mut self) {
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
    pub fn is_resized(&self) -> &PhysicalSize<u32> {
        &self.size
    }
    pub fn should_close(&self) -> bool {
        self.close_requested
    }

    pub(crate) fn size(&self) -> PhysicalSize<u32> {
        self.size
    }
}

pub struct EventRegistry {
    windows: HashMap<WindowId, WindowEventRegistry>,
    start: Instant,
    mouse_delta: (f64, f64),
}

impl EventRegistry {
    pub fn new() -> Self {
        Self {
            windows: Default::default(),
            start: Instant::now(),
            mouse_delta: (0., 0.),
        }
    }

    pub(crate) fn update(&mut self) {
        for i in &mut self.windows {
            i.1.update();
        }

        self.start = Instant::now();
        self.mouse_delta = (0.0, 0.0);
    }

    fn process_window_event(&mut self, id: &WindowId, event: &WindowEvent<'_>) {
        let events = self.windows.entry(*id).or_default();
        events.push(event);
    }
    pub(crate) fn process<T>(&mut self, event: &Event<'_, T>) {
        match event {
            Event::WindowEvent { window_id, event } => self.process_window_event(window_id, event),
            Event::DeviceEvent { event, .. } => {
                self.process_device_event(event);
            }
            _ => {}
        }
    }

    pub fn get_window_events(&self, id: WindowId) -> &WindowEventRegistry {
        self.windows.get(&id).unwrap_or(&EMPTY_WINDOW_EVENTS)
    }
    pub fn all_window_events(
        &self,
    ) -> std::collections::hash_map::Values<'_, WindowId, WindowEventRegistry> {
        self.windows.values()
    }

    pub fn calculate_delta_time(&self) -> Duration {
        Instant::now() - self.start
    }

    fn process_device_event(&mut self, event: &DeviceEvent) {
        if let DeviceEvent::MouseMotion { delta } = event {
            self.mouse_delta = *delta;
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

pub struct EventChannelRegistry {
    event_registry: EventRegistry,
    sender: Sender<Event<'static, EngineEvent>>,
    receiver: Receiver<Event<'static, EngineEvent>>,
}

impl AsRef<EventRegistry> for EventChannelRegistry {
    fn as_ref(&self) -> &EventRegistry {
        &self.event_registry
    }
}
impl AsMut<EventRegistry> for EventChannelRegistry {
    fn as_mut(&mut self) -> &mut EventRegistry {
        &mut self.event_registry
    }
}

impl EventChannelRegistry {
    pub fn new() -> (Self, Sender<Event<'static, EngineEvent>>) {
        let event_registry = EventRegistry::default();
        let (sender, receiver) = channel();
        (
            Self {
                event_registry,
                sender: sender.clone(),
                receiver,
            },
            sender,
        )
    }
    pub fn update_events_from_channel(&mut self) {
        for event in self.receiver.try_iter() {
            self.event_registry.process(&event);
        }
    }
}
