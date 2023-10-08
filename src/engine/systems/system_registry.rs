use std::{
    cell::{RefCell, UnsafeCell},
    ptr::NonNull,
    sync::Arc,
};

use tokio::sync::{Mutex, OwnedMutexGuard};

use crate::engine::scenes::entities::Entity;

use super::{System, SystemSend};

pub struct BoxedSystem<Args> {
    system: Box<UnsafeCell<dyn System<Args>>>,
    id: Entity,
    running: bool,
}

pub struct SystemRef<Args> {
    system: NonNull<dyn System<Args>>,
    id: Entity,
}
impl<Args> SystemRef<Args> {
    fn new(system: NonNull<dyn System<Args>>, id: Entity) -> Self {
        Self { id, system }
    }

    pub unsafe fn system_mut(&mut self) -> &mut dyn System<Args> {
        self.system.as_mut()
    }
}

impl<Args> BoxedSystem<Args> {
    pub fn new(sys: impl System<Args> + 'static) -> Self {
        Self {
            system: Box::new(UnsafeCell::new(sys)),
            id: Entity::new(),
            running: false,
        }
    }
    pub fn try_lock(&mut self) -> Result<SystemRef<Args>, ()> {
        if self.running {
            Err(())
        } else {
            self.running = true;
            Ok(SystemRef::new(
                NonNull::new(self.system.get()).unwrap(),
                self.id.clone(),
            ))
        }
    }
}
pub struct BoxedSystemSend<Args> {
    system: Box<UnsafeCell<dyn SystemSend<Args>>>,
    running: bool,
    id: Entity,
}
pub struct SystemSendRef<Args> {
    system: NonNull<dyn SystemSend<Args>>,
    id: Entity,
}

unsafe impl<Args> Send for SystemSendRef<Args> {}

impl<Args> SystemSendRef<Args> {
    fn new(system: NonNull<dyn SystemSend<Args>>, id: Entity) -> SystemSendRef<Args> {
        Self { id, system }
    }

    pub unsafe fn system_mut(&mut self) -> &mut dyn SystemSend<Args> {
        self.system.as_mut()
    }
}

impl<Args> BoxedSystemSend<Args> {
    pub fn new(sys: impl SystemSend<Args> + 'static) -> Self {
        Self {
            system: Box::new(UnsafeCell::new(sys)),
            running: false,
            id: Entity::new(),
        }
    }
    pub fn try_lock(&mut self) -> Result<SystemSendRef<Args>, ()> {
        if self.running {
            Err(())
        } else {
            self.running = true;
            Ok(SystemSendRef::new(
                NonNull::new(self.system.get()).unwrap(),
                self.id.clone(),
            ))
        }
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
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push_send(&mut self, sys: impl SystemSend<Args> + 'static) {
        self.send.push(BoxedSystemSend::new(sys));
    }

    pub fn push_non_send(&mut self, sys: impl System<Args> + 'static) {
        self.non_send.push(BoxedSystem::new(sys));
    }
    pub fn try_lock_iter_send<'a>(&'a mut self) -> impl Iterator<Item = SystemSendRef<Args>> + 'a {
        self.send.iter_mut().filter_map(|x| x.try_lock().ok())
    }
    pub fn try_lock_iter_non_send<'a>(&'a mut self) -> impl Iterator<Item = SystemRef<Args>> + 'a {
        self.non_send.iter_mut().filter_map(|x| x.try_lock().ok())
    }
}
