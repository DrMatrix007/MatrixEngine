use std::any::TypeId;

use matrix_engine::{
    components::{Component, ComponentRegistryBuilder},
    engine::Engine,
    entity::Entity,
    query::{Action},
    systems::System,
};
#[derive(Debug)]
struct A;

impl Component for A {}

impl System for A {
    fn update(&mut self, args: &mut matrix_engine::systems::SystemArgs) {
        let mut data = args.query(
            [
                Action::Read(TypeId::of::<A>()),
                Action::Read(TypeId::of::<B>()),
            ]
            .into_iter(),
        );

        
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
    let mut engine = Engine::with_registry({
        let mut r = ComponentRegistryBuilder::default();

        for _ in 0..2 {
            let e = Entity::default();
            r.insert(e, A {}).unwrap();
            r.insert(e, B {}).unwrap();
        }

        r.build()
    });

    engine.insert_system(|| Box::new(A));

    engine.run();
}
