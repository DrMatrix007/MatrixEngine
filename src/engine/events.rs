use std::{
    any::TypeId,
    collections::{HashSet, VecDeque},
    fmt::Debug,
    hash::Hash,
    time::Instant,
};

use winit::{
    dpi::PhysicalSize,
    event::{DeviceEvent, ElementState, MouseButton, MouseScrollDelta, WindowEvent},
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
    ChangedResource(TypeId),
    Custom(Custom),
}

pub trait MatrixEventable: Clone + Send + Sync + Debug + 'static {}
impl<T: Clone + Send + Sync + Debug + 'static> MatrixEventable for T {}

#[derive(Debug)]
pub struct Events<CustomEvents: MatrixEventable> {
    keyboard_input: InputManager<KeyCode>, // Input manager for keyboard
    mouse_input: InputManager<MouseButton>, // Input manager for mouse buttons
    close_requested: bool,
    matrix_events: VecDeque<MatrixEvent<CustomEvents>>,
    new_inner_size: Option<PhysicalSize<u32>>,
    mouse_dx: (f32, f32),   // Mouse delta (movement)
    mouse_pos: (f32, f32),  // Mouse position
    mouse_wheel_delta: f32, // Mouse wheel delta
    start_frame: Instant,
    dt: f32,
}

impl<CustomEvents: MatrixEventable> Default for Events<CustomEvents> {
    fn default() -> Self {
        Self::new()
    }
}

impl<CustomEvents: MatrixEventable> Events<CustomEvents> {
    pub fn new() -> Self {
        Events {
            matrix_events: VecDeque::new(),
            keyboard_input: InputManager::new(),
            mouse_input: InputManager::new(),
            mouse_pos: (0., 0.),
            mouse_wheel_delta: 0.,
            close_requested: false,
            new_inner_size: None,
            dt: 0.,
            start_frame: Instant::now(),
            mouse_dx: (0., 0.),
        }
    }

    pub fn handle_window_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(keycode) = event.physical_key {
                    match event.state {
                        ElementState::Pressed if !event.repeat => {
                            self.keyboard_input.on_press(keycode)
                        }
                        ElementState::Released => self.keyboard_input.on_release(keycode),
                        _ => {}
                    }
                }
            }
            WindowEvent::MouseInput { state, button, .. } => match state {
                ElementState::Pressed => self.mouse_input.on_press(*button),
                ElementState::Released => self.mouse_input.on_release(*button),
            },
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_pos = (position.x as _, position.y as _);
            }
            WindowEvent::CloseRequested => {
                self.close_requested = true;
            }
            WindowEvent::Resized(size) => {
                self.new_inner_size = Some(*size);
            }

            _ => (),
        }
    }
    pub fn handle_matrix_event(&mut self, event: MatrixEvent<CustomEvents>) {
        self.matrix_events.push_back(event);
    }
    pub(crate) fn handle_device_event(&mut self, event: DeviceEvent) {
        match event {
            DeviceEvent::MouseMotion { delta: (x, y) } => self.mouse_dx = (x as _, y as _),
            DeviceEvent::MouseWheel { delta } => {
                self.mouse_wheel_delta = match delta {
                    MouseScrollDelta::LineDelta(x, y) => y,
                    MouseScrollDelta::PixelDelta(physical_position) => physical_position.y as _,
                }
            }
            DeviceEvent::Added => {
                println!("added device");
            }
            _ => {}
        }
    }

    pub fn matrix_events(&self) -> impl Iterator<Item = &MatrixEvent<CustomEvents>> {
        self.matrix_events.iter()
    }

    pub fn reset(&mut self) {
        self.keyboard_input.reset();
        self.mouse_input.reset();
        self.close_requested = false;
        self.new_inner_size = None;

        let now = Instant::now();
        self.dt = (now - self.start_frame).as_secs_f32();
        self.start_frame = now;

        self.mouse_dx = (0., 0.);
        self.mouse_wheel_delta = 0.;
    }

    pub fn new_inner_size(&self) -> Option<&PhysicalSize<u32>> {
        self.new_inner_size.as_ref()
    }

    pub fn close_requested(&self) -> bool {
        self.close_requested
    }

    pub fn dt(&self) -> f32 {
        self.dt
    }

    pub fn keyboard(&self) -> &InputManager<KeyCode> {
        &self.keyboard_input
    }

    pub fn mouse(&self) -> &InputManager<MouseButton> {
        &self.mouse_input
    }

    pub fn mouse_dx(&self) -> (f32, f32) {
        self.mouse_dx
    }

    pub fn mouse_wheel_delta(&self) -> f32 {
        self.mouse_wheel_delta
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
    events: DataState<Events<CustomEvents>>,
    event_loop_proxy: Option<DataState<EventWriter<CustomEvents>>>,
}

impl<CustomEvents: MatrixEventable> EventRegistry<CustomEvents> {
    pub fn new(event_loop_proxy: Option<EventLoopProxy<MatrixEvent<CustomEvents>>>) -> Self {
        Self {
            events: DataState::new(Events::new()),
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
    ) -> Result<ReadDataState<Events<CustomEvents>>, DataStateAccessError> {
        self.events.read()
    }
    pub fn check_reader(&mut self) -> bool {
        self.events.can_read()
    }
    pub fn consume_reader(
        &mut self,
        data: ReadDataState<Events<CustomEvents>>,
    ) -> Result<(), DataStateAccessError> {
        self.events.consume_read(data)
    }

    pub fn get_writer(
        &mut self,
    ) -> Option<Result<ReadDataState<EventWriter<CustomEvents>>, DataStateAccessError>> {
        self.event_loop_proxy.as_mut().map(|x| x.read())
    }
    pub fn check_writer(&mut self) -> Option<bool> {
        self.event_loop_proxy.as_mut().map(|x| x.can_read())
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

    pub fn events(&mut self) -> &mut DataState<Events<CustomEvents>> {
        &mut self.events
    }
}

#[derive(Debug)]
pub struct InputManager<T: Eq + Hash + Copy> {
    currently_pressed: HashSet<T>,
    just_pressed: HashSet<T>,
    just_released: HashSet<T>,
}

impl<T: Eq + Hash + Copy> InputManager<T> {
    // Create a new InputManager
    pub fn new() -> Self {
        Self {
            currently_pressed: HashSet::new(),
            just_pressed: HashSet::new(),
            just_released: HashSet::new(),
        }
    }

    // Handle input press event
    pub fn on_press(&mut self, input: T) {
        self.just_pressed.insert(input);
        self.currently_pressed.insert(input);
    }

    // Handle input release event
    pub fn on_release(&mut self, input: T) {
        self.just_released.insert(input);
        self.currently_pressed.remove(&input);
    }

    // Check if the input is currently pressed
    pub fn is_pressed(&self, input: T) -> bool {
        self.currently_pressed.contains(&input)
    }

    // Check if the input was just pressed in this frame
    pub fn is_just_pressed(&self, input: T) -> bool {
        self.just_pressed.contains(&input)
    }

    // Check if the input was just released in this frame
    pub fn is_just_released(&self, input: T) -> bool {
        self.just_released.contains(&input)
    }

    // Reset just_pressed and just_released for the next frame
    pub fn reset(&mut self) {
        self.just_pressed.clear();
        self.just_released.clear();
    }
}

impl<T: Eq + Hash + Copy> Default for InputManager<T> {
    fn default() -> Self {
        Self::new()
    }
}
