use std::{
    sync::{atomic::AtomicBool, Arc},
};

use crate::{components::ComponentRegistry};

#[derive(Default)]
pub struct Scene {
    components: ComponentRegistry,
}

impl Scene {
    pub fn component_registry_mut(&mut self) -> &mut ComponentRegistry {
        &mut self.components
    }
    pub fn component_registry(&self) -> &ComponentRegistry {
        &self.components
    }
}

#[derive(Clone)]
pub struct SceneUpdateArgs {
    pub quit: Arc<AtomicBool>,
}
