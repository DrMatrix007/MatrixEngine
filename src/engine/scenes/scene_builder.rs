use super::{Scene, SceneRegistry};

pub struct SceneBuilder {
    builder: Box<dyn Fn(&mut SceneRegistry)>,
}

impl SceneBuilder {
    pub fn new(builder: impl Fn(&mut SceneRegistry) + 'static) -> Self {
        Self {
            builder: Box::new(builder),
        }
    }
    pub fn build(&self) -> Scene {
        let mut registry = SceneRegistry::new();
        (self.builder)(&mut registry);
        Scene::new(registry)
    }
}
