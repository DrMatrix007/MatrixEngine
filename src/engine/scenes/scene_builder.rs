use crate::engine::systems::{query::ComponentQueryArgs, system_registry::SystemRegistry};

use super::{Scene, SceneRegistry};

pub struct SceneBuilder {
    builder: Box<dyn Fn(&mut SceneRegistry, &mut SystemRegistry<ComponentQueryArgs>)>,
}

impl SceneBuilder {
    pub fn new(
        builder: impl Fn(&mut SceneRegistry, &mut SystemRegistry<ComponentQueryArgs>) + 'static,
    ) -> Self {
        Self {
            builder: Box::new(builder),
        }
    }
    pub fn build(&self) -> Scene {
        let mut registry = SceneRegistry::new();
        let mut systems = SystemRegistry::new();
        (self.builder)(&mut registry, &mut systems);
        Scene::new(registry, systems)
    }
}
