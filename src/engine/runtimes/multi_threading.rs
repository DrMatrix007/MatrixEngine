use std::{
    collections::VecDeque,
    sync::mpsc::{channel, Receiver, Sender},
};

use scoped_threadpool::Pool;

use crate::engine::systems::BoxedSendSystem;

use super::Runtime;

pub struct MultiThreaded<Queryable, SendArgs: Send + Sync> {
    thread_pool: Pool,
    sender: Sender<BoxedSendSystem<Queryable, SendArgs>>,
    reciever: Receiver<BoxedSendSystem<Queryable, SendArgs>>,
    systems_store: VecDeque<BoxedSendSystem<Queryable, SendArgs>>,
}

impl<Queryable, SendArgs: Send + Sync> MultiThreaded<Queryable, SendArgs> {
    pub fn new(n: u32) -> Self {
        let (sender, reciever) = channel();
        Self {
            thread_pool: Pool::new(n),
            systems_store: VecDeque::new(),
            reciever,
            sender,
        }
    }
    pub fn with_cpu_count() -> Self {
        Self::new(num_cpus::get() as _)
    }
}

impl<Queryable, SendArgs: Send + Sync, NonSendArgs> Runtime<Queryable, SendArgs, NonSendArgs>
    for MultiThreaded<Queryable, SendArgs>
{
    fn run(
        &mut self,
        systems: &mut crate::engine::systems::SystemRegistry<Queryable, SendArgs, NonSendArgs>,
        queryable: &mut Queryable,
        send_args: SendArgs,
        non_send_args: NonSendArgs,
    ) {
        self.thread_pool.scoped(|scope| {
            let mut sys_count = systems.send_systems().len();

            let send_args = &send_args;
            let sender = &self.sender;
            for mut system in systems.send_systems_mut().drain(..) {
                match system.prepare_args(queryable) {
                    Ok(_) => {
                        scope.execute(move || {
                            system.run(send_args).unwrap();
                            sender.send(system).unwrap();
                        });
                    }
                    Err(_) => {
                        self.systems_store.push_back(system);
                    }
                }
            }

            while !self.systems_store.is_empty() {
                let stored_sys_count = self.systems_store.len();
                let mut sys = self.reciever.recv().unwrap();
                sys.consume(queryable).unwrap();
                systems.send_systems_mut().push(sys);

                sys_count -= 1;

                for _ in 0..stored_sys_count {
                    let mut system = self.systems_store.pop_front().unwrap();
                    match system.prepare_args(queryable) {
                        Ok(_) => {
                            scope.execute(move || {
                                system.run(send_args).unwrap();
                                sender.send(system).unwrap();
                            });
                        }
                        Err(_) => {
                            self.systems_store.push_back(system);
                        }
                    }
                }
            }
            for _ in 0..sys_count {
                let mut sys = self.reciever.recv().unwrap();
                sys.consume(queryable).unwrap();
                systems.send_systems_mut().push(sys);
            }
        });

        for i in systems.non_send_systems_mut() {
            i.prepare_args(queryable).unwrap();
            // stopwatch.debug_elapesd();
            i.run(&non_send_args).unwrap();
            // stopwatch.debug_elapesd();
            i.consume(queryable).unwrap();
            // stopwatch.debug_elapesd();
            // println!();
        }
    }
}
