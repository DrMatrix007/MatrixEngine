use tokio::sync::OwnedMutexGuard;

use crate::engine::scenes::{
    components::component_registry::ComponentRegistry,
    resources::resource_registry::ResourceRegistry, SceneRegistry,
};

#[derive(Debug)]
pub enum QueryError {
    CurrentlyNotAvailable,
    UnknownError,
}

pub trait Query<Args>: QueryCleanup<Args> + Sized + 'static {
    fn get(args: &mut Args) -> Result<Self, QueryError>;
    fn available(args: &mut Args) -> bool;
}

pub trait QueryCleanup<Args> {
    fn cleanup(&mut self, args: &mut Args);
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

    pub fn resources(&self) -> &ResourceRegistry {
        &self.resources_registry
    }
    pub fn resources_mut(&mut self) -> &mut ResourceRegistry {
        &mut self.resources_registry
    }
}
pub mod components {
    use std::marker::PhantomData;

    use crate::engine::scenes::components::{
        component_registry::{ComponentsMut, ComponentsRef},
        Component,
    };

    use super::{ComponentQueryArgs, Query, QueryCleanup, QueryError};

    pub struct ReadC<C: Component> {
        marker: PhantomData<C>,
        data: ComponentsRef<C>,
    }

    pub struct WriteC<C: Component> {
        marker: PhantomData<C>,
        data: ComponentsMut<C>,
    }

    impl<C: Component + 'static> QueryCleanup<ComponentQueryArgs> for WriteC<C> {
        fn cleanup(&mut self, args: &mut ComponentQueryArgs) {
            args.components_mut().try_recieve_mut(&self.data).unwrap();
        }
    }

    impl<C: Component + 'static> Query<ComponentQueryArgs> for WriteC<C> {
        fn get(args: &mut ComponentQueryArgs) -> Result<Self, QueryError> {
            args.components_mut()
                .try_write()
                .map_err(|_| QueryError::CurrentlyNotAvailable)
                .map(|x| WriteC {
                    marker: PhantomData,
                    data: x,
                })
        }

        fn available(args: &mut ComponentQueryArgs) -> bool {
            args.components().available_for_write::<C>()
        }
    }

    impl<C: Component + 'static> QueryCleanup<ComponentQueryArgs> for ReadC<C> {
        fn cleanup(&mut self, args: &mut ComponentQueryArgs) {
            args.components_mut().try_recieve_ref(&self.data).unwrap();
        }
    }

    impl<C: Component + 'static> Query<ComponentQueryArgs> for ReadC<C> {
        fn get(args: &mut ComponentQueryArgs) -> Result<Self, QueryError> {
            args.components_mut()
                .try_read()
                .map_err(|_| QueryError::CurrentlyNotAvailable)
                .map(|x| ReadC {
                    marker: PhantomData,
                    data: x,
                })
        }

        fn available(args: &mut ComponentQueryArgs) -> bool {
            args.components().available_for_read::<C>()
        }
    }
}
pub mod resources {
    use std::marker::PhantomData;

    use crate::engine::scenes::resources::{
        resource_registry::{ResourceMut, ResourceRef},
        Resource,
    };

    use super::{ComponentQueryArgs, Query, QueryCleanup, QueryError};

    pub struct ReadR<R: Resource> {
        marker: PhantomData<R>,
        data: ResourceRef<R>,
    }

    pub struct WriteR<R: Resource> {
        marker: PhantomData<R>,
        data: ResourceMut<R>,
    }

    impl<R: Resource + 'static> QueryCleanup<ComponentQueryArgs> for WriteR<R> {
        fn cleanup(&mut self, args: &mut ComponentQueryArgs) {
            args.resources_mut().try_recieve_mut(&self.data).unwrap();
        }
    }
    impl<R: Resource + 'static> Query<ComponentQueryArgs> for WriteR<R> {
        fn get(args: &mut ComponentQueryArgs) -> Result<Self, QueryError> {
            args.resources_mut()
                .try_write()
                .map_err(|_| QueryError::CurrentlyNotAvailable)
                .map(|x| WriteR {
                    marker: PhantomData,
                    data: x,
                })
        }

        fn available(args: &mut ComponentQueryArgs) -> bool {
            args.resources().available_for_write::<R>()
        }
    }
    impl<R: Resource + 'static> QueryCleanup<ComponentQueryArgs> for ReadR<R> {
        fn cleanup(&mut self, args: &mut ComponentQueryArgs) {
            args.resources_mut().try_recieve_ref(&self.data).unwrap();
        }
    }

    impl<R: Resource + 'static> Query<ComponentQueryArgs> for ReadR<R> {
        fn get(args: &mut ComponentQueryArgs) -> Result<Self, QueryError> {
            args.resources_mut()
                .try_read()
                .map_err(|_| QueryError::CurrentlyNotAvailable)
                .map(|x| ReadR {
                    marker: PhantomData,
                    data: x,
                })
        }

        fn available(args: &mut ComponentQueryArgs) -> bool {
            args.resources().available_for_read::<R>()
        }
    }
}

pub struct ReadEvents {}

impl QueryCleanup<ComponentQueryArgs> for ReadEvents {
    fn cleanup(&mut self, args: &mut ComponentQueryArgs) {}
}

// impl Query<ComponentQueryArgs> for ReadEvents {
//     fn get(args: &mut ComponentQueryArgs) -> Result<Self, QueryError> {

//     }
// }

pub trait QuerySend<Args>: Query<Args> + Sized
where
    Self: Send + Sync,
{
    fn get(args: &mut Args) -> Result<Self, QueryError> {
        <Self as Query<Args>>::get(args)
    }
}

impl<T: Query<Args>, Args> QuerySend<Args> for T where T: Send + Sync {}

impl<Args> QueryCleanup<Args> for () {
    fn cleanup(&mut self, args: &mut Args) {}
}

impl<Args> Query<Args> for () {
    fn get(args: &mut Args) -> Result<Self, QueryError> {
        Ok(())
    }

    fn available(args: &mut Args) -> bool {
        true
    }
}
