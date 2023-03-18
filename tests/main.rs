use std::any::TypeId;

use matrix_engine::{
    components::{Component, ComponentRegistryBuilder},
    engine::{Engine, EngineArgs, EngineBuilder},
    entity::Entity,
    query::Action,
    systems::{QueryResultData, System},
};
#[derive(Debug)]
struct A;

impl Component for A {}

impl System for A {
    fn update(&mut self, args: &mut matrix_engine::systems::SystemArgs) {
        let mut data = args.query(
            [
                Action::Write(TypeId::of::<A>()),
                Action::Read(TypeId::of::<B>()),
            ]
            .into_iter(),
        );
        
        // {
        let mut i = QueryResultData::<(&A, &B)>::from(&mut data);

        for it in i.iter_mut() {
            println!("{:?}", it);
        }
        // }
        // println!("started:");
        // for (e, data) in data.iter() {
        //     println!("{e:?} {:?}", data.len());
        // }

        data.finish();

        args.stop();
    }
}

#[derive(Debug)]
struct B;

impl Component for B {}

fn main() {
    let engine = EngineBuilder::new()
        .with_fps(144)
        .with_system(A)
        .with_system(A)
        .with_registry_builder(|reg| {
            for _ in 0..5 {
                let e = Entity::default();
                reg.insert(e, A {}).unwrap();
                reg.insert(e, B {}).unwrap();
            }
        })
        .build();

    engine.run();
}
