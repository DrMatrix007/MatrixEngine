use std::{collections::VecDeque, io};

use crate::dispatchers::{systems::UnsafeBoxedDispatcher, dispatchers::DispatcherArgs};

use super::{
    access::Access,
    thread_pool::{ThreadPool, ThreadPoolSender}, schedulers::Scheduler,
};

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

    unsafe fn send_dispatcher(
        sender: &ThreadPoolSender<UnsafeBoxedDispatcher>,
        mut dis: UnsafeBoxedDispatcher,
        args: &mut DispatcherArgs,
    ) {
        let data = unsafe { dis.as_mut().dispatch(args) };

        sender
            .send(move || {
                dis.as_mut()
                    .try_run(data)
                    .map_err(|_| ())
                    .expect("this function should work");
                dis
            })
            .expect("this value should be sent");
    }
}

impl Scheduler for MultiThreadedScheduler {
    fn run<'a>(
        &mut self,
        dispatchers: &mut Vec<UnsafeBoxedDispatcher>,
        args: &mut DispatcherArgs<'a>,
    ) {
        self.access_state.clear();
        let sender = self.pool.sender();

        while let Some(dis) = dispatchers.pop() {
            match self.access_state.try_combine(dis.as_access()) {
                Ok(_) => {
                    unsafe { Self::send_dispatcher(&sender, dis, args) };
                }
                Err(_) => self.pending.push_back(dis),
            }
            for dis in self.pool.try_recv_iter() {
                self.access_state.remove(dis.as_access());
                self.done.push_back(dis);

                for _ in 0..self.pending.len() {
                    let dis = self.pending.pop_back().expect("this should work");
                    match self.access_state.try_combine(dis.as_access()) {
                        Ok(_) => {
                            unsafe { Self::send_dispatcher(&sender, dis, args) };
                        }
                        Err(_) => self.pending.push_front(dis),
                    }
                }
            }
        }
        for i in self.pool.recv_iter() {
            self.access_state.remove(i.as_access());
            self.done.push_back(i);
            for _ in 0..self.pending.len() {
                let dis = self.pending.pop_back().expect("this should work");
                match self.access_state.try_combine(dis.as_access()) {
                    Ok(_) => {
                        unsafe { Self::send_dispatcher(&sender, dis, args) };
                    }
                    Err(_) => self.pending.push_front(dis),
                }
            }
        }
        while let Some(dis) = self.done.pop_back() {
            dispatchers.push(dis);
        }
    }
}