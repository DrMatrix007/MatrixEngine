use core::time;
use std::{
    slice::{Iter, IterMut},
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc, RwLock, RwLockReadGuard, RwLockWriteGuard,
    }, thread, time::Duration,
};

use super::{ecs::registry::Registry, event::Events, utils::clock::Clock};

pub struct LayerArgs {
    pub(super) events: Arc<RwLock<Events>>,
    pub(super) registry: Arc<RwLock<Registry>>,
    pub time: Clock,
    pub(crate) quit_ref: Arc<AtomicBool>,
    pub(super) target_nanos_per_second: Arc<RwLock<Duration>>,
}
unsafe impl Send for LayerArgs {}
impl LayerArgs {
    pub fn stop_application(&self) {
        self.quit_ref.store(true, Ordering::Relaxed);
    }
    pub(super) fn is_running(&self) -> bool {
        self.quit_ref.load(Ordering::Relaxed)
    }
    pub(super) fn get_frame_time(&self) -> Duration {
        *self.target_nanos_per_second.read().unwrap()
    }

    pub fn read_registry(&self) -> Option<RwLockReadGuard<Registry>> {
        self.registry.try_read().ok()
    }
    pub fn write_registry(&self) -> Option<RwLockWriteGuard<Registry>> {
        self.registry.write().ok()
    }

    pub fn write_events(&self) -> Option<RwLockReadGuard<Events>> {
        self.events.try_read().ok()
    }
    pub fn read_events(&self) -> Option<RwLockWriteGuard<Events>> {
        self.events.try_write().ok()
    }
}
impl Clone for LayerArgs {
    fn clone(&self) -> Self {
        Self {
            events: self.events.clone(),
            registry: self.registry.clone(),
            time: self.time,
            quit_ref: self.quit_ref.clone(),
            target_nanos_per_second: self.target_nanos_per_second.clone(),
        }
    }
}

pub trait Layer: Send {
    fn on_start(&mut self, _args: &LayerArgs);
    fn on_update(&mut self, _args: &LayerArgs);
    fn on_clean_up(&mut self);
}

pub(crate) struct LayerHolder {
    layer: Box<dyn Layer>,
    started: bool,
}

unsafe impl Send for LayerHolder {}
unsafe impl Sync for LayerHolder {}

impl LayerHolder {
    pub(crate) fn new(b: Box<dyn Layer>) -> Self {
        LayerHolder {
            layer: b,
            started: false,
        }
    }
    pub(super) fn begin_thread(mut self, thread_c: Arc<AtomicUsize>, args: LayerArgs) {
        thread_c.fetch_add(1, Ordering::Relaxed);
        thread::spawn(move || {
            self.start(&args);
            let mut dt;
            let mut dt_checker = Clock::start_new();
            let mut target;
            let mut d = Duration::default();
            while !args.is_running() {
                target = args.get_frame_time();
                // println!("check: {}",dt_checker.restart().as_secs_f64());
                
                self.update(&args);
                dt_checker.restart();

                dt =dt_checker.elapsed();
                // println!("{}",d.as_secs_f64());
                if dt < target {
                    d = target-dt;
                    spin_sleep::sleep(d);
                    // dt_checker.restart();
                }

                // println!("{} {}",dt_checker.restart().as_secs_f64(),(d+dt).as_secs_f64());
                // dt_checker.restart();
            }
            self.layer.on_clean_up();
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

pub(super) struct LayerPool {
    vec: Vec<LayerHolder>,
    args: Option<LayerArgs>,
    counter: Arc<AtomicUsize>,
}
impl LayerPool {
    pub fn new() -> Self {
        LayerPool {
            vec: Vec::new(),
            args: None,
            counter: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn count_running(&self) -> usize {
        self.counter.load(Ordering::Relaxed)
    }
    pub fn iter(&self) -> Iter<LayerHolder> {
        self.vec.iter()
    }
    pub fn is_done(&self) -> bool {
        self.args.is_some() && (self.counter.load(Ordering::Relaxed) == 0)
    }
    pub fn iter_mut(&mut self) -> IterMut<LayerHolder> {
        self.vec.iter_mut()
    }
    pub fn start_all(&mut self, args: LayerArgs) {
        self.args = Some(args.clone());
        while let Some(layer) = self.vec.pop() {
            layer.begin_thread(self.counter.clone(), args.clone());
        }
    }
    pub fn push_layer<T: Layer + 'static>(&mut self, layer: T) {
        if let Some(args) = &self.args {
            let v = LayerHolder::new(Box::new(layer));
            v.begin_thread(self.counter.clone(), args.clone());
        } else {
            self.vec.push(LayerHolder::new(Box::new(layer)));
        }
    }
}
