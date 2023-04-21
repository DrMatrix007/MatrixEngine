use std::sync::Arc;

use crate::{
    components::resources::ResourceRegistry,
    dispatchers::{
        dispatchers::DispatcherArgs,
        system_registry::{BoxedExclusiveSystem, BoxedSystem, SystemGroup, SystemRegistryRefMut},
        systems::{AsyncSystem, ExclusiveSystem, SystemArgs},
    },
    scene::Scene,
};

pub(crate) struct WorldRefMut<'a> {
    pub args: DispatcherArgs<'a>,
    pub startups: &'a mut SystemGroup,
    pub systems: &'a mut SystemGroup,
}

#[derive(Default)]
pub struct World {
    scene: Scene,
    resources: ResourceRegistry,
}

impl World {
    pub fn new(scene: Scene, resources: ResourceRegistry) -> Self {
        Self { scene, resources }
    }

    pub fn add_startup_system(
        &mut self,
        sys: impl for<'a> AsyncSystem<'a, DispatchArgs = DispatcherArgs<'a>, RunArgs = Arc<SystemArgs>>
            + 'static,
    ) -> &mut Self
where {
        self.scene
            .system_registry_mut()
            .add_startup_system(BoxedSystem::new(sys));
        self
    }

    pub fn add_system(
        &mut self,
        sys: impl for<'a> AsyncSystem<'a, DispatchArgs = DispatcherArgs<'a>, RunArgs = Arc<SystemArgs>>
            + 'static,
    ) -> &mut Self
where {
        self.scene
            .system_registry_mut()
            .add_system(BoxedSystem::new(sys));
        self
    }

    pub fn add_exclusive_system(
        &mut self,
        sys: impl for<'a> ExclusiveSystem<
                'a,
                DispatchArgs = DispatcherArgs<'a>,
                RunArgs = Arc<SystemArgs>,
            > + 'static,
    ) {
        self.scene
            .system_registry_mut()
            .add_exclusive_system(BoxedExclusiveSystem::new(sys))
    }

    pub fn add_exclusive_startup_system(
        &mut self,
        sys: impl for<'a> ExclusiveSystem<
                'a,
                DispatchArgs = DispatcherArgs<'a>,
                RunArgs = Arc<SystemArgs>,
            > + 'static,
    ) {
        self.scene
            .system_registry_mut()
            .add_exclusive_startup_system(BoxedExclusiveSystem::new(sys))
    }

    pub fn scene(&self) -> &Scene {
        &self.scene
    }
    pub fn scene_mut(&mut self) -> &mut Scene {
        &mut self.scene
    }
    pub(crate) fn unpack(&mut self) -> WorldRefMut<'_> {
        let World { scene, resources } = self;
        let (sys, reg) = scene.unpack();
        let SystemRegistryRefMut {
            runtime_systems,
            startup_systems,
        } = sys.unpack();
        WorldRefMut {
            args: DispatcherArgs::new(reg, resources),
            startups: startup_systems,
            systems: runtime_systems,
        }
    }

    pub fn resource_registry_mut(&mut self) -> &mut ResourceRegistry {
        &mut self.resources
    }
    pub fn resource_registry(&self) -> &ResourceRegistry {
        &self.resources
    }
}
