use std::marker::PhantomData;

use winit::event_loop::EventLoopWindowTarget;

use crate::{
    components::{
        component::{Component, ComponentCollection, ComponentRegistry},
        resources::{Resource, ResourceHolder, ResourceRegistry},
        storage::{Storage, StorageReadGuard, StorageWriteGuard},
    },
    events::event_registry::EventRegistry,
    impl_all,
};

pub struct DispatcherArgs<'a> {
    components: &'a mut Storage<ComponentRegistry>,
    resources: &'a mut Storage<ResourceRegistry>,
    events: &'a mut Storage<EventRegistry>,
    target: &'a EventLoopWindowTarget<()>,
}

impl<'a> DispatcherArgs<'a> {
    pub fn new(
        components: &'a mut Storage<ComponentRegistry>,
        resources: &'a mut Storage<ResourceRegistry>,
        events: &'a mut Storage<EventRegistry>,
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
        self.target
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct DispatchError;
pub trait Dispatcher<BoxedData, RunArgs> {
    fn dispatch(&mut self, args: &mut DispatcherArgs<'_>) -> Result<BoxedData, DispatchError>;

    fn try_run(&mut self, args: &RunArgs, b: BoxedData) -> Result<(), DispatchError>;
}

pub struct ReadStorage<T> {
    data: StorageReadGuard<T>,
}

impl<T> From<StorageReadGuard<T>> for ReadStorage<T> {
    fn from(value: StorageReadGuard<T>) -> Self {
        ReadStorage { data: value }
    }
}

impl<T> ReadStorage<T> {
    pub fn new(data: StorageReadGuard<T>) -> Self {
        Self { data }
    }

    pub fn get(&self) -> &T {
        self.data.get()
    }
}
pub struct WriteStorage<T> {
    data: StorageWriteGuard<T>,
}

impl<T> WriteStorage<T> {
    pub fn new(data: StorageWriteGuard<T>) -> Self {
        Self { data }
    }

    pub fn get(&self) -> &T {
        self.data.get()
    }
    pub fn get_mut(&mut self) -> &mut T {
        self.data.get_mut()
    }
}

impl<T> From<StorageWriteGuard<T>> for WriteStorage<T> {
    fn from(value: StorageWriteGuard<T>) -> Self {
        WriteStorage { data: value }
    }
}

pub trait DispatchedData {
    type Target: 'static;

    fn dispatch(args: &mut DispatcherArgs<'_>) -> Result<Self::Target, DispatchError>
    where
        Self: Sized;

    fn from_target_to_data<'b>(data: Self::Target) -> Self
    where
        Self: Sized;
}
impl<T: Component + Sync + 'static> DispatchedData for ReadStorage<ComponentCollection<T>> {
    type Target = StorageReadGuard<ComponentCollection<T>>;
    fn dispatch(args: &mut DispatcherArgs) -> Result<Self::Target, DispatchError> {
        args.get_components::<T>().ok_or(DispatchError)
    }

    fn from_target_to_data<'b>(data: Self::Target) -> Self
    where
        Self: Sized,
    {
        data.into()
    }
}

pub trait DispatchedSendData: DispatchedData + Send + Sync {
    type Target: 'static + Send + Sync;
    fn dispatch(
        args: &mut DispatcherArgs<'_>,
    ) -> Result<<Self as DispatchedSendData>::Target, DispatchError>
    where
        Self: Sized;

    fn from_target_to_data<'b>(data: <Self as DispatchedSendData>::Target) -> Self
    where
        Self: Sized;
}

impl<T: DispatchedData + Send + Sync> DispatchedSendData for T
where
    T::Target: Send + Sync,
{
    type Target = <Self as DispatchedData>::Target;

    fn dispatch(
        args: &mut DispatcherArgs<'_>,
    ) -> Result<<Self as DispatchedSendData>::Target, DispatchError>
    where
        Self: Sized,
    {
        <Self as DispatchedData>::dispatch(args)
    }

    fn from_target_to_data<'b>(data: <Self as DispatchedSendData>::Target) -> Self
    where
        Self: Sized,
    {
        <Self as DispatchedData>::from_target_to_data(data)
    }
}

impl<T: Component + 'static> DispatchedData for WriteStorage<ComponentCollection<T>> {
    type Target = StorageWriteGuard<ComponentCollection<T>>;
    fn dispatch<'b>(args: &mut DispatcherArgs<'_>) -> Result<Self::Target, DispatchError> {
        args.get_components_mut().ok_or(DispatchError)
    }

    fn from_target_to_data<'b>(data: Self::Target) -> Self
    where
        Self: Sized,
    {
        data.into()
    }
}

impl<T: Resource + Sync + 'static> DispatchedData for ReadStorage<ResourceHolder<T>> {
    type Target = StorageReadGuard<ResourceHolder<T>>;
    fn dispatch(args: &mut DispatcherArgs) -> Result<Self::Target, DispatchError> {
        args.get_resource::<T>().ok_or(DispatchError)
    }

