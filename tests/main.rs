use std::time::Duration;

use matrix_engine::{components::{components::{Component, ComponentCollection}, resources::{Resource, ResourceHolder}}, dispatchers::systems::System, world::World, entity::Entity, engine::{Engine, EngineArgs}, schedulers::schedulers::MultiThreadedScheduler
};

#[derive(Debug)]
struct A(pub i128);
impl Component for A {}

#[derive(Debug)]
struct B;
impl Component for B {}

struct D;
impl<'a> System<'a> for D {
    type Query = (&'a ComponentCollection<A>,&'a ResourceHolder<Data>);

    fn run(&mut self, (_a,b): Self::Query) {
        let b = b.get().unwrap();
        println!("start D");
        println!("DATA: {}",b.0);
        spin_sleep::sleep(Duration::new(2, 0));
        println!("end D");
    }
}

struct C;
impl<'a> System<'a> for C {
    type Query = (&'a ComponentCollection<A>,&'a ResourceHolder<Data>);

    fn run(&mut self, (_a,b): Self::Query) {
        let b = b.get().unwrap();
        println!("start C");
        println!("DATA: {}",b.0);

        spin_sleep::sleep(Duration::new(2, 0));
        println!("end C");
    }
}
struct E;
impl<'a> System<'a> for E {
    type Query = (&'a mut ComponentCollection<A>,&'a mut ResourceHolder<Data>);

    fn run(&mut self, (_a,b): Self::Query) {
        let b=  b.get_mut().unwrap();
        println!("start E");
        b.0+=1;
        spin_sleep::sleep(Duration::new(2, 0));
        println!("end E");
    }
}

struct Data(pub i32);
impl Resource for Data {}

fn main() {
    let mut world = World::default();

    let scene = world.scene_mut();
    let reg = scene.component_registry_mut();

    for i in 0..100 {
        reg.insert(Entity::default(), A(i));
    }

    let resources = world.resource_registry_mut();

    resources.insert(Data(10));

    // world.add_startup(D {}).add_startup(D {});
    world.add_system(C {}).add_system(E {}).add_system(D {});
    let mut engine = Engine::new(EngineArgs {
        world,
        scheduler: MultiThreadedScheduler::with_amount_of_cores().unwrap(),
    });

    engine.run();
}
