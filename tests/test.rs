use std::time::Instant;

use matrix_engine::core::{
    engine::Engine,
    entity::Entity,
    runtimes::single_threaded::SingleThreaded,
    scene::SceneBuilder,
    systems::{QuerySystem, ReadC},
    window::Window,
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
        print!("{:?}             \r",now-self.0);
        self.0 = now;
    }
}

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    let _window = Window::new(&mut glfw, (1000, 500), "nice").unwrap();

    let mut scene = SceneBuilder::new(|reg| {
        for _ in 0..2 {
            let e = Entity::new();
            reg.set(e, A);
        }
    })
    .build(SingleThreaded);

    scene.add_system(B::default());
    
    let engine = Engine::new(scene);

    engine.run();
}
