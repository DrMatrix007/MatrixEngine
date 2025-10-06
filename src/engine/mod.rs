use ::winit::{event_loop::ActiveEventLoop, window::WindowId};

use crate::{
    engine::{
        component::ComponentRegistry,
        entity::EntityCreator,
        query::Query,
        resources::ResourceRegistry,
        runtime::{Runtime, RuntimeContainer},
        system_registries::{Stage, StageDescriptor, SystemRegistry},
        systems::{QuerySystem, QuerySystemHolder},
    },
    lockable::{Lockable, LockableError, LockableWriteGuard},
};

pub mod commands;
pub mod component;
pub mod entity;
pub mod query;
pub mod resources;
pub mod runtime;
pub mod system_registries;
pub mod systems;
pub mod window_registry;
pub mod winit;

#[derive(Default)]
pub struct SceneRegistry {
    pub components: ComponentRegistry,
}

pub struct Scene {
    pub registry: Lockable<SceneRegistry>,
    pub systems: SystemRegistry<EngineState>,
    pub entity_counter: Lockable<EntityCreator>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            registry: Lockable::new(Default::default()),
            systems: Default::default(),
            entity_counter: Lockable::new(Default::default()),
        }
    }

    pub fn add_system<Args: Query<Registry = EngineState> + 'static>(
        &mut self,
        stage: StageDescriptor,
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

pub struct EngineRegistry {
    pub scene: Scene,
    pub resource_registry: Lockable<ResourceRegistry>,
}

pub struct Engine {
    registry: EngineRegistry,
    runtime: RuntimeContainer<EngineState>,
}

impl Engine {
    pub fn new(runtime: impl Runtime<EngineState> + 'static) -> Self {
        Engine {
            registry: EngineRegistry {
                scene: Scene::new(),
                resource_registry: Default::default(),
            },
            runtime: RuntimeContainer::new(runtime),
        }
    }

    pub fn prepare_state(
        &mut self,
        active_event_loop: &ActiveEventLoop,
        stage: Stage,
    ) -> Result<EngineState, LockableError> {
        Ok(EngineState {
            registry: self.registry.scene.registry.write()?,
            resources: self.registry.resource_registry.write()?,
            entity_creator: self.registry.scene.entity_counter.write()?,
            active_event_loop,
            stage,
        })
    }
    pub fn consume_state(&mut self, state: EngineState) -> Result<(), LockableError> {
        self.registry.scene.registry.consume_write(state.registry)?;
        self.registry
            .resource_registry
            .consume_write(state.resources)?;
        self.registry
            .scene
            .entity_counter
            .consume_write(state.entity_creator)?;
        Ok(())
    }

    pub fn startup(&mut self, active_event_loop: &ActiveEventLoop) {
        self.run_stages(&[Stage::Startup], active_event_loop);
    }

    pub fn frame_update(&mut self, active_event_loop: &ActiveEventLoop) {
        let stages = [Stage::PreUpdate, Stage::Update, Stage::PostUpdate];
        self.run_stages(&stages, active_event_loop);
    }

    fn run_stages(&mut self, stages: &[Stage], active_event_loop: &ActiveEventLoop) {
        for stage in stages {
            let mut args = self.prepare_state(active_event_loop, *stage).unwrap();
            self.runtime.run(
                &mut args,
                self.registry
                    .scene
                    .systems
                    .get_system_collection(&stage.to_descriptor()),
                *stage,
            );
            self.consume_state(args).unwrap();
        }
    }

    pub fn frame_render(&mut self, id: &WindowId, active_event_loop: &ActiveEventLoop) {
        self.run_stages(
            &[Stage::PreRender(*id), Stage::Render(*id)],
            active_event_loop,
        );
    }

    pub fn scene_mut(&mut self) -> &mut Scene {
        &mut self.registry.scene
    }

    pub fn scene(&self) -> &Scene {
        &self.registry.scene
    }
}

pub struct EngineState {
    pub registry: LockableWriteGuard<SceneRegistry>,
    pub resources: LockableWriteGuard<ResourceRegistry>,
    pub entity_creator: LockableWriteGuard<EntityCreator>,
    pub stage: Stage,
    active_event_loop: *const ActiveEventLoop,
}

impl EngineState {
    pub fn active_event_loop(&self) -> &ActiveEventLoop {
        unsafe { &*self.active_event_loop }
    }
}
