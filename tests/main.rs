use matrix_engine::{
    components::{Component, ComponentCollection},
    engine::{Engine, EngineArgs},
    entity::Entity,
    schedulers::SingleThreadScheduler,
    systems::{StartupSystem, System},
    world::World,
};

#[derive(Debug)]
struct A(pub i128);
impl Component for A {}

#[derive(Debug)]
struct B;
impl Component for B{}

struct D;
impl StartupSystem for D {
    type Query<'a> = &'a mut ComponentCollection<A>;

    fn startup(&mut self, comps: &mut ComponentCollection<A>) {
        println!("{}", comps.iter().count());
    }
}

struct C;
impl System for C {
    type Query<'a> = (&'a mut ComponentCollection<A>,&'a ComponentCollection<B>) ;

    fn update<'a>(&mut self,(a,b):Self::Query<'a>) {
        println!("{:?}",a.iter().next());
        println!("{:?}",b.iter().next());
    }
}

fn main() {
    let mut world = World::default();

    let scene = world.scene_mut();
    let reg = scene.component_registry_mut();
    
    for i in 0..100 {
        reg.insert(Entity::default(), A(i));
    }

    world.add_startup(D {}).add_startup(D {});
    world.add_system(C {}).add_system(C {});
    let mut engine = Engine::new(EngineArgs {
        world,
        scheduler: SingleThreadScheduler,
    });

    engine.run();
}
