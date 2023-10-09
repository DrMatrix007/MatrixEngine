use log::{info, logger};
use matrix_engine::engine::{
    runtime::{MultiThreaded, SingleThreaded},
    scenes::{
        components::Component, entities::entity_builder::EntityBuilder, scene_builder::SceneBuilder,
    },
    systems::{
        query::components::{ReadC, WriteC},
        QuerySystem, SystemControlFlow,
    },
    Engine,
};
use simple_logger::SimpleLogger;
use std::time::Duration;

#[derive(Debug)]
struct A;

impl Component for A {}

struct SysReadA;

impl QuerySystem for SysReadA {
    type Query = ReadC<A>;

    fn run(
        &mut self,
        _args: &mut Self::Query,
    ) -> matrix_engine::engine::systems::SystemControlFlow {
        info!("read A");
        // spin_sleep::sleep(Duration::from_secs(2));

        SystemControlFlow::Continue
    }
}

struct SysWriteA(pub usize);

impl QuerySystem for SysWriteA {
    type Query = WriteC<A>;

    fn run(&mut self, args: &mut Self::Query) -> SystemControlFlow {
        info!("start write A {}", self.0);
        // spin_sleep::sleep(Duration::from_secs(2));
        info!("end   write A {}", self.0);

        SystemControlFlow::Continue
    }
}

fn main() {
    SimpleLogger::new().init().unwrap();
    let runtime = MultiThreaded::new();
    let engine = Engine::new(runtime, 1);

    let scene_builder = SceneBuilder::new(|reg, sys| {
        EntityBuilder::new(reg.components_mut()).add(A).unwrap();

        // sys.push_send(SysReadA);
        sys.push_send(SysWriteA(0));
        sys.push_send(SysWriteA(1));
        sys.push_send(SysWriteA(2));
    });

    engine.run(&scene_builder);
}
