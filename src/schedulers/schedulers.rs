use std::{collections::VecDeque, io};

use crate::{
    dispatchers::systems::UnsafeBoxedDispatcher,
    scene::{Scene, SceneUpdateArgs},
};

use super::{
    access::Access,
    thread_pool::{Job, ThreadPool, ThreadPoolSender},
};


pub trait Scheduler {
    fn run(
        &mut self,
        scene: &mut Scene,
        args: &SceneUpdateArgs,
    );
}

pub struct SingleThreadScheduler;

impl Scheduler for SingleThreadScheduler {
    fn run(
        &mut self,
        scene: &mut Scene,
        _: &SceneUpdateArgs,
    ) {
        
    }
    // fn run_group(&mut self,dis:Vec<UnsafeBoxedDispatcher>)
}


pub struct MultiThreadedScheduler {
    pool: ThreadPool<UnsafeBoxedDispatcher>,
    done: VecDeque<UnsafeBoxedDispatcher>,
    pending: VecDeque<UnsafeBoxedDispatcher>,
    access_state: Access,
}

impl MultiThreadedScheduler {
    pub fn new(thread_count: usize) -> Self {
        Self {
            pool: ThreadPool::new(thread_count),
            done: Default::default(),
            pending: Default::default(),
            access_state: Default::default(),
        }
    }
    pub fn with_amount_of_cores() -> io::Result<Self> {
        Ok(Self::new(std::thread::available_parallelism()?.get()))
    }
    fn try_run_dispatcher(
        pool: &mut ThreadPoolSender<UnsafeBoxedDispatcher>,
        access_state: &mut Access,
        mut dis: UnsafeBoxedDispatcher,
        pending: &mut VecDeque<UnsafeBoxedDispatcher>,
        scene: &mut Scene,
    ) {
        
    }
}

impl Scheduler for MultiThreadedScheduler {
    fn run(
        &mut self,
        scene: &mut Scene,
        _args: &SceneUpdateArgs,
    ) {
           }
}
