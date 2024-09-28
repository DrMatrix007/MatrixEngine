pub mod components;
pub mod entity;
pub mod plugins;
pub mod query;
pub mod runtimes;
pub mod scene;
pub mod systems;
pub mod event_registry;

use plugins::Plugin;
use runtimes::Runtime;
use scene::{
    NonSendEngineArgs, NonSendEngineStartupArgs, Scene, SceneManager, SceneRegistry,
    SendEngineArgs, SendEngineStartupArgs,
};
use winit::{error::EventLoopError, event_loop::EventLoop};

pub enum MatrixEvent<Custom> {
    Custom(Custom),
}

pub struct EngineArgs {
    pub runtime: Box<dyn Runtime<SceneRegistry, SendEngineArgs, NonSendEngineArgs>>,
    pub startup_runtime:
        Box<dyn Runtime<SceneRegistry, SendEngineStartupArgs, NonSendEngineStartupArgs>>,
}

pub struct Engine<CustomEvents: 'static = ()> {
    event_loop: EventLoop<MatrixEvent<CustomEvents>>,
    scene: SceneManager<CustomEvents>,
}

impl<CustomEvents: 'static> Engine<CustomEvents> {
    pub fn new(args: EngineArgs) -> Self {
        let event_loop = EventLoop::with_user_event().build().unwrap();
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

        Engine {
            event_loop,
            scene: SceneManager::<CustomEvents>::new(args.runtime, args.startup_runtime),
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
