use std::{collections::VecDeque, sync::Arc};

use log::info;
use tokio::sync::{Mutex, RwLock};
use winit::event_loop::EventLoopProxy;

use self::thread_pool::{ThreadPool, WorkerError};

use super::{
    events::engine_event::EngineEvent,
    systems::{
        query::{ComponentQueryArgs, QueryCleanup},
        system_registry::{SystemRef, SystemRegistry, SystemSendRef},
        Dispatcher, DispatcherSend, SystemControlFlow,
    },
};

pub mod thread_pool;

pub trait Runtime<Args> {
    fn add_send(&mut self, s: SystemSendRef<Args>, args: &mut Args);

    fn add_non_send(&mut self, s: SystemRef<Args>, args: &mut Args);

    fn add_available(&mut self, system_registry: &mut SystemRegistry<Args>, args: &mut Args) {
        for i in system_registry.try_lock_iter_non_send() {
            self.add_non_send(i, args)
        }

        for i in system_registry.try_lock_iter_send().map(|f| f) {
            self.add_send(i, args);
        }
    }
    // fn cleanup_systems(
    //     &mut self,
    //     args: &mut Args,
    //     system_registries: &mut [&mut SystemRegistry<Args>],
    // );

    fn process_engine_event(
        &mut self,
        event: &EngineEvent,
        args: &mut Args,
        all_system_registries: &mut [&mut SystemRegistry<Args>],
    );

    fn use_event_loop_proxy(&mut self, proxy: EventLoopProxy<EngineEvent>);

    fn is_done(&self) -> bool;
}

pub struct SingleThreaded {
    proxy: Option<EventLoopProxy<EngineEvent>>,
}

impl SingleThreaded {
    pub fn new() -> Self {
        Self { proxy: None }
    }
}

impl<Args: 'static> Runtime<Args> for SingleThreaded {
    fn add_send(&mut self, s: SystemSendRef<Args>, args: &mut Args) {
        let (mut clean, (system_ref, control_flow)) =
            s.dispatch(args).map_err(|e| (e.1)).unwrap()();
        clean.cleanup(args);
        if let Some(proxy) = &self.proxy {
            proxy
                .send_event(EngineEvent::SystemDone(system_ref.id(), control_flow))
                .map_err(|_| ())
                .unwrap();
        }
    }

    fn add_non_send(&mut self, s: SystemRef<Args>, args: &mut Args) {
        let (mut clean, (system_ref, control_flow)) =
            s.dispatch(args).map_err(|e| (e.1)).unwrap()();
        clean.cleanup(args);
        if let Some(proxy) = &self.proxy {
            proxy
                .send_event(EngineEvent::SystemDone(system_ref.id(), control_flow))
                .map_err(|_| ())
                .unwrap();
        }
    }

    fn use_event_loop_proxy(&mut self, proxy: EventLoopProxy<EngineEvent>) {
        self.proxy = Some(proxy);
    }

    fn process_engine_event(
        &mut self,
        event: &EngineEvent,
        args: &mut Args,
        all_system_registries: &mut [&mut SystemRegistry<Args>],
    ) {
        match event {
            EngineEvent::SystemDone(id, control_flow) => {
                all_system_registries
                    .iter_mut()
                    .find_map(|registry| {
                        if registry.try_recieve_send_with_id(&id).is_ok() {
                            match control_flow {
                                SystemControlFlow::Continue => {}
                                SystemControlFlow::Quit => panic!("Quit"),
                                SystemControlFlow::Remove => {
                                    registry.remove_system_send(id);
                                }
                            }
                            Some(())
                        } else {
                            None
                        }
                    })
                    .unwrap();
            }
        }
    }

    fn is_done(&self) -> bool {
        true
    }

    // fn cleanup_systems(
    //     &mut self,
    //     args: &mut Args,
    //     system_registries: &mut [&mut SystemRegistry<Args>],
    // ) {
    // }
}

pub struct MultiThreaded<Args: 'static> {
    pool: ThreadPool<(
        Box<dyn QueryCleanup<Args> + Send + Sync>,
        (SystemSendRef<Args>, SystemControlFlow),
    )>,
    send_queue: VecDeque<SystemSendRef<Args>>,
    non_send_queue: VecDeque<SystemRef<Args>>,
    proxy: Arc<RwLock<Option<EventLoopProxy<EngineEvent>>>>,
}

