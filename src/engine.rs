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
    events::Events,
    scene::{Scene, SceneUpdateArgs},
    schedulers::scheduler::Scheduler,
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
    events: Storage<Events>,
}

impl Engine {
    pub fn new<S: Scheduler + 'static>(args: EngineArgs<S>) -> Self {
        let target_fps = Arc::new(AtomicU64::from(args.fps));
        Self {
            scene: args.scene,
            quit: Arc::new(false.into()),
            scheduler: Box::new(args.scheduler),
            target_fps,
            event_loop: EventLoop::new(),
            resources: args.resources.unwrap_or_default().into(),
            events: Storage::default(),
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
                    .update();
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
