use std::sync::Arc;

use crate::schedulers::access::Access;

use super::{
    dispatchers::{AsyncBoxedData, Dispatcher, DispatcherArgs, ExclusiveBoxedData},
    systems::{AsyncSystem, ExclusiveSystem, SystemArgs},
};

pub struct BoxedAsyncSystem {
    system: Box<
        dyn for<'a> Dispatcher<
                'a,
                DispatchArgs = DispatcherArgs<'a>,
                RunArgs = Arc<SystemArgs>,
                BoxedData = AsyncBoxedData,
            > + Send
            + Sync,
    >,
    access: Access,
}

impl BoxedAsyncSystem {
    pub fn new<
        T: for<'a> AsyncSystem<
                'a,
                DispatchArgs = DispatcherArgs<'a>,
                RunArgs = Arc<SystemArgs>,
            > + 'static,
    >(
        system: T,
    ) -> Self {
        let access = T::access();
        Self {
            system: Box::new(system),
            access,
        }
    }

    pub(crate) fn get_mut(
        &mut self,
    ) -> &mut dyn for<'a> Dispatcher<
        'a,
        DispatchArgs = DispatcherArgs<'a>,
        RunArgs = Arc<SystemArgs>,
        BoxedData = AsyncBoxedData,
    > {
        self.system.as_mut()
    }

    pub(crate) fn as_ref(
        &self,
    ) -> &(dyn for<'a> Dispatcher<
        'a,
        DispatchArgs = DispatcherArgs<'a>,
        RunArgs = Arc<SystemArgs>,
        BoxedData = AsyncBoxedData,
    >) {
        self.system.as_ref()
    }

    pub(crate) fn as_mut(
        &mut self,
    ) -> &mut (dyn for<'a> Dispatcher<
        'a,
        DispatchArgs = DispatcherArgs<'a>,
        RunArgs = Arc<SystemArgs>,
        BoxedData = AsyncBoxedData,
    >) {
        self.system.as_mut()
    }
    pub(crate) fn as_access(&self) -> &Access {
        &self.access
    }
}

pub struct BoxedExclusiveSystem {
    system: Box<
        dyn for<'a> Dispatcher<
            'a,
            DispatchArgs = DispatcherArgs<'a>,
            RunArgs = Arc<SystemArgs>,
            BoxedData = ExclusiveBoxedData,
        >,
    >,
}

impl BoxedExclusiveSystem {
    pub fn new<
        T: for<'a> ExclusiveSystem<
                'a,
                DispatchArgs = DispatcherArgs<'a>,
                RunArgs = Arc<SystemArgs>,
                BoxedData = ExclusiveBoxedData,
            > + 'static,
    >(
        system: T,
    ) -> Self {
        Self {
            system: Box::new(system),
        }
    }

    pub(crate) fn get_mut(
        &mut self,
    ) -> &mut dyn for<'a> Dispatcher<
        'a,
        DispatchArgs = DispatcherArgs<'a>,
        RunArgs = Arc<SystemArgs>,
        BoxedData = ExclusiveBoxedData,
    > {
        self.system.as_mut()
    }

    pub(crate) fn as_ref(
        &self,
    ) -> &(dyn for<'a> Dispatcher<
        'a,
        DispatchArgs = DispatcherArgs<'a>,
        RunArgs = Arc<SystemArgs>,
        BoxedData = ExclusiveBoxedData,
    >) {
        self.system.as_ref()
    }

    pub(crate) fn as_mut(
        &mut self,
    ) -> &mut (dyn for<'a> Dispatcher<
        'a,
        DispatchArgs = DispatcherArgs<'a>,
        RunArgs = Arc<SystemArgs>,
        BoxedData = ExclusiveBoxedData,
    >) {
        self.system.as_mut()
    }
}

#[derive(Default)]
pub struct SystemGroup {
    normal: Vec<BoxedAsyncSystem>,
    exclusives: Vec<BoxedExclusiveSystem>,
}

impl SystemGroup {
    pub fn push_normal(&mut self, b: BoxedAsyncSystem) {
        self.normal.push(b);
    }
    pub fn push_exclusive(&mut self, b: BoxedExclusiveSystem) {
        self.exclusives.push(b);
    }

    pub fn iter_normal(&mut self) -> impl Iterator<Item = &mut BoxedAsyncSystem> {
        self.normal.iter_mut()
    }

    pub fn iter_exclusive(&mut self) -> impl Iterator<Item = &mut BoxedExclusiveSystem> {
        self.exclusives.iter_mut()
    }

    pub(crate) fn pop_normal(&mut self) -> Option<BoxedAsyncSystem> {
        self.normal.pop()
    }
}

pub(crate) struct SystemRegistryRefMut<'a> {
    pub startup_systems: &'a mut SystemGroup,
    pub runtime_systems: &'a mut SystemGroup,
}

#[derive(Default)]
pub struct SystemRegistry {
    pub(crate) startup_systems: SystemGroup,
    pub(crate) runtime_systems: SystemGroup,
}

impl SystemRegistry {
    pub(crate) fn add_system(&mut self, dispatcher: BoxedAsyncSystem) {
        self.runtime_systems.push_normal(dispatcher);
    }
    pub(crate) fn add_startup_system(&mut self, dispatcher: BoxedAsyncSystem) {
        self.startup_systems.push_normal(dispatcher);
    }
    pub(crate) fn add_exclusive_system(&mut self, distpatcher: BoxedExclusiveSystem) {
        self.runtime_systems.push_exclusive(distpatcher);
    }
    pub(crate) fn add_exclusive_startup_system(&mut self, dispatcher: BoxedExclusiveSystem) {
        self.startup_systems.push_exclusive(dispatcher);
    }

    pub(crate) fn unpack(&mut self) -> SystemRegistryRefMut<'_> {
        SystemRegistryRefMut {
            startup_systems: &mut self.startup_systems,
            runtime_systems: &mut self.runtime_systems,
        }
    }
}
