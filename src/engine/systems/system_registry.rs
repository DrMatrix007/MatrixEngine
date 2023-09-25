use std::sync::Arc;

use tokio::sync::{Mutex, OwnedMutexGuard};

use super::{System, SystemSend};

pub struct BoxedSystem<Args> {
    system: Arc<Mutex<dyn System<Args>>>,
}

impl<Args> BoxedSystem<Args> {
    pub fn new(sys: impl System<Args> + 'static) -> Self {
        Self {
            system: Arc::new(Mutex::new(sys)),
        }
    }
    pub fn try_lock(&self) -> Result<OwnedMutexGuard<dyn System<Args>>, tokio::sync::TryLockError> {
        self.system.clone().try_lock_owned()
    }
}
pub struct BoxedSystemSend<Args> {
    system: Arc<Mutex<dyn SystemSend<Args>>>,
}

impl<Args> BoxedSystemSend<Args> {
    pub fn new(sys: impl SystemSend<Args> + 'static) -> Self {
        Self {
            system: Arc::new(Mutex::new(sys)),
        }
    }
    pub fn try_lock(
        &self,
    ) -> Result<OwnedMutexGuard<dyn SystemSend<Args>>, tokio::sync::TryLockError> {
        self.system.clone().try_lock_owned()
    }
}

pub struct SystemRegistry<Args> {
    send: Vec<BoxedSystemSend<Args>>,

    non_send: Vec<BoxedSystem<Args>>,
}

impl<Args> Default for SystemRegistry<Args> {
    fn default() -> Self {
        Self {
            non_send: Default::default(),
            send: Default::default(),
        }
    }
}

impl<Args> SystemRegistry<Args> {
    pub fn push_send(&mut self, sys: impl SystemSend<Args> + 'static) {
        self.send.push(BoxedSystemSend::new(sys));
    }

    pub fn push_non_send(&mut self, sys: impl System<Args> + 'static) {
        self.non_send.push(BoxedSystem::new(sys));
    }
    pub fn try_lock_iter_send<'a>(
        &'a self,
    ) -> impl Iterator<Item = OwnedMutexGuard<dyn SystemSend<Args>>> + 'a {
        self.send.iter().filter_map(|x| x.clone().try_lock().ok())
    }
}
