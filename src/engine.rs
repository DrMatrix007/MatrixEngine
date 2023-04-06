use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
    Arc,
};

use crate::{event_loop::EventLoop, schedulers::schedulers::Scheduler, world::World};

pub struct EngineArgs<S: Scheduler> {
    pub world: World,
    pub scheduler: S,
    pub fps: u64,
}

pub struct Engine {
    world: World,
    quit: Arc<AtomicBool>,
    target_fps: Arc<AtomicU64>,
    scheduler: Box<dyn Scheduler>,
    event_loop: EventLoop,
}

impl Engine {
    pub fn new<S: Scheduler + 'static>(args: EngineArgs<S>) -> Self {
        let target_fps = Arc::new(AtomicU64::from(args.fps));
        Self {
            world: args.world,
            quit: Arc::new(false.into()),
            scheduler: Box::new(args.scheduler),
            target_fps: target_fps.clone(),
            event_loop: EventLoop::new(target_fps),
        }
    }

    pub fn run(&mut self) {
        let mut args = self.world.unpack();
        self.scheduler.run(args.startups, &mut args.args);

        while !self.quit.load(Ordering::Acquire) {
            self.event_loop.capture();
            self.scheduler.run(args.systems, &mut args.args);
            self.event_loop.wait();
        }
    }
}
