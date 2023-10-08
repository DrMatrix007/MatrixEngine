use std::{
    any::{Any, TypeId},
    cell::{Cell, OnceCell, RefCell, UnsafeCell},
    collections::BTreeMap,
    ops::{Deref, DerefMut},
    ptr::NonNull,
    rc::Rc,
    sync::Arc,
};

use tokio::sync::RwLock;

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
    pub fn iter(&self) -> impl Iterator<Item = (&'_ Entity, &'_ C)> {
        self.map.iter()
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&'_ Entity, &'_ mut C)> {
        self.map.iter_mut()
    }
}

impl<C: Component> Default for Components<C> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub enum ComponentsState<T: Component> {
    Ok(Box<UnsafeCell<Components<T>>>, Option<i64>),
    Taken,
}
#[derive(Debug)]
pub struct ComponentsNotAvailable;

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
    fn try_read(&mut self) -> Result<ComponentsRef<T>, ComponentsNotAvailable> {
        match self {
            ComponentsState::Ok(components, count) => {
                match count {
                    Some(count) => *count += 1,
                    count @ None => *count = Some(1),
                }
                return Ok(ComponentsRef::new(NonNull::new(components.get()).unwrap()));
            }
            ComponentsState::Taken => Err(ComponentsNotAvailable),
        }
    }

    fn try_write(&mut self) -> Result<ComponentsMut<T>, ComponentsNotAvailable> {
        match self {
            ComponentsState::Ok(components, count) => match count {
                Some(count) => Err(ComponentsNotAvailable),
                count @ None => {
                    let comps_mut = ComponentsMut::new(NonNull::new(components.get()).unwrap());
                    core::mem::replace(self, Self::Taken);
                    Ok(comps_mut)
                }
            },
            ComponentsState::Taken => Err(ComponentsNotAvailable),
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
                Box::new(ComponentsState::Ok(
                    Box::new(UnsafeCell::new(Components::<C>::new())),
                    Some(0),
                ))
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
                Box::new(ComponentsState::Ok(
                    Box::new(UnsafeCell::new(Components::<C>::new())),
                    None,
                ))
            })
            .downcast_mut::<ComponentsState<C>>()
            .unwrap()
            .try_write()
    }

    pub fn try_add_component<C: Component + 'static>(&mut self, e: Entity, c: C) -> Result<(), C> {
        match self.try_write() {
            Ok(mut map) => Ok(map.add(e, c)),
            Err(_) => Err(c),
        }
    }
}

impl Default for ComponentRegistry {
    fn default() -> Self {
        Self::new()
    }
}
