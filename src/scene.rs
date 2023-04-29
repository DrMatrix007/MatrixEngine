use std::sync::{
    atomic::{AtomicBool, AtomicU64},
    Arc,
};

use winit::event_loop::EventLoopWindowTarget;

use crate::{
    components::{
        components::ComponentRegistry,
        resources::ResourceRegistry,
        storage::{Storage, StorageReadGuard, StorageWriteGuard},
    },
    dispatchers::{
        dispatchers::DispatcherArgs,
        system_registry::{BoxedAsyncSystem, BoxedExclusiveSystem, SystemRegistry},
        systems::{AsyncSystem, ExclusiveSystem, SystemArgs},
    },
    events::Events,
    schedulers::schedulers::Scheduler,
};

pub struct Scene {
    pub(crate) components: Storage<ComponentRegistry>,
    pub(crate) systems: SystemRegistry,
    is_started: bool,
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            components: Default::default(),
            systems: Default::default(),
            is_started: false,
        }
    }
}

impl Scene {
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
        self.system_registry_mut()
            .add_startup_system(BoxedAsyncSystem::new(sys));
        self
    }

    pub fn add_async_system(&mut self, sys: impl AsyncSystem + 'static) -> &mut Self
where {
        self.system_registry_mut()
            .add_system(BoxedAsyncSystem::new(sys));
        self
    }

    pub fn add_exclusive_system(
        &mut self,
        sys: impl for<'a> ExclusiveSystem + 'static,
    ) -> &mut Self {
        self.system_registry_mut()
            .add_exclusive_system(BoxedExclusiveSystem::new(sys));
        self
    }

    pub fn add_startup_exclusive_system(
        &mut self,
        sys: impl ExclusiveSystem + 'static,
    ) -> &mut Self {
        self.system_registry_mut()
            .add_exclusive_startup_system(BoxedExclusiveSystem::new(sys));
        self
    }

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
                Arc::new(SystemArgs::new(args.quit, args.fps)),
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
                Arc::new(SystemArgs::new(args.quit, args.fps)),
            );
        }
    }
}

pub struct SceneUpdateArgs<'a> {
    pub quit: Arc<AtomicBool>,
    pub fps: Arc<AtomicU64>,
    pub scheduler: &'a mut dyn Scheduler,
    pub resources: &'a mut Storage<ResourceRegistry>,
    pub events: &'a mut Storage<Events>,
    pub window_target: &'a EventLoopWindowTarget<()>,
}
