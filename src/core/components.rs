use std::{
    any::{Any, TypeId},
    collections::BTreeMap,
};

use super::{entity::Entity, read_write_state::{RwState, RwStateAccessError}};

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
    pub fn get<T: Component>(&mut self) -> &mut RwState<ComponentMap<T>> {
        unsafe {
            self.map
                .entry(TypeId::of::<T>())
                .or_insert_with(|| Box::new(RwState::new(ComponentMap::<T>::new())))
                .downcast_mut_unchecked::<RwState<ComponentMap<T>>>()
        }
    }

    pub fn try_set<C:Component>(&mut self, e: Entity, comp: C) -> Result<(),RwStateAccessError> {
        let mut a = self.get::<C>().write()?;
        a.push(e,comp);
        self.get::<C>().consume_write(a).unwrap();
        Ok(())
    }
}

impl Default for ComponentRegistry {
    fn default() -> Self {
        Self::new()
    }
}
