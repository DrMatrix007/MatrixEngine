use std::marker::PhantomData;

use winit::application::ApplicationHandler;

use super::{components::ComponentRegistry, MatrixEvent};

pub struct SceneRegistry {
    pub components: ComponentRegistry,
}
pub struct Scene<Custom> {
    marker: PhantomData<Custom>,
    registry: SceneRegistry,
}

impl<T> Scene<T> {
    pub fn new() -> Self {
        Self {
            marker: PhantomData,
            registry: SceneRegistry {
                components: ComponentRegistry::new(),
            },
        }
    }
}

impl<T> Default for Scene<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SceneManager<Custom> {
    current_scene: Scene<Custom>,
    marker: PhantomData<Custom>,
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
        _event: winit::event::WindowEvent,
    ) {
    }
}
