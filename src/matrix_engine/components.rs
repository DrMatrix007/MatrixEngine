use std::{collections::{HashMap, btree_map::Entry}};

use super::entity::Entity;

pub trait Component:Send+Sync {}


#[derive(Debug)]
pub struct ComponentCollection<T: Component>(HashMap<Entity, T>);

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

    pub fn get(&self,e:&Entity) -> Option<&T> {
        self.0.get(e)
    }
    pub fn get_mut(&mut self,e:&Entity) -> Option<&mut T> {
        self.0.get_mut(e)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Entity, &T)> {
        self.0.iter()
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&Entity, &mut T)> {
        self.0.iter_mut()
    }
}
