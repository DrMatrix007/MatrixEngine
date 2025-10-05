use ::winit::window::WindowId;

use crate::engine::{
    component::ComponentRegistry,
    query::Query,
    runtime::{Runtime, RuntimeContainer},
    system_registries::{Stage, SystemRegistry},
    systems::{QuerySystem, QuerySystemHolder},
};

pub mod commands;
pub mod component;
pub mod entity;
pub mod query;
pub mod runtime;
pub mod system_registries;
pub mod systems;
pub mod winit;

#[derive(Default)]
pub struct SceneRegistry {
    pub components: ComponentRegistry,
}

pub struct Scene {
    pub registry: SceneRegistry,
    pub systems: SystemRegistry<SceneRegistry>,
    pub running: bool,
    pub entity_counter: usize,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            registry: Default::default(),
            systems: Default::default(),
            running: true,
            entity_counter: 0
        }
    }

    pub fn add_system<Args: Query<Registry = SceneRegistry> + 'static>(
        &mut self,
        stage: Stage,
        system: impl QuerySystem<Args> + 'static,
    ) {
        let system = QuerySystemHolder::new(system);
        self.systems.add_system(stage, system);
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
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

    pub fn startup(&mut self) {
        self.runtime.run(
            &mut self.scene.registry,
            self.scene.systems.startup_systems_mut(),
            Stage::Startup,
        );
    }

    pub fn frame_update(&mut self) {
        let stages = [Stage::PreUpdate, Stage::Update, Stage::PostUpdate];
        self.run_stages(&stages);
    }

    fn run_stages(&mut self, stages: &[Stage]) {
        for stage in stages {
            self.runtime.run(
                &mut self.scene.registry,
                self.scene.systems.get_system_collection(stage),
                *stage,
            );
        }
    }

    pub fn frame_render(&mut self, id: &WindowId) {
        self.run_stages(&[Stage::PreRender(*id), Stage::Render(*id)]);
    }

    pub fn scene_mut(&mut self) -> &mut Scene {
        &mut self.scene
    }

    pub fn scene(&self) -> &Scene {
        &self.scene
    }
}
