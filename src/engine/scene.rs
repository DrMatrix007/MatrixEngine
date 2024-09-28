use std::marker::PhantomData;

use winit::application::ApplicationHandler;

use super::{components::ComponentRegistry, systems::SystemRegistry, MatrixEvent};

pub struct SceneRegistry {
    pub components: ComponentRegistry,
}

struct SendEngineArgs;
struct NonSendEngineArgs;

pub struct Scene<CustomEvents> {
    marker: PhantomData<CustomEvents>,
    registry: SceneRegistry,
    systems: SystemRegistry<SceneRegistry, SendEngineArgs, NonSendEngineArgs>,
}

impl<T> Scene<T> {
    pub fn new() -> Self {
        Self {
            marker: PhantomData,
            registry: SceneRegistry {
                components: ComponentRegistry::new(),
            },
            systems: SystemRegistry::new(),
        }
    }
}

impl<T> Default for Scene<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SceneManager<CustomEvents> {
    current_scene: Scene<CustomEvents>,
    marker: PhantomData<CustomEvents>,
}

impl<T> SceneManager<T> {
    pub fn new(scene: Scene<T>) -> Self {
        Self {
            current_scene: scene,
            marker: PhantomData,
        }
    }
}
impl<Custom: 'static> ApplicationHandler<MatrixEvent<Custom>> for SceneManager<Custom> {
    fn resumed(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {}

    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        println!("event: {:?}", event);
    }
}
