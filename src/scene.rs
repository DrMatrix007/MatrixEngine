use std::sync::{atomic::AtomicBool, Arc};

use crate::{components::components::ComponentRegistry, dispatchers::systems::SystemRegistry};

#[derive(Default)]
pub struct Scene {
    components: ComponentRegistry,
    systems: SystemRegistry,
}

impl Scene {
    pub fn component_registry_mut(&mut self) -> &mut ComponentRegistry {
        &mut self.components
    }
    pub fn component_registry(&self) -> &ComponentRegistry {
        &self.components
    }
    pub fn system_registry_mut(&mut self) -> &mut SystemRegistry {
        &mut self.systems
    }
    pub fn system_registry(&self) -> &SystemRegistry {
        &self.systems
    }
    pub(crate) fn unpack(&mut self) -> (&mut SystemRegistry,&mut ComponentRegistry) {
        (&mut self.systems,&mut self.components)
    }
}

#[derive(Clone)]
pub struct SceneUpdateArgs {
    pub quit: Arc<AtomicBool>,
}
