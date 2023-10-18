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
    use std::{
        marker::PhantomData,
        ops::{Deref, DerefMut},
    };

    use crate::engine::scenes::components::{
        component_registry::{ComponentsMut, ComponentsRef},
        Component,
    };

    use super::{ComponentQueryArgs, Query, QueryCleanup, QueryError};

    pub struct ReadC<C: Component> {
        marker: PhantomData<C>,
        data: ComponentsRef<C>,
    }

    impl<C: Component> Deref for ReadC<C> {
        type Target = ComponentsRef<C>;

        fn deref(&self) -> &Self::Target {
            &self.data
        }
    }

    impl<C: Component> Deref for WriteC<C> {
        type Target = ComponentsMut<C>;

        fn deref(&self) -> &Self::Target {
            &self.data
        }
    }

    impl<C: Component> DerefMut for WriteC<C> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.data
        }
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
    use std::{
        marker::PhantomData,
        ops::{Deref, DerefMut},
    };

    use crate::engine::scenes::resources::{
        resource_registry::{ResourceHolder, ResourceMut, ResourceRef},
        Resource,
    };

    use super::{ComponentQueryArgs, Query, QueryCleanup, QueryError};

    pub struct ReadR<R: Resource> {
        marker: PhantomData<R>,
        data: ResourceRef<R>,
    }
    impl<R: Resource> ReadR<R> {
        pub fn get(&self) -> Option<&R> {
            self.data.as_ref().as_ref()
        }
    }

    pub struct WriteR<R: Resource> {
        marker: PhantomData<R>,
        data: ResourceMut<R>,
    }

    impl<R: Resource> WriteR<R> {
        pub fn get_mut(&mut self) -> Option<&mut R> {
            self.data.as_mut().as_mut()
        }

        pub fn get(&self) -> Option<&R> {
            self.data.as_ref().as_ref()
        }
    }

    impl<R: Resource> AsMut<ResourceHolder<R>> for WriteR<R> {
        fn as_mut(&mut self) -> &mut ResourceHolder<R> {
            &mut self.data
        }
    }

    impl<R: Resource> AsRef<ResourceHolder<R>> for WriteR<R> {
        fn as_ref(&self) -> &ResourceHolder<R> {
            &self.data
        }
    }
    impl<R: Resource> AsRef<ResourceHolder<R>> for ReadR<R> {
        fn as_ref(&self) -> &ResourceHolder<R> {
            &self.data
        }
    }

    impl<R: Resource> Deref for WriteR<R> {
        type Target = ResourceHolder<R>;

        fn deref(&self) -> &Self::Target {
            &self.data
        }
    }

    impl<R: Resource> Deref for ReadR<R> {
        type Target = ResourceHolder<R>;

        fn deref(&self) -> &Self::Target {
            &self.data
        }
    }

    impl<R: Resource> DerefMut for WriteR<R> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.data
        }
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
