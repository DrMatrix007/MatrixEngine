use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
    sync::Arc,
};

use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::impl_all;

use super::{
    component::{Component, ComponentMap, ComponentRegistry},
    resources::{Resource, ResourceHolder},
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
    fn resource<R: Resource>(&self) -> Option<&Arc<RwLock<ResourceHolder<R>>>>;
    fn ensure_isntalled_components<C: Component>(&mut self);
    fn ensure_isntalled_resource<R: Resource>(&mut self);
}

pub trait QuerySystem: Send + Sync + 'static {
    type Query: Query;

    fn run(&mut self, args: <Self::Query as Query>::Data<'_>);
}

pub trait Query: 'static {
    type Data<'a>;
    fn ensure_installed(queryable: &mut impl Queryable);
    fn try_query(queryable: &impl Queryable) -> Result<Self::Data<'_>, QueryError>;
    fn is_send() -> bool;
}

pub struct QuerySystemWrapper<Q:QuerySystem>(Q);
impl<S: QuerySystem, Q: Queryable> System<Q> for QuerySystemWrapper<S> {
    fn run(&mut self, queryable: &Q) -> Result<(), QueryError> {
        let res = S::Query::try_query(queryable);
        match res {
            Ok(data) => {
                self.0.run(data);
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

#[derive(Debug)]
pub struct ReadC<T>(PhantomData<T>);
#[derive(Debug)]
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

#[derive(Debug)]
pub struct ReadR<R: Resource + Send>(PhantomData<R>);
#[derive(Debug)]
pub struct WriteR<R: Resource + Send>(PhantomData<R>);

#[derive(Debug)]
pub struct ReadNonSendR<R: Resource>(PhantomData<R>);
#[derive(Debug)]
pub struct WriteNonSendR<R: Resource>(PhantomData<R>);

impl<R: Resource + Send> Query for ReadR<R> {
    type Data<'a> = RwLockReadGuard<'a, ResourceHolder<R>>;

    fn ensure_installed(queryable: &mut impl Queryable) {
        queryable.ensure_isntalled_resource::<R>();
    }

    fn try_query(queryable: &impl Queryable) -> Result<Self::Data<'_>, QueryError> {
        let r = queryable.resource();
        match r {
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

impl<R: Resource + Send> Query for WriteR<R> {
    type Data<'a> = RwLockWriteGuard<'a, ResourceHolder<R>>;

    fn ensure_installed(queryable: &mut impl Queryable) {
        queryable.ensure_isntalled_resource::<R>();
    }

    fn try_query(queryable: &impl Queryable) -> Result<Self::Data<'_>, QueryError> {
        let r = queryable.resource();
        match r {
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

impl<R: Resource> Query for ReadNonSendR<R> {
    type Data<'a> = RwLockReadGuard<'a, ResourceHolder<R>>;

    fn ensure_installed(queryable: &mut impl Queryable) {
        queryable.ensure_isntalled_resource::<R>();
    }

    fn try_query(queryable: &impl Queryable) -> Result<Self::Data<'_>, QueryError> {
        let r = queryable.resource();
        match r {
            Some(data) => match data.try_read() {
                Ok(data) => Ok(data),
                Err(_) => Err(QueryError::CurrentlyNotAvailable),
            },
            None => Err(QueryError::DoesntExist),
        }
    }

    fn is_send() -> bool {
        false
    }
}

impl<R: Resource> Query for WriteNonSendR<R> {
    type Data<'a> = RwLockWriteGuard<'a, ResourceHolder<R>>;

    fn ensure_installed(queryable: &mut impl Queryable) {
        queryable.ensure_isntalled_resource::<R>();
    }

    fn try_query(queryable: &impl Queryable) -> Result<Self::Data<'_>, QueryError> {
        let r = queryable.resource();
        match r {
            Some(data) => match data.try_write() {
                Ok(data) => Ok(data),
                Err(_) => Err(QueryError::CurrentlyNotAvailable),
            },
            None => Err(QueryError::DoesntExist),
        }
    }

    fn is_send() -> bool {
        false
    }
}

impl Query for () {
    type Data<'a> = ();

    fn ensure_installed(_queryable: &mut impl Queryable) {
    }

    fn try_query(_queryable: &impl Queryable) -> Result<Self::Data<'_>, QueryError> {
        Ok(())
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


#[derive(Debug)]
pub struct QueryData<'a, Q: Query>(Q::Data<'a>);

impl<'a, Q: Query> DerefMut for QueryData<'a, Q> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a, Q: Query> Deref for QueryData<'a, Q> {
    type Target = Q::Data<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct FnWrapper<Q: Query,F:Fn(QueryData<'_,Q>)+Send+Sync>(F,PhantomData<Q>);
unsafe impl<Q: Query,F:Fn(QueryData<'_,Q>)+Send+Sync> Send for FnWrapper<Q,F> where F:Send{}
unsafe impl<Q: Query,F:Fn(QueryData<'_,Q>)+Send+Sync> Sync for FnWrapper<Q,F> where F:Send{}

impl<Q: Query,F:Fn(QueryData<'_,Q>)+Send+Sync+'static> QuerySystem for FnWrapper<Q,F>  {
    type Query = Q;

    fn run(&mut self, args: <Self::Query as Query>::Data<'_>) {
        self.0(QueryData(args));
    }
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
    pub(crate) fn add(&mut self, system: impl System<Q>) {
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

pub struct QuerySystemMarker;
pub struct FunctionSystemMarker;

pub trait IntoSystem<Marker,Q: Queryable,Qu:Query> {
    fn into_system(self) -> impl System<Q>;
}

impl<S: QuerySystem, Q: Queryable> IntoSystem<QuerySystemMarker,Q,S::Query> for S {
    fn into_system(self) -> impl System<Q> {
        QuerySystemWrapper(self)
    }
}
impl<Q: Queryable, Qu: Query,F:Fn(QueryData<'_,Qu>)+Send+Sync+'static> IntoSystem<FunctionSystemMarker,Q,Qu> for F {
    fn into_system(self) -> impl System<Q> {
        FnWrapper(self,PhantomData).into_system()
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{
        runtimes::single_threaded::SingleThreaded,
        scene::{SceneBuilder, SceneRegistry},
    };

    use super::{IntoSystem, QueryData, ReadC, ReadNonSendR, SystemRegistry};

    struct A(*const u8);
    fn systeme_a(_args: QueryData<'_, ReadNonSendR<A>>) {}

    #[test]
    fn a1() {
        let mut scene = SceneBuilder::new(|_, _| {}).build(SingleThreaded);
        scene.add_system(systeme_a);
    }
}
