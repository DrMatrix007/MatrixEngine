use tokio::sync::OwnedMutexGuard;

use crate::{
    engine::{
        events::event_registry::EventRegistry,
        scenes::{
            components::component_registry::ComponentRegistry,
            resources::resource_registry::ResourceRegistry, SceneRegistry,
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
    scene_registry: OwnedMutexGuard<SceneRegistry>,
    resources_registry: OwnedMutexGuard<ResourceRegistry>,
}

impl ComponentQueryArgs {
    pub fn new(
        scene_registry: OwnedMutexGuard<SceneRegistry>,
        resources_registry: OwnedMutexGuard<ResourceRegistry>,
    ) -> Self {
        Self {
            scene_registry,
            resources_registry,
        }
    }

    pub fn components_mut(&mut self) -> &mut ComponentRegistry {
        self.scene_registry.components_mut()
    }

    pub fn components(&self) -> &ComponentRegistry {
        &self.scene_registry.components()
    }
    pub fn events(&self) -> &EventRegistry {
        self.scene_registry.events()
    }
    pub fn resources(&self) -> &ResourceRegistry {
        &self.resources_registry
    }
    pub fn resources_mut(&mut self) -> &mut ResourceRegistry {
        &mut self.resources_registry
    }
}
pub mod components {
    use std::marker::PhantomData;

    use tokio::sync::{OwnedRwLockReadGuard, OwnedRwLockWriteGuard};

    use crate::engine::scenes::components::{component_registry::Components, Component};

    use super::{ComponentQueryArgs, Query, QueryError};

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
}
pub mod resources {
    use std::marker::PhantomData;

    use tokio::sync::{OwnedRwLockReadGuard, OwnedRwLockWriteGuard};

    use crate::engine::scenes::resources::{resource_registry::ResourceHolder, Resource};

    use super::{ComponentQueryArgs, Query, QueryError};

    pub struct ReadR<R: Resource> {
        marker: PhantomData<R>,
    }
    pub struct WriteR<R: Resource> {
        marker: PhantomData<R>,
    }

    impl<C: Resource + 'static> Query<ComponentQueryArgs> for WriteR<C> {
        type Target = OwnedRwLockWriteGuard<ResourceHolder<C>>;

        fn get(args: &mut ComponentQueryArgs) -> Result<Self::Target, QueryError> {
            args.resources_mut()
                .try_write()
                .map_err(|_| QueryError::CurrentlyNotAvailable)
        }
    }

    impl<C: Resource + 'static> Query<ComponentQueryArgs> for ReadR<C> {
        type Target = OwnedRwLockReadGuard<ResourceHolder<C>>;

        fn get(args: &mut ComponentQueryArgs) -> Result<Self::Target, QueryError> {
            args.resources_mut()
                .try_read()
                .map_err(|_| QueryError::CurrentlyNotAvailable)
        }
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

impl<Args> Query<Args> for () {
    type Target = ();

    fn get(args: &mut Args) -> Result<Self::Target, QueryError> {
        Ok(())
    }
}
