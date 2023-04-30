use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
    Arc,
};

use winit::{
    event::Event,
    event_loop::{ControlFlow, EventLoop},
};

use crate::{
    components::{resources::ResourceRegistry, storage::Storage},
    scene::{Scene, SceneUpdateArgs},
    schedulers::scheduler::Scheduler, events::{event_registry::EventRegistry, matrix_event::{MatrixEventSender, MatrixEventReceiver, channel_matrix_event}},
};

pub struct EngineArgs<S: Scheduler> {
    pub scene: Scene,
    pub scheduler: S,
    pub resources: Option<ResourceRegistry>,
    pub fps: u64,
}

pub struct Engine {
    scene: Scene,
    quit: Arc<AtomicBool>,
    target_fps: Arc<AtomicU64>,
    scheduler: Box<dyn Scheduler>,
    resources: Storage<ResourceRegistry>,
    event_loop: EventLoop<()>,
    events: Storage<EventRegistry>,
    event_sender: MatrixEventSender,
    event_receiver: MatrixEventReceiver,

}

impl Engine {
    pub fn new<S: Scheduler + 'static>(args: EngineArgs<S>) -> Self {
        let target_fps = Arc::new(AtomicU64::from(args.fps));
        let events = EventRegistry::default();
        let (event_sender, event_receiver) = channel_matrix_event();
        Self {
            scene: args.scene,
            quit: Arc::new(false.into()),
            scheduler: Box::new(args.scheduler),
            target_fps,
            event_loop: EventLoop::new(),
            resources: args
                .resources
                .unwrap_or_else(|| ResourceRegistry::empty(event_sender.clone()))
                .into(),
            event_receiver,
            event_sender,
            events: Storage::from(events),
        }
    }

    pub fn run(mut self) -> ! {
        self.event_loop.run(move |event, target, control_flow| {
            if let Event::MainEventsCleared = event {
                self.scene.update(SceneUpdateArgs {
                    fps: self.target_fps.clone(),
                    quit: self.quit.clone(),
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
                if self.quit.load(Ordering::Acquire) {
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
}
