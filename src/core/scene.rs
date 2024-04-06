use super::{
    component::ComponentRegistry,
    resources::{self, ResourceRegistry},
    runtimes::Runtime,
    systems::{IntoSystem, Query, Queryable, System, SystemRegistry},
};

pub struct SceneRegistry {
    pub components: ComponentRegistry,
    pub resources: ResourceRegistry,
}

impl Queryable for SceneRegistry {
    fn components<C: super::component::Component>(
        &self,
    ) -> Option<&std::sync::Arc<tokio::sync::RwLock<super::component::ComponentMap<C>>>> {
        self.components.get()
    }

    fn ensure_isntalled_components<C: super::component::Component>(&mut self) {
        self.components.get_or_insert::<C>();
    }

    fn resource<R: resources::Resource>(
        &self,
    ) -> Option<&std::sync::Arc<tokio::sync::RwLock<resources::ResourceHolder<R>>>> {
        self.resources.get()
    }

    fn ensure_isntalled_resource<R:resources::Resource>(&mut self) {
        self.resources.get_or_insert::<R>();
    }
}

impl SceneRegistry {
    pub fn new(components: ComponentRegistry, resources: ResourceRegistry) -> Self {
        Self {
            components,
            resources,
        }
    }
}

pub struct Scene {
    registry: SceneRegistry,
    systems: SystemRegistry<SceneRegistry>,
    runtime: Box<dyn Runtime<SceneRegistry>>,
}

impl Scene {
    pub fn new(
        runtime: impl Runtime<SceneRegistry> + 'static,
        components: ComponentRegistry,
        resources: ResourceRegistry,
    ) -> Self {
        Self {
            // components: components.into(),
            systems: SystemRegistry::new(),
            registry: SceneRegistry::new(components, resources),
            runtime: Box::new(runtime), //      scenes,
        }
    }

    pub(crate) fn update(&mut self) {
        self.runtime.run(&mut self.systems, &mut self.registry);
    }
    pub fn add_system<Q:Query,Marker>(&mut self, sys: impl IntoSystem<Marker,SceneRegistry,Q>) {
        let sys = sys.into_system();
        sys.ensure_installed(&mut self.registry);
        self.systems.add(sys);
    }
}

pub struct SceneBuilder {
    build_components: Box<dyn FnMut(&mut ComponentRegistry, &mut ResourceRegistry)>,
}

impl SceneBuilder {
    pub fn new(
        build_components: impl FnMut(&mut ComponentRegistry, &mut ResourceRegistry) + 'static,
    ) -> Self {
        Self {
            build_components: Box::new(build_components),
        }
    }

    pub fn build(&mut self, runtime: impl Runtime<SceneRegistry> + 'static) -> Scene {
        let mut components = ComponentRegistry::new();
        let mut resources = ResourceRegistry::new();
        (self.build_components)(&mut components, &mut resources);
        Scene::new(runtime, components, resources)
    }
}
