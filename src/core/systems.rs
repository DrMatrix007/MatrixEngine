use std::sync::Arc;

use tokio::sync::RwLock;

use super::component::{Component, ComponentMap, ComponentRegistry};

pub trait System {
    fn run<T>(&mut self, queryable: &impl Queryable);
}

pub trait Queryable {
    fn component<C: Component>(&self) -> Option<&Arc<RwLock<ComponentMap<C>>>>;
}

impl Queryable for ComponentRegistry {
    fn component<C: Component>(&self) -> Option<&Arc<RwLock<ComponentMap<C>>>> {
        self.get()
    }
}


pub trait QuerySystem {
    type Query: Query;
}

#[derive(Debug,Clone, Copy)]
pub enum QueryError{
    CurrentlyNotAvailable,
    DoesntExist
}

pub trait Query {
    type Data<'a>;
    fn ensure_installed(queryable:&mut impl Queryable);
    fn try_query<'a>(queryable:&impl Queryable) -> Result<Self::Data<'a>,QueryError>;
}
