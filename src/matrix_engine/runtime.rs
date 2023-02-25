use std::{
    any::TypeId,
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Arc,
    },
    thread,
    time::{Duration, Instant},
};

use super::{
    registry::Registry,
    systems::{System, SystemArgs, SystemCreator},
};

#[derive(Default)]
pub struct Runtime {
    registry: Registry,
    systems: Vec<SystemCreator>,
    quit: Arc<AtomicBool>,
    target: Arc<AtomicU64>,
}



impl Runtime {
    pub fn with_registry(r: Registry) -> Self {
        Self {
            registry: r,
            ..Default::default()
        }
    }

    pub fn run(self) {
        let mut v = Vec::new();

        for sys in self.systems {
            let quit = self.quit.clone();
            let target_fps = self.target.clone();
            let comps = self.registry.get_component_registry();
            v.push(thread::spawn(move || {
                let mut sys = sys.create();
                let mut target;
                let mut fps;
                let mut start = Instant::now();
                let mut len;
                while !quit.load(std::sync::atomic::Ordering::Relaxed) {
                    sys.update(SystemArgs::new(quit.clone(), comps.clone()));
                    fps = 1.0 / target_fps.load(Ordering::Relaxed) as f64;
                    if fps.is_finite() {
                        target = Duration::from_secs_f64(fps);
                        len = Instant::now() - start;
                        if len < target {
                            spin_sleep::sleep(target - len);
                        }
                        start = Instant::now();
                    }
                }
            }))
        }

        for i in v {
            i.join().unwrap();
        }
    }

    pub(crate) fn insert(&mut self, a: SystemCreator)  {
        self.systems.push(a);
    }
}
