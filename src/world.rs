use crate::{
    scene::Scene,
    systems::{System, UnsafeBoxedDispatcher},
};

#[derive(Default)]
pub struct World {
    scene: Scene,
    startups: Vec<UnsafeBoxedDispatcher>,
    systems: Vec<UnsafeBoxedDispatcher>,
}

impl World {
    pub fn add_startup(
        &mut self,
        sys: impl for<'a> System<DispatchArgs = Scene> + 'static,
    ) -> &mut Self
where {
        self.startups.push(sys.into());
        self
    }

    pub fn add_system(
        &mut self,
        sys: impl for<'a> System<DispatchArgs = Scene> + 'static,
    ) -> &mut Self
where {
        self.systems.push(sys.into());
        self
    }

    pub(crate) fn startups_mut(&mut self) -> &mut Vec<UnsafeBoxedDispatcher> {
        &mut self.startups
    }

    pub fn scene(&self) -> &Scene {
        &self.scene
    }
    pub fn scene_mut(&mut self) -> &mut Scene {
        &mut self.scene
    }
    pub fn unpack(
        &mut self,
    ) -> (
        &mut Scene,
        &mut Vec<UnsafeBoxedDispatcher>,
        &mut Vec<UnsafeBoxedDispatcher>,
    ) {
        let World {
            scene,
            startups,
            systems,
        } = self;
        (scene, startups, systems)
    }
}
