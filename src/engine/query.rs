use std::{
    any::Any,
    ops::{Deref, DerefMut},
};

use super::{
    components::{Component, Components},
    data_state::{DataStateAccessError, ReadDataState, WriteDataState},
    entity::EntitySystem,
    events::{EventWriter, Events, MatrixEventable},
    resources::{Resource, ResourceHolder},
    scene::SceneRegistryRefs,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum QueryError {
    NotAvailable,
}

pub trait Query<Queryable>: Any {
    fn check(queryable: &mut Queryable, id: &EntitySystem) -> bool
    where
        Self: Sized;

    fn query_unchecked(queryable: &mut Queryable, id: &EntitySystem) -> Self
    where
        Self: Sized;

    fn query(queryable: &mut Queryable, id: &EntitySystem) -> Result<Self, QueryError>
    where
        Self: Sized,
    {
        if Self::check(queryable, id) {
            Ok(Self::query_unchecked(queryable, id))
        } else {
            Err(QueryError::NotAvailable)
        }
    }

    fn consume(
        self,
        queryable: &mut Queryable,
        id: &EntitySystem,
    ) -> Result<(), DataStateAccessError>;
}

#[derive(Debug)]
pub struct ReadC<C: Component> {
    data: ReadDataState<Components<C>>,
}
unsafe impl<C: Component + Send> Send for ReadC<C> {}

impl<C: Component> Deref for ReadC<C> {
    type Target = Components<C>;

    fn deref(&self) -> &Self::Target {
        self.data.deref()
    }
}

impl<C: Component> ReadC<C> {
    pub fn new(data: ReadDataState<Components<C>>) -> Self {
        Self { data }
    }
}

impl<C: Component, CustomEvents: MatrixEventable> Query<SceneRegistryRefs<CustomEvents>>
    for ReadC<C>
{
    fn check(queryable: &mut SceneRegistryRefs<CustomEvents>, _id: &EntitySystem) -> bool {
        queryable.components.check_read::<C>()
    }

    fn query_unchecked(
        queryable: &mut SceneRegistryRefs<CustomEvents>,
        _id: &EntitySystem,
    ) -> Self {
        Self::new(queryable.components.read::<C>().unwrap())
    }

    fn consume(
        self,
        queryable: &mut SceneRegistryRefs<CustomEvents>,
        _id: &EntitySystem,
    ) -> Result<(), DataStateAccessError> {
        queryable.components.consume_read(self.data)
    }
}

#[derive(Debug)]
pub struct WriteC<C: Component> {
    data: WriteDataState<Components<C>>,
}
unsafe impl<C: Component + Send> Send for WriteC<C> {}

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
    pub fn new(data: WriteDataState<Components<C>>) -> Self {
        Self { data }
    }
}

impl<C: Component, CustomEvents: MatrixEventable> Query<SceneRegistryRefs<CustomEvents>>
    for WriteC<C>
{
    fn check(queryable: &mut SceneRegistryRefs<CustomEvents>, _id: &EntitySystem) -> bool {
        queryable.components.check_write::<C>()
    }

    fn query_unchecked(
        queryable: &mut SceneRegistryRefs<CustomEvents>,
        _id: &EntitySystem,
    ) -> Self {
        Self::new(queryable.components.write::<C>().unwrap())
    }

    fn consume(
        self,
        queryable: &mut SceneRegistryRefs<CustomEvents>,
        _id: &EntitySystem,
    ) -> Result<(), DataStateAccessError> {
        queryable.components.consume_write(self.data)
    }
}

pub struct ReadE {
    data: ReadDataState<Events>,
}

impl ReadE {
    fn new(data: ReadDataState<Events>) -> Self {
        Self { data }
    }
}

impl Deref for ReadE {
    type Target = Events;

    fn deref(&self) -> &Self::Target {
        self.data.deref()
    }
}

impl<CustomEvents: MatrixEventable> Query<SceneRegistryRefs<CustomEvents>> for ReadE {
    fn check(queryable: &mut SceneRegistryRefs<CustomEvents>, id: &EntitySystem) -> bool
    where
        Self: Sized,
    {
        queryable.events.check_reader(id)
    }

    fn query_unchecked(queryable: &mut SceneRegistryRefs<CustomEvents>, id: &EntitySystem) -> Self
    where
        Self: Sized,
    {
        Self::new(queryable.events.get_reader(*id).unwrap())
    }

    fn consume(
        self,
        queryable: &mut SceneRegistryRefs<CustomEvents>,
        id: &EntitySystem,
    ) -> Result<(), DataStateAccessError> {
        queryable.events.consume_reader(id, self.data)
    }
}

#[derive(Debug)]
pub struct WriteE<CustomEvents: MatrixEventable> {
    sender: ReadDataState<EventWriter<CustomEvents>>,
}

impl<CustomEvents: MatrixEventable> Deref for WriteE<CustomEvents> {
    type Target = EventWriter<CustomEvents>;

    fn deref(&self) -> &Self::Target {
        &self.sender
    }
}

