use std::{
    any::{Any, TypeId},
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

use super::entity::Entity;

pub trait Component: 'static {}

impl<T: 'static> Component for T {}

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

pub struct ComponentRegistry {
    map: BTreeMap<TypeId, Box<dyn Any>>,
}

impl ComponentRegistry {
    pub fn new() -> Self {
        Self {
            map: Default::default(),
        }
    }
    pub fn get<T: Component>(&mut self) -> &Arc<Mutex<ComponentMap<T>>> {
        self.map
            .entry(TypeId::of::<T>())
            .or_insert_with(|| Box::new(Arc::new(Mutex::new(ComponentMap::<T>::new()))))
            .downcast_ref::<Arc<Mutex<ComponentMap<T>>>>()
            .clone()
            .unwrap()
    }
}
