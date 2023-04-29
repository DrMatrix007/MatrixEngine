use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use crate::entity::Entity;

use super::storage::{Storage, StorageReadGuard, StorageWriteGuard};

pub trait Component: Send {}

pub struct ComponentCollection<T: Component> {
    data: HashMap<Entity, T>,
}

impl<T: Component> Default for ComponentCollection<T> {
    fn default() -> Self {
        Self {
            data: Default::default(),
        }
    }
}

impl<T: Component> ComponentCollection<T> {
    pub fn iter(&self) -> std::collections::hash_map::Iter<Entity, T> {
        self.data.iter()
    }
    pub fn iter_mut(&mut self) -> std::collections::hash_map::IterMut<Entity, T> {
        self.data.iter_mut()
    }
    pub fn insert(&mut self, e: Entity, comp: T) {
        self.data.insert(e, comp);
    }
    pub fn get(&self, e: &Entity) -> Option<&T> {
        self.data.get(e)
    }
    pub fn get_mut(&mut self, e: &Entity) -> Option<&mut T> {
        self.data.get_mut(e)
    }
    pub fn get_all(&self) -> ComponentCollectionRef<'_, T> {
        self.iter().into()
    }
    pub fn get_all_mut(&mut self) -> ComponentCollectionRefMut<'_, T> {
        self.iter_mut().into()
    }
}
pub struct ComponentCollectionRef<'a, T> {
    data: HashMap<&'a Entity, &'a T>,
}

impl<'a, T, A: Iterator<Item = (&'a Entity, &'a T)>> From<A> for ComponentCollectionRef<'a, T> {
    fn from(value: A) -> Self {
        ComponentCollectionRef {
            data: value.collect(),
        }
    }
}

impl<'a, T> ComponentCollectionRef<'a, T> {
    pub fn iter(&self) -> std::collections::hash_map::Iter<&'a Entity, &'a T> {
        self.data.iter()
    }
    pub fn iter_mut(&mut self) -> std::collections::hash_map::IterMut<&'a Entity, &'a T> {
        self.data.iter_mut()
    }
    pub fn get(&self, e: &Entity) -> Option<&&'a T> {
        self.data.get(e)
    }
    pub fn get_mut(&mut self, e: &Entity) -> Option<&mut &'a T> {
        self.data.get_mut(e)
    }
}

pub struct ComponentCollectionRefMut<'a, T> {
    data: HashMap<&'a Entity, &'a mut T>,
}

impl<'a, T> ComponentCollectionRefMut<'a, T> {
    pub fn iter(&self) -> std::collections::hash_map::Iter<&'a Entity, &'a mut T> {
        self.data.iter()
    }
    pub fn iter_mut(&mut self) -> std::collections::hash_map::IterMut<&'a Entity, &'a mut T> {
        self.data.iter_mut()
    }
    pub fn get(&self, e: &Entity) -> Option<&&'a mut T> {
        self.data.get(e)
    }
    pub fn get_mut(&mut self, e: &Entity) -> Option<&mut &'a mut T> {
        self.data.get_mut(e)
    }
}

impl<'a, T, A: Iterator<Item = (&'a Entity, &'a mut T)>> From<A>
    for ComponentCollectionRefMut<'a, T>
{
    fn from(value: A) -> Self {
        ComponentCollectionRefMut {
            data: value.collect(),
        }
    }
}

#[derive(Default)]
pub struct ComponentRegistry {
    data: HashMap<TypeId, BoxedCollection>,
}

struct BoxedCollection(Box<dyn Any>);

impl BoxedCollection {
    pub fn new<T: Component + 'static>() -> Self {
        Self(Box::new(Storage::new(ComponentCollection::<T>::default())))
    }
    pub fn downcast_ref<T: Component + 'static>(&self) -> Option<&Storage<ComponentCollection<T>>> {
        self.0.downcast_ref::<Storage<ComponentCollection<T>>>()
    }
    pub fn downcast_mut<T: Component + 'static>(
        &mut self,
    ) -> Option<&mut Storage<ComponentCollection<T>>> {
        self.0.downcast_mut::<Storage<ComponentCollection<T>>>()
    }
}

impl ComponentRegistry {
    pub fn get<T: Component + 'static>(
        &mut self,
    ) -> Option<StorageReadGuard<ComponentCollection<T>>> {
        self.data
            .entry(TypeId::of::<T>())
            .or_insert(BoxedCollection::new::<T>())
            .downcast_ref::<T>()
            .expect("this value should be of this type")
            .read()
    }

    pub fn get_mut<T: Component + 'static>(
        &mut self,
    ) -> Option<StorageWriteGuard<ComponentCollection<T>>> {
        self.data
            .entry(TypeId::of::<T>())
            .or_insert(BoxedCollection::new::<T>())
            .downcast_ref::<T>()
            .expect("this value should be of this type")
            .write()
    }

    pub fn insert<T: Component + 'static>(&mut self, e: Entity, c: T) -> Result<(), T> {
        let Some(mut data) = self.get_mut() else {
            return Err(c);
        };

        data.get_mut().insert(e, c);

        Ok(())
    }
}
