pub mod components;
pub mod data_state;
pub mod entity;
pub mod events;
pub mod group_query;
pub mod plugins;
pub mod query;
pub mod resources;
pub mod runtimes;
pub mod scene;
pub mod systems;
pub mod typeid;
pub mod transform;
pub mod component_iters;

use std::marker::PhantomData;

use events::{MatrixEvent, MatrixEventable};
use plugins::Plugin;
use runtimes::Runtime;
use scene::{
    NonSendEngineArgs, NonSendEngineStartupArgs, SceneManager, SceneRegistryRefs, SendEngineArgs,
    SendEngineStartupArgs,
};
use winit::{error::EventLoopError, event_loop::EventLoop};

pub struct EngineArgs<
    RuntimeA: Runtime<SceneRegistryRefs<CustomEvents>, SendEngineArgs, NonSendEngineArgs>,
    RuntimeB: Runtime<SceneRegistryRefs<CustomEvents>, SendEngineStartupArgs, NonSendEngineStartupArgs>,
    CustomEvents: MatrixEventable,
> {
    pub runtime: RuntimeA,
    pub startup_runtime: RuntimeB,
    marker: PhantomData<CustomEvents>,
}

impl<
        RuntimeA: Runtime<SceneRegistryRefs<CustomEvents>, SendEngineArgs, NonSendEngineArgs>,
        RuntimeB: Runtime<SceneRegistryRefs<CustomEvents>, SendEngineStartupArgs, NonSendEngineStartupArgs>,
        CustomEvents: MatrixEventable,
    > EngineArgs<RuntimeA, RuntimeB, CustomEvents>
{
    pub fn new(runtime: RuntimeA, startup_runtime: RuntimeB) -> Self {
        Self {
            runtime,
            startup_runtime,
            marker: PhantomData,
        }
    }
}

pub struct Engine<CustomEvents: MatrixEventable = ()> {
    event_loop: EventLoop<MatrixEvent<CustomEvents>>,
    scene: SceneManager<CustomEvents>,
}

impl<CustomEvents: MatrixEventable> Engine<CustomEvents> {
    pub fn new<
        A: Runtime<SceneRegistryRefs<CustomEvents>, SendEngineArgs, NonSendEngineArgs> + 'static,
        B: Runtime<
                SceneRegistryRefs<CustomEvents>,
                SendEngineStartupArgs,
                NonSendEngineStartupArgs,
            > + 'static,
    >(
        args: EngineArgs<A, B, CustomEvents>,
    ) -> Self {
        let event_loop = EventLoop::with_user_event().build().unwrap();
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

        Engine {
            scene: SceneManager::<CustomEvents>::new(
                Box::new(args.runtime),
                Box::new(args.startup_runtime),
                event_loop.create_proxy(),
            ),
            event_loop,
        }
    }
    pub fn run(mut self) -> Result<(), EventLoopError> {
        self.event_loop.run_app(&mut self.scene)
    }

    pub fn event_loop(&self) -> &EventLoop<MatrixEvent<CustomEvents>> {
        &self.event_loop
    }

    pub fn add_scene_plugin(&mut self, new: impl Plugin<CustomEvents> + 'static) {
        self.scene.add_plugin(new);
    }
}
