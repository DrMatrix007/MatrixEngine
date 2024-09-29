use std::{collections::VecDeque, marker::PhantomData, ops::Deref};

use winit::{
    application::ApplicationHandler,
    event_loop::{ActiveEventLoop, EventLoopProxy},
};

use super::{
    components::ComponentRegistry,
    data_state::{DataState, DataStateAccessError, WriteDataState},
    events::{EventRegistry, MatrixEvent, MatrixEventable},
    plugins::Plugin,
    resources::ResourceRegistry,
    runtimes::Runtime,
    systems::{IntoNonSendSystem, IntoSendSystem, SystemRegistry},
};

#[derive(Debug)]
pub struct SceneRegistryRefs<CustomEvents: MatrixEventable = ()> {
    pub components: WriteDataState<ComponentRegistry>,
    pub events: WriteDataState<EventRegistry<CustomEvents>>,
    pub resources: WriteDataState<ResourceRegistry>,
}

pub(crate) struct DummySceneRegistry<CustomEvents: MatrixEventable = ()> {
    pub registry: SceneRegistryRefs<CustomEvents>,
    components: DataState<ComponentRegistry>,
    events: DataState<EventRegistry<CustomEvents>>,
    resources: DataState<ResourceRegistry>,
}

impl<CustomEvents: MatrixEventable> SceneRegistryRefs<CustomEvents> {
    pub(crate) fn dummy() -> DummySceneRegistry<CustomEvents> {
        DummySceneRegistry::new()
    }
}

impl<CustomEvents: MatrixEventable> DummySceneRegistry<CustomEvents> {
    pub(crate) fn new() -> Self {
        let mut components = DataState::default();
        let mut events = DataState::new(EventRegistry::new_no_events());
        let mut resources = DataState::default();
        DummySceneRegistry {
            registry: SceneRegistryRefs {
                components: components.write().unwrap(),
                events: events.write().unwrap(),
                resources: resources.write().unwrap(),
            },
            components,
            events,
            resources,
        }
    }
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

pub struct Scene<CustomEvents: MatrixEventable> {
    marker: PhantomData<CustomEvents>,
    components: DataState<ComponentRegistry>,
    events: DataState<EventRegistry<CustomEvents>>,
    systems: SystemRegistry<SceneRegistryRefs<CustomEvents>, SendEngineArgs, NonSendEngineArgs>,
    startup_systems: SystemRegistry<
        SceneRegistryRefs<CustomEvents>,
        SendEngineStartupArgs,
        NonSendEngineStartupArgs,
    >,
    plugins: VecDeque<Box<dyn Plugin<CustomEvents>>>,
}

impl<CustomEvents: MatrixEventable> Scene<CustomEvents> {
    pub fn new(proxy: EventLoopProxy<MatrixEvent<CustomEvents>>) -> Self {
        Self {
            marker: PhantomData,
            components: Default::default(),
            events: DataState::new(EventRegistry::new_with_events(proxy)),
            systems: SystemRegistry::new(),
            startup_systems: SystemRegistry::new(),
            plugins: VecDeque::new(),
        }
    }

    pub fn add_send_system<P: 'static>(
        &mut self,
        sys: impl IntoSendSystem<SceneRegistryRefs<CustomEvents>, SendEngineArgs, P> + 'static,
    ) {
        self.systems.add_send_system(Box::new(sys.into_system()));
    }

    pub fn add_non_send_system<P: 'static>(
        &mut self,
        sys: impl IntoNonSendSystem<SceneRegistryRefs<CustomEvents>, NonSendEngineArgs, P> + 'static,
    ) {
        self.systems
            .add_non_send_system(Box::new(sys.into_system()));
    }

    pub fn add_send_startup_system<P: 'static>(
        &mut self,
        sys: impl IntoSendSystem<SceneRegistryRefs<CustomEvents>, SendEngineStartupArgs, P> + 'static,
    ) {
        self.startup_systems
            .add_send_system(Box::new(sys.into_system()));
    }

    pub fn add_non_send_startup_system<P: 'static>(
        &mut self,
        sys: impl IntoNonSendSystem<SceneRegistryRefs<CustomEvents>, NonSendEngineStartupArgs, P>
            + 'static,
    ) {
        self.startup_systems
            .add_non_send_system(Box::new(sys.into_system()));
    }

    pub fn components(&self) -> Result<&ComponentRegistry, DataStateAccessError> {
        self.components.get()
    }
    pub fn components_mut(&mut self) -> Result<&mut ComponentRegistry, DataStateAccessError> {
        self.components.get_mut()
    }
}

