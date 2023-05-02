use std::collections::VecDeque;

use crate::dispatchers::{
    dispatcher::DispatcherArgs,
    system_registry::{BoxedAsyncSystem, BoxedExclusiveSystem, SystemGroup},
};

use super::scheduler::Scheduler;

pub struct SingleThreadScheduler {
    done_async: VecDeque<BoxedAsyncSystem>,
    done_exclusive: VecDeque<BoxedExclusiveSystem>,
}

impl Scheduler for SingleThreadScheduler {
    fn run(&mut self, dis: &mut SystemGroup, args: &mut DispatcherArgs<'_>) {
        while let Some(mut i) = dis.pop_async() {
            let mut data = i
                .as_mut()
                .dispatch(args)
                .expect("this runs only on the main thread and its should not crash");
            i.try_run(&mut data)
                .map_err(|_| ())
                .expect("this function should not return Err(())");
            self.done_async.push_back(i);
        }

        while let Some(mut i) = dis.pop_exclusive() {
            let mut data = i
                .as_mut()
                .dispatch(args)
                .expect("this runs only on the main thread and its should not crash");
            i.try_run(&mut data)
                .map_err(|_| ())
                .expect("this function should not return Err(())");
            self.done_exclusive.push_back(i);
        }

        while let Some(i) = self.done_async.pop_front() {
            if !i.ctx_ref().is_destroyed() {
                dis.push_async(i);
            }
        }

        while let Some(i) = self.done_exclusive.pop_front() {
            if !i.ctx_ref().is_destroyed() {
                dis.push_exclusive(i);
            }
        }
    }
}
