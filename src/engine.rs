use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use crate::{scene::{Scene, SceneUpdateArgs}, thread_pool::ThreadPool};

pub struct EngineArgs {
    pub scene:Scene,
    pub thread_count: Option<usize>,
}

pub struct Engine {
    scene: Scene,
    quit: Arc<AtomicBool>,
    thread_pool: ThreadPool<()>,
}

impl Engine {
    pub fn new(args:EngineArgs) -> Self {
        Self {
            scene:args.scene,
            quit: Arc::new(AtomicBool::new(false)),
            thread_pool: ThreadPool::new(args.thread_count.unwrap_or(8)), 
        }
    }

    pub fn run(&mut self) {
        self.scene.setup();
        let args = SceneUpdateArgs {
            quit: self.quit.clone(),
            pool: &self.thread_pool
        };
        while !self.quit.load(Ordering::Acquire) {
            self.scene.update(&args);
        }
    }
}
