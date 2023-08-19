use std::{any::Any, marker::PhantomData};

use crate::{
    components::component::{Component, ComponentMap},
    par::storage::{ReadStorageGuard, StorageError, WriteStorageGuard},
    scenes::scene::Scene,
};

pub enum QueryError {
    StorageError(StorageError),
}

pub struct QueryArgs {
    scene: WriteStorageGuard<Scene>,
}

impl QueryArgs {
    pub fn new(scene: WriteStorageGuard<Scene>) -> Self {
        Self { scene }
    }

    pub fn scene_mut(&mut self) -> &mut Scene {
        self.scene.as_mut()
    }
    pub fn scene(&self) -> &Scene {
        &self.scene
    }
}

pub trait Query {
    type Target: Any;
    fn query(args: &mut QueryArgs) -> Result<Self::Target, QueryError>;

    fn query_boxed(args: &mut QueryArgs) -> Result<Box<dyn Any>, QueryError> {
        Self::query(args).map(|x| Box::<dyn Any>::from(Box::new(x)))
    }
}

#[derive(Default)]
struct ReadComponents<C: Component>(PhantomData<C>);

#[derive(Default)]
struct WriteComponents<C: Component>(PhantomData<C>);

impl<C: Component> Query for ReadComponents<C> {
    type Target = ReadStorageGuard<ComponentMap<C>>;
    fn query(args: &mut QueryArgs) -> Result<Self::Target, QueryError> {
        args.scene_mut()
            .components_mut()
            .try_get_map()
            .map_err(|x| QueryError::StorageError(x))
    }
}

impl<C: Component> Query for WriteComponents<C> {
    type Target = WriteStorageGuard<ComponentMap<C>>;
    fn query(args: &mut QueryArgs) -> Result<Self::Target, QueryError> {
        args.scene_mut()
            .components_mut()
            .try_get_map_mut()
            .map_err(|x| QueryError::StorageError(x))
    }
}

pub trait QuerySend: Query
where
    <Self as Query>::Target: Send,
{
    fn query_boxed_send(args: &mut QueryArgs) -> Result<Box<dyn Any + Send>, QueryError> {
        Self::query(args).map(|x| Box::<dyn Any + Send>::from(Box::new(x)))
    }
}
impl<Q: Query> QuerySend for Q where Q::Target: Send {}
