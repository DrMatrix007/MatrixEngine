use std::collections::VecDeque;

use tokio::sync::OwnedMutexGuard;

use self::thread_pool::ThreadPool;

use super::systems::{Dispatcher, System, SystemSend};

pub mod thread_pool;

pub trait Runtime<Args> {
    fn add_send(&mut self, s: OwnedMutexGuard<dyn System<Args>>, args: &mut Args);
    fn add_non_send(&mut self, s: OwnedMutexGuard<dyn System<Args>>, args: &mut Args);
}

pub struct SingleThreaded;

impl<Args: 'static> Runtime<Args> for SingleThreaded {
    fn add_send(&mut self, s: OwnedMutexGuard<dyn System<Args>>, args: &mut Args) {
        s.dispatch(args).unwrap()();
    }

    fn add_non_send(&mut self, s: OwnedMutexGuard<dyn System<Args>>, args: &mut Args) {
        s.dispatch(args).unwrap()();
    }
}

pub struct MultiThreaded<Args> {
    pool: ThreadPool<()>,
    send_queue: VecDeque<OwnedMutexGuard<dyn SystemSend<Args>>>,
}

impl<Args: 'static> Runtime<Args> for MultiThreaded<Args> {
    fn add_send(&mut self, s: OwnedMutexGuard<dyn System<Args>>, args: &mut Args) {}

    fn add_non_send(&mut self, s: OwnedMutexGuard<dyn System<Args>>, args: &mut Args) {
        todo!()
    }
}
