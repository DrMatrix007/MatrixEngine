use std::sync::{atomic::AtomicBool, Arc};

use crate::{
    components::{
        components::ComponentRegistry,
        storage::{Storage, StorageReadGuard, StorageWriteGuard},
    },
    dispatchers::system_registry::SystemRegistry,
};

#[derive(Default)]
pub struct Scene {
    components: Storage<ComponentRegistry>,
    systems: SystemRegistry,
}

impl Scene {
    pub fn component_registry_mut(&self) -> Option<StorageWriteGuard<ComponentRegistry>> {
        self.components.write()
    }
    pub fn component_registry(&self) -> Option<StorageReadGuard<ComponentRegistry>> {
        self.components.read()
    }
    pub fn system_registry_mut(&mut self) -> &mut SystemRegistry {
        &mut self.systems
    }
    pub fn system_registry(&self) -> &SystemRegistry {
        &self.systems
    }
    pub(crate) fn unpack(&mut self) -> (&mut SystemRegistry, &mut Storage<ComponentRegistry>) {
        (&mut self.systems, &mut self.components)
    }
}

#[derive(Clone)]
pub struct SceneUpdateArgs {
    pub quit: Arc<AtomicBool>,
}
