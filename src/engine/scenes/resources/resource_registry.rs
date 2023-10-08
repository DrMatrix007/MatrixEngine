use std::{
    any::{Any, TypeId},
    cell::UnsafeCell,
    collections::BTreeMap,
    ops::{Deref, DerefMut},
    ptr::NonNull,
    rc::Rc,
    sync::Arc,
};

use tokio::sync::{OwnedRwLockReadGuard, OwnedRwLockWriteGuard, RwLock};

use super::Resource;

#[derive(Default, Debug)]
pub struct ResourceRegistry {
    map: BTreeMap<TypeId, Box<dyn Any>>,
}

#[derive(Debug)]
pub enum ResourcesState<T: Resource> {
    Write(Rc<UnsafeCell<ResourceHolder<T>>>),
    Read(Rc<UnsafeCell<ResourceHolder<T>>>),
    Taken,
}
#[derive(Debug)]
pub struct ResourcesNotAvailable;

pub struct ResourceRef<T: Resource> {
    ptr: NonNull<ResourceHolder<T>>,
}
unsafe impl<T: Resource + Send> Send for ResourceRef<T> {}
unsafe impl<T: Resource + Send> Sync for ResourceRef<T> {}

impl<T: Resource> Deref for ResourceRef<T> {
    type Target = ResourceHolder<T>;

    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}

pub struct ResourceMut<T: Resource> {
    ptr: NonNull<ResourceHolder<T>>,
}

unsafe impl<T: Resource + Send> Send for ResourceMut<T> {}
unsafe impl<T: Resource + Send> Sync for ResourceMut<T> {}

impl<T: Resource> DerefMut for ResourceMut<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.ptr.as_mut() }
    }
}

impl<T: Resource> Deref for ResourceMut<T> {
    type Target = ResourceHolder<T>;

    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T: Resource> ResourceRef<T> {
    fn new(ptr: NonNull<ResourceHolder<T>>) -> Self {
        Self { ptr }
    }
}
impl<T: Resource> ResourceMut<T> {
    fn new(ptr: NonNull<ResourceHolder<T>>) -> Self {
        Self { ptr }
    }
}

impl<T: Resource> ResourcesState<T> {
    fn try_read(&mut self) -> Result<ResourceRef<T>, ResourcesNotAvailable> {
        match self {
            ResourcesState::Write(a) => {
                let a = a.clone();
                let r = ResourceRef::new(NonNull::new(a.get()).unwrap());
                *self = ResourcesState::Read(a);
                Ok(r)
            }
            ResourcesState::Read(a) => Ok(ResourceRef::new(NonNull::new(a.get()).unwrap())),
            ResourcesState::Taken => Err(ResourcesNotAvailable),
        }
    }

    fn try_write(&mut self) -> Result<ResourceMut<T>, ResourcesNotAvailable> {
        match self {
            ResourcesState::Write(a) => {
                let r = ResourceMut::new(NonNull::new(a.get()).unwrap());
                *self = ResourcesState::Taken;

                Ok(r)
            }
            _ => Err(ResourcesNotAvailable),
        }
    }
}

impl ResourceRegistry {
    pub fn try_read<R: Resource + 'static>(
        &mut self,
    ) -> Result<ResourceRef<R>, ResourcesNotAvailable> {
        self.map
            .entry(TypeId::of::<R>())
            .or_insert_with(|| {
                Box::new(ResourcesState::Read(Rc::new(UnsafeCell::new(
                    ResourceHolder::<R>::default(),
                ))))
            })
            .downcast_mut::<ResourcesState<R>>()
            .unwrap()
            .try_read()
    }

    pub fn try_write<C: Resource + 'static>(
        &mut self,
    ) -> Result<ResourceMut<C>, ResourcesNotAvailable> {
        self.map
            .entry(TypeId::of::<C>())
            .or_insert_with(|| {
                Box::new(ResourcesState::Write(Rc::new(UnsafeCell::new(
                    ResourceHolder::<C>::default(),
                ))))
            })
            .downcast_mut::<ResourcesState<C>>()
            .unwrap()
            .try_write()
    }
}

#[derive(Debug)]
pub struct ResourceHolder<R: Resource> {
    data: Option<R>,
}

impl<R: Resource> AsMut<Option<R>> for ResourceHolder<R> {
    fn as_mut(&mut self) -> &mut Option<R> {
        &mut self.data
    }
}

impl<R: Resource> AsRef<Option<R>> for ResourceHolder<R> {
    fn as_ref(&self) -> &Option<R> {
        &self.data
    }
}

impl<R: Resource> Default for ResourceHolder<R> {
    fn default() -> Self {
        Self {
            data: Default::default(),
        }
    }
}
