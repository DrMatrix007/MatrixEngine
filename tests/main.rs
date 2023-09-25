use matrix_engine::engine::{
    runtime::SingleThreaded,
    scenes::{components::Component, entities::Entity, scene_builder::SceneBuilder},
    Engine, systems::{System, SceneSystem, query::{ReadC, WriteC}},
};
#[derive(Debug)]
struct A;

impl Component for A {}


struct SysA(pub i128);

impl SceneSystem for SysA {
    type Query = ReadC<A>;

    fn run(&mut self, args: &mut <Self::Query as matrix_engine::engine::systems::query::Query<matrix_engine::engine::systems::query::ComponentQueryArgs>>::Target) {
        println!("{} ",self.0);
    }
}

fn main() {
    let runtime = SingleThreaded::new();
    let engine = Engine::new(runtime);

    let scene_builder = SceneBuilder::new(|reg, sys| {
        let e = Entity::new();
        reg.components_mut().try_add_component(e, A).unwrap();

        sys.push_send(SysA(1));
        sys.push_send(SysA(2));
        sys.push_send(SysA(3));
    });

    engine.run(&scene_builder);
}
