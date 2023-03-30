use std::sync::Arc;

use matrix_engine::{
    components::{Component, ComponentCollection, ComponentCollectionRef, ComponentRegistry},
    dispatchers::ReadColl,
    engine::{Engine, EngineArgs},
    scene::Scene,
    systems::StartupSystem, entity::Entity,
};

struct A(pub i128);
impl Component for A {}

struct B {}
impl StartupSystem for B {
    type Query = ReadColl<A>;

    fn startup(
        &mut self,
        comps: <Self::Query as matrix_engine::dispatchers::DispatchData>::Target<'_>,
    ) {
        println!("{}",comps.iter().count());
    }
}

fn main() {
    let mut scene = Scene::default();
    let reg = scene.get_component_registry();

    for i in 0..100 {
        reg.insert(Entity::default(), A(i));
    }

    scene.add_startup(B {});

    let mut engine = Engine::new(EngineArgs {
        scene,
        thread_count: None,
    });

    engine.run();
}
