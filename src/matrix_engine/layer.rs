use std::{
    sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}},
    time::Duration,
};

use super::{ecs::registry::Registry, event::Events};

pub struct LayerArgs<'a,'b> {
    pub events: &'a Events,
    pub registry: Arc<Mutex<Registry>>,
    pub delta_time: Duration,
    pub time: Duration,
    pub(crate) quit_ref: &'b AtomicBool,
}

impl<'a,'b> LayerArgs<'a,'b> {
    pub fn stop_application(&self) {
        self.quit_ref.store(true,Ordering::Relaxed);
    }
}

pub trait Layer {
    fn on_start(&mut self, _args: &LayerArgs);
    fn on_update(&mut self, _args: &LayerArgs);
    fn on_clean_up(&mut self);
}

pub(crate) struct LayerHolder {
    layer: Box<dyn Layer>,
    started: bool,
}
impl LayerHolder {
    pub(crate) fn new(b: Box<dyn Layer>) -> Self {
        LayerHolder {
            layer: b,
            started: false,
        }
    }
    pub(crate) fn update(&mut self, _args: &LayerArgs) {
        self.layer.as_mut().on_update(_args);
    }
    pub(crate) fn start(&mut self, _args: &LayerArgs) {
        if !self.started {
            self.started = true;
            self.layer.as_mut().on_start(_args);
        }
    }

    pub(crate) fn clean_up(&mut self) {
        self.layer.as_mut().on_clean_up();
    }
}
