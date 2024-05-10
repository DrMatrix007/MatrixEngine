use std::collections::{BTreeMap, VecDeque};

use super::{
    component::ComponentRegistry,
    plugins::Plugin,
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

    fn ensure_isntalled_resource<R: resources::Resource>(&mut self) {
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
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SystemOrdering {
    Events = 0,
    Logic = 1,
    Last = 2,
}

pub struct Scene {
    registry: SceneRegistry,
    ordered_systems: BTreeMap<SystemOrdering, SystemRegistry<SceneRegistry>>,
    runtime: Box<dyn Runtime<SceneRegistry>>,
    startup_systems: SystemRegistry<SceneRegistry>,
    plugins: VecDeque<Box<dyn Plugin>>,
}

impl Scene {
    pub fn new(
        runtime: impl Runtime<SceneRegistry> + 'static,
        components: ComponentRegistry,
        resources: ResourceRegistry,
    ) -> Self {
        Self {
            // components: components.into(),
            ordered_systems: BTreeMap::new(),
            startup_systems: SystemRegistry::new(),
            registry: SceneRegistry::new(components, resources),
            runtime: Box::new(runtime),
            plugins: VecDeque::new(),
        }
    }

    pub(crate) fn update(&mut self) {
        for systems in &mut self.ordered_systems.values_mut() {
            self.runtime.run(systems, &mut self.registry);
        }
    }
    pub fn add_logic_system<Q: Query, Marker>(
        &mut self,
        sys: impl IntoSystem<Marker, SceneRegistry, Q>,
    ) {
        self.add_system(SystemOrdering::Logic, sys);
    }
    pub fn add_system<Q: Query, Marker>(
        &mut self,
        order: SystemOrdering,
        sys: impl IntoSystem<Marker, SceneRegistry, Q>,
    ) {
        let sys = sys.into_system();
        sys.ensure_installed(&mut self.registry);
        self.ordered_systems.entry(order).or_default().add(sys);
    }
    pub fn add_startup_system<Q: Query, Marker>(
        &mut self,
        sys: impl IntoSystem<Marker, SceneRegistry, Q>,
    ) {
        let sys = sys.into_system();
        sys.ensure_installed(&mut self.registry);
        self.startup_systems.add(sys);
    }

    pub(crate) fn registry(&self) -> &SceneRegistry {
        &self.registry
    }
    pub(crate) fn registry_mut(&mut self) -> &mut SceneRegistry {
        &mut self.registry
    }

    pub fn build_plugin(&mut self, p: impl Plugin + 'static) {
        p.build(self);
    }
}

type BuildFn = Box<dyn FnOnce(&mut ComponentRegistry, &mut ResourceRegistry)>;

pub struct SceneBuilder {
    build_components: BuildFn,
}

impl SceneBuilder {
    pub fn new(
        build_components: impl FnOnce(&mut ComponentRegistry, &mut ResourceRegistry) + 'static,
    ) -> Self {
        Self {
            build_components: Box::new(build_components),
        }
    }

    pub fn build(self, runtime: impl Runtime<SceneRegistry> + 'static) -> Scene {
        let mut components = ComponentRegistry::new();
        let mut resources = ResourceRegistry::new();
        (self.build_components)(&mut components, &mut resources);
        Scene::new(runtime, components, resources)
    }
}