pub struct SceneManager<CustomEvents: MatrixEventable> {
    current_scene: Scene<CustomEvents>,
    runtime: Box<dyn Runtime<SceneRegistryRefs<CustomEvents>, SendEngineArgs, NonSendEngineArgs>>,
    startup_runtime: Box<
        dyn Runtime<
            SceneRegistryRefs<CustomEvents>,
            SendEngineStartupArgs,
            NonSendEngineStartupArgs,
        >,
    >,
    resources: DataState<ResourceRegistry>,
    marker: PhantomData<CustomEvents>,
}

impl<CustomEvents: MatrixEventable> SceneManager<CustomEvents> {
    pub fn new(
        runtime: Box<
            dyn Runtime<SceneRegistryRefs<CustomEvents>, SendEngineArgs, NonSendEngineArgs>,
        >,
        startup_runtime: Box<
            dyn Runtime<
                SceneRegistryRefs<CustomEvents>,
                SendEngineStartupArgs,
                NonSendEngineStartupArgs,
            >,
        >,
        proxy: EventLoopProxy<MatrixEvent<CustomEvents>>,
    ) -> Self {
        Self {
            current_scene: Scene::new(proxy),
            runtime,
            startup_runtime,
            resources: DataState::default(),
            marker: PhantomData,
        }
    }

    pub(crate) fn add_plugin(&mut self, new: impl Plugin<CustomEvents> + 'static) {
        self.current_scene.plugins.push_back(Box::new(new));
    }
}
impl<CustomEvents: MatrixEventable> ApplicationHandler<MatrixEvent<CustomEvents>>
    for SceneManager<CustomEvents>
{
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        while let Some(plugin) = self.current_scene.plugins.pop_back() {
            plugin.build(&mut self.current_scene);
        }

        let mut reg = SceneRegistryRefs {
            components: self
                .current_scene
                .components
                .write()
                .expect("this should be available"),
            events: self
                .current_scene
                .events
                .write()
                .expect("this should be available"),
            resources: self.resources.write().expect("this should be available"),
        };
        self.startup_runtime.run(
            &mut self.current_scene.startup_systems,
            &mut reg,
            SendEngineStartupArgs,
            NonSendEngineStartupArgs {
                event_loop: ActiveEventLoopRef::new(event_loop),
            },
        );

        self.current_scene
            .components
            .consume_write(reg.components)
            .unwrap();
        self.current_scene.events.consume_write(reg.events).unwrap();
        self.resources.consume_write(reg.resources).unwrap();
    }

    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        for (_, events) in self
            .current_scene
            .events
            .get_mut()
            .expect("this should be available")
            .iter_events()
        {
            events
                .get_mut()
                .expect("this should not be accessed now")
                .handle_event(&event);
        }

        if event == winit::event::WindowEvent::RedrawRequested {
            let mut reg = SceneRegistryRefs {
                components: self
                    .current_scene
                    .components
                    .write()
                    .expect("this should be available"),
                events: self
                    .current_scene
                    .events
                    .write()
                    .expect("this should be available"),
                resources: self.resources.write().expect("this should be available"),
            };

            self.runtime.run(
                &mut self.current_scene.systems,
                &mut reg,
                SendEngineArgs,
                NonSendEngineArgs,
            );

            for (_, events) in reg.events.iter_events() {
                events.get_mut().unwrap().reset();
            }

            self.current_scene
                .components
                .consume_write(reg.components)
                .unwrap();
            self.current_scene.events.consume_write(reg.events).unwrap();
            self.resources.consume_write(reg.resources).unwrap();
        }

        // println!("event: {event:?}");
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: MatrixEvent<CustomEvents>) {
        if let MatrixEvent::Exit = event {
            println!("lets go");
            event_loop.exit();
        }
    }
}
