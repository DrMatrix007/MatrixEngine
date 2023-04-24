use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use super::storage::{Storage, StorageReadGuard, StorageWriteGuard};

pub trait Resource: Send {}

pub struct ResourceHolder<T> {
    data: Option<T>,
}

impl<T> ResourceHolder<T> {
    pub fn get_or_default(&mut self) -> &mut T
    where
        T: Default,
    {
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
}

impl<T> From<T> for ResourceHolder<T> {
    fn from(value: T) -> Self {
        ResourceHolder { data: Some(value) }
    }
}

impl<T> Default for ResourceHolder<T> {
    fn default() -> Self {
        Self { data: None }
    }
}

#[derive(Default)]
pub struct ResourceRegistry {
    data: HashMap<TypeId, Box<dyn Any>>,
}

impl ResourceRegistry {
    pub fn get_mut<T: Resource + 'static>(
        &mut self,
    ) -> Option<StorageWriteGuard<ResourceHolder<T>>> {
        self.data
            .entry(TypeId::of::<T>())
            .or_insert(Box::new(Storage::new(ResourceHolder::<T>::default())))
            .downcast_mut::<Storage<ResourceHolder<T>>>()
            .expect("this value should be of this type")
            .write()
    }

    pub fn get<T: Resource + 'static>(
        &mut self,
    ) -> Option<StorageReadGuard<ResourceHolder<T>>> {
        self.data
            .entry(TypeId::of::<T>())
            .or_insert(Box::new(Storage::new(ResourceHolder::<T>::default())))
            .downcast_ref::<Storage<ResourceHolder<T>>>()
            .expect("this value should be of this type")
            .read()
    }
    pub fn insert<T: Resource + 'static>(&mut self, resource: T) {
        self.data
            .insert(TypeId::of::<T>(), Box::new(ResourceHolder::from(resource)));
    }
}

mod tests {

    #[test]
    fn test() {}
}
