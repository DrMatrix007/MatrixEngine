use matrix_engine::{
    components::Component, engine::EngineBuilder, entity::Entity, systems::System,
};
#[derive(Debug)]
struct A(pub i32);

impl Component for A {}

struct C;

impl System for C {
    fn update(&mut self, args: &mut matrix_engine::systems::SystemArgs) {
        let mut data = args.query::<(&A, &B)>();

        for (_, (a, _)) in data.iter_mut() {
            println!("{:?}", a);
        }
        println!();

        args.submit(data);
        args.stop();
    }
}

#[derive(Debug)]
struct B;

impl Component for B {}

fn main() {
    let engine = EngineBuilder::new()
        .with_fps(2)
        // .with_group([C.to_builder(), C.to_builder()])
        .with_single(C)
        .with_single(C)
        .with_registry_builder(|reg| {
            for i in 0..10 {
                let e = Entity::default();
                reg.insert(e, A(i)).unwrap();
                reg.insert(e, B {}).unwrap();
            }
        })
        .build();

    engine.run();
}
