use crate::schedulers::access::Access;

use super::dispatchers::{DispatchData, Dispatcher, DispatcherArgs};

pub struct UnsafeBoxedDispatcher(
    Box<dyn for<'a> Dispatcher<'a, DispatchArgs = DispatcherArgs<'a>>>,
    Access,
);

impl UnsafeBoxedDispatcher {
    pub unsafe fn get_ptr_mut(
        &mut self,
    ) -> *mut dyn for<'a> Dispatcher<'a, DispatchArgs = DispatcherArgs<'a>> {
        self.0.as_mut()
    }

    pub(crate) fn get_mut(
        &mut self,
    ) -> &mut dyn for<'a> Dispatcher<'a, DispatchArgs = DispatcherArgs<'a>> {
        self.0.as_mut()
    }
}

impl<T: for<'a> Dispatcher<'a, DispatchArgs = DispatcherArgs<'a>> + 'static> From<T>
    for UnsafeBoxedDispatcher
{
    fn from(value: T) -> Self {
        let access = T::access();
        UnsafeBoxedDispatcher(Box::new(value), access)
    }
}

unsafe impl Send for UnsafeBoxedDispatcher {}

impl UnsafeBoxedDispatcher {
    pub(crate) fn as_ref(
        &self,
    ) -> &(dyn for<'a> Dispatcher<'a, DispatchArgs = DispatcherArgs<'a>>) {
        self.0.as_ref()
    }

    pub(crate) fn as_mut(
        &mut self,
    ) -> &mut (dyn for<'a> Dispatcher<'a, DispatchArgs = DispatcherArgs<'a>>) {
        self.0.as_mut()
    }
    pub(crate) fn as_access(&self) -> &Access {
        &self.1
    }
}

pub trait System<'a>: Dispatcher<'a, DispatchArgs = DispatcherArgs<'a>> {
    type Query: DispatchData<'a>;

    fn run(&mut self, comps: Self::Query);
}

pub(crate) struct SystemRegistryRefMut<'a> {
    pub startup_systems: &'a mut Vec<UnsafeBoxedDispatcher>,
    pub runtime_systems: &'a mut Vec<UnsafeBoxedDispatcher>,
}

#[derive(Default)]
pub struct SystemRegistry {
    startup_systems: Vec<UnsafeBoxedDispatcher>,
    runtime_systems: Vec<UnsafeBoxedDispatcher>,
}

impl SystemRegistry {
    pub(crate) fn add_system(&mut self, dispatcher: UnsafeBoxedDispatcher) {
        self.runtime_systems.push(dispatcher);
    }
    pub(crate) fn add_startup_system(&mut self, dispatcher: UnsafeBoxedDispatcher) {
        self.startup_systems.push(dispatcher);
    }
    pub(crate) fn unpack(&mut self) -> SystemRegistryRefMut<'_> {
        SystemRegistryRefMut {
            startup_systems: &mut self.startup_systems,
            runtime_systems: &mut self.runtime_systems,
        }
    }
}
