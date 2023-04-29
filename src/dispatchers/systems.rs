use std::{
    any::Any,
    sync::{
        atomic::{AtomicBool, AtomicU64},
        Arc,
    },
};

pub struct BoxedData {
    data: Box<dyn Any>,
}

impl BoxedData {
    pub fn new(data: impl Any) -> Self {
        Self {
            data: Box::new(data),
        }
    }
    pub fn downcast_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.data.downcast_mut()
    }
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.data.downcast_ref()
    }
}

pub struct BoxedSendData {
    data: Box<dyn Any + Send + Sync>,
}

impl BoxedSendData {
    pub fn new(data: impl Any + Send + Sync) -> Self {
        Self {
            data: Box::new(data),
        }
    }
    pub fn downcast_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.data.downcast_mut()
    }
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.data.downcast_ref()
    }
}

use super::dispatchers::{
    DispatchError, DispatchedData, DispatchedSendData, Dispatcher, DispatcherArgs,
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

pub trait ExclusiveSystem: Dispatcher<BoxedData, Arc<SystemArgs>> {
    type Query<'a>: DispatchedData<'a>;

    fn run(&mut self, args: &SystemArgs, comps: <Self as ExclusiveSystem>::Query<'_>);
}

impl<T: ExclusiveSystem> Dispatcher<BoxedData, Arc<SystemArgs>> for T {
    fn dispatch<'b>(&mut self, args: &mut DispatcherArgs<'b>) -> Result<BoxedData, DispatchError> {
        match <T::Query<'b> as DispatchedData<'b>>::dispatch(args) {
            Ok(data) => Ok(BoxedData::new(data)),
            Err(err) => Err(err),
        }
    }

    fn try_run<'a>(
        &mut self,
        args: Arc<SystemArgs>,
        b: &'a mut BoxedData,
    ) -> Result<(), DispatchError> {
        let Some(data) = b.downcast_mut::<<T::Query<'a> as DispatchedData<'a>>::Target>() else {
            return Err(DispatchError);
        };
        self.run(
            args.as_ref(),
            <T::Query<'a> as DispatchedData<'a>>::from_target_to_data(data),
        );
        Ok(())
    }
}

pub trait AsyncSystem: Dispatcher<BoxedSendData, Arc<SystemArgs>> + Send + Sync {
    type Query<'a>: DispatchedSendData<'a>;

    fn run<'a>(&mut self, args: &SystemArgs, comps: <Self as AsyncSystem>::Query<'a>);
}

impl<T: AsyncSystem> Dispatcher<BoxedSendData, Arc<SystemArgs>> for T {
    fn dispatch<'b>(
        &mut self,
        args: &mut DispatcherArgs<'b>,
    ) -> Result<BoxedSendData, DispatchError> {
        match <T::Query<'b> as DispatchedSendData<'b>>::dispatch(args) {
            Ok(data) => Ok(BoxedSendData::new(data)),
            Err(err) => Err(err),
        }
    }

    fn try_run<'a>(
        &mut self,
        args: Arc<SystemArgs>,
        b: &'a mut BoxedSendData,
    ) -> Result<(), DispatchError> {
        let Some(data) = b.downcast_mut::<<T::Query<'a> as DispatchedSendData<'a>>::Target>() else {
            return Err(DispatchError);
        };
        self.run(
            args.as_ref(),
            <T::Query<'a> as DispatchedSendData<'a>>::from_target_to_data(data),
        );
        Ok(())
    }
}
