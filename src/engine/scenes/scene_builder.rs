

use super::Scene;

pub struct SceneBuilder {
    builder: Box<dyn Fn(&mut Scene)>,
}

impl SceneBuilder {
    pub fn new(builder: impl Fn(&mut Scene) + 'static) -> Self {
        Self {
            builder: Box::new(builder),
        }
    }
    pub fn build(&self) -> Scene {
        let mut scene = Scene::new();
        (self.builder)(&mut scene);
        scene
    }
}

