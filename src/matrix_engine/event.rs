use core::slice::Iter;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

pub trait Event {}
pub trait ApplicationEvent: Event {}

#[allow(dead_code)]
pub struct Events {
    data: HashMap<TypeId, Box<dyn Any>>,
}
unsafe impl Send for Events {}

unsafe impl Sync for Events {}

#[allow(dead_code)]
impl Events {
    pub fn new() -> Self {
        Events {
            data: HashMap::new(),
        }
    }
    pub fn add_event<T: Event + 'static>(&mut self, e: T) {
        let ent = self.data.entry(TypeId::of::<T>());

        let b = ent.or_insert_with(|| Box::new(Vec::<T>::new()));
        if let Some(vec) = b.downcast_mut::<Vec<T>>() {
            vec.insert_event(e);
        }
    }
    pub fn read_events<T: Event + 'static>(&self) -> Iter<T> {
        self.data
            .get(&TypeId::of::<T>())
            .and_then(|x| x.downcast_ref::<Vec<T>>().map(|x| x.iter()))
            .unwrap_or_else(|| [].iter())
    }
}

impl Default for Events {
    fn default() -> Self {
        Self::new()
    }
}

trait EventVec<T> {
    fn insert_event(&mut self, i: T);
    fn clear(&mut self);
}

impl<T: Event> EventVec<T> for Vec<T> {
    fn insert_event(&mut self, i: T) {
        self.push(i);
    }

    fn clear(&mut self) {
        self.clear();
    }
}
