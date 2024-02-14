use super::components::{ComponentMap, ComponentRegistry};

pub struct Scene {
    components: ComponentRegistry,
    scenes: SceneRegistry,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            components: ComponentRegistry::new(),
            scenes,
        }
    }
}
