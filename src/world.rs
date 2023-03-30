use crate::{dispatchers::Dispatcher, scene::Scene, systems::{StartupSystem, System}};

#[derive(Default)]
pub struct World {
    scene: Scene,
    startups: Vec<Box<dyn Dispatcher<DispatchArgs = Scene>>>,
    systems: Vec<Box<dyn Dispatcher<DispatchArgs = Scene>>>,
}

impl World {
    pub fn add_startup(
        &mut self,
        sys: impl for<'a> StartupSystem<DispatchArgs = Scene> + 'static,
    ) -> &mut Self
where {
        self.startups.push(Box::new(sys));
        self
    }

    pub fn add_system(
        &mut self,
        sys: impl for<'a> System<DispatchArgs= Scene> + 'static,
    ) -> &mut Self
where {
        self.systems.push(Box::new(sys));
        self
    }

    pub(crate) fn startups_mut(&mut self) -> &mut Vec<Box<dyn Dispatcher<DispatchArgs = Scene>>> {
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
        &mut Vec<Box<dyn Dispatcher<DispatchArgs = Scene>>>,
        &mut Vec<Box<dyn Dispatcher<DispatchArgs = Scene>>>,
    ) {
        let World {
            scene,
            startups,
            systems,
        } = self;
        (scene, startups, systems)
    }

}
