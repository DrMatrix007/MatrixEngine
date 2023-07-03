use crate::components::component::ComponentRegistry;

#[derive(Default)]
pub struct Scene {
    components:ComponentRegistry
}

impl Scene {
    pub fn components(&self) -> &ComponentRegistry {
        &self.components
    }
    pub fn components_mut(&mut self) -> &mut ComponentRegistry {
        &mut self.components
    }
}

