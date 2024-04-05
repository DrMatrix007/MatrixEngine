use std::{
    any::{Any, TypeId},
    collections::BTreeMap,
    sync::Arc,
};

use tokio::sync::RwLock;

use super::entity::Entity;

pub trait Component: Send + Sync + 'static {}

impl<T: Send + Sync + 'static> Component for T {}

pub struct ComponentMap<T: Component> {
    map: BTreeMap<Entity, T>,
}

impl<T: Component> ComponentMap<T> {
    pub fn new() -> Self {
        Self {
            map: Default::default(),
        }
    }
    pub fn push(&mut self, e: Entity, t: T) -> Option<T> {
        self.map.insert(e, t)
    }
    pub fn iter(&self) -> impl Iterator<Item = (&'_ Entity, &'_ T)> {
        self.map.iter()
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&'_ Entity, &'_ mut T)> {
        self.map.iter_mut()
    }
}

impl<T: Component> Default for ComponentMap<T> {
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
    pub fn get_or_insert<T: Component>(&mut self) -> &Arc<RwLock<ComponentMap<T>>> {
        unsafe {
            self.map
                .entry(TypeId::of::<T>())
                .or_insert_with(|| Box::new(Arc::new(RwLock::new(ComponentMap::<T>::new()))))
                .downcast_ref_unchecked::<Arc<RwLock<ComponentMap<T>>>>()
        }
    }
    pub fn get<T: Component>(&self) -> Option<&Arc<RwLock<ComponentMap<T>>>> {
        self.map
            .get(&TypeId::of::<T>())
            .map(|x| unsafe { x.downcast_ref_unchecked() })
    }
}

impl Default for ComponentRegistry {
    fn default() -> Self {
        Self::new()
    }
}
