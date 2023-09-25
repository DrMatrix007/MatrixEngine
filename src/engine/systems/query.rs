use std::marker::PhantomData;

use tokio::sync::{OwnedMutexGuard, OwnedRwLockReadGuard, OwnedRwLockWriteGuard};

use crate::{
    engine::{
        events::event_registry::EventRegistry,
        scenes::{
            components::{
                component_registry::{ComponentRegistry, Components},
                Component,
            },
            SceneRegistry,
        },
    },
    impl_all,
};

#[derive(Debug)]
pub enum QueryError {
    CurrentlyNotAvailable,
    UnknownError,
}

pub trait Query<Args> {
    type Target: 'static;

    fn get(args: &mut Args) -> Result<Self::Target, QueryError>;
}

pub struct ComponentQueryArgs {
    registry: OwnedMutexGuard<SceneRegistry>,
}

impl ComponentQueryArgs {
    pub fn new(registry: OwnedMutexGuard<SceneRegistry>) -> Self {
        Self { registry }
    }

    pub fn components_mut(&mut self) -> &mut ComponentRegistry {
        self.registry.components_mut()
    }

    pub fn components(&self) -> &ComponentRegistry {
        &self.registry.components()
    }
    pub fn events(&self) -> &EventRegistry {
        self.registry.events()
    }
}

pub struct ReadC<C: Component> {
    marker: PhantomData<C>,
}

pub struct WriteC<C: Component> {
    marker: PhantomData<C>,
}

impl<C: Component + 'static> Query<ComponentQueryArgs> for WriteC<C> {
    type Target = OwnedRwLockWriteGuard<Components<C>>;

    fn get(args: &mut ComponentQueryArgs) -> Result<Self::Target, QueryError> {
        args.components_mut()
            .try_write()
            .map_err(|_| QueryError::CurrentlyNotAvailable)
    }
}

impl<C: Component + 'static> Query<ComponentQueryArgs> for ReadC<C> {
    type Target = OwnedRwLockReadGuard<Components<C>>;

    fn get(args: &mut ComponentQueryArgs) -> Result<Self::Target, QueryError> {
        args.components_mut()
            .try_read()
            .map_err(|_| QueryError::CurrentlyNotAvailable)
    }
}
macro_rules! impl_query_components {
    ($t1:tt $(,$t:tt)*) => {
        impl<Args, $t1:Query<Args>,$($t:Query<Args>),*> Query<Args> for ($t1,$($t),*) {
            type Target = ($t1::Target,$($t::Target),*);

            fn get(args:&mut Args) -> Result<<Self as Query<Args>>::Target,QueryError>{
                Ok(($t1::get(args)?,$($t::get(args)?),*))
            }
        }
    };
}

impl_all!(impl_query_components);

pub trait QuerySend<Args>: Query<Args>
where
    <Self as Query<Args>>::Target: Send + Sync,
{
    fn get(args: &mut Args) -> Result<<Self as Query<Args>>::Target, QueryError> {
        <Self as Query<Args>>::get(args)
    }
}

impl<T: Query<Args>, Args> QuerySend<Args> for T where T::Target: Send + Sync {}
