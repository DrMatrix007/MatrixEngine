use std::sync::Arc;

use super::{
    dispatcher::{Dispatcher, DispatcherArgs},
    systems::{AsyncSystem, BoxedAsyncData, BoxedData, ExclusiveSystem, SystemContext},
};

type AsyncDispatcher = dyn Dispatcher<BoxedAsyncData, SystemContext> + Send + Sync;
type ExclusiveDispatcher = dyn Dispatcher<BoxedData, SystemContext>;

pub struct BoxedAsyncSystem {
    system: Box<AsyncDispatcher>,
    ctx: SystemContext,
}

impl BoxedAsyncSystem {
    pub fn new<T: AsyncSystem + 'static>(system: T, ctx: SystemContext) -> Self {
        Self {
            system: Box::new(system),
            ctx,
        }
    }

    pub(crate) fn as_ref(&self) -> &AsyncDispatcher {
        self.system.as_ref()
    }

    pub(crate) fn as_mut(&mut self) -> &mut AsyncDispatcher {
        self.system.as_mut()
    }

    pub(crate) fn try_run(
        &mut self,
        b: &mut BoxedAsyncData,
    ) -> Result<(), super::dispatcher::DispatchError> {
        self.system.try_run(&self.ctx, b)
    }
}

pub struct BoxedExclusiveSystem {
    system: Box<ExclusiveDispatcher>,
    ctx: SystemContext,
}

impl BoxedExclusiveSystem {
    pub fn new<T: for<'a> ExclusiveSystem + 'static>(system: T, ctx: SystemContext) -> Self {
        Self {
            system: Box::new(system),
            ctx,
        }
    }

    pub(crate) fn as_mut(&mut self) -> &mut ExclusiveDispatcher {
        self.system.as_mut()
    }

    pub(crate) fn as_ref(&self) -> &ExclusiveDispatcher {
        self.system.as_ref()
    }
    pub(crate) fn try_run(&mut self,b:&mut BoxedData) -> Result<(), super::dispatcher::DispatchError> {
        self.system.try_run(&self.ctx, b)
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
    pub(crate) fn add_exclusive_system(&mut self, dispatcher: BoxedExclusiveSystem) {
        self.runtime_systems.push_exclusive(dispatcher);
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
