use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use crate::{
    scene::{SceneUpdateArgs},
    world::World, schedulers::schedulers::Scheduler,
};

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
        let args = SceneUpdateArgs {
            quit: self.quit.clone(),
        };
        let (scene, resources) = self.world.unpack();
        self.scheduler.run(scene, &args);

        while !self.quit.load(Ordering::Acquire) {
            self.scheduler.run(scene, &args);
        }
    }
}
