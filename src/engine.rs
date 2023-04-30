use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};

use winit::{
    event::Event,
    event_loop::{ControlFlow, EventLoop},
};

use crate::{
    components::{resources::ResourceRegistry, storage::Storage},
    dispatchers::context::Context,
    events::{
        event_registry::EventRegistry,
        matrix_event::{channel_matrix_event, MatrixEventReceiver, MatrixEventSender},
    },
    scenes::scene::{Scene, SceneUpdateArgs},
    schedulers::scheduler::Scheduler,
};

pub struct EngineArgs<S: Scheduler> {
    pub scheduler: S,
    pub resources: Option<ResourceRegistry>,
    pub fps: u64,
}

pub struct Engine {
    scheduler: Box<dyn Scheduler>,
    resources: Storage<ResourceRegistry>,
    event_loop: EventLoop<()>,
    events: Storage<EventRegistry>,
    event_sender: MatrixEventSender,
    event_receiver: MatrixEventReceiver,
    ctx: Context,
}

impl Engine {
    pub fn new<S: Scheduler + 'static>(args: EngineArgs<S>) -> Self {
        let target_fps = Arc::new(AtomicU64::from(args.fps));
        let quit = Arc::new(false.into());
        let events = EventRegistry::default();
        let (event_sender, event_receiver) = channel_matrix_event();
        Self {
            ctx: Context::new(quit, target_fps, event_sender.clone()),
            scheduler: Box::new(args.scheduler),
            event_loop: EventLoop::new(),
            resources: args
                .resources
                .unwrap_or_else(|| ResourceRegistry::empty(event_sender.clone()))
                .into(),
            event_sender,
            event_receiver,
            events: Storage::from(events),
        }
    }

    pub fn run(mut self, mut scene: Scene) -> ! {
        self.event_loop.run(move |event, target, control_flow| {
            if let Event::MainEventsCleared = event {
                scene.update(SceneUpdateArgs {
                    resources: &mut self.resources,
                    scheduler: self.scheduler.as_mut(),
                    events: &mut self.events,
                    window_target: target,
                });
                self.events
                    .write()
                    .expect("nothing should be holding the Events value")
                    .get_mut()
                    .update(&self.event_receiver);
                if self.ctx.quit.load(Ordering::Acquire) {
                    *control_flow = ControlFlow::Exit;
                }
            } else {
                self.events
                    .write()
                    .expect("nothing should be holding the Events value")
                    .get_mut()
                    .push(event);
            }
        });
    }

    pub fn ctx(&self) -> Context {
        self.ctx.clone()
    }
}
