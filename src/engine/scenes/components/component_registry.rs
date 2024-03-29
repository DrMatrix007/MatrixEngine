use std::{
    any::{Any, TypeId},
    cell::UnsafeCell,
    collections::{btree_map, BTreeMap},
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

use crate::engine::scenes::entities::Entity;

use super::Component;

#[derive(Debug)]
pub struct Components<C: Component> {
    map: BTreeMap<Entity, C>,
}

impl<C: Component> Components<C> {
    pub fn new() -> Self {
        Self {
            map: Default::default(),
        }
    }
    pub fn add(&mut self, e: Entity, c: C) {
        self.map.insert(e, c);
    }
    pub fn get(&self, e: &Entity) -> Option<&C> {
        self.map.get(e)
    }
    pub fn get_mut(&mut self, e: &Entity) -> Option<&mut C> {
        self.map.get_mut(e)
    }
    pub fn iter(&self) -> btree_map::Iter<'_, Entity, C> {
        self.map.iter()
    }
    pub fn iter_mut(&mut self) -> btree_map::IterMut<'_, Entity, C> {
        self.map.iter_mut()
    }
}

impl<C: Component> Default for Components<C> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
enum State {
    Write,
    Read(i64),
    Taken,
}

#[derive(Debug)]
pub struct ComponentsState<T: Component> {
    comps: Box<UnsafeCell<Components<T>>>,
    state: State,
}
#[derive(Debug)]
pub struct ComponentsNotAvailable;

#[derive(Debug)]
pub struct NotSuitableComponentsRecieve;
pub struct ComponentsRef<T: Component> {
    ptr: NonNull<Components<T>>,
}
unsafe impl<T: Component + Send> Send for ComponentsRef<T> {}
unsafe impl<T: Component + Send> Sync for ComponentsRef<T> {}

impl<T: Component> Deref for ComponentsRef<T> {
    type Target = Components<T>;

    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}

pub struct ComponentsMut<T: Component> {
    ptr: NonNull<Components<T>>,
}

unsafe impl<T: Component + Send> Send for ComponentsMut<T> {}
unsafe impl<T: Component + Send> Sync for ComponentsMut<T> {}

impl<T: Component> DerefMut for ComponentsMut<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.ptr.as_mut() }
    }
}

impl<T: Component> Deref for ComponentsMut<T> {
    type Target = Components<T>;

    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T: Component> ComponentsRef<T> {
    fn new(ptr: NonNull<Components<T>>) -> Self {
        Self { ptr }
    }
}
impl<T: Component> ComponentsMut<T> {
    fn new(ptr: NonNull<Components<T>>) -> Self {
        Self { ptr }
    }
}

impl<T: Component> ComponentsState<T> {
    pub fn new(comps: Box<UnsafeCell<Components<T>>>) -> Self {
        Self {
            comps,
            state: State::Write,
        }
    }

    fn try_read(&mut self) -> Result<ComponentsRef<T>, ComponentsNotAvailable> {
        match &mut self.state {
            State::Write => {
                self.state = State::Read(1);
                Ok(ComponentsRef::new(NonNull::new(self.comps.get()).unwrap()))
            }
            State::Read(counter) => {
                *counter += 1;
                Ok(ComponentsRef::new(NonNull::new(self.comps.get()).unwrap()))
            }
            State::Taken => Err(ComponentsNotAvailable),
        }
    }

    fn try_write(&mut self) -> Result<ComponentsMut<T>, ComponentsNotAvailable> {
        match &self.state {
            State::Write => {
                self.state = State::Taken;
                Ok(ComponentsMut::new(NonNull::new(self.comps.get()).unwrap()))
            }
            _ => Err(ComponentsNotAvailable),
        }
    }

