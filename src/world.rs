use crate::{
    components::resources::ResourceRegistry,
    dispatchers::{
        dispatchers::DispatcherArgs,
        systems::{System, SystemRegistryRefMut, UnsafeBoxedDispatcher},
    },
    scene::Scene,
};

pub(crate) struct WorldRefMut<'a> {
    pub args: DispatcherArgs<'a>,
    pub startups: &'a mut Vec<UnsafeBoxedDispatcher>,
    pub systems: &'a mut Vec<UnsafeBoxedDispatcher>,
}

#[derive(Default)]
pub struct World {
    scene: Scene,
    resources: ResourceRegistry,
}

impl World {
    pub fn add_startup_system(
        &mut self,
        sys: impl for<'a> System<'a, DispatchArgs = DispatcherArgs<'a>> + 'static,
    ) -> &mut Self
where {
        self.scene
            .system_registry_mut()
            .add_startup_system(sys.into());
        self
    }

    pub fn add_system(
        &mut self,
        sys: impl for<'a> System<'a, DispatchArgs = DispatcherArgs<'a>> + 'static,
    ) -> &mut Self
where {
        self.scene.system_registry_mut().add_system(sys.into());
        self
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
            startups: runtime_systems,
            systems: startup_systems,
        }
    }
}
