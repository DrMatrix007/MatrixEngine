use crate::{
    scene::Scene, dispatchers::{systems::{UnsafeBoxedDispatcher, System, SystemRegistry}, dispatchers::DispatcherArgs}, components::{resources::ResourceRegistry, components::ComponentRegistry},
};


#[derive(Default)]
pub struct World {
    scene: Scene,
    resources:ResourceRegistry,
    
}


impl World {
    pub fn add_startup_system(
        &mut self,
        sys: impl for<'a> System<'a,DispatchArgs = DispatcherArgs<'a>> + 'static,
    ) -> &mut Self
where {
        self.scene.system_registry_mut().add_startup_system(sys.into());
        self
    }

    pub fn add_system(
        &mut self,
        sys: impl for<'a> System<'a,DispatchArgs = DispatcherArgs<'a>> + 'static,
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
    pub fn unpack(
        &mut self,
    ) -> (
        &mut Scene,
        &mut ResourceRegistry,
        
    ) {
        let World {
            scene,
            resources,
        } = self;
        (scene, resources)
    }
}
