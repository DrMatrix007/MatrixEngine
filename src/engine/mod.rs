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

use events::{MatrixEvent, MatrixEventable};
use plugins::Plugin;
use runtimes::Runtime;
use scene::{
    NonSendEngineArgs, NonSendEngineStartupArgs, SceneManager, SceneRegistryRefs, SendEngineArgs,
    SendEngineStartupArgs,
};
use winit::{error::EventLoopError, event_loop::EventLoop};

pub struct EngineArgs<CustomEvents: MatrixEventable> {
    pub runtime:
        Box<dyn Runtime<SceneRegistryRefs<CustomEvents>, SendEngineArgs, NonSendEngineArgs>>,
    pub startup_runtime: Box<
        dyn Runtime<
            SceneRegistryRefs<CustomEvents>,
            SendEngineStartupArgs,
            NonSendEngineStartupArgs,
        >,
    >,
}

pub struct Engine<CustomEvents: MatrixEventable = ()> {
    event_loop: EventLoop<MatrixEvent<CustomEvents>>,
    scene: SceneManager<CustomEvents>,
}

impl<CustomEvents: MatrixEventable> Engine<CustomEvents> {
    pub fn new(args: EngineArgs<CustomEvents>) -> Self {
        let event_loop = EventLoop::with_user_event().build().unwrap();
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

        Engine {
            scene: SceneManager::<CustomEvents>::new(
                args.runtime,
                args.startup_runtime,
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
