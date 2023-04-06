use std::{
    sync::{atomic::AtomicU64, Arc},
    time::{Instant, Duration},
};

pub struct EventLoop {
    target_fps: Arc<AtomicU64>,
    last_frame: Instant,
}

impl EventLoop {
    pub fn new(target_fps: Arc<AtomicU64>) -> Self {
        Self {
            target_fps,
            last_frame: Instant::now(),
        }
    }
    pub fn wait(&self) {
        let fps = self.target_fps.load(std::sync::atomic::Ordering::Relaxed);
        let s = 1. / fps as f64;
        let now = Instant::now();
        let current = (now - self.last_frame).as_secs_f64();
        if  current < s {
            spin_sleep::sleep(Duration::from_secs_f64(s - current));
        }
    }
    pub fn capture(&mut self) {
        self.last_frame = Instant::now();
    }
}
