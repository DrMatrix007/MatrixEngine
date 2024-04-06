use std::sync::RwLock;

use super::{component::{self, ComponentRegistry}, runtimes::Runtime, systems::{Queryable, SystemRegistry}};

pub struct SceneRegistry {
    
    pub components: ComponentRegistry,
}

impl Queryable for SceneRegistry {
    fn components<C: super::component::Component>(&self) -> Option<&std::sync::Arc<tokio::sync::RwLock<super::component::ComponentMap<C>>>> {
        self.components.get()
    }

    fn ensure_isntalled_components<C: super::component::Component>(&mut self) {
        self.components.get_or_insert::<C>();
    }
}

impl SceneRegistry {
    pub fn new(components: ComponentRegistry) -> Self {
        Self { components }
    }
}


pub struct Scene {
    registry: SceneRegistry,
    systems: SystemRegistry<SceneRegistry>,
    runtime: Box<dyn Runtime<SceneRegistry>>   
}

impl Scene {
    pub fn new(runtime: impl Runtime<SceneRegistry>+'static,components:ComponentRegistry) -> Self {
        Self {
            // components: components.into(),
            systems: SystemRegistry::new(),
            registry: SceneRegistry::new(components),
            runtime: Box::new(runtime)
            //      scenes,
        }
    }

    pub(crate) fn update(&mut self)  {

        self.runtime.run(&mut self.systems,&mut self.registry);

    }
    pub fn systems(&mut self) -> &mut SystemRegistry<SceneRegistry> {
        &mut self.systems
    }
}

pub struct SceneBuilder {
    build_components: Box<dyn FnMut(&mut ComponentRegistry)>,
}

impl SceneBuilder {
    pub fn new(build_components: impl FnMut(&mut ComponentRegistry)+'static) -> Self {
        Self { build_components:Box::new(build_components) }
    }

    pub fn build(&mut self,runtime: impl Runtime<SceneRegistry>+'static) -> Scene {
        let mut components = ComponentRegistry::new();
        (self.build_components)(&mut components);
        Scene::new(runtime, components)
    }
}
