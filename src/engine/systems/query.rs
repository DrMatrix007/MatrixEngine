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

pub trait Query<Args>: QueryCleanup<Args> + Sized + 'static {
    fn get(args: &mut Args) -> Result<Self, QueryError>;
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
    }
}

macro_rules! impl_query_components {
    ($t1:tt $(,$t:tt)*) => {
        #[allow(non_snake_case)]
        impl<Args, $t1:Query<Args>,$($t:Query<Args>),*> Query<Args> for ($t1,$($t),*) {

            fn get(args:&mut Args) -> Result<Self,QueryError>{
                Ok(($t1::get(args)?,$($t::get(args)?),*))
            }
        }
        #[allow(non_snake_case)]
        impl<Args, $t1:Query<Args>,$($t:Query<Args>),*> QueryCleanup<Args> for ($t1,$($t),*) {

            fn cleanup(&mut self,args:&mut Args){
                let (ref mut $t1,$(ref mut $t),*)= self;
                $t1::cleanup($t1, args);$($t::cleanup($t,args));*
            }
        }
    };
}

// impl<Args, A: Query<Args>, B: Query<Args>> Query<Args> for (A, B) {
//     fn get(args: &mut Args) -> Result<Self, QueryError> {
//         Ok((A::get(args)?, B::get(args)?))
//     }
// }
// impl<Args, A: Query<Args>, B: Query<Args>> QueryCleanup<Args> for (A, B) {
//     fn cleanup(&mut self, sargs: &mut Args) {
//         let (ref mut A, ref mut B) = self;
//     }
// }

impl_all!(impl_query_components);
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
}
