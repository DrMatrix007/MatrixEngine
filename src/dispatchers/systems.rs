use std::sync::{
    atomic::{AtomicBool, AtomicU64},
    Arc,
};

use crate::schedulers::access::Access;

use super::dispatchers::{
    AsyncBoxedData, AsyncDispatchData, DispatchData, DispatchError, Dispatcher, DispatcherArgs,
    ExclusiveBoxedData, ExclusiveDispatchData,
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
    Dispatcher<
    'a,
    DispatchArgs = DispatcherArgs<'a>,
    RunArgs = Arc<SystemArgs>,
    BoxedData = ExclusiveBoxedData,
>
{
    type Query: ExclusiveDispatchData<'a>;

    fn run(&mut self, args: &SystemArgs, comps: <Self as ExclusiveSystem<'a>>::Query);
}


pub trait AsyncSystem<'a>:
    Dispatcher<
        'a,
        DispatchArgs = DispatcherArgs<'a>,
        RunArgs = Arc<SystemArgs>,
        BoxedData = AsyncBoxedData,
    > + Send
    + Sync
{
    type Query: AsyncDispatchData<'a>;

    fn run(&mut self, args: &SystemArgs, comps: <Self as AsyncSystem<'a>>::Query);
}

