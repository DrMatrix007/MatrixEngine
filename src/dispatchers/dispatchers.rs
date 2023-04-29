use winit::event_loop::EventLoopWindowTarget;

use crate::{
    components::{
        components::{Component, ComponentCollection, ComponentRegistry},
        resources::{Resource, ResourceHolder, ResourceRegistry},
        storage::{Storage, StorageReadGuard, StorageWriteGuard},
    },
    events::Events,
};

pub struct DispatcherArgs<'a> {
    components: &'a mut Storage<ComponentRegistry>,
    resources: &'a mut Storage<ResourceRegistry>,
    events: &'a mut Storage<Events>,
    target: &'a EventLoopWindowTarget<()>,
}

impl<'a> DispatcherArgs<'a> {
    pub fn new(
        components: &'a mut Storage<ComponentRegistry>,
        resources: &'a mut Storage<ResourceRegistry>,
        events: &'a mut Storage<Events>,
        target: &'a EventLoopWindowTarget<()>,
    ) -> Self {
        Self {
            components,
            resources,
            events,
            target,
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
    pub fn get_window_target(&self) -> &EventLoopWindowTarget<()> {
        self.target.clone()
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct DispatchError;
pub trait Dispatcher<BoxedData, RunArgs> {
    fn dispatch<'a>(&mut self, args: &mut DispatcherArgs<'a>) -> Result<BoxedData, DispatchError>;

    fn try_run(&mut self, args: RunArgs, b: &mut BoxedData) -> Result<(), DispatchError>;
}

pub trait DispatchedData<'a> {
    type Target: 'static;

    fn dispatch(args: &mut DispatcherArgs<'a>) -> Result<Self::Target, DispatchError>
    where
        Self: Sized;

    fn from_target_to_data<'b: 'a>(data: &'b mut Self::Target) -> Self
    where
        Self: Sized;
}
impl<'a, T: Component + Sync + 'static> DispatchedData<'a> for &'a ComponentCollection<T> {
    type Target = StorageReadGuard<ComponentCollection<T>>;

    fn dispatch<'b>(args: &mut DispatcherArgs<'b>) -> Result<Self::Target, DispatchError> {
        args.get_components::<T>().ok_or(DispatchError)
    }

    fn from_target_to_data<'b: 'a>(data: &'b mut Self::Target) -> Self
    where
        Self: Sized,
    {
        data.get()
    }
}

pub trait DispatchedSendData<'a>: DispatchedData<'a> {
    type Target: 'static + Send + Sync;

    fn dispatch(
        args: &mut DispatcherArgs<'a>,
    ) -> Result<<Self as DispatchedSendData<'a>>::Target, DispatchError>
    where
        Self: Sized;

    fn from_target_to_data<'b: 'a>(data: &'b mut <Self as DispatchedSendData<'a>>::Target) -> Self
    where
        Self: Sized;
}

impl<'a, T: DispatchedData<'a>> DispatchedSendData<'a> for T
where
    T::Target: Send + Sync,
{
    type Target = <Self as DispatchedData<'a>>::Target;

    fn dispatch(
        args: &mut DispatcherArgs<'a>,
    ) -> Result<<Self as DispatchedSendData<'a>>::Target, DispatchError>
    where
        Self: Sized,
    {
        <Self as DispatchedData<'a>>::dispatch(args)
    }

    fn from_target_to_data<'b: 'a>(data: &'b mut <Self as DispatchedSendData<'a>>::Target) -> Self
    where
        Self: Sized,
    {
        <Self as DispatchedData<'a>>::from_target_to_data(data)
    }
}

impl<'a, T: Component + 'static> DispatchedData<'a> for &'a mut ComponentCollection<T> {
    type Target = StorageWriteGuard<ComponentCollection<T>>;

    fn dispatch<'b>(args: &mut DispatcherArgs<'a>) -> Result<Self::Target, DispatchError> {
        args.get_components_mut().ok_or(DispatchError)
    }

    fn from_target_to_data<'b: 'a>(data: &'b mut Self::Target) -> Self
    where
        Self: Sized,
    {
        data.get_mut()
    }
}

impl<'a, T: Resource + Sync + 'static> DispatchedData<'a> for &'a ResourceHolder<T> {
    type Target = StorageReadGuard<ResourceHolder<T>>;

    fn dispatch<'b>(args: &mut DispatcherArgs<'b>) -> Result<Self::Target, DispatchError> {
        args.get_resource::<T>().ok_or(DispatchError)
    }

    fn from_target_to_data<'b: 'a>(data: &'b mut Self::Target) -> Self
    where
        Self: Sized,
    {
        data.get()
    }
}

impl<'a, T: Resource + 'static> DispatchedData<'a> for &'a mut ResourceHolder<T> {
    type Target = StorageWriteGuard<ResourceHolder<T>>;

    fn dispatch<'b>(args: &mut DispatcherArgs<'b>) -> Result<Self::Target, DispatchError> {
        match args.get_resource_mut::<T>() {
            Some(data) => Ok(data),
            None => Err(DispatchError),
        }
    }

    fn from_target_to_data<'b: 'a>(data: &'b mut Self::Target) -> Self
    where
        Self: Sized,
    {
        data.get_mut()
    }
}

impl<'a> DispatchedData<'a> for &'a Events {
    type Target = StorageReadGuard<Events>;

    fn dispatch(args: &mut DispatcherArgs<'a>) -> Result<Self::Target, DispatchError>
    where
        Self: Sized,
    {
        args.events.read().ok_or(DispatchError)
    }

    fn from_target_to_data<'b: 'a>(data: &'b mut Self::Target) -> Self
    where
        Self: Sized,
    {
        data.get()
    }
}

impl<'a> DispatchedData<'a> for () {
    type Target = ();

    fn dispatch(_: &mut DispatcherArgs<'a>) -> Result<Self::Target, DispatchError>
    where
        Self: Sized,
    {
        Ok(())
    }

    fn from_target_to_data<'b: 'a>(_: &'b mut Self::Target) -> Self
    where
        Self: Sized,
    {
    }
}

impl<'a> DispatchedData<'a> for &'a EventLoopWindowTarget<()> {
    type Target = *const EventLoopWindowTarget<()>;

    fn dispatch(args: &mut DispatcherArgs<'a>) -> Result<Self::Target, DispatchError>
    where
        Self: Sized,
    {
        Ok(&*args.get_window_target())
    }

    fn from_target_to_data<'b: 'a>(data: &'b mut Self::Target) -> Self
    where
        Self: Sized,
    {
        unsafe { &*(data.to_owned()) }
    }
}

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
        impl<'a,$($t: DispatchedData<'a>,)*> DispatchedData<'a> for ($($t,)*) {
            type Target = ($($t::Target,)*);
            fn dispatch(scene:&mut DispatcherArgs<'a>) -> Result<Self::Target,DispatchError> {
                Ok(($($t::dispatch(scene)?,)*))
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
