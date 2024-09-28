use std::{
    any::{Any, TypeId},
    collections::{btree_map, BTreeMap, HashMap},
    ops::{Deref, DerefMut},
};

use super::entity::Entity;

pub trait Component: Send + Sync + Any {}
impl<T: Send + Sync + Any> Component for T {}

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

enum State {
    Ready,
    Reading { readers: i32 },
    Writing,
}

pub struct ComponentState<C: Component> {
    data: Box<Components<C>>,
    state: State,
}
impl<C: Component> ComponentState<C> {
    pub fn new() -> Self {
        Self {
            data: Box::new(Components::new()),
            state: State::Ready,
        }
    }

    pub fn read(&mut self) -> Result<ReadComponentState<C>, ComponentAccessError> {
        match &mut self.state {
            State::Ready => {
                self.state = State::Reading { readers: 1 };
                Ok(ReadComponentState::new(&*self.data))
            }
            State::Reading { readers } => {
                *readers += 1;
                Ok(ReadComponentState::new(&*self.data))
            }
            State::Writing => Err(ComponentAccessError::NotAvailableError),
        }
    }
    pub fn write(&mut self) -> Result<WriteComponentState<C>, ComponentAccessError> {
        match self.state {
            State::Reading { .. } | State::Writing => Err(ComponentAccessError::NotAvailableError),
            State::Ready => {
                self.state = State::Writing;
                Ok(WriteComponentState::new(&mut *self.data))
            }
        }
    }

    pub fn consume_write(
        &mut self,
        data: WriteComponentState<C>,
    ) -> Result<(), ComponentAccessError> {
        if &*self.data as *const _ != data.data {
            return Err(ComponentAccessError::WrongFuckingData);
        }
        match self.state {
            State::Writing => {
                self.state = State::Ready;
                Ok(())
            }
            State::Reading { .. } | State::Ready => Err(ComponentAccessError::NotAvailableError),
        }
    }
    pub fn consume_read(
        &mut self,
        data: ReadComponentState<C>,
    ) -> Result<(), ComponentAccessError> {
        if &*self.data as *const _ != data.data {
            return Err(ComponentAccessError::WrongFuckingData);
        }
        match &mut self.state {
            State::Reading { readers } => {
                *readers -= 1;
                if *readers <= 0 {
                    self.state = State::Ready;
                }
                Ok(())
            }
            State::Writing | State::Ready => Err(ComponentAccessError::NotAvailableError),
        }
    }
    pub fn can_read(&self) -> bool {
        match self.state {
            State::Ready | State::Reading { .. } => true,
            State::Writing => false,
        }
    }
    pub fn can_write(&self) -> bool {
        match self.state {
            State::Ready => true,
            State::Reading { .. } | State::Writing => false,
        }
    }
}

#[derive(Debug)]
#[must_use = "needs to be consumed"]
pub struct ReadComponentState<C: Component> {
    data: *const Components<C>,
}

impl<C: Component> ReadComponentState<C> {
    fn new(data: *const Components<C>) -> Self {
        Self { data }
    }
}

impl<C: Component> Deref for ReadComponentState<C> {
    fn deref(&self) -> &Components<C> {
        unsafe { &*self.data }
    }

    type Target = Components<C>;
}

#[must_use = "needs to be consumed"]
#[derive(Debug)]
pub struct WriteComponentState<C: Component> {
    data: *mut Components<C>,
}

impl<C: Component> WriteComponentState<C> {
    fn new(data: *mut Components<C>) -> Self {
        Self { data }
    }
}

impl<C: Component> Deref for WriteComponentState<C> {
    type Target = Components<C>;
    fn deref(&self) -> &Components<C> {
        unsafe { &*self.data }
    }
}
impl<C: Component> DerefMut for WriteComponentState<C> {
    fn deref_mut(&mut self) -> &mut Components<C> {
        unsafe { &mut *self.data }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum ComponentAccessError {
    NotAvailableError,
    WrongFuckingData,
}

impl<C: Component> Default for ComponentState<C> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ComponentRegistry {
    data: HashMap<TypeId, Box<dyn Any + Send>>,
}

impl ComponentRegistry {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn read<C: Component>(&mut self) -> Result<ReadComponentState<C>, ComponentAccessError> {
        self.data
            .entry(TypeId::of::<C>())
            .or_insert_with(|| Box::new(ComponentState::<C>::new()))
            .downcast_mut::<ComponentState<C>>()
            .unwrap()
            .read()
    }
    pub fn write<C: Component>(&mut self) -> Result<WriteComponentState<C>, ComponentAccessError> {
        self.data
            .entry(TypeId::of::<C>())
            .or_insert_with(|| Box::new(ComponentState::<C>::new()))
            .downcast_mut::<ComponentState<C>>()
            .unwrap()
            .write()
    }

    pub fn consume_read<C: Component>(
        &mut self,
        read: ReadComponentState<C>,
    ) -> Result<(), ComponentAccessError> {
        self.data
            .get_mut(&TypeId::of::<C>())
            .ok_or(ComponentAccessError::NotAvailableError)
            .map(|data| data.downcast_mut::<ComponentState<C>>().unwrap())
            .and_then(|data| data.consume_read(read))
    }

    pub fn consume_write<C: Component>(
        &mut self,
        write: WriteComponentState<C>,
    ) -> Result<(), ComponentAccessError> {
        self.data
            .get_mut(&TypeId::of::<C>())
            .ok_or(ComponentAccessError::NotAvailableError)
            .map(|data| data.downcast_mut::<ComponentState<C>>().unwrap())
            .and_then(|data| data.consume_write(write))
    }

    pub fn check_read<C: Component>(&self) -> bool {
        self.data
            .get(&TypeId::of::<C>())
            .map(|x| x.downcast_ref::<ComponentState<C>>().unwrap().can_read())
            .unwrap_or(true) // the unwrap_or(true) is if the comp list does not exist, and thus it will be default and can be read of
    }
    pub fn check_write<C: Component>(&self) -> bool {
        self.data
            .get(&TypeId::of::<C>())
            .map(|x| x.downcast_ref::<ComponentState<C>>().unwrap().can_write())
            .unwrap_or(true) // the unwrap_or(true) is if the comp list does not exist, and thus it will be default and can be read of
    }

    pub fn try_insert<C: Component>(
        &mut self,
        e: Entity,
        c: C,
    ) -> Result<(), ComponentAccessError> {
        let mut w = self.write()?;
        w.insert(e, c);
        self.consume_write(w)?;
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
