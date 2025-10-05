use crate::engine::system_registries::{Stage, SystemCollection};


pub trait Runtime<Registry> {
    fn run(
        &mut self,
        registry: &mut Registry,
        systems: &mut SystemCollection<Registry>,
        stage: Stage,
    );
}

pub struct SingleThreadedRuntime;

impl<Registry> Runtime<Registry> for SingleThreadedRuntime {
    fn run(&mut self, registry: &mut Registry, systems: &mut SystemCollection<Registry>, _: Stage) {
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
        systems: &mut SystemCollection<Registry>,
        stage: Stage,
    ) {
        self.runtime.run(registry, systems, stage);
    }
}
