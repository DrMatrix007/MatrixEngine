use std::{any::Any, marker::PhantomData};

use crate::{
    components::component::{Component, ComponentMap},
    par::storage::{ReadStorageGuard, WriteStorageGuard},
    scenes::scene::Scene,
};

pub enum QueryError {
    NotAvailable,
}

pub struct QueryArgs<'a> {
    scene: &'a mut Scene,
}

impl<'a> QueryArgs<'a> {
    pub fn new(scene: &'a mut Scene) -> Self {
        Self { scene }
    }

    pub fn scene_mut(&mut self) -> &mut Scene {
        self.scene
    }
    pub fn scene(&self) -> &Scene {
        &self.scene
    }
}

pub trait Query {
    type Target: Any;
    fn query(args: &mut QueryArgs<'_>) -> Result<Self::Target, QueryError>;

    fn query_boxed(args: &mut QueryArgs<'_>) -> Result<Box<dyn Any>, QueryError> {
        Self::query(args).map(|x| Box::<dyn Any>::from(Box::new(x)))
    }
}

#[derive(Default)]
struct ReadComponents<C: Component>(PhantomData<C>);

#[derive(Default)]
struct WriteComponents<C: Component>(PhantomData<C>);

impl<C: Component> Query for ReadComponents<C> {
    type Target = ReadStorageGuard<ComponentMap<C>>;
    fn query(args: &mut QueryArgs<'_>) -> Result<Self::Target, QueryError> {
        match args.scene_mut().components_mut().try_get_map() {
            Some(data) => Ok(data),
            None => Err(QueryError::NotAvailable),
        }
    }
}

impl<C: Component> Query for WriteComponents<C> {
    type Target = WriteStorageGuard<ComponentMap<C>>;
    fn query(args: &mut QueryArgs<'_>) -> Result<Self::Target, QueryError> {
        match args.scene_mut().components_mut().try_get_map_mut() {
            Some(data) => Ok(data),
            None => Err(QueryError::NotAvailable),
        }
    }
}

pub trait QuerySend: Query 
where
    <Self as Query>::Target: Send,
{
    fn query_boxed_send(args: &mut QueryArgs<'_>) -> Result<Box<dyn Any + Send>, QueryError> {
        Self::query(args).map(|x| Box::<dyn Any + Send>::from(Box::new(x)))
    }
}
impl<Q: Query> QuerySend for Q where Q::Target: Send {}