impl<Args: 'static> MultiThreaded<Args> {
    pub fn new() -> Self {
        let p = Arc::new(RwLock::new(Option::<EventLoopProxy<EngineEvent>>::None));
        let proxy = p.clone();
        let pool = ThreadPool::new(10);
        pool.add_proxy(
            move |data: &mut (
                Box<dyn QueryCleanup<Args> + Send + Sync>,
                (SystemSendRef<Args>, SystemControlFlow),
            )| {
                let (cleanup, (system_ref, control_flow)) = data;
                match p.blocking_read().as_ref() {
                    Some(proxy) => proxy
                        .send_event(EngineEvent::SystemDone(system_ref.id(), *control_flow))
                        .map_err(|_| ())
                        .unwrap(),
                    None => {}
                }
            },
        );
        Self {
            send_queue: Default::default(),
            non_send_queue: Default::default(),
            pool,
            proxy,
        }
    }

    fn try_run_send(&mut self, s: SystemSendRef<Args>, args: &mut Args) {
        match s.dispatch_send(args) {
            Ok(f) => {
                self.pool.add(f).unwrap();
            }
            Err((s, e)) => {
                self.send_queue.push_back(s);
            }
        };
    }

    fn try_run_non_send(&mut self, s: SystemRef<Args>, args: &mut Args) {
        match s.dispatch(args) {
            Ok(f) => {
                let (cleanup, (s, control_flow)) = f();
                if let Some(proxy) = self.proxy.blocking_read().as_ref() {
                    proxy
                        .send_event(EngineEvent::SystemDone(s.id(), control_flow))
                        .map_err(|_| ())
                        .unwrap();
                }
            }
            Err((s, e)) => {
                self.non_send_queue.push_back(s);
            }
        };
    }
}

impl<Args: 'static> Runtime<Args> for MultiThreaded<Args> {
    fn add_send(&mut self, s: SystemSendRef<Args>, args: &mut Args) {
        self.try_run_send(s, args);
    }

    fn add_non_send(&mut self, s: SystemRef<Args>, args: &mut Args) {
        self.try_run_non_send(s, args);
    }

    fn use_event_loop_proxy(&mut self, proxy: EventLoopProxy<EngineEvent>) {
        *self.proxy.blocking_write() = Some(proxy);
    }

    fn process_engine_event(
        &mut self,
        event: &EngineEvent,
        args: &mut Args,
        all_system_registries: &mut [&mut SystemRegistry<Args>],
    ) {
        match event {
            EngineEvent::SystemDone(id, control_flow) => {
                all_system_registries
                    .iter_mut()
                    .find_map(|registry| {
                        if registry.try_recieve_send_with_id(&id).is_ok() {
                            match control_flow {
                                SystemControlFlow::Continue => {}
                                SystemControlFlow::Quit => panic!("Quit"),
                                SystemControlFlow::Remove => {
                                    registry.remove_system_send(id);
                                }
                            }
                            Some(())
                        } else {
                            None
                        }
                    })
                    .unwrap();
                match self.pool.recv_iter().next().unwrap() {
                    Ok((mut data, _)) => {
                        data.cleanup(args);
                    }
                    Err(WorkerError::Panic) => panic!("subthread panicked"),
                }
                let len = self.send_queue.len();
                for i in 0..len {
                    let s = self
                        .send_queue
                        .pop_front()
                        .expect("the len promises that there is a value here");
                    self.try_run_send(s, args);
                }
                let len = self.non_send_queue.len();
                for i in 0..len {
                    let s = self
                        .non_send_queue
                        .pop_front()
                        .expect("the len promises that there is a value here");
                    self.try_run_non_send(s, args);
                }
            }
        }
    }

    fn is_done(&self) -> bool {
        self.send_queue.len() == 0 && self.non_send_queue.len() == 0 && self.pool.jobs_count() == 0
    }

    // fn cleanup_systems(
    //     &mut self,
    //     args: &mut Args,
    //     all_system_registries: &mut [&mut SystemRegistry<Args>],
    // ) {
    //     for data in self.pool.try_recv_iter() {
    //         match data {
    //             Ok((mut cleanup, (system, state))) => {
    //                 cleanup.cleanup(args);

    //                 all_system_registries
    //                     .iter_mut()
    //                     .find_map(|registry| {
    //                         if registry.try_recieve_send_ref(&system).is_ok() {
    //                             match state {
    //                                 SystemControlFlow::Continue => {}
    //                                 SystemControlFlow::Quit => panic!("Quit"),
    //                                 SystemControlFlow::Remove => {
    //                                     registry.remove_system_send(&system.id());
    //                                 }
    //                             }
    //                             Some(())
    //                         } else {
    //                             None
    //                         }
    //                     })
    //                     .unwrap();
    //             }
    //             Err(WorkerError::Panic) => panic!("subthread panicked"),
    //         }
    //     }
    // }
}
