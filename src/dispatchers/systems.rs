use std::sync::{
    atomic::{AtomicBool, AtomicU64},
    Arc,
};

use crate::schedulers::access::Access;

use super::dispatchers::{BoxedData, DispatchData, DispatchError, Dispatcher, DispatcherArgs};

pub struct SystemArgs {
    quit: Arc<AtomicBool>,
    fps: Arc<AtomicU64>,
}

impl SystemArgs {
    pub fn new(
        quit: Arc<AtomicBool>,
        fps: Arc<AtomicU64>,
    ) -> Self {
        Self {
            quit,
            fps,
        }
    }

    pub fn stop(&self) {
        self.quit.store(true, std::sync::atomic::Ordering::Relaxed);
    }
}

pub trait BaseSystem<'a>:
    Dispatcher<'a, DispatchArgs = DispatcherArgs<'a>, RunArgs = Arc<SystemArgs>>
{
    type Query: DispatchData<'a>;

    fn run(&mut self, args: &SystemArgs, comps: Self::Query);
}

impl<'a, S: BaseSystem<'a> + 'static> Dispatcher<'a> for S
where
    S::Query: DispatchData<'a, DispatcherArgs = DispatcherArgs<'a>>,
{
    fn dispatch(&mut self, args: &mut Self::DispatchArgs) -> Result<BoxedData, DispatchError> {
        Ok(BoxedData::new(
            <<S as BaseSystem<'a>>::Query as DispatchData<'a>>::dispatch(args)?,
        ))
    }
    type RunArgs = Arc<SystemArgs>;
    type DispatchArgs = DispatcherArgs<'a>;

    fn try_run(&mut self, args: Self::RunArgs, b: &'a mut BoxedData) -> Result<(), DispatchError> {
        let Some(data) = b.downcast_mut::<<S::Query as DispatchData<'a>>::Target>() else {
            return Err(DispatchError);
        };
        self.run(
            args.as_ref(),
            <S::Query as DispatchData>::from_target_to_data(data),
        );
        Ok(())
    }
    fn access() -> Access
    where
        Self: Sized,
    {
        <Self as BaseSystem>::Query::access()
    }
}

pub trait ExclusiveSystem<'a>:
    BaseSystem<'a, Query = <Self as ExclusiveSystem<'a>>::Query>
{
    type Query: DispatchData<'a>;

    fn run(&mut self, args: &SystemArgs, comps: <Self as ExclusiveSystem<'a>>::Query);
}
impl<'a, T: ExclusiveSystem<'a>> BaseSystem<'a> for T {
    type Query = <T as ExclusiveSystem<'a>>::Query;

    fn run(&mut self, args: &SystemArgs, comps: <Self as BaseSystem<'a>>::Query) {
        <T as ExclusiveSystem<'a>>::run(self, args, comps);
    }
}

pub trait AsyncSystem<'a>:
    Send + Sync + ExclusiveSystem<'a, Query = <Self as AsyncSystem<'a>>::Query>
{
    type Query: DispatchData<'a>;

    fn run(&mut self, args: &SystemArgs, comps: <Self as AsyncSystem<'a>>::Query);
}
impl<'a, T: AsyncSystem<'a>> ExclusiveSystem<'a> for T {
    type Query = <T as AsyncSystem<'a>>::Query;

    fn run(&mut self, args: &SystemArgs, comps: <Self as BaseSystem<'a>>::Query) {
        <T as AsyncSystem<'a>>::run(self, args, comps);
    }
}
