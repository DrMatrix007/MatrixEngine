use crate::engine::{
    component::ComponentRegistry,
    query::Query,
    runtime::{RUNTIME_STAGES, Runtime, RuntimeContainer, Stage},
    system_registries::SystemRegistry,
    systems::{QuerySystem, QuerySystemHolder, System},
};

pub mod component;
pub mod entity;
pub mod query;
pub mod runtime;
pub mod system_registries;
pub mod systems;

#[derive(Default)]
pub struct SceneRegistry {
    pub components: ComponentRegistry,
}

pub struct Scene {
    pub registry: SceneRegistry,
    pub systems: Vec<SystemRegistry<SceneRegistry>>,
    pub startup_systems: Option<SystemRegistry<SceneRegistry>>,
    pub running: bool,
}

impl Scene {
    pub fn new() -> Self {
        let mut systems = Vec::<SystemRegistry<SceneRegistry>>::new();
        systems.resize_with(RUNTIME_STAGES.len(), Default::default);
        Self {
            registry: Default::default(),
            systems,
            startup_systems: Some(SystemRegistry::default()),
            running: true,
        }
    }

    pub fn add_system<Args: Query<Registry = SceneRegistry> + 'static>(
        &mut self,
        stage: Stage,
        system: impl QuerySystem<Args, Registry = SceneRegistry> + 'static,
    ) {
        let system = QuerySystemHolder::new(system);
        match stage {
            Stage::Startup => self
                .startup_systems
                .get_or_insert_default()
                .add_system(system),
            stage @ _ => self.systems[stage as usize].add_system(system),
        }
    }
}

pub struct Engine {
    scene: Scene,
    runtime: RuntimeContainer<SceneRegistry>,
}

impl Engine {
    pub fn new(runtime: impl Runtime<SceneRegistry> + 'static) -> Self {
        Engine {
            scene: Scene::new(),
            runtime: RuntimeContainer::new(runtime),
        }
    }

    pub fn run(&mut self) {
        if let Some(startup_systems) = self.scene.startup_systems.as_mut() {
            self.runtime
                .run(&mut self.scene.registry, startup_systems, Stage::Startup);
        }

        while self.scene.running {
            for stage in RUNTIME_STAGES {
                self.runtime.run(
                    &mut self.scene.registry,
                    &mut self.scene.systems[*stage as usize],
                    *stage,
                );
            }
        }
    }

    pub fn scene_mut(&mut self) -> &mut Scene {
        &mut self.scene
    }

    pub fn scene(&self) -> &Scene {
        &self.scene
    }
}
