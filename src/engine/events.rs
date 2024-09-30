use std::{
    any::TypeId,
    collections::{HashMap, HashSet, VecDeque},
    fmt::Debug,
};

use winit::{
    event::{ElementState, WindowEvent},
    event_loop::{EventLoopClosed, EventLoopProxy},
    keyboard::{KeyCode, PhysicalKey},
};

use super::{
    data_state::{DataState, DataStateAccessError, ReadDataState},
    entity::SystemEntity,
};

#[derive(Debug, Clone)]
pub enum MatrixEvent<Custom: MatrixEventable> {
    Exit,
    DestroySystem(SystemEntity),
    ResourceCreated(TypeId),
    Custom(Custom),
}

pub trait MatrixEventable: Clone + Send + Sync + Debug + 'static {}
impl<T: Clone + Send + Sync + Debug + 'static> MatrixEventable for T {}

#[derive(Debug)]
pub struct Events<CustomEvents: MatrixEventable> {
    currently_pressed: HashSet<KeyCode>,
    just_pressed: HashSet<KeyCode>,
    just_released: HashSet<KeyCode>,
    close_requested: bool,
    matrix_events: VecDeque<MatrixEvent<CustomEvents>>,
}

impl<CustomEvents: MatrixEventable> Default for Events<CustomEvents> {
    fn default() -> Self {
        Self {
            currently_pressed: Default::default(),
            just_pressed: Default::default(),
            just_released: Default::default(),
            close_requested: Default::default(),
            matrix_events: Default::default(),
        }
    }
}

impl<CustomEvents: MatrixEventable> Events<CustomEvents> {
    pub fn new() -> Self {
        Events {
            currently_pressed: HashSet::new(),
            just_pressed: HashSet::new(),
            just_released: HashSet::new(),
            close_requested: false,
            matrix_events: VecDeque::new(),
        }
    }

    pub fn handle_window_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(keycode) = event.physical_key {
                    match event.state {
                        ElementState::Pressed if !event.repeat => self.on_key_press(keycode),
                        ElementState::Released => self.on_key_release(keycode),
                        _ => {}
                    }
                }
            }
            WindowEvent::CloseRequested => {
                self.close_requested = true;
            }
            _ => (),
        }
    }
    pub fn handle_matrix_event(&mut self, event: MatrixEvent<CustomEvents>) {
        self.matrix_events.push_back(event);
    }

    pub fn matrix_events(&self) -> impl Iterator<Item = &MatrixEvent<CustomEvents>> {
        self.matrix_events.iter()
    }

    pub fn reset(&mut self) {
        self.just_pressed.clear();
        self.just_released.clear();
        self.close_requested = false;
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

#[derive(Debug)]
pub struct EventWriter<CustomEvents: MatrixEventable> {
    proxy: EventLoopProxy<MatrixEvent<CustomEvents>>,
}

impl<CustomEvents: MatrixEventable> EventWriter<CustomEvents> {
    pub fn new(proxy: EventLoopProxy<MatrixEvent<CustomEvents>>) -> Self {
        Self { proxy }
    }
    pub fn send(
        &self,
        event: MatrixEvent<CustomEvents>,
    ) -> Result<(), EventLoopClosed<MatrixEvent<CustomEvents>>> {
        self.proxy.send_event(event)
    }
}

#[derive(Debug)]
pub struct EventRegistry<CustomEvents: MatrixEventable> {
    events: HashMap<SystemEntity, DataState<Events<CustomEvents>>>,
    event_loop_proxy: Option<DataState<EventWriter<CustomEvents>>>,
}

impl<CustomEvents: MatrixEventable> EventRegistry<CustomEvents> {
    pub fn new(event_loop_proxy: Option<EventLoopProxy<MatrixEvent<CustomEvents>>>) -> Self {
        Self {
            events: HashMap::new(),
            event_loop_proxy: event_loop_proxy.map(|x| DataState::new(EventWriter::new(x))),
        }
    }
    pub fn new_with_events(event_loop_proxy: EventLoopProxy<MatrixEvent<CustomEvents>>) -> Self {
        Self::new(Some(event_loop_proxy))
    }
    pub fn new_no_events() -> Self {
        Self::new(None)
    }

    pub fn get_reader(
        &mut self,
        e: SystemEntity,
    ) -> Result<ReadDataState<Events<CustomEvents>>, DataStateAccessError> {
        self.events.entry(e).or_default().read()
    }
    pub fn check_reader(&mut self, e: &SystemEntity) -> bool {
        self.events.get(e).map(|x| x.can_read()).unwrap_or(true) // the unwrap_or(true) is when the events will be created and thus available for fetching
    }
    pub fn consume_reader(
        &mut self,
        e: &SystemEntity,
        data: ReadDataState<Events<CustomEvents>>,
    ) -> Result<(), DataStateAccessError> {
        self.events
            .get_mut(e)
            .expect("missing events. this should not happend 💀")
            .consume_read(data)
    }

    pub fn get_writer(
        &mut self,
    ) -> Option<Result<ReadDataState<EventWriter<CustomEvents>>, DataStateAccessError>> {
        self.event_loop_proxy.as_mut().map(|x| x.read())
    }
    pub fn check_writer(&mut self) -> bool {
        self.event_loop_proxy
            .as_mut()
            .map(|x| x.can_read())
            .unwrap_or(true)
    }
    pub fn consume_writer(
        &mut self,
        data: ReadDataState<EventWriter<CustomEvents>>,
    ) -> Result<(), DataStateAccessError> {
        if let Some(proxy) = &mut self.event_loop_proxy {
            proxy.consume_read(data)
        } else {
            panic!("consumed an event writer, whene there is no writer at all")
        }
    }

    pub(crate) fn iter_events(
        &mut self,
    ) -> impl Iterator<Item = (&SystemEntity, &mut DataState<Events<CustomEvents>>)> {
        self.events.iter_mut()
    }
}
