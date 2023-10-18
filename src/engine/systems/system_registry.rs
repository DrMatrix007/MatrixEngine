use std::{cell::UnsafeCell, collections::BTreeMap, ptr::NonNull, time::Instant};

use crate::engine::scenes::entities::Entity;

use super::{System, SystemSend};

pub struct BoxedSystem<Args> {
    system: Box<UnsafeCell<dyn System<Args>>>,
    id: Entity,
    running: bool,
    taken_when: Instant,
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

    pub fn id(&self) -> Entity {
        self.id
    }
}

impl<Args> BoxedSystem<Args> {
    pub fn new(sys: impl System<Args> + 'static) -> Self {
        Self {
            system: Box::new(UnsafeCell::new(sys)),
            id: Entity::new(),
            running: false,
            taken_when: Instant::now(),
        }
    }
    pub fn try_lock(&mut self) -> Result<SystemRef<Args>, ()> {
        if self.running {
            Err(())
        } else {
            self.running = true;
            self.taken_when = Instant::now();
            Ok(SystemRef::new(
                NonNull::new(self.system.get()).unwrap(),
                self.id.clone(),
            ))
        }
    }

    pub fn id(&self) -> Entity {
        self.id
    }

    fn try_receive_ref(&mut self, system_ref: &Entity) -> Result<(), NotSuitableSystemReceive> {
        if self.id == *system_ref {
            self.running = false;
            Ok(())
        } else {
            Err(NotSuitableSystemReceive)
        }
    }

    pub fn taken_when(&self) -> Instant {
        self.taken_when
    }
}
pub struct BoxedSystemSend<Args> {
    system: Box<UnsafeCell<dyn SystemSend<Args>>>,
    running: bool,
    id: Entity,
    taken_when: Instant,
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

    pub fn id(&self) -> Entity {
        self.id
    }
}

#[derive(Debug)]
pub struct NotSuitableSystemReceive;

impl<Args> BoxedSystemSend<Args> {
    pub fn new(sys: impl SystemSend<Args> + 'static) -> Self {
        Self {
            system: Box::new(UnsafeCell::new(sys)),
            running: false,
            id: Entity::new(),
            taken_when: Instant::now(),
        }
    }
    pub fn try_lock(&mut self) -> Result<SystemSendRef<Args>, ()> {
        if self.running {
            Err(())
        } else {
            self.running = true;
            self.taken_when = Instant::now();
            Ok(SystemSendRef::new(
                NonNull::new(self.system.get()).unwrap(),
                self.id.clone(),
            ))
        }
    }

    pub fn id(&self) -> Entity {
        self.id
    }

    fn try_receive_ref(&mut self, system_ref: &Entity) -> Result<(), NotSuitableSystemReceive> {
        if self.id == *system_ref {
            self.running = false;
            Ok(())
        } else {
            Err(NotSuitableSystemReceive)
        }
    }

    pub fn taken_when(&self) -> Instant {
        self.taken_when
    }
}

pub struct SystemRegistry<Args> {
    send: BTreeMap<Entity, BoxedSystemSend<Args>>,

    non_send: BTreeMap<Entity, BoxedSystem<Args>>,
}

impl<Args> Default for SystemRegistry<Args> {
    fn default() -> Self {
        Self {
            non_send: Default::default(),
            send: Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct SystemNotFound;

impl<Args> SystemRegistry<Args> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push_send(&mut self, sys: impl SystemSend<Args> + 'static) {
        let sys = BoxedSystemSend::new(sys);
        self.send.insert(sys.id(), sys);
    }

    pub fn push_non_send(&mut self, sys: impl System<Args> + 'static) {
        let sys = BoxedSystem::new(sys);
        self.non_send.insert(sys.id(), sys);
    }
    pub fn try_lock_iter_send<'a>(&'a mut self) -> impl Iterator<Item = SystemSendRef<Args>> + 'a {
        self.send.iter_mut().filter_map(|x| x.1.try_lock().ok())
    }
    pub fn try_lock_iter_non_send<'a>(&'a mut self) -> impl Iterator<Item = SystemRef<Args>> + 'a {
        self.non_send.iter_mut().filter_map(|x| x.1.try_lock().ok())
    }

    pub fn try_recieve_send_with_id(
        &mut self,
        system_ref: &Entity,
    ) -> Result<&mut BoxedSystemSend<Args>, SystemNotFound> {
        match self.send.get_mut(&system_ref) {
            Some(system) => {
                system.try_receive_ref(system_ref).unwrap();
                Ok(system)
            }
            None => Err(SystemNotFound),
        }
    }

    pub fn try_recieve_non_send_with_id(
        &mut self,
        system_ref: &Entity,
    ) -> Result<(), SystemNotFound> {
        match self.non_send.get_mut(&system_ref) {
            Some(system) => {
                system.try_receive_ref(system_ref).unwrap();
                Ok(())
            }
            None => Err(SystemNotFound),
        }
    }

    pub fn remove_system_send(&mut self, id: &Entity) {
        self.send.remove(id);
    }
    pub fn remove_system_non_send(&mut self, id: &Entity) {
        self.non_send.remove(id);
    }
}
