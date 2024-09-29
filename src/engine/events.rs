use std::{
    collections::{HashMap, HashSet},
    marker::PhantomData,
};

use winit::{
    event::{ElementState, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
};

use super::{
    data_state::{DataState, DataStateAccessError, ReadDataState},
    entity::Entity,
};

pub enum MatrixEvent<Custom: MatrixEventable> {
    Custom(Custom),
}


pub trait MatrixEventable: Send + Sync+'static {}
impl<T: Send + Sync+'static> MatrixEventable for T {}

#[derive(Debug, Default)]
pub struct Events {
    currently_pressed: HashSet<KeyCode>,
    just_pressed: HashSet<KeyCode>,
    just_released: HashSet<KeyCode>,
    close_requested: bool,
}

impl Events {
    pub fn new() -> Self {
        Events {
            currently_pressed: HashSet::new(),
            just_pressed: HashSet::new(),
            just_released: HashSet::new(),
            close_requested: false,
        }
    }

    pub fn handle_event(&mut self, event: &WindowEvent) {
        if let WindowEvent::KeyboardInput { event, .. } = event {
            if let PhysicalKey::Code(keycode) = event.physical_key {
                match event.state {
                    ElementState::Pressed if !event.repeat => self.on_key_press(keycode),
                    ElementState::Released => self.on_key_release(keycode),
                    _ => {}
                }
            }
        }
    }

    // This method simulates receiving a key press event in the current frame
    fn on_key_press(&mut self, key: KeyCode) {
        self.just_pressed.insert(key);
        self.currently_pressed.insert(key);
    }

    // This method simulates receiving a key release event in the current frame
    fn on_key_release(&mut self, key: KeyCode) {
        self.just_released.insert(key);
        self.currently_pressed.remove(&key);
    }

    // Reset just_pressed and just_released for the next frame
    pub fn reset(&mut self) {
        self.just_pressed.clear();
        self.just_released.clear();
    }

    // Check if a key is currently being pressed
    pub fn is_pressed(&self, key: KeyCode) -> bool {
        self.currently_pressed.contains(&key)
    }

    // Check if a key was just pressed in the current frame
    pub fn is_just_pressed(&self, key: KeyCode) -> bool {
        self.just_pressed.contains(&key)
    }

    // Check if a key was just released in the current frame
    pub fn is_just_released(&self, key: KeyCode) -> bool {
        self.just_released.contains(&key)
    }

    pub fn close_requested(&self) -> bool {
        self.close_requested
    }
}

pub struct EventsState {}

#[derive(Debug)]
pub struct EventRegistry<CustomEvents> {
    events: HashMap<Entity, DataState<Events>>,
    // senders: HashMap<Entity,DataState<>>
    marker: PhantomData<CustomEvents>,
}

impl<CustomEvents> EventRegistry<CustomEvents> {
    pub fn new() -> Self {
        Self {
            events: HashMap::new(),
            marker: PhantomData,
        }
    }

    pub fn read(&mut self, e: Entity) -> Result<ReadDataState<Events>, DataStateAccessError> {
        self.events.entry(e).or_default().read()
    }
    pub fn check_read(&mut self, e: &Entity) -> bool {
        self.events.get(e).map(|x| x.can_read()).unwrap_or(true) // the unwrap_or(true) is when the events will be created and thus available for fetching
    }
    pub fn consume_read(
        &mut self,
        e: &Entity,
        data: ReadDataState<Events>,
    ) -> Result<(), DataStateAccessError> {
        self.events
            .get_mut(e)
            .expect("missing events. this should not happend 💀")
            .consume_read(data)
    }

    pub(crate) fn iter_events(
        &mut self,
    ) -> impl Iterator<Item = (&Entity, &mut DataState<Events>)> {
        self.events.iter_mut()
    }
}

impl<T> Default for EventRegistry<T> {
    fn default() -> Self {
        Self::new()
    }
}
