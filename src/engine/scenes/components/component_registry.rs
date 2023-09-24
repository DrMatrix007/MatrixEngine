use std::{
    any::{Any, TypeId},
    collections::BTreeMap,
    sync::Arc,
};

use tokio::sync::RwLock;

use crate::engine::scenes::entities::Entity;

use super::Component;

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
    ) -> Result<tokio::sync::OwnedRwLockReadGuard<Components<C>>, tokio::sync::TryLockError> {
        self.map
            .entry(TypeId::of::<C>())
            .or_insert_with(|| Box::new(Arc::new(RwLock::new(Components::<C>::new()))))
            .downcast_mut::<Arc<RwLock<Components<C>>>>()
            .unwrap()
            .clone()
            .try_read_owned()
    }

    pub fn try_write<C: Component + 'static>(
        &mut self,
    ) -> Result<tokio::sync::OwnedRwLockWriteGuard<Components<C>>, tokio::sync::TryLockError> {
        self.map
            .entry(TypeId::of::<C>())
            .or_insert_with(|| Box::new(Arc::new(RwLock::new(Components::<C>::new()))))
            .downcast_mut::<Arc<RwLock<Components<C>>>>()
            .unwrap()
            .clone()
            .try_write_owned()
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
