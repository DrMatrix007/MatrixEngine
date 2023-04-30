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

pub struct BoxedAsyncData {
    data: Box<dyn Any + Send + Sync>,
}

impl BoxedAsyncData {
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

use crate::events::matrix_event::MatrixEventSender;

use super::dispatcher::{
    DispatchError, DispatchedData, DispatchedSendData, Dispatcher, DispatcherArgs,
};

#[derive(Clone)]
pub struct SystemContext {
    quit: Arc<AtomicBool>,
    fps: Arc<AtomicU64>,
    sender: MatrixEventSender,
}


impl SystemContext {
    pub fn new(quit: Arc<AtomicBool>, fps: Arc<AtomicU64>, sender: MatrixEventSender) -> Self {
        Self { quit, fps, sender }
    }

    pub fn stop(&self) {
        self.quit.store(true, std::sync::atomic::Ordering::Relaxed);
    }
}

pub trait ExclusiveSystem: Dispatcher<BoxedData, SystemContext> {
    type Query<'a>: DispatchedData<'a>;

    fn run(&mut self, args: &SystemContext, comps: <Self as ExclusiveSystem>::Query<'_>);
}

impl<T: ExclusiveSystem> Dispatcher<BoxedData, SystemContext> for T {
    fn dispatch<'b>(&mut self, args: &mut DispatcherArgs<'b>) -> Result<BoxedData, DispatchError> {
        match <T::Query<'b> as DispatchedData<'b>>::dispatch(args) {
            Ok(data) => Ok(BoxedData::new(data)),
            Err(err) => Err(err),
        }
    }

    fn try_run<'a>(
        &mut self,
        args: &SystemContext,
        b: &'a mut BoxedData,
    ) -> Result<(), DispatchError> {
        let Some(data) = b.downcast_mut::<<T::Query<'a> as DispatchedData<'a>>::Target>() else {
            return Err(DispatchError);
        };
        self.run(
            args,
            <T::Query<'a> as DispatchedData<'a>>::from_target_to_data(data),
        );
        Ok(())
    }
}

pub trait AsyncSystem: Dispatcher<BoxedAsyncData, SystemContext> + Send + Sync {
    type Query<'a>: DispatchedSendData<'a>;

    fn run(&mut self, args: &SystemContext, comps: <Self as AsyncSystem>::Query<'_>);
}

impl<T: AsyncSystem> Dispatcher<BoxedAsyncData, SystemContext> for T {
    fn dispatch<'b>(
        &mut self,
        args: &mut DispatcherArgs<'b>,
    ) -> Result<BoxedAsyncData, DispatchError> {
        match <T::Query<'b> as DispatchedSendData<'b>>::dispatch(args) {
            Ok(data) => Ok(BoxedAsyncData::new(data)),
            Err(err) => Err(err),
        }
    }

    fn try_run<'a>(
        &mut self,
        args: &SystemContext,
        b: &'a mut BoxedAsyncData,
    ) -> Result<(), DispatchError> {
        let Some(data) = b.downcast_mut::<<T::Query<'a> as DispatchedSendData<'a>>::Target>() else {
            return Err(DispatchError);
        };
        self.run(
            args,
            <T::Query<'a> as DispatchedSendData<'a>>::from_target_to_data(data),
        );
        Ok(())
    }
}
