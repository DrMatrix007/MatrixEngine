use winit::event_loop::EventLoopWindowTarget;

use crate::{
    components::{
        component::ComponentRegistry,
        resources::ResourceRegistry,
        storage::{Storage, StorageReadGuard, StorageWriteGuard},
    },
    dispatchers::{
        context::Context,
        dispatcher::{DispatcherArgs},
        system_registry::{BoxedAsyncSystem, BoxedExclusiveSystem, SystemRegistry},
        systems::{AsyncSystem, ExclusiveSystem},
    },
    events::event_registry::EventRegistry,
    schedulers::scheduler::Scheduler,
};

pub struct Scene {
    pub(crate) components: Storage<ComponentRegistry>,
    pub(crate) systems: SystemRegistry,
    ctx: Context,
    is_started: bool,
}

impl Scene {
    pub(crate) fn empty(ctx: Context) -> Self {
        Self {
            components: Default::default(),
            systems: Default::default(),
            ctx,
            is_started: false,
        }
    }

    pub fn component_registry_mut(&self) -> Option<StorageWriteGuard<ComponentRegistry>> {
        self.components.write()
    }
    pub fn component_registry(&self) -> Option<StorageReadGuard<ComponentRegistry>> {
        self.components.read()
    }
    pub fn system_registry_mut(&mut self) -> &mut SystemRegistry {
        &mut self.systems
    }
    pub fn system_registry(&self) -> &SystemRegistry {
        &self.systems
    }
    pub(crate) fn unpack(&mut self) -> (&mut SystemRegistry, &mut Storage<ComponentRegistry>) {
        (&mut self.systems, &mut self.components)
    }

    pub fn add_startup_async_system(&mut self, sys: impl AsyncSystem + 'static) -> &mut Self
where {
        self.systems
            .add_startup_system(BoxedAsyncSystem::new(sys, self.ctx.clone()));
        self
    }

    pub fn add_async_system(&mut self, sys: impl AsyncSystem + 'static) -> &mut Self
where {
        self.systems
            .add_system(BoxedAsyncSystem::new(sys, self.ctx.clone()));
        self
    }

    pub fn add_exclusive_system(
        &mut self,
        sys: impl for<'a> ExclusiveSystem + 'static,
    ) -> &mut Self {
        self.systems
            .add_exclusive_system(BoxedExclusiveSystem::new(sys, self.ctx.clone()));
        self
    }

    pub fn add_startup_exclusive_system(
        &mut self,
        sys: impl for<'a> ExclusiveSystem + 'static,
    ) -> &mut Self {
        self.systems
            .add_exclusive_startup_system(BoxedExclusiveSystem::new(sys, self.ctx.clone()));
        self
    }

    //     pub fn with_startup_async_system(
    //         mut self,
    //         sys: impl AsyncSystem + 'static,
    //     ) -> Self
    // where {
    //         self.system_registry_mut()
    //             .add_startup_system(BoxedAsyncSystem::new(sys, ctx));
    //         self
    //     }

    //     pub fn with_async_system(
    //         mut self,
    //         sys: impl AsyncSystem + 'static,
    //     ) -> Self
    // where {
    //         self.system_registry_mut()
    //             .add_system(BoxedAsyncSystem::new(sys, ctx));
    //         self
    //     }

    //     pub fn with_exclusive_system(
    //         mut self,
    //         sys: impl for<'a> ExclusiveSystem + 'static,
    //     ) -> Self {
    //         self.system_registry_mut()
    //             .add_exclusive_system(BoxedExclusiveSystem::new(sys, self.ctx));
    //         self
    //     }

    //     pub fn with_startup_exclusive_system(
    //         mut self,
    //         sys: impl ExclusiveSystem + 'static,
    //     ) -> Self {
    //         self.system_registry_mut()
    //             .add_exclusive_startup_system(BoxedExclusiveSystem::new(sys, ctx));
    //         self
    //     }

    pub(crate) fn update(&mut self, args: SceneUpdateArgs) {
        if !self.is_started {
            args.scheduler.run(
                &mut self.systems.startup_systems,
                &mut DispatcherArgs::new(
                    &mut self.components,
                    args.resources,
                    args.events,
                    args.window_target,
                ),
            );
            self.is_started = true;
        } else {
            args.scheduler.run(
                &mut self.systems.runtime_systems,
                &mut DispatcherArgs::new(
                    &mut self.components,
                    args.resources,
                    args.events,
                    args.window_target,
                ),
            );
        }
    }
}

pub struct SceneUpdateArgs<'a> {
    pub scheduler: &'a mut dyn Scheduler,
    pub resources: &'a mut Storage<ResourceRegistry>,
    pub events: &'a mut Storage<EventRegistry>,
    pub window_target: &'a EventLoopWindowTarget<()>,
}
