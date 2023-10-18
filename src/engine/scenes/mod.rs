use std::sync::Arc;

use tokio::sync::{Mutex, OwnedMutexGuard, TryLockError};
use winit::{
    event::Event,
    event_loop::{ControlFlow, EventLoopWindowTarget},
};

use self::{
    components::component_registry::ComponentRegistry,
    resources::resource_registry::ResourceRegistry,
};

use super::{
    events::engine_event::EngineEvent,
    runtime::Runtime,
    systems::{query::ComponentQueryArgs, system_registry::SystemRegistry},
};

pub mod components;
pub mod entities;
pub mod resources;
pub mod scene_builder;

pub struct Scene {
    registry: Arc<Mutex<SceneRegistry>>,
    systems: SystemRegistry<ComponentQueryArgs>,
}

impl Scene {
    fn new(registry: SceneRegistry, systems: SystemRegistry<ComponentQueryArgs>) -> Self {
        Self {
            registry: Arc::new(Mutex::new(registry)),
            systems,
        }
    }

    fn frame(
        &mut self,
        runtime: &mut dyn Runtime<ComponentQueryArgs>,
        _target: &EventLoopWindowTarget<EngineEvent>,
        resources: OwnedMutexGuard<ResourceRegistry>,
    ) {
    }

    pub fn process_event(
        &mut self,
        event: &Event<EngineEvent>,
        target: &EventLoopWindowTarget<EngineEvent>,
        runtime: &mut dyn Runtime<ComponentQueryArgs>,
        resources: OwnedMutexGuard<ResourceRegistry>,
        control_flow: &mut ControlFlow,
    ) {
        match event {
            Event::MainEventsCleared => {
                self.frame(runtime, target, resources);
            }
            _ => {}
        }
    }

    pub fn registry(&self) -> &Arc<Mutex<SceneRegistry>> {
        &self.registry
    }

    pub(crate) fn try_lock_registry(&self) -> Result<OwnedMutexGuard<SceneRegistry>, TryLockError> {
        self.registry.clone().try_lock_owned()
    }

    pub fn systems_mut(&mut self) -> &mut SystemRegistry<ComponentQueryArgs> {
        &mut self.systems
    }

    pub fn systems(&self) -> &SystemRegistry<ComponentQueryArgs> {
        &self.systems
    }
}

pub struct SceneRegistry {
    components: ComponentRegistry,
}

impl SceneRegistry {
    fn new() -> Self {
        Self {
            components: ComponentRegistry::new(),
        }
    }

    pub fn components(&self) -> &ComponentRegistry {
        &self.components
    }

    pub fn components_mut(&mut self) -> &mut ComponentRegistry {
        &mut self.components
    }
}
