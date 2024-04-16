use std::time::Instant;

use matrix_engine::core::{
    engine::Engine,
    entity::Entity,
    runtimes::single_threaded::SingleThreaded,
    scene::SceneBuilder,
    systems::{QueryData, QuerySystem, ReadC},
};

#[derive(Debug)]
struct A;

struct B(Instant);

impl Default for B {
    fn default() -> Self {
        B(Instant::now())
    }
}
impl QuerySystem for B {
    type Query = ReadC<A>;

    fn run(&mut self, args: <Self::Query as matrix_engine::core::systems::Query>::Data<'_>) {
        for i in args.iter() {
            print!("{:?} ", i);
        }
        let now = Instant::now();
        print!("{:?}             \r", now - self.0);
        self.0 = now;
    }
}

fn c(_args: QueryData<ReadC<A>>) {}

fn main() {
    let mut scene = SceneBuilder::new(move |reg, _res| {
        for _ in 0..2 {
            let e = Entity::new();
            reg.set(e, A);
        }
    })
    .build(SingleThreaded);

    scene.add_logic_system(B::default());
    scene.add_logic_system(c);
    // scene.add_system(|arg:QueryData<ReadC<A>>|{println!("{:?}",arg)});
    let engine = Engine::new(scene);

    engine.run();
}
