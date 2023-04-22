use std::time::Duration;

use matrix_engine::{
    components::{
        components::{Component, ComponentCollection, ComponentRegistry},
        resources::{Resource, ResourceHolder},
    },
    dispatchers::{
        dispatchers::RegistryData,
        systems::{AsyncSystem, ExclusiveSystem, SystemArgs},
    },
    engine::{Engine, EngineArgs},
    entity::Entity,
    schedulers::multi_threaded_scheduler::MultiThreadedScheduler,
    world::World,
};

#[derive(Debug)]
struct A(pub i128);
impl Component for A {}

#[derive(Debug)]
struct B;
impl Component for B {}

struct D;
impl<'a> AsyncSystem<'a> for D {
    type Query = (&'a ComponentCollection<A>, &'a ResourceHolder<Data>);

    fn run(&mut self, _args: &SystemArgs, (_a, b): <Self as AsyncSystem<'a>>::Query) {
        let _b = b.get().unwrap();
        println!("start D");
        spin_sleep::sleep(Duration::from_secs_f64(1.0));
        println!("end D");
    }
}

struct C;
impl<'a> AsyncSystem<'a> for C {
    type Query = (&'a ComponentCollection<A>, &'a ResourceHolder<Data>);

    fn run(&mut self, args: &SystemArgs, (_a, b): <Self as AsyncSystem<'a>>::Query) {
        let b = b.get().unwrap();
        println!("start C");

        spin_sleep::sleep(Duration::from_secs_f64(1.0));
        //println!("DATA: {}", b.0);
        println!("end C");
        if b.0 > 15 {
            args.stop();
        }
    }
}
struct E;
impl<'a> AsyncSystem<'a> for E {
    type Query = (&'a mut ComponentCollection<A>, &'a mut ResourceHolder<Data>);

    fn run(&mut self, _args: &SystemArgs, (_a, b): <Self as AsyncSystem<'a>>::Query) {
        let b = b.get_mut().unwrap();
        println!("start E");
        b.0 += 1;
        println!("end E");
    }
}

struct Data(pub i32);
impl Resource for Data {}

struct Test(*const ());

impl<'a> ExclusiveSystem<'a> for Test {
    type Query = RegistryData<'a>;

    fn run(&mut self, _: &SystemArgs, q: <Self as ExclusiveSystem<'a>>::Query) {
        println!("start ex");
        unsafe { q.components.get_ptr::<A>() };
        spin_sleep::sleep(Duration::from_secs_f64(1.0));
        println!("end ex");
    }
}

struct Other;

impl<'a> AsyncSystem<'a> for Other {
    type Query = RegistryData<'a>;

    fn run(&mut self, _: &SystemArgs, _: <Self as AsyncSystem<'a>>::Query) {
        println!("start other");
        spin_sleep::sleep(Duration::from_secs_f64(3.0));
        println!("end other");
    }
}

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
    world
        .add_system(C {})
        .add_system(E {})
        .add_system(D {})
        .add_system(Other {})
        .add_exclusive_system(Test(std::ptr::null()));
    let mut engine = Engine::new(EngineArgs {
        fps: 1,
        world,
        scheduler: MultiThreadedScheduler::with_amount_of_cpu_cores().unwrap(),
    });
    engine.run();
}