impl<CustomEvents: MatrixEventable> Query<SceneRegistryRefs<CustomEvents>>
    for WriteE<CustomEvents>
{
    fn check(queryable: &mut SceneRegistryRefs<CustomEvents>, _id: &EntitySystem) -> bool
    where
        Self: Sized,
    {
        queryable.events.check_writer()
    }

    fn query_unchecked(queryable: &mut SceneRegistryRefs<CustomEvents>, _id: &EntitySystem) -> Self
    where
        Self: Sized,
    {
        Self::new(queryable.events.get_writer().unwrap().unwrap())
    }

    fn consume(
        self,
        queryable: &mut SceneRegistryRefs<CustomEvents>,
        _id: &EntitySystem,
    ) -> Result<(), DataStateAccessError> {
        queryable.events.consume_writer(self.sender)
    }
}

impl<CustomEvents: MatrixEventable> WriteE<CustomEvents> {
    pub fn new(sender: ReadDataState<EventWriter<CustomEvents>>) -> Self {
        Self { sender }
    }
}

pub struct ReadR<R: Resource> {
    data: ReadDataState<ResourceHolder<R>>,
}

impl<R: Resource> Deref for ReadR<R> {
    type Target = Option<R>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<R: Resource> ReadR<R> {
    pub fn new(data: ReadDataState<ResourceHolder<R>>) -> Self {
        Self { data }
    }
}

impl<R: Resource, CustomEvents: MatrixEventable> Query<SceneRegistryRefs<CustomEvents>>
    for ReadR<R>
{
    fn check(queryable: &mut SceneRegistryRefs<CustomEvents>, _id: &EntitySystem) -> bool
    where
        Self: Sized,
    {
        queryable.resources.check_read::<R>()
    }

    fn query_unchecked(queryable: &mut SceneRegistryRefs<CustomEvents>, _id: &EntitySystem) -> Self
    where
        Self: Sized,
    {
        Self::new(queryable.resources.read::<R>().unwrap())
    }

    fn consume(
        self,
        queryable: &mut SceneRegistryRefs<CustomEvents>,
        _id: &EntitySystem,
    ) -> Result<(), DataStateAccessError> {
        queryable.resources.consume_read(self.data)
    }
}

pub struct WriteR<R: Resource> {
    data: WriteDataState<ResourceHolder<R>>,
}

impl<R: Resource> DerefMut for WriteR<R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<R: Resource> Deref for WriteR<R> {
    type Target = Option<R>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<R: Resource> WriteR<R> {
    pub fn new(data: WriteDataState<ResourceHolder<R>>) -> Self {
        Self { data }
    }
}

impl<R: Resource, CustomEvents: MatrixEventable> Query<SceneRegistryRefs<CustomEvents>>
    for WriteR<R>
{
    fn check(queryable: &mut SceneRegistryRefs<CustomEvents>, _id: &EntitySystem) -> bool
    where
        Self: Sized,
    {
        queryable.resources.check_write::<R>()
    }

    fn query_unchecked(queryable: &mut SceneRegistryRefs<CustomEvents>, _id: &EntitySystem) -> Self
    where
        Self: Sized,
    {
        Self::new(queryable.resources.write::<R>().unwrap())
    }

    fn consume(
        self,
        queryable: &mut SceneRegistryRefs<CustomEvents>,
        _id: &EntitySystem,
    ) -> Result<(), DataStateAccessError> {
        queryable.resources.consume_write(self.data)
    }
}

pub struct ReadSystemID {
    id: EntitySystem,
}

impl Deref for ReadSystemID {
    type Target = EntitySystem;

    fn deref(&self) -> &Self::Target {
        &self.id
    }
}

impl ReadSystemID {
    fn new(id: EntitySystem) -> Self {
        Self { id }
    }
}

impl<Queryable> Query<Queryable> for ReadSystemID {
    fn check(_queryable: &mut Queryable, _id: &EntitySystem) -> bool
    where
        Self: Sized,
    {
        true
    }

    fn query_unchecked(_queryable: &mut Queryable, id: &EntitySystem) -> Self
    where
        Self: Sized,
    {
        Self::new(*id)
    }

    fn consume(
        self,
        _queryable: &mut Queryable,
        _id: &EntitySystem,
    ) -> Result<(), DataStateAccessError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use crate::engine::{
        components::ComponentRegistry,
        entity::{Entity, EntitySystem},
        scene::SceneRegistryRefs,
    };

    use super::{Query, ReadC, ReadE, WriteC};

    #[test]
    fn query() {
        let mut reg = ComponentRegistry::new();

        reg.try_insert(Entity::new(), 10).unwrap();
        let reg = <SceneRegistryRefs>::dummy();
        let mut reg = reg.registry;

        let q1 = ReadC::<i32>::query(&mut reg, &EntitySystem::new()).unwrap();
        let q2 = ReadC::<i32>::query(&mut reg, &EntitySystem::new()).unwrap();
        q1.consume(&mut reg, &EntitySystem::new()).unwrap();
        q2.consume(&mut reg, &EntitySystem::new()).unwrap();

        let q1 = WriteC::<i32>::query(&mut reg, &EntitySystem::new()).unwrap();
        WriteC::<i32>::query(&mut reg, &EntitySystem::new()).unwrap_err();

        q1.consume(&mut reg, &EntitySystem::new()).unwrap();

        let id = EntitySystem::new();
        let events = ReadE::query(&mut reg, &id).unwrap();
        events.consume(&mut reg, &id).unwrap();
    }
}
