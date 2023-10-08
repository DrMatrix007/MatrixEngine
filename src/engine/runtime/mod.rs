use std::{collections::VecDeque, sync::Arc};

use tokio::sync::Mutex;
use winit::event_loop::EventLoopProxy;

use self::thread_pool::ThreadPool;

use super::{
    events::engine_event::EngineEvent,
    systems::{
        system_registry::{SystemRef, SystemRegistry, SystemSendRef},
        Dispatcher, DispatcherSend,
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

        for i in system_registry.try_lock_iter_non_send() {
            self.add_non_send(i, args)
        }
    }

    fn process_engine_event(&mut self, event: &EngineEvent, args: &mut Args) {}

    fn use_event_loop_proxy(&mut self, proxy: EventLoopProxy<EngineEvent>);
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
        s.dispatch(args).map_err(|e| (e.1)).unwrap()();
    }

    fn add_non_send(&mut self, s: SystemRef<Args>, args: &mut Args) {
        s.dispatch(args).map_err(|e| (e.1)).unwrap()();
    }

    fn use_event_loop_proxy(&mut self, proxy: EventLoopProxy<EngineEvent>) {
        self.proxy = Some(proxy);
    }
}

pub struct MultiThreaded<Args> {
    pool: ThreadPool,
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
                self.pool
                    .add(|| {
                        f();
                    })
                    .unwrap();
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
    fn add_send(&mut self, s: SystemSendRef<Args>, args: &mut Args) {
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
}
