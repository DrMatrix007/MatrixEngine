use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use crate::{schedulers::schedulers::Scheduler, world::World};

pub struct EngineArgs<S: Scheduler> {
    pub world: World,
    pub scheduler: S,
}

pub struct Engine {
    world: World,
    quit: Arc<AtomicBool>,
    scheduler: Box<dyn Scheduler>,
}

impl Engine {
    pub fn new<S: Scheduler + 'static>(args: EngineArgs<S>) -> Self {
        Self {
            world: args.world,
            quit: Arc::new(AtomicBool::new(false)),
            scheduler: Box::new(args.scheduler),
        }
    }

    pub fn run(&mut self) {
        
        let mut args = self.world.unpack();
        self.scheduler.run(args.startups, &mut args.args);

        while !self.quit.load(Ordering::Acquire) {
            self.scheduler.run(args.systems, &mut args.args);
        }
    }
}
