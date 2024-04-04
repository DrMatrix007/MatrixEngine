use super::{components::ComponentRegistry, read_write_state::RwState, systems::{SystemArgs, SystemRegistry}, window::WindowRegistry};

pub struct Scene {
    components: RwState<ComponentRegistry>,
    systems: SystemRegistry
    //scenes: SceneRegistry,
}

impl Scene {
    pub fn new(components:ComponentRegistry) -> Self {
        Self {
            components: components.into(),
            systems: SystemRegistry::new(),
            //      scenes,
        }
    }
    pub fn components(&mut self) -> &mut RwState<ComponentRegistry> {
        &mut self.components
    }

    pub(crate) fn update(&mut self)  {
        let components= self.components.write().unwrap();
        let args = SystemArgs {
            components
        };

        for system in self.systems.iter_mut() {
            let s = system.read().unwrap();
            
            s.prepare_args(args)

            system.consume_read(s).unwrap();
        }


        self.components.consume_write(args.components).unwrap();
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
        let mut components = ComponentRegistry::default();
        (self.build_components)(&mut components);
        Scene::new(components)
    }
}
