use std::{
    sync::{
        atomic::{AtomicBool, Ordering, AtomicU64},
        Arc,
    },
    thread,
    time::{Duration, Instant},
};

use super::{registry::Registry, systems::SystemArgs};

pub struct Application {
    registry: Registry,
    running: Arc<AtomicBool>,
    target_fps: Arc<AtomicU64>,
}

impl Default for Application {
    fn default() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(true)),
            registry: Default::default(),
            target_fps: Arc::new(AtomicU64::new(200)),
        }
    }
}

impl Application {
    pub fn from_registry(r: Registry) -> Self {
        Self {
            registry: r,
            ..Default::default()
        }
    }
    pub fn mod_registry(&mut self, f: impl FnOnce(&mut Registry)) {
        f(&mut self.registry);
    }

    pub fn run(self) {
        let mut threads = Vec::new();
        for (_, mut sys) in self.registry.systems.into_iter() {
            let running = self.running.clone();
            let comps = self.registry.data.clone();
            let target_fps = self.target_fps.clone();
            threads.push(thread::spawn(move || {
                let mut target;
                let mut fps;
                let mut start = Instant::now();
                let mut len;
                while running.load(Ordering::Acquire) {
                    sys.update(SystemArgs::new(running.clone(), comps.clone()));
                    fps = 1.0 / target_fps.load(Ordering::Acquire) as f64;
                    if fps.is_finite() {
                        target = Duration::from_secs_f64(fps);
                        len = Instant::now() - start;
                        if len < target {
                            spin_sleep::sleep(target - len);
                        }
                        start = Instant::now();
                    }

                }
            }));
        }
        for t in threads {
            t.join().unwrap();
        }
    }
}
