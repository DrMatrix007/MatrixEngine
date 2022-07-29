
use std::{
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc, RwLock, RwLockReadGuard, RwLockWriteGuard,
    },
    thread,
};

use super::{
    application::Application, ecs::registry::Registry, event::Events, utils::clock::Clock,
};

pub struct LayerArgs {
    pub events: Arc<RwLock<Events>>,
    pub(super) registry: Arc<RwLock<Registry>>,
    pub time: Clock,
    pub(crate) quit_ref: Arc<AtomicBool>,
}
impl Clone for LayerArgs {
    fn clone(&self) -> Self {
        Self {
            events: self.events.clone(),
            registry: self.registry.clone(),
            time: self.time.clone(),
            quit_ref: self.quit_ref.clone(),
        }
    }
}

unsafe impl Send for LayerArgs {}

impl LayerArgs {
    pub fn stop_application(&self) {
        self.quit_ref.store(true, Ordering::Relaxed);
    }
    pub fn read_registry(&self) -> Option<RwLockReadGuard<'_, Registry>> {
        self.registry.read().ok()
    }
    pub fn write_registry(&self) -> Option<RwLockWriteGuard<'_, Registry>> {
        self.registry.write().ok()
    }
}

pub trait Layer: Send + Sync {
    fn on_start(&mut self, _args: &LayerArgs);
    fn on_update(&mut self, _args: &LayerArgs);
    fn on_clean_up(&mut self);
}

pub(crate) struct LayerHolder {
    layer: Box<dyn Layer>,
    started: bool,
}

unsafe impl Send for LayerHolder {}

impl LayerHolder {
    pub(crate) fn new(b: Box<dyn Layer>) -> Self {
        LayerHolder {
            layer: b,
            started: false,
        }
    }
    pub(super) fn begin_thread(mut self, thread_c: Arc<AtomicUsize>, args: LayerArgs) {
        thread_c.fetch_add(1, Ordering::SeqCst);
        thread::spawn(move || {

            self.start(&args);

            let mut dt_checker = Clock::start_new();
            let mut dt;
            let mut target;
            while !args.is_running() {
                dt_checker.restart();
                self.update(&args);

                dt = dt_checker.elapsed().as_secs_f64();
                target = args.get_frame_time_as_secs();
                // println!("dt = {}; target = {}", dt, target);
                if dt < target {
                    thread::sleep(Duration::from_secs_f64(target - dt));
                }

            }
            thread_c.fetch_sub(1, Ordering::Relaxed)
        });
    }


    
    fn update(&mut self, _args: &LayerArgs) {
        self.layer.as_mut().on_update(_args);
    }
    fn start(&mut self, _args: &LayerArgs) {
        if !self.started {
            self.started = true;
            self.layer.as_mut().on_start(_args);
        }
    }

    pub(crate) fn clean_up(&mut self) {
        self.layer.as_mut().on_clean_up();
    }
}

pub(crate) struct LayerPool {
    vec: Arc<RwLock<Vec<LayerHolder>>>,
    threads: Arc<AtomicUsize>,
}

impl LayerPool {
    pub fn new(v:Arc<RwLock<Vec<LayerHolder>>>) -> Self {
        Self {
            vec: v,
            threads: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn push_layer<T: Layer + 'static>(&mut self, l: T) {
        (self.vec.write().unwrap()).push(LayerHolder::new(Box::new(l)));
    }

    pub fn start_execution(&self, events:Arc<RwLock<Events>>,registry:Arc<RwLock<Registry>>,quit_ref:Arc<AtomicBool>) {
        let clock = Clock::start_new();
        for l in self.vec.write().unwrap().iter_mut() {
            let _args = LayerArgs {
                events: events.clone(),
                registry: registry.clone(),
                time: clock,
                quit_ref: quit_ref.clone(),
            };
            thread::spawn(move || {
                l.start(&_args);
                loop {
                    l.update(&_args);
                }
            });
        }
    }
}
