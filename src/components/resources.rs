use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::Arc,
};

use crate::{events::matrix_event::{MatrixEvent, MatrixEventSender}, dispatchers::systems::SystemContext};

use super::storage::{Storage, StorageReadGuard, StorageWriteGuard};

pub trait Resource {}

pub struct ResourceHolder<T: Resource> {
    data: Option<T>,
}

impl<T: Resource + 'static> ResourceHolder<T> {
    pub fn new_empty() -> Self {
        ResourceHolder { data: None }
    }
    pub fn new(data: T) -> Self {
        ResourceHolder { data: Some(data) }
    }

    pub fn get_or_default(&mut self,ctx:SystemContext) -> &mut T
    where
        T: Default,
    {
        // ctx..
        self.data.get_or_insert_with(Default::default)
    }
    pub fn get_mut(&mut self) -> Option<&mut T> {
        match &mut self.data {
            Some(data) => Some(data),
            None => None,
        }
    }
    pub fn get(&self) -> Option<&T> {
        match &self.data {
            Some(data) => Some(data),
            None => None,
        }
    }
    pub fn get_or_insert(&mut self, data: T) -> &mut T {
        self.data.get_or_insert(data)
    }
    pub fn get_or_insert_with(&mut self, data: impl FnOnce() -> T) -> &mut T {
        self.data.get_or_insert_with(data)
    }
    pub fn clear(&mut self) {
        self.data.take();
    }
}

pub struct ResourceRegistry {
    data: HashMap<TypeId, Box<dyn Any>>,
    event_handler: MatrixEventSender,
}

impl ResourceRegistry {
    pub fn empty(event_handler: MatrixEventSender) -> Self {
        Self {
            data: Default::default(),
            event_handler,
        }
    }

    pub fn get_mut<T: Resource + 'static>(
        &mut self,
    ) -> Option<StorageWriteGuard<ResourceHolder<T>>> {
        self.data
            .entry(TypeId::of::<T>())
            .or_insert(Box::new(Storage::new(ResourceHolder::<T>::new_empty())))
            .downcast_mut::<Storage<ResourceHolder<T>>>()
            .expect("this value should be of this type")
            .write()
    }

    pub fn get<T: Resource + 'static>(&mut self) -> Option<StorageReadGuard<ResourceHolder<T>>> {
        self.data
            .entry(TypeId::of::<T>())
            .or_insert(Box::new(Storage::new(ResourceHolder::<T>::new_empty())))
            .downcast_ref::<Storage<ResourceHolder<T>>>()
            .expect("this value should be of this type")
            .read()
    }
    pub fn insert<T: Resource + 'static>(&mut self, resource: T) {
        self.data
            .insert(TypeId::of::<T>(), Box::new(ResourceHolder::new(resource)));
    }
}

mod tests {

    #[test]
    fn test() {}
}
