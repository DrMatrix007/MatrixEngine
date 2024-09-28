use std::{marker::PhantomData, ops::Deref};

use winit::{application::ApplicationHandler, event_loop::ActiveEventLoop};

use super::{
    components::ComponentRegistry,
    runtimes::Runtime,
    systems::{IntoNonSendSystem, IntoSendSystem, SystemRegistry},
    MatrixEvent,
};

pub struct SceneRegistry {
    pub components: ComponentRegistry,
}

pub struct ActiveEventLoopRef {
    ptr: *const ActiveEventLoop,
}

impl ActiveEventLoopRef {
    pub fn new(ptr: *const ActiveEventLoop) -> Self {
        Self { ptr }
    }
}
impl Deref for ActiveEventLoopRef {
    type Target = ActiveEventLoop;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr }
    }
}

pub struct SendEngineStartupArgs;
pub struct NonSendEngineStartupArgs {
    pub event_loop: ActiveEventLoopRef,
}
pub struct SendEngineArgs;
pub struct NonSendEngineArgs;

pub struct Scene<CustomEvents> {
    marker: PhantomData<CustomEvents>,
    registry: SceneRegistry,
    systems: SystemRegistry<SceneRegistry, SendEngineArgs, NonSendEngineArgs>,
    startup_systems: SystemRegistry<SceneRegistry, SendEngineStartupArgs, NonSendEngineStartupArgs>,
}

impl<T> Scene<T> {
    pub fn new() -> Self {
        Self {
            marker: PhantomData,
            registry: SceneRegistry {
                components: ComponentRegistry::new(),
            },
            systems: SystemRegistry::new(),
            startup_systems: SystemRegistry::new(),
        }
    }

    pub fn add_send_system<P: 'static>(
        &mut self,
        sys: impl IntoSendSystem<SceneRegistry, SendEngineArgs, P> + 'static,
    ) {
        self.systems.add_send_system(Box::new(sys.into_system()));
    }

    pub fn add_non_send_system<P: 'static>(
        &mut self,
        sys: impl IntoNonSendSystem<SceneRegistry, NonSendEngineArgs, P> + 'static,
    ) {
        self.systems
            .add_non_send_system(Box::new(sys.into_system()));
    }

    pub fn add_send_startup_system<P: 'static>(
        &mut self,
        sys: impl IntoSendSystem<SceneRegistry, SendEngineStartupArgs, P> + 'static,
    ) {
        self.startup_systems
            .add_send_system(Box::new(sys.into_system()));
    }

    pub fn add_non_send_startup_system<P: 'static>(
        &mut self,
        sys: impl IntoNonSendSystem<SceneRegistry, NonSendEngineStartupArgs, P> + 'static,
    ) {
        self.startup_systems
            .add_non_send_system(Box::new(sys.into_system()));
    }

    pub fn components(&self) -> &ComponentRegistry {
        &self.registry.components
    }
    pub fn components_mut(&mut self) -> &mut ComponentRegistry {
        &mut self.registry.components
    }
}

impl<T> Default for Scene<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SceneManager<CustomEvents> {
    current_scene: Scene<CustomEvents>,
    runtime: Box<dyn Runtime<SceneRegistry, SendEngineArgs, NonSendEngineArgs>>,
    startup_runtime:
        Box<dyn Runtime<SceneRegistry, SendEngineStartupArgs, NonSendEngineStartupArgs>>,
    marker: PhantomData<CustomEvents>,
}

impl<T> SceneManager<T> {
    pub fn new(
        runtime: Box<dyn Runtime<SceneRegistry, SendEngineArgs, NonSendEngineArgs>>,
        startup_runtime: Box<
            dyn Runtime<SceneRegistry, SendEngineStartupArgs, NonSendEngineStartupArgs>,
        >,
        scene: Scene<T>,
    ) -> Self {
        Self {
            current_scene: scene,
            runtime,
            startup_runtime,
            marker: PhantomData,
        }
    }
}
impl<Custom: 'static> ApplicationHandler<MatrixEvent<Custom>> for SceneManager<Custom> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.startup_runtime.run(
            &mut self.current_scene.startup_systems,
            &mut self.current_scene.registry,
            SendEngineStartupArgs,
            NonSendEngineStartupArgs {
                event_loop: ActiveEventLoopRef::new(event_loop),
            },
        );
    }

    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if event == winit::event::WindowEvent::RedrawRequested {
            self.runtime.run(
                &mut self.current_scene.systems,
                &mut self.current_scene.registry,
                SendEngineArgs,
                NonSendEngineArgs,
            );
        }
        // println!("event: {event:?}");
    }
}
