use std::{any::Any, collections::HashMap, sync::atomic::AtomicUsize};

static ENTITY_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Entity {
    id: usize,
}
impl Entity {
    pub fn new() -> Self {
        Entity {
            id: ENTITY_COUNTER.fetch_add(1, std::sync::atomic::Ordering::AcqRel),
        }
    }
}

impl Default for Entity {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Component {}

pub trait IComponentVec {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct ComponentVec<T: Component> {
    pub data: HashMap<Entity, T>,
}

impl<T: Component> Default for ComponentVec<T> {
    fn default() -> Self {
        Self {
            data: Default::default(),
        }
    }
}
impl<T: Component> ComponentVec<T> {
    pub(super) fn new() -> Self {
        Default::default()
    }
    pub fn insert(&mut self, e:Entity, t: T) {
        self.data.insert(e, t);
    }

    pub fn remove(&mut self, e: &Entity) -> Option<T> {
        self.data.remove(e)
    }
    pub fn iter(&self) -> std::collections::hash_map::Iter<Entity, T> {
        self.data.iter()
    }
    pub fn iter_mut(&mut self) -> std::collections::hash_map::IterMut<Entity, T> {
        self.data.iter_mut()
    }
}

impl<T: Component + 'static> IComponentVec for ComponentVec<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
