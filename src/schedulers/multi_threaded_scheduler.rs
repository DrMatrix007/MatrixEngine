use std::{collections::VecDeque, io, sync::Arc};

use crate::dispatchers::{
    dispatcher::DispatcherArgs,
    system_registry::{BoxedAsyncSystem, SystemGroup},
    systems::SystemContext,
};

use super::{
    scheduler::Scheduler,
    thread_pool::{ThreadPool, ThreadPoolSender},
};

pub struct MultiThreadedScheduler {
    pool: ThreadPool<BoxedAsyncSystem>,
    done: VecDeque<BoxedAsyncSystem>,
    pending: VecDeque<BoxedAsyncSystem>,
    // access_state: Access,
}

impl MultiThreadedScheduler {
    pub fn new(thread_count: usize) -> Self {
        Self {
            pool: ThreadPool::new(thread_count),
            done: Default::default(),
            pending: Default::default(),
            // access_state: Default::default(),
        }
    }
    pub fn with_amount_of_cpu_cores() -> io::Result<Self> {
        Ok(Self::new(std::thread::available_parallelism()?.get()))
    }

    fn send_dispatcher(
        sender: &ThreadPoolSender<BoxedAsyncSystem>,
        mut dis: BoxedAsyncSystem,
        args: &mut DispatcherArgs<'_>,
    ) -> Result<(), BoxedAsyncSystem> {
        let mut data = match dis.as_mut().dispatch(args) {
            Ok(data) => data,
            Err(_) => return Err(dis),
        };

        sender
            .send(move || {
                dis.try_run(&mut data).expect("this function should work");
                dis
            })
            .expect("this value should be sent");

        Ok(())
    }
}

impl Scheduler for MultiThreadedScheduler {
    fn run(&mut self, dispatchers: &mut SystemGroup, args: &mut DispatcherArgs<'_>) {
        let sender = self.pool.sender();

        while let Some(dis) = dispatchers.pop_normal() {
            if let Err(dis) = Self::send_dispatcher(&sender, dis, args) {
                self.pending.push_back(dis)
            };

            for dis in self.pool.try_recv_iter() {
                let dis = dis.expect("thread panicked");
                self.done.push_back(dis);

                for _ in 0..self.pending.len() {
                    let dis = self.pending.pop_back().expect("this should work");
                    if let Err(dis) = Self::send_dispatcher(&sender, dis, args) {
                        self.pending.push_back(dis);
                    };
                }
            }
        }
        for dis in self.pool.recv_iter() {
            let dis = dis.expect("thread panicked");
            self.done.push_back(dis);
            for _ in 0..self.pending.len() {
                let dis = self.pending.pop_back().expect("this should work");

                if let Err(dis) = Self::send_dispatcher(&sender, dis, args) {
                    self.pending.push_back(dis);
                };
            }
        }
        while let Some(dis) = self.done.pop_back() {
            dispatchers.push_normal(dis);
        }

        for b in dispatchers.iter_exclusive() {
            let mut data = b
                .as_mut()
                .dispatch(args)
                .expect("this should not crash because it is on the same thread");

            let Ok(_) = b.try_run(&mut data) else {
                panic!("Unknown error");
            };
            drop(data);
        }
    }
}
