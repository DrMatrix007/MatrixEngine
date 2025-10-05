use crate::engine::{Scene, SceneRegistry, system_registries::SystemRegistry};

#[derive(Debug, Clone, Copy)]
#[repr(usize)]
pub enum Stage {
    PreUpdate = 0,
    Update,
    PostUpdate,
    PreRender,
    Render,

    Startup,
}

pub static RUNTIME_STAGES: &[Stage] = &[
    Stage::PreUpdate,
    Stage::Update,
    Stage::PostUpdate,
    Stage::PreRender,
    Stage::Render,
];

pub trait Runtime<Registry> {
    fn run(
        &mut self,
        registry: &mut Registry,
        systems: &mut SystemRegistry<Registry>,
        stage: Stage,
    );
}

pub struct SingleThreadedRuntime;

impl<Registry> Runtime<Registry> for SingleThreadedRuntime {
    fn run(&mut self, registry: &mut Registry, systems: &mut SystemRegistry<Registry>, _: Stage) {
        while let Some(mut system) = systems.take_out_system() {
            system.prepare_args(registry).unwrap();
            system.run();
            system.consume_args(registry).unwrap();

            systems.take_back_system(system);
        }
        systems.prepare_next_frame().unwrap();
    }
}

pub struct RuntimeContainer<Registry> {
    runtime: Box<dyn Runtime<Registry>>,
}

impl<Registry> RuntimeContainer<Registry> {
    pub fn new(runtime: impl Runtime<Registry> + 'static) -> Self {
        Self {
            runtime: Box::new(runtime),
        }
    }

    pub fn run(
        &mut self,
        registry: &mut Registry,
        systems: &mut SystemRegistry<Registry>,
        stage: Stage,
    ) {
        self.runtime.run(registry, systems, stage);
    }
}
