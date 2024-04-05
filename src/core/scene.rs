use std::sync::RwLock;

use super::component::ComponentRegistry;


pub struct Scene {
    components: RwLock<ComponentRegistry>,
    // systems: SystemRegistry
    //scenes: SceneRegistry,
}

impl Scene {
    pub fn new(components:ComponentRegistry) -> Self {
        Self {
            components: components.into(),
            // systems: SystemRegistry::new(),
            //      scenes,
        }
    }
    pub fn components(&mut self) -> &mut RwLock<ComponentRegistry> {
        &mut self.components
    }

    pub(crate) fn update(&mut self)  {



    }
}

pub struct SceneBuilder {
    build_components: Box<dyn FnMut(&mut ComponentRegistry)>,
}

impl SceneBuilder {
    pub fn new(build_components: impl FnMut(&mut ComponentRegistry)+'static) -> Self {
        Self { build_components:Box::new(build_components) }
    }

    pub fn build(&mut self) -> Scene {
        let mut components = ComponentRegistry::new();
        (self.build_components)(&mut components);
        Scene::new(components)
    }
}
