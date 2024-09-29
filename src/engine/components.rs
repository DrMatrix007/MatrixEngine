use std::{
    any::{Any, TypeId},
    collections::{btree_map, BTreeMap, HashMap},
};

use super::{
    data_state::{DataState, DataStateAccessError, ReadDataState, WriteDataState},
    entity::Entity,
};

pub trait Component: Send + Sync + Any {}
impl<T: Send + Sync + Any> Component for T {}

#[derive(Debug)]
pub struct Components<C: Component> {
    data: BTreeMap<Entity, C>,
}
pub struct Iter<'a, C: Component> {
    data: btree_map::Iter<'a, Entity, C>,
}

impl<'a, C: Component> Iterator for Iter<'a, C> {
    type Item = (&'a Entity, &'a C);

    fn next(&mut self) -> Option<Self::Item> {
        self.data.next()
    }
}
impl<'a, C: Component> Iter<'a, C> {
    fn new(data: btree_map::Iter<'a, Entity, C>) -> Self {
        Self { data }
    }
}

pub struct IterMut<'a, C: Component> {
    data: btree_map::IterMut<'a, Entity, C>,
}

impl<'a, C: Component> Iterator for IterMut<'a, C> {
    type Item = (&'a Entity, &'a mut C);

    fn next(&mut self) -> Option<Self::Item> {
        self.data.next()
    }
}

impl<'a, C: Component> IterMut<'a, C> {
    fn new(data: btree_map::IterMut<'a, Entity, C>) -> Self {
        Self { data }
    }
}

impl<C: Component> Components<C> {
    pub fn new() -> Self {
        Self {
            data: BTreeMap::new(),
        }
    }

    pub fn iter(&self) -> Iter<'_, C> {
        Iter::<C>::new(self.data.iter())
    }
    pub fn iter_mut(&mut self) -> IterMut<'_, C> {
        IterMut::<C>::new(self.data.iter_mut())
    }
    pub fn insert(&mut self, e: Entity, c: C) {
        self.data.insert(e, c);
    }
    pub fn get(&self, e: &Entity) -> Option<&C> {
        self.data.get(e)
    }
    pub fn get_mut(&mut self, e: &Entity) -> Option<&mut C> {
        self.data.get_mut(e)
    }
}

impl<C: Component> Default for Components<C> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct ComponentRegistry {
    data: HashMap<TypeId, Box<dyn Any + Send>>,
}

impl ComponentRegistry {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    fn get_state<C: Component>(&mut self) -> &mut DataState<Components<C>> {
        self.data
            .entry(TypeId::of::<C>())
            .or_insert_with(|| Box::new(DataState::<Components<C>>::default()))
            .downcast_mut::<DataState<Components<C>>>()
            .expect("Failed to downcast data.")
    }

    pub fn read<C: Component>(
        &mut self,
    ) -> Result<ReadDataState<Components<C>>, DataStateAccessError> {
        self.get_state::<C>().read()
    }

    pub fn write<C: Component>(
        &mut self,
    ) -> Result<WriteDataState<Components<C>>, DataStateAccessError> {
        self.get_state::<C>().write()
    }

    pub fn consume_read<C: Component>(
        &mut self,
        read: ReadDataState<Components<C>>,
    ) -> Result<(), DataStateAccessError> {
        self.get_state::<C>().consume_read(read)
    }

    pub fn consume_write<C: Component>(
        &mut self,
        write: WriteDataState<Components<C>>,
    ) -> Result<(), DataStateAccessError> {
        self.get_state::<C>().consume_write(write)
    }

    pub fn check_read<C: Component>(&mut self) -> bool {
        self.get_state::<C>().can_read()
    }

    pub fn check_write<C: Component>(&mut self) -> bool {
        self.get_state::<C>().can_write()
    }

    pub fn try_insert<C: Component>(
        &mut self,
        e: Entity,
        c: C,
    ) -> Result<(), DataStateAccessError> {
        let w = self.get_state::<C>().get_mut()?;
        w.insert(e, c);
        Ok(())
    }
}

impl Default for ComponentRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::engine::entity::Entity;

    use super::ComponentRegistry;

    #[test]
    pub fn simple_insert() {
        let mut reg = ComponentRegistry::new();
        let mut v = vec![];
        for i in 0..100 {
            let e = Entity::new();
            reg.try_insert::<i32>(e, i).unwrap();
            v.push((e, i));
        }

        let data = reg.read::<i32>().unwrap();

        for (e, i) in v {
            assert!(*data.get(&e).unwrap() == i);
        }
    }
    #[test]
    pub fn states() {
        let mut reg = ComponentRegistry::new();
        reg.try_insert(Entity::new(), ()).unwrap();

        let r = reg.read::<()>().unwrap();
        let b = reg.read::<()>().unwrap();

        let _ = reg.write::<()>().unwrap_err();

        reg.consume_read(r).unwrap();
        reg.consume_read(b).unwrap();

        let a = reg.write::<()>().unwrap();

        reg.read::<()>().unwrap_err();
        reg.write::<()>().unwrap_err();

        reg.consume_write(a).unwrap();
    }
}
