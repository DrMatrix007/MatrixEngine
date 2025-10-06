use anymap::AnyMap;
use winit::window::Window;

use crate::lockable::{Lockable, LockableError, LockableWriteGuard};

pub trait Resource: Send {}

pub struct ResourceHolder<T> {
    data: Option<T>,
}

impl<T> Default for ResourceHolder<T> {
    fn default() -> Self {
        Self::new(None)
    }
}

impl<T> ResourceHolder<T> {
    pub fn new(data: Option<T>) -> Self {
        Self { data }
    }

    pub fn replace(&mut self, data: T) -> Option<T> {
        self.data.replace(data)
    }

    pub fn as_ref(&self) -> Option<&T> {
        self.data.as_ref()
    }

    pub fn as_mut(&mut self) -> Option<&mut T> {
        self.data.as_mut()
    }
}

pub struct ResourceRegistry {
    resources: AnyMap,
}

impl Default for ResourceRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ResourceRegistry {
    pub fn new() -> Self {
        Self {
            resources: AnyMap::new(),
        }
    }
    pub fn insert<T: Resource + 'static>(&mut self, data: T) -> Result<Option<T>, LockableError> {
        let mut guard = self.write::<T>()?;

        let res = guard.replace(data);
        
        self.write_consume(guard)?;
        
        Ok(res)
    }

    pub fn write<T: Resource + 'static>(
        &mut self,
    ) -> Result<LockableWriteGuard<ResourceHolder<T>>, LockableError> {
        self.resources
            .entry::<Lockable<ResourceHolder<T>>>()
            .or_insert_with(Default::default)
            .write()
    }

    pub fn write_consume<T: Resource + 'static>(
        &mut self,
        data: LockableWriteGuard<ResourceHolder<T>>,
    ) -> Result<(), LockableError> {
        self.resources
            .entry::<Lockable<ResourceHolder<T>>>()
            .or_insert_with(Default::default)
            .consume_write(data)
    }
}


impl Resource for Window {}