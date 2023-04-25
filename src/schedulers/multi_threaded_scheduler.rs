use std::{collections::VecDeque, io, sync::Arc};

use crate::dispatchers::{
    dispatchers::DispatcherArgs,
    system_registry::{BoxedSystem, SystemGroup},
    systems::SystemArgs,
};

use super::{
    schedulers::Scheduler,
    thread_pool::{ThreadPool, ThreadPoolSender},
};

pub struct MultiThreadedScheduler {
    pool: ThreadPool<BoxedSystem>,
    done: VecDeque<BoxedSystem>,
    pending: VecDeque<BoxedSystem>,
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

    fn send_dispatcher<'a>(
        sender: &ThreadPoolSender<BoxedSystem>,
        mut dis: BoxedSystem,
        args: &mut DispatcherArgs,
        system_args: Arc<SystemArgs>,
    ) -> Result<(), BoxedSystem> {
        let mut data = match dis.as_mut().dispatch(args) {
            Ok(data) => data,
            Err(_) => return Err(dis),
        };

        sender
            .send(move || {
                dis.as_mut()
                    .try_run(system_args, &mut data)
                    .expect("this function should work");
                dis
            })
            .expect("this value should be sent");

        Ok(())
    }
}

impl Scheduler for MultiThreadedScheduler {
    fn run<'a>(
        &mut self,
        dispatchers: &mut SystemGroup,
        args: &mut DispatcherArgs<'a>,
        system_args: Arc<SystemArgs>,
    ) {
        let sender = self.pool.sender();

        while let Some(dis) = dispatchers.pop_normal() {
            if let Err(dis) = Self::send_dispatcher(&sender, dis, args, system_args.clone()) {
                self.pending.push_back(dis)
            };

            for dis in self.pool.try_recv_iter() {
                let dis = dis.expect("thread panicked");
                self.done.push_back(dis);

                for _ in 0..self.pending.len() {
                    let dis = self.pending.pop_back().expect("this should work");
                    if let Err(dis) = Self::send_dispatcher(&sender, dis, args, system_args.clone())
                    {
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

                if let Err(dis) = Self::send_dispatcher(&sender, dis, args, system_args.clone()) {
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

            let Ok(_) = b.as_mut().try_run(system_args.clone(), &mut data) else {
                panic!("Uknown error");
            };
            drop(data);
        }
    }
}
