use std::{
    any::Any,
    ops::{Deref, DerefMut},
};

use super::{
    components::{
        Component, ComponentAccessError, Components, ReadComponentState, WriteComponentState,
    },
    scene::SceneRegistry,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum QueryError {
    NotAvailable,
}

pub trait Query<Queryable>: Any {
    fn check(queryable: &mut Queryable) -> bool
    where
        Self: Sized;

    fn query_unchecked(queryable: &mut Queryable) -> Self
    where
        Self: Sized;

    fn query(queryable: &mut Queryable) -> Result<Self, QueryError>
    where
        Self: Sized,
    {
        if Self::check(queryable) {
            Ok(Self::query_unchecked(queryable))
        } else {
            Err(QueryError::NotAvailable)
        }
    }

    fn consume(self, queryable: &mut Queryable) -> Result<(), ComponentAccessError>;
}

#[derive(Debug)]
pub struct ReadC<C: Component> {
    data: ReadComponentState<C>,
}
unsafe impl<C:Component+Send> Send for ReadC<C>{}

impl<C: Component> Deref for ReadC<C> {
    type Target = Components<C>;

    fn deref(&self) -> &Self::Target {
        self.data.deref()
    }
}

impl<C: Component> ReadC<C> {
    pub fn new(data: ReadComponentState<C>) -> Self {
        Self { data }
    }
}

impl<C: Component> Query<SceneRegistry> for ReadC<C> {
    fn check(queryable: &mut SceneRegistry) -> bool {
        queryable.components.check_read::<C>()
    }

    fn query_unchecked(queryable: &mut SceneRegistry) -> Self {
        Self::new(queryable.components.read::<C>().unwrap())
    }

    fn consume(self, queryable: &mut SceneRegistry) -> Result<(), ComponentAccessError> {
        queryable.components.consume_read(self.data)
    }
}

#[derive(Debug)]
pub struct WriteC<C: Component> {
    data: WriteComponentState<C>,
}
unsafe impl<C:Component+Send> Send for WriteC<C>{}

impl<C: Component> Deref for WriteC<C> {
    type Target = Components<C>;

    fn deref(&self) -> &Self::Target {
        self.data.deref()
    }
}
impl<C: Component> DerefMut for WriteC<C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data.deref_mut()
    }
}

impl<C: Component> WriteC<C> {
    pub fn new(data: WriteComponentState<C>) -> Self {
        Self { data }
    }
}

impl<C: Component> Query<SceneRegistry> for WriteC<C> {
    fn check(queryable: &mut SceneRegistry) -> bool {
        queryable.components.check_write::<C>()
    }

    fn query_unchecked(queryable: &mut SceneRegistry) -> Self {
        Self::new(queryable.components.write::<C>().unwrap())
    }

    fn consume(self, queryable: &mut SceneRegistry) -> Result<(), ComponentAccessError> {
        queryable.components.consume_write(self.data)
    }
}

#[cfg(test)]
mod tests {

    use crate::engine::{components::ComponentRegistry, entity::Entity, scene::SceneRegistry};

    use super::{Query, ReadC, WriteC};

    #[test]
    fn query() {
        let mut reg = ComponentRegistry::new();

        reg.try_insert(Entity::new(), 10).unwrap();
        let mut reg = SceneRegistry { components: reg };
        let q1 = ReadC::<i32>::query(&mut reg).unwrap();
        let q2 = ReadC::<i32>::query(&mut reg).unwrap();
        q1.consume(&mut reg).unwrap();
        q2.consume(&mut reg).unwrap();

        let q1 = WriteC::<i32>::query(&mut reg).unwrap();
        WriteC::<i32>::query(&mut reg).unwrap_err();

        q1.consume(&mut reg).unwrap();
    }
}
