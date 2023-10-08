use std::{collections::VecDeque, sync::Arc};

use tokio::sync::Mutex;
use winit::event_loop::EventLoopProxy;

use self::thread_pool::ThreadPool;

use super::{
    events::engine_event::EngineEvent,
    scenes::entities::Entity,
    systems::{
        query::QueryCleanup,
        system_registry::{SystemRef, SystemRegistry, SystemSendRef},
        Dispatcher, DispatcherSend, SystemControlFlow,
    },
};

pub mod thread_pool;

pub trait Runtime<Args> {
    fn run_send(&mut self, s: SystemSendRef<Args>, args: &mut Args);

    fn add_non_send(&mut self, s: SystemRef<Args>, args: &mut Args);

    fn add_available(&mut self, system_registry: &mut SystemRegistry<Args>, args: &mut Args) {
        for i in system_registry.try_lock_iter_non_send() {
            self.add_non_send(i, args)
        }

        for i in system_registry.try_lock_iter_send().map(|f| f) {
            self.run_send(i, args);
        }
    }
    fn cleanup_systems(
        &mut self,
        args: &mut Args,
        system_registries: &mut [&mut SystemRegistry<Args>],
    );

    fn process_engine_event(&mut self, event: &EngineEvent, args: &mut Args) {}

    fn use_event_loop_proxy(&mut self, proxy: EventLoopProxy<EngineEvent>);
}

pub struct SingleThreaded<Args> {
    proxy: Option<EventLoopProxy<EngineEvent>>,
    send_systems_flow: Vec<(SystemSendRef<Args>, SystemControlFlow)>,
    non_send_systems_flow: Vec<(SystemRef<Args>, SystemControlFlow)>,
}

impl<Args> SingleThreaded<Args> {
    pub fn new() -> Self {
        Self {
            proxy: None,
            non_send_systems_flow: Vec::new(),
            send_systems_flow: Vec::new(),
        }
    }
}

impl<Args: 'static> Runtime<Args> for SingleThreaded<Args> {
    fn run_send(&mut self, s: SystemSendRef<Args>, args: &mut Args) {
        let (mut clean, sys_ref_and_flow) = s.dispatch(args).map_err(|e| (e.1)).unwrap()();
        clean.cleanup(args);
        self.send_systems_flow.push(sys_ref_and_flow);
    }

    fn add_non_send(&mut self, s: SystemRef<Args>, args: &mut Args) {
        let (mut clean, sys_ref_and_flow) = s.dispatch(args).map_err(|e| (e.1)).unwrap()();
        clean.cleanup(args);
        self.non_send_systems_flow.push(sys_ref_and_flow);
    }

    fn use_event_loop_proxy(&mut self, proxy: EventLoopProxy<EngineEvent>) {
        self.proxy = Some(proxy);
    }

    fn cleanup_systems(
        &mut self,
        args: &mut Args,
        system_registries: &mut [&mut SystemRegistry<Args>],
    ) {
    }
}

pub struct MultiThreaded<Args> {
    pool: ThreadPool<(
        Box<dyn QueryCleanup<Args> + Send + Sync>,
        (SystemSendRef<Args>, SystemControlFlow),
    )>,
    send_queue: VecDeque<SystemSendRef<Args>>,
    non_send_queue: VecDeque<SystemRef<Args>>,
    proxy: Arc<Mutex<Option<EventLoopProxy<EngineEvent>>>>,
}

impl<Args: 'static> MultiThreaded<Args> {
    pub fn new() -> Self {
        let p = Arc::new(Mutex::new(Option::<EventLoopProxy<EngineEvent>>::None));
        let proxy = p.clone();
        let pool = ThreadPool::new(10);
        pool.add_proxy(move |x| match p.blocking_lock().as_ref() {
            Some(a) => a.send_event(EngineEvent::SystemDone).unwrap(),
            None => {}
        });
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
                f();
            }
            Err((s, e)) => {
                self.non_send_queue.push_back(s);
            }
        };
    }
}

impl<Args: 'static> Runtime<Args> for MultiThreaded<Args> {
    fn run_send(&mut self, s: SystemSendRef<Args>, args: &mut Args) {
        self.try_run_send(s, args);
    }

    fn add_non_send(&mut self, s: SystemRef<Args>, args: &mut Args) {
        self.try_run_non_send(s, args);
    }

    fn use_event_loop_proxy(&mut self, proxy: EventLoopProxy<EngineEvent>) {
        *self.proxy.blocking_lock() = Some(proxy);
    }

    fn process_engine_event(&mut self, event: &EngineEvent, args: &mut Args) {
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

    fn cleanup_systems(
        &mut self,
        args: &mut Args,
        system_registries: &mut [&mut SystemRegistry<Args>],
    ) {
        for data in self.pool.try_recv_iter() {
            match data {
                Ok((mut cleanup, (system, state))) => {
                    cleanup.cleanup(args);

                    system_registries
                        .iter_mut()
                        .find_map(|registry| {
                            if registry.try_recieve_send_ref(&system).is_ok() {
                                match state {
                                    SystemControlFlow::Continue => {}
                                    SystemControlFlow::Quit => panic!("Quit"),
                                    SystemControlFlow::Remove => {
                                        registry.remove_system_send(&system.id());
                                    }
                                }
                                Some(())
                            } else {
                                None
                            }
                        })
                        .unwrap();
                }
                Err(err) => panic!("subthread panicked with {:?}", err),
            }
        }
    }
}
