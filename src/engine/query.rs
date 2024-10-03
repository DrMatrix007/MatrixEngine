use std::{
    any::Any,
    ops::{Deref, DerefMut},
};

use super::typeid::TypeIDable;
use super::{
    components::{Component, Components},
    data_state::{DataStateAccessError, ReadDataState, WriteDataState},
    entity::SystemEntity,
    events::{EventWriter, Events, MatrixEvent, MatrixEventable},
    resources::{Resource, ResourceHolder},
    scene::SceneRegistryRefs,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum QueryError {
    NotAvailable,
}

pub trait Query<Queryable>: Any {
    fn query(queryable: &mut Queryable, id: &SystemEntity) -> Result<Self, DataStateAccessError>
    where
        Self: Sized;

    fn consume(
        self,
        queryable: &mut Queryable,
        id: &SystemEntity,
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
    fn query(
        queryable: &mut SceneRegistryRefs<CustomEvents>,
        _id: &SystemEntity,
    ) -> Result<Self, DataStateAccessError> {
        Ok(Self::new(queryable.components.read::<C>()?))
    }

    fn consume(
        self,
        queryable: &mut SceneRegistryRefs<CustomEvents>,
        _id: &SystemEntity,
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
    fn query(
        queryable: &mut SceneRegistryRefs<CustomEvents>,
        _id: &SystemEntity,
    ) -> Result<Self, DataStateAccessError> {
        Ok(Self::new(queryable.components.write::<C>()?))
    }

    fn consume(
        self,
        queryable: &mut SceneRegistryRefs<CustomEvents>,
        _id: &SystemEntity,
    ) -> Result<(), DataStateAccessError> {
        queryable.components.consume_write(self.data)
    }
}

pub struct ReadE<CustomEvents: MatrixEventable> {
    data: ReadDataState<Events<CustomEvents>>,
}

impl<CustomEvents: MatrixEventable> ReadE<CustomEvents> {
    fn new(data: ReadDataState<Events<CustomEvents>>) -> Self {
        Self { data }
    }
}

impl<CustomEvents: MatrixEventable> Deref for ReadE<CustomEvents> {
    type Target = Events<CustomEvents>;

    fn deref(&self) -> &Self::Target {
        self.data.deref()
    }
}

impl<CustomEvents: MatrixEventable> Query<SceneRegistryRefs<CustomEvents>> for ReadE<CustomEvents> {
    fn query(
        queryable: &mut SceneRegistryRefs<CustomEvents>,
        _id: &SystemEntity,
    ) -> Result<Self, DataStateAccessError>
    where
        Self: Sized,
    {
        Ok(Self::new(queryable.events.get_reader()?))
    }

    fn consume(
        self,
        queryable: &mut SceneRegistryRefs<CustomEvents>,
        _id: &SystemEntity,
    ) -> Result<(), DataStateAccessError> {
        queryable.events.consume_reader(self.data)
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
    fn query(
        queryable: &mut SceneRegistryRefs<CustomEvents>,
        _id: &SystemEntity,
    ) -> Result<Self, DataStateAccessError>
    where
        Self: Sized,
    {
        Ok(Self::new(queryable.events.get_writer().unwrap()?))
    }

    fn consume(
        self,
        queryable: &mut SceneRegistryRefs<CustomEvents>,
        _id: &SystemEntity,
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

impl<R: Resource> ReadR<R> {
    pub fn new(data: ReadDataState<ResourceHolder<R>>) -> Self {
        Self { data }
    }
    pub fn get(&self) -> Option<&R> {
        self.data.as_ref()
    }
}

impl<R: Resource, CustomEvents: MatrixEventable> Query<SceneRegistryRefs<CustomEvents>>
    for ReadR<R>
{
    fn query(
        queryable: &mut SceneRegistryRefs<CustomEvents>,
        _id: &SystemEntity,
    ) -> Result<Self, DataStateAccessError>
    where
        Self: Sized,
    {
        Ok(Self::new(queryable.resources.read::<R>()?))
    }

    fn consume(
        self,
        queryable: &mut SceneRegistryRefs<CustomEvents>,
        _id: &SystemEntity,
    ) -> Result<(), DataStateAccessError> {
        queryable.resources.consume_read(self.data)
    }
}

pub struct WriteR<R: Resource, CustomEvents: MatrixEventable = ()> {
    data: WriteDataState<ResourceHolder<R>>,
    proxy: ReadDataState<EventWriter<CustomEvents>>,
}

impl<R: Resource, CustomEvents: MatrixEventable> WriteR<R, CustomEvents> {
    pub fn new(
        data: WriteDataState<ResourceHolder<R>>,
        proxy: ReadDataState<EventWriter<CustomEvents>>,
    ) -> Self {
        Self { data, proxy }
    }

    pub fn get(&self) -> Option<&R> {
        self.data.as_ref()
    }
    pub fn get_mut(&mut self) -> Option<&mut R> {
        self.data.as_mut()
    }
    pub fn insert_and_notify(&mut self, data: R) {
        **self.data = Some(data);
        self.proxy
            .send(MatrixEvent::ChangedResource(R::get_type_id()))
            .unwrap();
    }
    pub fn unwrap_or_insert_with_and_notify(&mut self, data: impl FnOnce() -> R) -> &mut R {
        self.proxy
            .send(MatrixEvent::ChangedResource(R::get_type_id()))
            .unwrap();
        self.data.get_or_insert_with(data)
    }
    pub fn unwrap_or_insert_and_notify(&mut self, data: R) -> &mut R {
        self.proxy
            .send(MatrixEvent::ChangedResource(R::get_type_id()))
            .unwrap();
        self.data.get_or_insert(data)
    }
}

impl<R: Resource, CustomEvents: MatrixEventable> Query<SceneRegistryRefs<CustomEvents>>
    for WriteR<R, CustomEvents>
{
    fn query(
        queryable: &mut SceneRegistryRefs<CustomEvents>,
        _id: &SystemEntity,
    ) -> Result<Self, DataStateAccessError>
    where
        Self: Sized,
    {
        match (
            queryable.resources.write::<R>(),
            queryable.events.get_writer().unwrap(),
        ) {
            (Ok(data1), Ok(data2)) => Ok(Self::new(data1, data2)),
            (a, b) => {
                if let Ok(a) = a {
                    queryable.resources.consume_write(a)?;
                }
                if let Ok(b) = b {
                    queryable.events.consume_writer(b)?;
                }
                Err(DataStateAccessError::NotAvailableError)
            }
        }
    }

    fn consume(
        self,
        queryable: &mut SceneRegistryRefs<CustomEvents>,
        _id: &SystemEntity,
    ) -> Result<(), DataStateAccessError> {
        queryable.resources.consume_write(self.data)?;
        queryable.events.consume_writer(self.proxy)?;
        Ok(())
    }
}

pub struct ReadSystemID {
    id: SystemEntity,
}

impl Deref for ReadSystemID {
    type Target = SystemEntity;

    fn deref(&self) -> &Self::Target {
        &self.id
    }
}

impl ReadSystemID {
    fn new(id: SystemEntity) -> Self {
        Self { id }
    }
}

impl<Queryable> Query<Queryable> for ReadSystemID {
    fn query(_queryable: &mut Queryable, id: &SystemEntity) -> Result<Self, DataStateAccessError>
    where
        Self: Sized,
    {
        Ok(Self::new(*id))
    }

    fn consume(
        self,
        _queryable: &mut Queryable,
        _id: &SystemEntity,
    ) -> Result<(), DataStateAccessError> {
        Ok(())
    }
}

impl<T> Query<T> for () {
    fn query(_queryable: &mut T, _id: &SystemEntity) -> Result<Self, DataStateAccessError>
    where
        Self: Sized,
    {
        Ok(())
    }

    fn consume(self, _queryable: &mut T, _id: &SystemEntity) -> Result<(), DataStateAccessError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use crate::engine::{
        components::ComponentRegistry,
        entity::{Entity, SystemEntity},
        scene::SceneRegistryRefs,
    };

    use super::{Query, ReadC, ReadE, WriteC};

    #[test]
    fn query() {
        let mut reg = ComponentRegistry::new();

        reg.try_insert(Entity::new(), 10).unwrap();
        let reg = <SceneRegistryRefs>::dummy();
        let mut reg = reg.registry;

        let q1 = ReadC::<i32>::query(&mut reg, &SystemEntity::new()).unwrap();
        let q2 = ReadC::<i32>::query(&mut reg, &SystemEntity::new()).unwrap();
        q1.consume(&mut reg, &SystemEntity::new()).unwrap();
        q2.consume(&mut reg, &SystemEntity::new()).unwrap();

        let q1 = WriteC::<i32>::query(&mut reg, &SystemEntity::new()).unwrap();
        WriteC::<i32>::query(&mut reg, &SystemEntity::new()).unwrap_err();

        q1.consume(&mut reg, &SystemEntity::new()).unwrap();

        let id = SystemEntity::new();
        let events = ReadE::query(&mut reg, &id).unwrap();
        events.consume(&mut reg, &id).unwrap();
    }
}
