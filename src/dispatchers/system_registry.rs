use std::collections::VecDeque;

use super::{
    context::Context,
    dispatcher::Dispatcher,
    systems::{AsyncBoxedData, ExclusiveBoxedData},
};

pub struct BoxedAsyncSystem {
    system: Box<dyn Dispatcher<AsyncBoxedData, Context> + Send>,
    ctx: Context,
}

impl BoxedAsyncSystem {
    pub fn new(
        system: Box<dyn Dispatcher<AsyncBoxedData, Context> + Send>,
        ctx: Context,
    ) -> Self {
        Self {
            system,
            ctx,
        }
    }

    pub(crate) fn as_ref(&self) -> &dyn Dispatcher<AsyncBoxedData, Context> {
        self.system.as_ref()
    }

    pub(crate) fn as_mut(&mut self) -> &mut dyn Dispatcher<AsyncBoxedData, Context> {
        self.system.as_mut()
    }

    pub(crate) fn try_run(
        &mut self,
        b: AsyncBoxedData,
    ) -> Result<(), super::dispatcher::DispatchError> {
        self.system.try_run(&self.ctx, b)
    }

    pub(crate) fn ctx_ref(&self) -> &Context {
        &self.ctx
    }
}

pub struct BoxedExclusiveSystem {
    system: Box<dyn Dispatcher<ExclusiveBoxedData, Context>>,
    ctx: Context,
}

impl BoxedExclusiveSystem {
    pub fn new(
        system: Box<dyn Dispatcher<ExclusiveBoxedData, Context> + 'static>,
        ctx: Context,
    ) -> Self {
        Self {
            system,
            ctx,
        }
    }
    pub fn with_box(
        system: Box<dyn Dispatcher<ExclusiveBoxedData, Context>>,
        ctx: Context,
    ) -> Self
where {
        Self { system, ctx }
    }

    pub(crate) fn as_mut(&mut self) -> &mut dyn Dispatcher<ExclusiveBoxedData, Context> {
        self.system.as_mut()
    }

    pub(crate) fn as_ref(&self) -> &dyn Dispatcher<ExclusiveBoxedData, Context> {
        self.system.as_ref()
    }
    pub(crate) fn try_run(
        &mut self,
        b: ExclusiveBoxedData,
    ) -> Result<(), super::dispatcher::DispatchError> {
        self.system.try_run(&self.ctx, b)
    }

    pub(crate) fn ctx_ref(&self) -> &Context {
        &self.ctx
    }
}

#[derive(Default)]
pub struct SystemGroup {
    async_systems: VecDeque<BoxedAsyncSystem>,
    exclusive_system: VecDeque<BoxedExclusiveSystem>,
}

impl SystemGroup {
    pub fn push_async(&mut self, b: BoxedAsyncSystem) {
        self.async_systems.push_back(b);
    }
    pub fn push_exclusive(&mut self, b: BoxedExclusiveSystem) {
        self.exclusive_system.push_back(b);
    }

    pub(crate) fn pop_async(&mut self) -> Option<BoxedAsyncSystem> {
        self.async_systems.pop_front()
    }
    pub(crate) fn pop_exclusive(&mut self) -> Option<BoxedExclusiveSystem> {
        self.exclusive_system.pop_front()
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
        self.runtime_systems.push_async(dispatcher);
    }
    pub(crate) fn add_startup_system(&mut self, dispatcher: BoxedAsyncSystem) {
        self.startup_systems.push_async(dispatcher);
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
