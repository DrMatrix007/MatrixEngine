use std::time::Duration;

use matrix_engine::{
    components::{Component, ComponentCollection},
    engine::{Engine, EngineArgs},
    entity::Entity,
    schedulers::MultiThreadedScheduler,
    systems::System,
    world::World,
};

#[derive(Debug)]
struct A(pub i128);
impl Component for A {}

#[derive(Debug)]
struct B;
impl Component for B {}

struct D;
impl System for D {
    type Query<'a> = &'a mut ComponentCollection<A>;

    fn run(&mut self, _comps: &mut ComponentCollection<A>) {
        println!("start D");
        spin_sleep::sleep(Duration::new(2, 0));
        println!("end D");
    }
}

struct C;
impl System for C {
    type Query<'a> = (&'a mut ComponentCollection<A>,&'a ComponentCollection<B>,);

    fn run<'a>(&mut self, (_a,_b,): Self::Query<'a>) {
        println!("start C");
        spin_sleep::sleep(Duration::new(2, 0));
        println!("end C");
    }
}

fn main() {
    let mut world = World::default();

    let scene = world.scene_mut();
    let reg = scene.component_registry_mut();

    for i in 0..100 {
        reg.insert(Entity::default(), A(i));
    }

    // world.add_startup(D {}).add_startup(D {});
    world.add_system(C {}).add_system(D {});
    let mut engine = Engine::new(EngineArgs {
        world,
        scheduler: MultiThreadedScheduler::new(2),
    });

    engine.run();
}
