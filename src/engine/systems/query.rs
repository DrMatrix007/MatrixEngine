use std::marker::PhantomData;

use tokio::sync::{OwnedRwLockReadGuard, OwnedRwLockWriteGuard};

use crate::{
    engine::scenes::components::{
        component_registry::{ComponentRegistry, Components},
        Component,
    },
    impl_all,
};

pub enum QueryError {
    CurrentlyNotAvailable,
    UnknownError,
}

pub trait Query<Args> {
    type Target;

    fn get(args: &mut Args) -> Result<Self::Target, QueryError>;
}

pub struct ComponentQueryArgs {
    registry: OwnedRwLockWriteGuard<ComponentRegistry>,
}

impl ComponentQueryArgs {
    pub fn new(registry: OwnedRwLockWriteGuard<ComponentRegistry>) -> Self {
        Self { registry }
    }

    pub fn registry_mut(&mut self) -> &mut ComponentRegistry {
        &mut self.registry
    }

    pub fn registry(&self) -> &ComponentRegistry {
        &self.registry
    }
}

pub struct CRead<C: Component> {
    marker: PhantomData<C>,
}

pub struct CWrite<C: Component> {
    marker: PhantomData<C>,
}

impl<C: Component + 'static> Query<ComponentQueryArgs> for CWrite<C> {
    type Target = OwnedRwLockWriteGuard<Components<C>>;

    fn get(args: &mut ComponentQueryArgs) -> Result<Self::Target, QueryError> {
        args.registry_mut()
            .try_write()
            .map_err(|_| QueryError::CurrentlyNotAvailable)
    }
}

impl<C: Component + 'static> Query<ComponentQueryArgs> for CRead<C> {
    type Target = OwnedRwLockReadGuard<Components<C>>;

    fn get(args: &mut ComponentQueryArgs) -> Result<Self::Target, QueryError> {
        args.registry_mut()
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