    fn receive_ref(
        &mut self,
        comps: &ComponentsRef<T>,
    ) -> Result<(), NotSuitableComponentsRecieve> {
        match &mut self.state {
            State::Read(count) => match count {
                count if *count > 1 => {
                    *count -= 1;

                    Ok(())
                }
                count if *count == 1 => {
                    self.state = State::Write;
                    Ok(())
                }
                _ => Err(NotSuitableComponentsRecieve),
            },
            _ => Err(NotSuitableComponentsRecieve),
        }
    }
    fn recieve_mut(
        &mut self,
        comps: &ComponentsMut<T>,
    ) -> Result<(), NotSuitableComponentsRecieve> {
        match &mut self.state {
            State::Taken => {
                self.state = State::Write;
                Ok(())
            }
            _ => Err(NotSuitableComponentsRecieve),
        }
    }

    fn available_for_write(&self) -> bool {
        match self.state {
            State::Write => true,
            State::Read(count) => count == 0,
            State::Taken => false,
        }
    }
    fn available_for_read(&self) -> bool {
        match self.state {
            State::Taken => false,
            State::Read(_) | State::Write => true,
        }
    }
}

pub struct ComponentRegistry {
    map: BTreeMap<TypeId, Box<dyn Any>>,
}

impl ComponentRegistry {
    pub fn new() -> Self {
        Self {
            map: Default::default(),
        }
    }
    pub fn try_read<C: Component + 'static>(
        &mut self,
    ) -> Result<ComponentsRef<C>, ComponentsNotAvailable> {
        self.map
            .entry(TypeId::of::<C>())
            .or_insert_with(|| {
                Box::new(ComponentsState::new(Box::new(UnsafeCell::new(
                    Components::<C>::new(),
                ))))
            })
            .downcast_mut::<ComponentsState<C>>()
            .unwrap()
            .try_read()
    }

    pub fn try_write<C: Component + 'static>(
        &mut self,
    ) -> Result<ComponentsMut<C>, ComponentsNotAvailable> {
        self.map
            .entry(TypeId::of::<C>())
            .or_insert_with(|| {
                Box::new(ComponentsState::new(Box::new(UnsafeCell::new(
                    Components::<C>::new(),
                ))))
            })
            .downcast_mut::<ComponentsState<C>>()
            .unwrap()
            .try_write()
    }

    pub fn try_add_component<C: Component + 'static>(&mut self, e: Entity, c: C) -> Result<(), ()> {
        match self.try_write() {
            Ok(mut map) => {
                map.add(e, c);
                self.try_recieve_mut(&map).unwrap();
                Ok(())
            }
            Err(_) => Err(()),
        }
    }
    pub fn try_recieve_mut<C: Component + 'static>(
        &mut self,
        comps: &ComponentsMut<C>,
    ) -> Result<(), NotSuitableComponentsRecieve> {
        self.map
            .entry(TypeId::of::<C>())
            .or_insert_with(|| {
                Box::new(ComponentsState::new(Box::new(UnsafeCell::new(
                    Components::<C>::new(),
                ))))
            })
            .downcast_mut::<ComponentsState<C>>()
            .unwrap()
            .recieve_mut(comps)
    }
    pub fn try_recieve_ref<C: Component + 'static>(
        &mut self,
        comps: &ComponentsRef<C>,
    ) -> Result<(), NotSuitableComponentsRecieve> {
        self.map
            .entry(TypeId::of::<C>())
            .or_insert_with(|| {
                Box::new(ComponentsState::new(Box::new(UnsafeCell::new(
                    Components::<C>::new(),
                ))))
            })
            .downcast_mut::<ComponentsState<C>>()
            .unwrap()
            .receive_ref(&comps)
    }

    pub(crate) fn available_for_write<C: Component + 'static>(&self) -> bool {
        match self.map.get(&TypeId::of::<C>()) {
            Some(data) => data
                .downcast_ref::<ComponentsState<C>>()
                .unwrap()
                .available_for_write(),
            None => true,
        }
    }
    pub(crate) fn available_for_read<C: Component + 'static>(&self) -> bool {
        match self.map.get(&TypeId::of::<C>()) {
            Some(data) => data
                .downcast_ref::<ComponentsState<C>>()
                .unwrap()
                .available_for_read(),
            None => true,
        }
    }
}

impl Default for ComponentRegistry {
    fn default() -> Self {
        Self::new()
    }
}
