use std::{marker::PhantomData, sync::Arc};

use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::impl_all;

use super::{
    component::{Component, ComponentMap, ComponentRegistry},
    scene::SceneRegistry,
};

#[derive(Debug, Clone, Copy)]
pub enum QueryError {
    CurrentlyNotAvailable,
    DoesntExist,
}

pub trait System<Q: Queryable>: Send + Sync + 'static {
    fn ensure_installed(&self, queryable: &mut Q);
    fn run(&mut self, queryable: &Q) -> Result<(), QueryError>;
    fn is_send(&self) -> bool;
}

pub trait Queryable {
    fn components<C: Component>(&self) -> Option<&Arc<RwLock<ComponentMap<C>>>>;
    fn ensure_isntalled_components<C: Component>(&mut self);
}

impl Queryable for ComponentRegistry {
    fn components<C: Component>(&self) -> Option<&Arc<RwLock<ComponentMap<C>>>> {
        self.get()
    }

    fn ensure_isntalled_components<C: Component>(&mut self) {
        self.get_or_insert::<C>();
    }
}

pub trait QuerySystem: Send + Sync + 'static {
    type Query: Query;

    fn run(&mut self, args: <Self::Query as Query>::Data<'_>);
}

pub trait Query {
    type Data<'a>;
    fn ensure_installed(queryable: &mut impl Queryable);
    fn try_query(queryable: &impl Queryable) -> Result<Self::Data<'_>, QueryError>;
    fn is_send() -> bool;
}
pub trait QuerySend: Query
where
    Self: Send,
{
}

impl<S: QuerySystem, Q: Queryable> System<Q> for S {
    fn run(&mut self, queryable: &Q) -> Result<(), QueryError> {
        let res = S::Query::try_query(queryable);
        match res {
            Ok(data) => {
                self.run(data);
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    fn ensure_installed(&self, queryable: &mut Q) {
        S::Query::ensure_installed(queryable);
    }
    fn is_send(&self) -> bool {
        S::Query::is_send()
    }
}

pub struct ReadC<T>(PhantomData<T>);
pub struct WriteC<T>(PhantomData<T>);

impl<C: Component> Query for ReadC<C> {
    type Data<'a> = RwLockReadGuard<'a, ComponentMap<C>>;

    fn ensure_installed(queryable: &mut impl Queryable) {
        queryable.ensure_isntalled_components::<C>()
    }

    fn try_query(queryable: &impl Queryable) -> Result<Self::Data<'_>, QueryError> {
        match queryable.components() {
            Some(data) => match data.try_read() {
                Ok(data) => Ok(data),
                Err(_) => Err(QueryError::CurrentlyNotAvailable),
            },
            None => Err(QueryError::DoesntExist),
        }
    }
    fn is_send() -> bool {
        true
    }
}

impl<C: Component> Query for WriteC<C> {
    type Data<'a> = RwLockWriteGuard<'a, ComponentMap<C>>;

    fn ensure_installed(queryable: &mut impl Queryable) {
        queryable.ensure_isntalled_components::<C>()
    }

    fn try_query(queryable: &impl Queryable) -> Result<Self::Data<'_>, QueryError> {
        match queryable.components() {
            Some(data) => match data.try_write() {
                Ok(data) => Ok(data),
                Err(_) => Err(QueryError::CurrentlyNotAvailable),
            },
            None => Err(QueryError::DoesntExist),
        }
    }

    fn is_send() -> bool {
        true
    }
}

macro_rules! impl_query {
    ($($t:ident),+) => {
        impl<$($t:Query,)+> Query for ($($t,)+) {
            type Data<'a> = ($($t::Data<'a>,)+);

            fn try_query(data:&impl Queryable) -> Result<Self::Data<'_>,QueryError> {
                Ok(($($t::try_query(data)?,)+))
            }
            fn ensure_installed(data:&mut impl Queryable) {
                ($($t::ensure_installed(data)),+);
            }
            fn is_send() -> bool {
                ($($t::is_send())&&+)
            }
        }
    };
}

// impl_query!(A, B, C);
impl_all!(impl_query);

pub struct SystemRegistry<Q: Queryable> {
    send_systems: Vec<Box<dyn System<Q>>>,
    non_send_systems: Vec<Box<dyn System<Q>>>,
}
impl<Q: Queryable> SystemRegistry<Q> {
    pub fn new() -> Self {
        Self {
            send_systems: Vec::new(),
            non_send_systems: Vec::new(),
        }
    }
    pub fn send_systems(&self) -> impl Iterator<Item = &'_ Box<dyn System<Q>>> {
        self.send_systems.iter()
    }
    pub fn send_systems_mut(&mut self) -> impl Iterator<Item = &'_ mut Box<dyn System<Q>>> {
        self.send_systems.iter_mut()
    }

    pub fn non_send_systems(&self) -> impl Iterator<Item = &'_ Box<dyn System<Q>>> {
        self.non_send_systems.iter()
    }
    pub fn non_send_systems_mut(&mut self) -> impl Iterator<Item = &'_ mut Box<dyn System<Q>>> {
        self.non_send_systems.iter_mut()
    }
    pub fn add(&mut self, system: impl System<Q>) {
        if system.is_send() {
            self.send_systems.push(Box::new(system));
        } else {
            self.non_send_systems.push(Box::new(system))
        }
    }
}

impl<Q: Queryable> Default for SystemRegistry<Q> {
    fn default() -> Self {
        Self::new()
    }
}
