use std::{
    collections::VecDeque,
    sync::Arc,
    time::{Duration, Instant},
};

use tokio::sync::RwLock;
use winit::{event::Event, event_loop::EventLoopProxy};

use self::thread_pool::{JobResult, SystemThreadPool, SystemWorkerError};

use super::{
    events::{engine_event::EngineEvent, event_registry::EventRegistry},
    systems::{
        query::QueryCleanup,
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

    fn process_event(
        &mut self,
        event: Event<'_, EngineEvent>,
        args: &mut Args,
        frame_duration: Duration,
        all_system_registries: &mut [&mut SystemRegistry<Args>],
    );

    fn use_event_loop_proxy(&mut self, proxy: EventLoopProxy<EngineEvent>);

    fn is_done(&self) -> bool;
}

pub struct SingleThreaded {
    proxy: Option<EventLoopProxy<EngineEvent>>,
    event_registry: EventRegistry,
}

impl SingleThreaded {
    pub fn new() -> Self {
        Self {
            proxy: None,
            event_registry: EventRegistry::default(),
        }
    }
}

impl<Args: 'static> Runtime<Args> for SingleThreaded {
    fn add_send(&mut self, s: SystemSendRef<Args>, args: &mut Args) {
        let (mut clean, (system_ref, control_flow)) =
            s.dispatch(args).map_err(|e| (e.1)).unwrap()(&self.event_registry);
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
            s.dispatch(args).map_err(|e| (e.1)).unwrap()(&self.event_registry);
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

    fn process_event(
        &mut self,
        event: Event<'_, EngineEvent>,
        args: &mut Args,
        frame_duration: Duration,

        all_system_registries: &mut [&mut SystemRegistry<Args>],
    ) {
        if let Event::UserEvent(EngineEvent::SystemDone(id, control_flow)) = event {
            all_system_registries
                .iter_mut()
                .find_map(|registry| {
                    if registry.try_recieve_send_with_id(&id).is_ok() {
                        match control_flow {
                            SystemControlFlow::Continue => {}
                            SystemControlFlow::Quit => panic!("Quit"),
                            SystemControlFlow::Remove => {
                                registry.remove_system_send(&id);
                            }
                        }
                        Some(())
                    } else {
                        None
                    }
                })
                .unwrap();
        }

        if let Some(event) = &event.to_static() {
            self.event_registry.process(event);
        }
    }

    fn is_done(&self) -> bool {
        true
    }
}

pub struct MultiThreaded<Args: 'static> {
    pool: SystemThreadPool<(
        Box<dyn QueryCleanup<Args> + Send + Sync>,
        (SystemSendRef<Args>, SystemControlFlow),
    )>,
    send_queue: VecDeque<SystemSendRef<Args>>,
    non_send_queue: VecDeque<SystemRef<Args>>,
    proxy: Arc<RwLock<Option<EventLoopProxy<EngineEvent>>>>,
    non_send_event_registry: EventRegistry,
}

impl<Args: 'static> MultiThreaded<Args> {
    pub fn new(workers: usize) -> Self {
        let p = Arc::new(RwLock::new(Option::<EventLoopProxy<EngineEvent>>::None));
        let proxy = p.clone();
        let pool = SystemThreadPool::new(workers);
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
            non_send_event_registry: Default::default(),
        }
    }
    pub fn new_with_cpu_cores() -> Self {
        Self::new((num_cpus::get_physical() - 1).max(4))
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
                let (mut cleanup, (s, control_flow)) = f(&self.non_send_event_registry);
                if let Some(proxy) = self.proxy.blocking_read().as_ref() {
                    proxy
                        .send_event(EngineEvent::SystemDone(s.id(), control_flow))
                        .map_err(|_| ())
                        .unwrap();
                }
                cleanup.cleanup(args);
            }
            Err((s, e)) => {
                self.non_send_queue.push_back(s);
            }
        };
    }

    fn try_send_all_queue(&mut self, args: &mut Args) {
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

    fn process_event(
        &mut self,
        event: Event<'_, EngineEvent>,
        args: &mut Args,
        frame_duration: Duration,
        all_system_registries: &mut [&mut SystemRegistry<Args>],
    ) {
        if let Event::UserEvent(EngineEvent::SystemDone(id, control_flow)) = event {
            all_system_registries
                .iter_mut()
                .find_map(|registry| {
                    if let Ok(boxed) = registry.try_recieve_send_with_id(&id) {
                        let (started_at, ended_at) = match self.pool.recv_iter().next().unwrap() {
                            JobResult::Ok {
                                mut data,
                                started_at,
                                ended_at,
                            } => {
                                data.0.cleanup(args);
                                (started_at, ended_at)
                            }
                            JobResult::Err(SystemWorkerError::Panic) => {
                                panic!("subthread panicked")
                            }
                        };
                        boxed.set_started_at(started_at);

                        match control_flow {
                            SystemControlFlow::Continue => {
                                if (ended_at - started_at) > frame_duration {
                                    // self.try_run_send(boxed.try_lock().unwrap(), args);
                                }
                            }
                            SystemControlFlow::Quit => panic!("Quit"),
                            SystemControlFlow::Remove => {
                                registry.remove_system_send(&id);
                            }
                        }

                        Some(())
                    } else if let Ok(boxed) = registry.try_recieve_non_send_with_id(&id) {
                        match control_flow {
                            SystemControlFlow::Continue => {}
                            SystemControlFlow::Quit => panic!("Quit"),
                            SystemControlFlow::Remove => {
                                registry.remove_system_non_send(&id);
                            }
                        }
                        self.try_send_all_queue(args);
                        Some(())
                    } else {
                        None
                    }
                })
                .unwrap();
            self.try_send_all_queue(args);
        }
        if let Some(event) = &event.to_static() {
            self.non_send_event_registry.process(event);
            for worker in self.pool.workers() {
                worker.send_event(event);
            }
        }
    }

    fn is_done(&self) -> bool {
        self.send_queue.len() == 0 && self.non_send_queue.len() == 0 && self.pool.jobs_count() == 0
    }
}