    fn from_target_to_data<'b>(data: Self::Target) -> Self
    where
        Self: Sized,
    {
        data.into()
    }
}

impl<T: Resource + 'static> DispatchedData for WriteStorage<ResourceHolder<T>> {
    type Target = StorageWriteGuard<ResourceHolder<T>>;

    fn dispatch(args: &mut DispatcherArgs) -> Result<Self::Target, DispatchError> {
        match args.get_resource_mut::<T>() {
            Some(data) => Ok(data),
            None => Err(DispatchError),
        }
    }

    fn from_target_to_data<'b>(data: Self::Target) -> Self
    where
        Self: Sized,
    {
        data.into()
    }
}

impl DispatchedData for ReadStorage<EventRegistry> {
    type Target = StorageReadGuard<EventRegistry>;

    fn dispatch(args: &mut DispatcherArgs<'_>) -> Result<Self::Target, DispatchError>
    where
        Self: Sized,
    {
        args.events.read().ok_or(DispatchError)
    }

    fn from_target_to_data<'b>(data: Self::Target) -> Self
    where
        Self: Sized,
    {
        data.into()
    }
}

impl DispatchedData for () {
    type Target = ();

    fn dispatch(_: &mut DispatcherArgs<'_>) -> Result<Self::Target, DispatchError>
    where
        Self: Sized,
    {
        Ok(())
    }

    fn from_target_to_data<'b>(_: Self::Target) -> Self
    where
        Self: Sized,
    {
    }
}

pub struct ReadEventLoopWindowTarget {
    value: *const EventLoopWindowTarget<()>,
}
impl ReadEventLoopWindowTarget {
    pub fn get<'a>(&'a self) -> &'a EventLoopWindowTarget<()> {
        unsafe { &*self.value }
    }
}

impl From<*const EventLoopWindowTarget<()>> for ReadEventLoopWindowTarget {
    fn from(value: *const EventLoopWindowTarget<()>) -> Self {
        Self { value }
    }
}

impl DispatchedData for ReadEventLoopWindowTarget {
    type Target = *const EventLoopWindowTarget<()>;

    fn dispatch(args: &mut DispatcherArgs<'_>) -> Result<Self::Target, DispatchError>
    where
        Self: Sized,
    {
        Ok(args.get_window_target())
    }

    fn from_target_to_data<'b>(data: Self::Target) -> Self
    where
        Self: Sized,
    {
        data.into()
    }
}

macro_rules! impl_tuple_dispatch_data {
    ($($t:ident),*) => {

        #[allow(non_snake_case)]
        impl<$($t: DispatchedData,)*> DispatchedData for ($($t,)*) {
            type Target = ($($t::Target,)*);
            fn dispatch(scene:&mut DispatcherArgs<'_>) -> Result<Self::Target,DispatchError> {
                Ok(($($t::dispatch(scene)?,)*))
            }

            fn from_target_to_data<'b>(data: Self::Target) -> Self
            where
                Self: Sized,
            {
                let ($($t,)*) = data;
                ($($t::from_target_to_data($t),)*)
            }
        }
    };
}

impl_all!(impl_tuple_dispatch_data);
