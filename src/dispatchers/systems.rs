use std::sync::{
    atomic::{AtomicBool, AtomicU64},
    Arc,
};

use crate::schedulers::access::Access;

use super::dispatchers::{
    BoxedAsyncData, BoxedExclusiveData, DispatchError, DispatchedAsyncData,
    DispatchedExclusiveData, Dispatcher, DispatcherArgs, DispatchedData,
};

pub struct SystemArgs {
    quit: Arc<AtomicBool>,
    fps: Arc<AtomicU64>,
}

impl SystemArgs {
    pub fn new(quit: Arc<AtomicBool>, fps: Arc<AtomicU64>) -> Self {
        Self { quit, fps }
    }

    pub fn stop(&self) {
        self.quit.store(true, std::sync::atomic::Ordering::Relaxed);
    }
}

pub trait ExclusiveSystem<'a>:
    Dispatcher<'a, BoxedExclusiveData, DispatchArgs = DispatcherArgs<'a>, RunArgs = Arc<SystemArgs>>
{
    type Query: DispatchedExclusiveData<'a>;

    fn run(&mut self, args: &SystemArgs, comps: <Self as ExclusiveSystem<'a>>::Query);
}

impl<'a, T: ExclusiveSystem<'a>> Dispatcher<'a, BoxedExclusiveData> for T {
    type DispatchArgs = DispatcherArgs<'a>;
    type RunArgs = Arc<SystemArgs>;

    fn dispatch(
        &mut self,
        args: &mut Self::DispatchArgs,
    ) -> Result<BoxedExclusiveData, DispatchError> {
        match <T::Query as DispatchedExclusiveData<'a>>::dispatch(args) {
            Ok(data) => Ok(BoxedExclusiveData::new(data)),
            Err(err) => Err(err),
        }
    }

    fn try_run(
        &mut self,
        args: Self::RunArgs,
        b: &'a mut BoxedExclusiveData,
    ) -> Result<(), DispatchError> {
        let Some(data) = b.downcast_mut::<<T::Query as DispatchedExclusiveData<'a>>::Target>() else {
            return Err(DispatchError);
        };
        self.run(
            args.as_ref(),
            <T::Query as DispatchedExclusiveData<'a>>::from_target_to_data(data),
        );
        Ok(())
    }

    fn access() -> Access
    where
        Self: Sized,
    {
        <T::Query as DispatchedData<'a>>::access()
    }
}

pub trait AsyncSystem<'a>:
    Dispatcher<'a, BoxedAsyncData, DispatchArgs = DispatcherArgs<'a>, RunArgs = Arc<SystemArgs>>
    + Send
    + Sync
{
    type Query: DispatchedAsyncData<'a>;

    fn run(&mut self, args: &SystemArgs, comps: <Self as AsyncSystem<'a>>::Query);
}

impl<'a, T: AsyncSystem<'a>> Dispatcher<'a, BoxedAsyncData> for T {
    type DispatchArgs = DispatcherArgs<'a>;
    type RunArgs = Arc<SystemArgs>;

    fn dispatch(&mut self, args: &mut Self::DispatchArgs) -> Result<BoxedAsyncData, DispatchError> {
        match <T::Query as DispatchedAsyncData<'a>>::dispatch(args) {
            Ok(data) => Ok(BoxedAsyncData::new(data)),
            Err(err) => Err(err),
        }
    }

    fn try_run(
        &mut self,
        args: Self::RunArgs,
        b: &'a mut BoxedAsyncData,
    ) -> Result<(), DispatchError> {
        let Some(data) = b.downcast_mut::<<T::Query as DispatchedAsyncData<'a>>::Target>() else {
            return Err(DispatchError);
        };
        self.run(
            args.as_ref(),
            <T::Query as DispatchedAsyncData<'a>>::from_target_to_data(data),
        );
        Ok(())
    }

    fn access() -> Access
    where
        Self: Sized,
    {
        <T::Query as DispatchedData<'a>>::access()
    }
}
