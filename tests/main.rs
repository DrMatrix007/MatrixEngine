use std::time::Duration;

use matrix_engine::{components::components::{Component, ComponentCollection}, dispatchers::systems::System, world::World, entity::Entity, engine::{Engine, EngineArgs}, schedulers::schedulers::MultiThreadedScheduler
};

#[derive(Debug)]
struct A(pub i128);
impl Component for A {}

#[derive(Debug)]
struct B;
impl Component for B {}

struct D;
impl<'a> System<'a> for D {
    type Query = (&'a ComponentCollection<A>,);

    fn run(&mut self, (_a,): Self::Query) {
        println!("start D");
        spin_sleep::sleep(Duration::new(2, 0));
        println!("end D");
    }
}

struct C;
impl<'a> System<'a> for C {
    type Query = (&'a ComponentCollection<A>,);

    fn run(&mut self, (_a,): Self::Query) {
        println!("start C");
        spin_sleep::sleep(Duration::new(2, 0));
        println!("end C");
    }
}
struct E;
impl<'a> System<'a> for E {
    type Query = (&'a mut ComponentCollection<A>,);

    fn run(&mut self, (_a,): Self::Query) {
        println!("start E");
        spin_sleep::sleep(Duration::new(2, 0));
        println!("end E");
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
    world.add_system(C {}).add_system(E {}).add_system(D {});
    let mut engine = Engine::new(EngineArgs {
        world,
        scheduler: MultiThreadedScheduler::with_amount_of_cores().unwrap(),
    });

    engine.run();
}
