<<<<<<< HEAD
use std::collections::HashMap;

use super::entity::Entity;

pub trait Component: Send + Sync {}

#[derive(Debug)]
pub struct ComponentCollection<T: Component>(HashMap<Entity, T>);

=======
use std::{any::Any, collections::HashMap, fmt::Debug};

use super::entity::Entity;

pub trait Component: Send + Sync + Clone {}

pub trait IComponentCollection: Send + Sync {
    fn remove(&mut self, e: &Entity);
    fn clone_vec(&self) -> Box<dyn IComponentCollection>;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[derive(Debug, Clone)]
pub struct ComponentCollection<T: Component>(HashMap<Entity, T>);

impl<T: Component + Clone + 'static> IComponentCollection for ComponentCollection<T> {
    fn remove(&mut self, e: &Entity) {
        self.0.remove(e);
    }
    fn clone_vec(&self) -> Box<dyn IComponentCollection> {
        Box::new(self.clone())
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

>>>>>>> temp
impl<T: Component> Default for ComponentCollection<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T: Component> ComponentCollection<T> {
    pub fn insert(&mut self, e: Entity, t: T) -> Option<T> {
        self.0.insert(e, t)
    }
    pub fn remove(&mut self, e: Entity) -> Option<T> {
        self.0.remove(&e)
    }

    pub fn get(&self, e: &Entity) -> Option<&T> {
        self.0.get(e)
    }
    pub fn get_mut(&mut self, e: &Entity) -> Option<&mut T> {
        self.0.get_mut(e)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Entity, &T)> {
        self.0.iter()
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&Entity, &mut T)> {
        self.0.iter_mut()
    }
}
