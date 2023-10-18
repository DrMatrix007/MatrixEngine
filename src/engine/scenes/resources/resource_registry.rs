use std::{
    any::{Any, TypeId},
    cell::UnsafeCell,
    collections::BTreeMap,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

use super::Resource;

#[derive(Default, Debug)]
pub struct ResourceRegistry {
    map: BTreeMap<TypeId, Box<dyn Any>>,
}

#[derive(Debug)]
pub struct ResourcesNotAvailable;

pub struct ResourceRef<T: Resource> {
    ptr: NonNull<ResourceHolder<T>>,
}
unsafe impl<T: Resource + Send> Send for ResourceRef<T> {}
unsafe impl<T: Resource + Sync> Sync for ResourceRef<T> {}

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
unsafe impl<T: Resource + Sync> Sync for ResourceMut<T> {}

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

#[derive(Debug)]
pub struct NotSuitableResourceRecieve;
#[derive(Debug)]
enum State {
    Write,
    Read(i64),
    Taken,
}

#[derive(Debug)]
pub struct ResourceState<T: Resource> {
    comps: Box<UnsafeCell<ResourceHolder<T>>>,
    state: State,
}

impl<T: Resource> ResourceState<T> {
    pub fn new(comps: Box<UnsafeCell<ResourceHolder<T>>>) -> Self {
        Self {
            comps,
            state: State::Write,
        }
    }
    fn try_read(&mut self) -> Result<ResourceRef<T>, ResourcesNotAvailable> {
        match &mut self.state {
            State::Write => {
                self.state = State::Read(1);
                Ok(ResourceRef::new(NonNull::new(self.comps.get()).unwrap()))
            }
            State::Read(counter) => {
                *counter += 1;
                Ok(ResourceRef::new(NonNull::new(self.comps.get()).unwrap()))
            }
            State::Taken => Err(ResourcesNotAvailable),
        }
    }

    fn try_write(&mut self) -> Result<ResourceMut<T>, ResourcesNotAvailable> {
        match &self.state {
            State::Write => {
                self.state = State::Taken;
                Ok(ResourceMut::new(NonNull::new(self.comps.get()).unwrap()))
            }
            _ => Err(ResourcesNotAvailable),
        }
    }

    fn recieve_ref(&mut self, comps: &ResourceRef<T>) -> Result<(), NotSuitableResourceRecieve> {
        match &mut self.state {
            State::Read(count) => match count {
                count if *count > 0 => {
                    *count -= 1;
                    Ok(())
                }
                _ => Err(NotSuitableResourceRecieve),
            },
            _ => Err(NotSuitableResourceRecieve),
        }
    }
    fn recieve_mut(&mut self, comps: &ResourceMut<T>) -> Result<(), NotSuitableResourceRecieve> {
        match &mut self.state {
            State::Taken => {
                self.state = State::Write;
                Ok(())
            }
            _ => Err(NotSuitableResourceRecieve),
        }
    }

    fn available_for_read(&self) -> bool {
        match self.state {
            State::Taken => false,
            State::Write | State::Read(_) => true,
        }
    }
    fn available_for_write(&self) -> bool {
        match self.state {
            State::Taken => false,
            State::Read(count) => count == 0,
            State::Write => true,
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
                Box::new(ResourceState::new(Box::new(UnsafeCell::new(
                    ResourceHolder::<R>::default(),
                ))))
            })
            .downcast_mut::<ResourceState<R>>()
            .unwrap()
            .try_read()
    }

    pub fn try_write<C: Resource + 'static>(
        &mut self,
    ) -> Result<ResourceMut<C>, ResourcesNotAvailable> {
        self.map
            .entry(TypeId::of::<C>())
            .or_insert_with(|| {
                Box::new(ResourceState::new(Box::new(UnsafeCell::new(
                    ResourceHolder::<C>::default(),
                ))))
            })
            .downcast_mut::<ResourceState<C>>()
            .unwrap()
            .try_write()
    }

    pub fn try_recieve_mut<R: Resource + 'static>(
        &mut self,
        comps: &ResourceMut<R>,
    ) -> Result<(), NotSuitableResourceRecieve> {
        self.map
            .entry(TypeId::of::<R>())
            .or_insert_with(|| {
                Box::new(ResourceState::new(Box::new(UnsafeCell::new(
                    ResourceHolder::<R>::default(),
                ))))
            })
            .downcast_mut::<ResourceState<R>>()
            .unwrap()
            .recieve_mut(comps)
    }
    pub fn try_recieve_ref<R: Resource + 'static>(
        &mut self,
        comps: &ResourceRef<R>,
    ) -> Result<(), NotSuitableResourceRecieve> {
        self.map
            .entry(TypeId::of::<R>())
            .or_insert_with(|| {
                Box::new(ResourceState::new(Box::new(UnsafeCell::new(
                    ResourceHolder::<R>::default(),
                ))))
            })
            .downcast_mut::<ResourceState<R>>()
            .unwrap()
            .recieve_ref(&comps)
    }

    pub(crate) fn available_for_write<R: Resource + 'static>(&self) -> bool {
        match self.map.get(&TypeId::of::<R>()) {
            Some(data) => data
                .downcast_ref::<ResourceState<R>>()
                .unwrap()
                .available_for_write(),
            None => true,
        }
    }
    pub(crate) fn available_for_read<R: Resource + 'static>(&self) -> bool {
        match self.map.get(&TypeId::of::<R>()) {
            Some(data) => data
                .downcast_ref::<ResourceState<R>>()
                .unwrap()
                .available_for_read(),
            None => true,
        }
    }
}

#[derive(Debug)]
pub struct ResourceHolder<R: Resource> {
    data: Option<R>,
}

impl<R: Resource> ResourceHolder<R> {
    pub fn get_or_insert_with(&mut self, f: impl FnOnce() -> R) -> &mut R {
        self.data.get_or_insert_with(f)
    }
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
