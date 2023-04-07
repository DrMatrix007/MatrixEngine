use std::sync::{
    atomic::{AtomicBool, AtomicU64},
    Arc,
};

use crate::schedulers::access::Access;

use super::dispatchers::{BoxedData, DispatchData, Dispatcher, DispatcherArgs};

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

pub trait System<'a>:
    Dispatcher<'a, DispatchArgs = DispatcherArgs<'a>, RunArgs = Arc<SystemArgs>> + Send
{
    type Query: DispatchData<'a>;

    fn run(&mut self, args: &SystemArgs, comps: Self::Query);
}

impl<'a, S: System<'a> + 'static> Dispatcher<'a> for S
where
    S::Query: DispatchData<'a, DispatcherArgs = DispatcherArgs<'a>>,
{
    unsafe fn dispatch(&mut self, args: &mut Self::DispatchArgs) -> BoxedData {
        BoxedData::new(<<S as System<'a>>::Query as DispatchData<'a>>::dispatch(
            args,
        ))
    }
    type RunArgs = Arc<SystemArgs>;
    type DispatchArgs = DispatcherArgs<'a>;

    fn try_run(&mut self, args: Self::RunArgs, b: BoxedData) -> Result<(), BoxedData> {
        let data = *(b.downcast::<<S::Query as DispatchData<'a>>::Target>()?);
        self.run(args.as_ref(), unsafe {
            <S::Query as DispatchData<'a>>::from_target_to_data(data)
        });
        Ok(())
    }
    fn access() -> Access
    where
        Self: Sized,
    {
        <Self as System>::Query::access()
    }
}

pub(crate) struct SystemRegistryRefMut<'a> {
    pub startup_systems: &'a mut Vec<UnsafeBoxedSystem>,
    pub runtime_systems: &'a mut Vec<UnsafeBoxedSystem>,
}

#[derive(Default)]
pub struct SystemRegistry {
    startup_systems: Vec<UnsafeBoxedSystem>,
    runtime_systems: Vec<UnsafeBoxedSystem>,
}

impl SystemRegistry {
    pub(crate) fn add_system(&mut self, dispatcher: UnsafeBoxedSystem) {
        self.runtime_systems.push(dispatcher);
    }
    pub(crate) fn add_startup_system(&mut self, dispatcher: UnsafeBoxedSystem) {
        self.startup_systems.push(dispatcher);
    }
    pub(crate) fn unpack(&mut self) -> SystemRegistryRefMut<'_> {
        SystemRegistryRefMut {
            startup_systems: &mut self.startup_systems,
            runtime_systems: &mut self.runtime_systems,
        }
    }
}
pub struct UnsafeBoxedSystem {
    system: Box<
        dyn for<'a> Dispatcher<'a, DispatchArgs = DispatcherArgs<'a>, RunArgs = Arc<SystemArgs>>,
    >,
    access: Access,
}

impl UnsafeBoxedSystem {
    pub fn new<
        T: for<'a> System<'a, DispatchArgs = DispatcherArgs<'a>, RunArgs = Arc<SystemArgs>> + 'static,
    >(
        system: T,
    ) -> Self {
        let access = T::access();
        Self {
            system: Box::new(system),
            access,
        }
    }

    pub unsafe fn get_ptr_mut(
        &mut self,
    ) -> *mut dyn for<'a> Dispatcher<'a, DispatchArgs = DispatcherArgs<'a>, RunArgs = Arc<SystemArgs>>
    {
        self.system.as_mut()
    }

    pub(crate) fn get_mut(
        &mut self,
    ) -> &mut dyn for<'a> Dispatcher<'a, DispatchArgs = DispatcherArgs<'a>, RunArgs = Arc<SystemArgs>>
    {
        self.system.as_mut()
    }

    pub(crate) fn as_ref(
        &self,
    ) -> &(dyn for<'a> Dispatcher<'a, DispatchArgs = DispatcherArgs<'a>, RunArgs = Arc<SystemArgs>>)
    {
        self.system.as_ref()
    }

    pub(crate) fn as_mut(
        &mut self,
    ) -> &mut (dyn for<'a> Dispatcher<
        'a,
        DispatchArgs = DispatcherArgs<'a>,
        RunArgs = Arc<SystemArgs>,
    >) {
        self.system.as_mut()
    }
    pub(crate) fn as_access(&self) -> &Access {
        &self.access
    }
}

unsafe impl Send for UnsafeBoxedSystem {}
