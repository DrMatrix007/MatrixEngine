use matrix_engine::engine::{
    runtime::SingleThreaded,
    scenes::{
        components::Component, entities::entity_builder::EntityBuilder, scene_builder::SceneBuilder,
    },
    systems::{query::components::ReadC, QuerySystem, SystemState},
    Engine,
};
#[derive(Debug)]
struct A;

impl Component for A {}

struct SysA;

impl QuerySystem for SysA {
    type Query = ReadC<A>;

    fn run(&mut self, _args: &mut Self::Query) -> matrix_engine::engine::systems::SystemState {
        SystemState::Continue
    }
}

fn main() {
    let runtime = SingleThreaded::new();
    let engine = Engine::new(runtime, 1);

    let scene_builder = SceneBuilder::new(|reg, _sys| {
        EntityBuilder::new(reg.components_mut()).add(A).unwrap();

        // sys.push_send(SysA);
    });

    engine.run(&scene_builder);
}
