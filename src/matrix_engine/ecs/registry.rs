use core::fmt;
use std::{
    any::{Any, TypeId},
    collections::{
        hash_map::{Iter, IterMut},
        HashMap,
    },
    marker::PhantomData,
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use crate::unwrap_or_return;

use super::entity::Entity;

pub struct ComponentVecWriter<'a, T> {
    pd: PhantomData<T>,
    r: RwLockWriteGuard<'a, ComponentVec<T>>,
}
impl<'a, T> ComponentVecWriter<'a, T> {
    pub fn new(r: RwLockWriteGuard<'a, ComponentVec<T>>) -> Self {
        Self { pd: PhantomData, r }
    }
    pub fn push(&mut self, e: Entity, val: T) {
        self.r.insert(e, val);
    }
    pub fn len(&self) -> usize {
        self.r.data.len()
    }
    pub fn is_empty(&self) -> bool {
        self.r.data.is_empty()
    }
    pub fn iter_mut(&mut self) -> IterMut<Entity, T> {
        self.r.data.iter_mut()
    }
    pub fn iter(&self) -> Iter<Entity, T> {
        self.r.data.iter()
    }
}
pub struct ComponentVecReader<'a, T> {
    pd: PhantomData<T>,
    r: RwLockReadGuard<'a, ComponentVec<T>>,
}
impl<'a, T> ComponentVecReader<'a, T> {
    fn new(r: RwLockReadGuard<'a, ComponentVec<T>>) -> Self {
        Self { pd: PhantomData, r }
    }
    pub fn len(&self) -> usize {
        self.r.data.len()
    }
    pub fn is_empty(&self) -> bool {
        self.r.data.is_empty()
    }

    pub fn iter(&self) -> Iter<Entity, T> {
        self.r.data.iter()
    }
}

struct ComponentVecHolder {
    data: Arc<Box<dyn Any>>,
}
unsafe impl Send for ComponentVecHolder {}
unsafe impl Sync for ComponentVecHolder {}

impl Clone for ComponentVecHolder {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}

impl ComponentVecHolder {
    fn new<T: 'static>(data: ComponentVec<T>) -> Self {
        Self {
            data: Arc::new(Box::new(RwLock::new(data))),
        }
    }
    fn get_vec<T: 'static>(&self) -> Option<ComponentVecReader<T>> {
        let vec = self.data.as_ref();
        let vec = vec.downcast_ref::<RwLock<ComponentVec<T>>>();
        Some(ComponentVecReader::new(vec?.try_read().ok()?))
    }
    fn get_vec_mut<T: 'static>(&mut self) -> Option<ComponentVecWriter<T>> {
        let vec = self.data.as_ref();
        let vec = vec.downcast_ref::<RwLock<ComponentVec<T>>>();
        Some(ComponentVecWriter::new(vec?.try_write().ok()?))
    }
}

pub struct ComponentVec<T>
where
    T: Sized,
{
    data: HashMap<Entity, T>,
}
impl<T> ComponentVec<T> {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
    fn insert(&mut self, e: Entity, value: T) {
        self.data.insert(e, value);
    }
    fn borrow_component(&self, e: &Entity) -> Option<&T> {
        self.data.get(e)
    }
    fn borrow_component_mut(&mut self, e: &Entity) -> Option<&mut T> {
        self.data.get_mut(e)
    }
}
#[derive(Default)]
pub struct Registry {
    entity_counter: usize,
    data: HashMap<TypeId, ComponentVecHolder>,
}

impl Registry {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn read_vec<T: 'static>(&self) -> Option<ComponentVecReader<T>> {
        let map = self.data.get(&TypeId::of::<T>())?;
        map.get_vec()
    }
    pub fn write_vec<T: 'static>(&mut self) -> Option<ComponentVecWriter<T>> {
        let map = self
            .data
            .entry(TypeId::of::<T>())
            .or_insert_with(|| ComponentVecHolder::new(ComponentVec::<T>::new()));
        map.get_vec_mut()
    }
    pub fn create_entity(&mut self) -> Entity {
        let c = self.entity_counter;
        self.entity_counter += 1;
        Entity(c)
    }
    pub fn insert<T: 'static>(
        &mut self,
        e: Entity,
        value: T,
    ) -> Result<(), CantGetMultiThreadedValue> {
        let map = unwrap_or_return!(
            self.data.get_mut(&TypeId::of::<T>()),
            Err(CantGetMultiThreadedValue)
        );
        unwrap_or_return!(map.get_vec_mut(), Err(CantGetMultiThreadedValue)).push(e, value);

        Ok(())
    }
}

#[derive(Debug)]
pub struct CantGetMultiThreadedValue;

impl fmt::Display for CantGetMultiThreadedValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "cant get multi threaded value")
    }
}
