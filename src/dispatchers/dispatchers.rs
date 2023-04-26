use std::any::Any;

use crate::{
    components::{
        components::{Component, ComponentCollection, ComponentRegistry},
        resources::{Resource, ResourceHolder, ResourceRegistry},
        storage::{Storage, StorageReadGuard, StorageWriteGuard},
    },
    events::Events,
    schedulers::access::{Access, AccessAction, AccessType},
};

pub struct DispatcherArgs<'a> {
    components: &'a mut Storage<ComponentRegistry>,
    resources: &'a mut Storage<ResourceRegistry>,
    events: &'a mut Storage<Events>,
}

impl<'a> DispatcherArgs<'a> {
    pub fn new(
        components: &'a mut Storage<ComponentRegistry>,
        resources: &'a mut Storage<ResourceRegistry>,
        events: &'a mut Storage<Events>,
    ) -> Self {
        Self {
            components,
            resources,
            events,
        }
    }

    pub fn get_components<T: Component + 'static>(
        &mut self,
    ) -> Option<StorageReadGuard<ComponentCollection<T>>> {
        self.components.write()?.get_mut().get::<T>()
    }
    pub fn get_components_mut<T: Component + 'static>(
        &mut self,
    ) -> Option<StorageWriteGuard<ComponentCollection<T>>> {
        self.components.write()?.get_mut().get_mut::<T>()
    }

    pub fn get_resource<T: Resource + 'static>(
        &mut self,
    ) -> Option<StorageReadGuard<ResourceHolder<T>>> {
        self.resources.write()?.get_mut().get::<T>()
    }
    pub fn get_resource_mut<T: Resource + 'static>(
        &mut self,
    ) -> Option<StorageWriteGuard<ResourceHolder<T>>> {
        self.resources.write()?.get_mut().get_mut::<T>()
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct DispatchError;

pub trait Dispatcher<'a> {
    type DispatchArgs: 'a;
    type RunArgs;

    fn dispatch(&mut self, args: &mut Self::DispatchArgs) -> Result<BoxedData, DispatchError>;

    fn try_run(&mut self, args: Self::RunArgs, b: &'a mut BoxedData) -> Result<(), DispatchError>;

    fn access() -> Access
    where
        Self: Sized;
}

pub trait DispatchData<'a>: 'a {
    type DispatcherArgs: 'a;
    type Target: 'static;

    fn dispatch(args: &mut DispatcherArgs<'a>) -> Result<Self::Target, DispatchError>
    where
        Self: Sized;

    fn access() -> Access
    where
        Self: Sized;
    fn from_target_to_data<'b: 'a>(data: &'b mut Self::Target) -> Self
    where
        Self: Sized;
}

pub trait ExclusiveDispatchData<'a> {
    type DispatchArgs: 'a;
    type Target: 'static;

    unsafe fn dispatch(args: &mut DispatcherArgs<'a>) -> Self::Target
    where
        Self: Sized;

    unsafe fn from_target_to_data(data: &Self::Target) -> Self
    where
        Self: Sized;
}

pub struct BoxedData {
    pub data: Box<dyn Any>,
}

impl BoxedData {
    pub fn downcast_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.data.downcast_mut::<T>()
    }
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.data.downcast_ref::<T>()
    }
    pub fn new(t: impl Any) -> Self {
        Self { data: Box::new(t) }
    }
}

unsafe impl Send for BoxedData {}

impl<'a, T: Component + Sync + 'static> DispatchData<'a> for &'a ComponentCollection<T> {
    type DispatcherArgs = DispatcherArgs<'a>;

    type Target = StorageReadGuard<ComponentCollection<T>>;

    fn dispatch<'b>(args: &mut Self::DispatcherArgs) -> Result<Self::Target, DispatchError> {
        match args.get_components::<T>() {
            Some(data) => Ok(data),
            None => Err(DispatchError),
        }
    }

    fn access() -> Access
    where
        Self: Sized,
    {
        Access::from_iter([(AccessType::component::<T>(), AccessAction::Read(1))])
    }

    fn from_target_to_data<'b: 'a>(data: &'b mut Self::Target) -> Self
    where
        Self: Sized,
    {
        data.get()
    }
}

impl<'a, T: Component + 'static> DispatchData<'a> for &'a mut ComponentCollection<T> {
    type DispatcherArgs = DispatcherArgs<'a>;
    type Target = StorageWriteGuard<ComponentCollection<T>>;

    fn dispatch<'b>(args: &mut Self::DispatcherArgs) -> Result<Self::Target, DispatchError> {
        match args.get_components_mut::<T>() {
            Some(data) => Ok(data),
            None => Err(DispatchError),
        }
    }

    fn access() -> Access
    where
        Self: Sized,
    {
        Access::from_iter([(AccessType::component::<T>(), AccessAction::Write)])
    }

    fn from_target_to_data<'b: 'a>(data: &'b mut Self::Target) -> Self
    where
        Self: Sized,
    {
        data.get_mut()
    }
}

impl<'a, T: Resource + Sync + 'static> DispatchData<'a> for &'a ResourceHolder<T> {
    type DispatcherArgs = DispatcherArgs<'a>;

    type Target = StorageReadGuard<ResourceHolder<T>>;

    fn dispatch<'b>(args: &mut Self::DispatcherArgs) -> Result<Self::Target, DispatchError> {
        match args.get_resource::<T>() {
            Some(data) => Ok(data),
            None => Err(DispatchError),
        }
    }

    fn access() -> Access
    where
        Self: Sized,
    {
        Access::from_iter([(AccessType::resource::<T>(), AccessAction::Read(1))])
    }

    fn from_target_to_data<'b: 'a>(data: &'b mut Self::Target) -> Self
    where
        Self: Sized,
    {
        data.get()
    }
}

impl<'a, T: Resource + 'static> DispatchData<'a> for &'a mut ResourceHolder<T> {
    type DispatcherArgs = DispatcherArgs<'a>;
    type Target = StorageWriteGuard<ResourceHolder<T>>;

    fn dispatch<'b>(args: &mut Self::DispatcherArgs) -> Result<Self::Target, DispatchError> {
        match args.get_resource_mut::<T>() {
            Some(data) => Ok(data),
            None => Err(DispatchError),
        }
    }

    fn access() -> Access
    where
        Self: Sized,
    {
        Access::from_iter([(AccessType::resource::<T>(), AccessAction::Write)])
    }

    fn from_target_to_data<'b: 'a>(data: &'b mut Self::Target) -> Self
    where
        Self: Sized,
    {
        data.get_mut()
    }
}

impl<'a> DispatchData<'a> for &'a Events {
    type DispatcherArgs = DispatcherArgs<'a>;

    type Target = StorageReadGuard<Events>;

    fn dispatch(args: &mut DispatcherArgs<'a>) -> Result<Self::Target, DispatchError>
    where
        Self: Sized,
    {
        args.events.read().ok_or(DispatchError)
    }

    fn access() -> Access
    where
        Self: Sized,
    {
        todo!()
    }

    fn from_target_to_data<'b: 'a>(data: &'b mut Self::Target) -> Self
    where
        Self: Sized,
    {
        data.get()
    }
}

trait SingleDispatchData<'a>: DispatchData<'a> {}

impl<'a, T: Component + Sync + 'static> SingleDispatchData<'a> for &'a ComponentCollection<T> {}
impl<'a, T: Component + Sync + 'static> SingleDispatchData<'a> for &'a mut ComponentCollection<T> {}
impl<'a, T: Resource + Sync + 'static> SingleDispatchData<'a> for &'a ResourceHolder<T> {}
impl<'a, T: Resource + Sync + 'static> SingleDispatchData<'a> for &'a mut ResourceHolder<T> {}

macro_rules! impl_all {
    ($mac:ident, $t:ident, $($ts:ident),+) => {
        $mac!($t,$($ts),*);
        impl_all!($mac,$($ts),+);
    };
    ($mac:ident, $t:ident) => {
        $mac!($t);
    }
}

macro_rules! impl_tuple_dispatch_data {
    ($($t:ident),*) => {

        #[allow(non_snake_case)]
        impl<'a,$($t: SingleDispatchData<'a,DispatcherArgs=DispatcherArgs<'a>>,)*> DispatchData<'a> for ($($t,)*) {
            type Target = ($($t::Target,)*);
            type DispatcherArgs = DispatcherArgs<'a>;
            fn dispatch(scene:&mut Self::DispatcherArgs) -> Result<Self::Target,DispatchError> {
                Ok(($($t::dispatch(scene)?,)*))
            }
            fn access()-> Access where Self:Sized {
                let mut ans = Access::default();
                $(ans.try_combine(&$t::access()).expect("the access should not overlap");)*
                ans
            }
            fn from_target_to_data<'b:'a>(data: &'b mut Self::Target) -> Self
            where
                Self: Sized,
            {
                let ($($t,)*) = data;
                ($($t::from_target_to_data($t),)*)
            }
        }
    };
}

// impl_all!(impl_tuple_dispatch_data, A, B, C);
impl_all!(
    impl_tuple_dispatch_data,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z
);
// impl_tuple_dispatch_data!(A,B,C);
// impl_all!(impl_tuple_dispatch_data, A, B, C);

pub struct RegistryData<'a> {
    pub components: &'a mut ComponentRegistry,
    pub resources: &'a mut ResourceRegistry,
}

impl<'a> DispatchData<'a> for RegistryData<'a> {
    type DispatcherArgs = DispatcherArgs<'a>;

    type Target = (
        StorageWriteGuard<ComponentRegistry>,
        StorageWriteGuard<ResourceRegistry>,
    );

    fn dispatch<'b>(args: &mut Self::DispatcherArgs) -> Result<Self::Target, DispatchError> {
        Ok((
            match args.components.write() {
                Some(data) => data,
                None => return Err(DispatchError),
            },
            match args.resources.write() {
                Some(data) => data,
                None => return Err(DispatchError),
            },
        ))
    }

    fn access() -> Access
    where
        Self: Sized,
    {
        Access::all()
    }

    fn from_target_to_data<'b: 'a>(data: &'b mut Self::Target) -> Self
    where
        Self: Sized,
    {
        Self {
            components: data.0.get_mut(),
            resources: data.1.get_mut(),
        }
    }
}
impl<'a> DispatchData<'a> for () {
    type DispatcherArgs = DispatcherArgs<'a>;

    type Target = ();

    fn dispatch(_: &mut DispatcherArgs<'a>) -> Result<Self::Target, DispatchError>
    where
        Self: Sized,
    {
        Ok(())
    }

    fn access() -> Access
    where
        Self: Sized,
    {
        Access::empty()
    }

    fn from_target_to_data<'b: 'a>(_: &'b mut Self::Target) -> Self
    where
        Self: Sized,
    {
    }
}

mod tests {

    #[test]
    fn test_dispatchers() {
        use crate::components::components::{Component, ComponentCollection};
        use crate::dispatchers::dispatchers::DispatchData;
        struct A;
        impl Component for A {}

        struct B;
        impl Component for B {}

        type Q1 = (
            &'static mut ComponentCollection<A>,
            &'static ComponentCollection<B>,
        );
        type Q2 = (&'static ComponentCollection<B>,);

        Q1::access().try_combine(&Q2::access()).unwrap();
    }
}
