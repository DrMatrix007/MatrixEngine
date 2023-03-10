#![allow(dead_code, unused_imports, unused_variables)]

<<<<<<< HEAD
use std::{sync::Mutex, time::Instant};

use matrix_engine::systems::SystemArgs;
use matrix_engine::{components::Component, systems::System};

use matrix_engine::query;
use matrix_engine::{entity::Entity, registry::Registry, runtime::Runtime};

#[derive(Debug)]
=======
use std::{any::TypeId, sync::Mutex, time::Instant};

use matrix_engine::{
    components::{Component, ComponentCollection},
    entity::Entity,
    queries::query::Action,
    registry::{Registry, RegistryBuilder},
    runtime::Runtime,
    systems::{System, SystemArgs, SystemCreator},
};

#[derive(Debug, Clone)]
>>>>>>> temp
struct A;

impl Component for A {}

impl System for A {
    fn update(&mut self, args: &mut SystemArgs) {
<<<<<<< HEAD
        let data = query!(args.components(),read A);
        for (e, a) in data.iter() {
            println!("{:?} {:?}",e,a);
        }
        args.stop();
    }
    fn setup(&mut self, args: &mut SystemArgs) {
        println!("nice");
    }
}

#[derive(Debug)]
struct B;
impl System for B {
    fn update(&mut self, args: &mut SystemArgs) {
        let data = query!(args.components(),read A);

        args.stop();
    }
=======
        let ans = args
            .query([Action::Read(TypeId::of::<A>())].into_iter())
            .unwrap();

        args.stop();

        println!(
            "nice! {}",
            match ans {
                matrix_engine::queries::query::QueryResult::Ok { data } => format!(
                    "{}",
                    data.data
                        .iter()
                        .next()
                        .unwrap()
                        .1
                        .unpack_ref()
                        .as_any()
                        .downcast_ref::<ComponentCollection<A>>()
                        .unwrap()
                        .iter()
                        .count()
                ),
                matrix_engine::queries::query::QueryResult::Empty => "empty".to_owned(),
            }
        );
    }
}

#[derive(Clone, Debug)]
struct B;
impl System for B {
    fn update(&mut self, args: &mut SystemArgs) {}
>>>>>>> temp
}

impl Component for B {}

fn main() {
    let start = Instant::now();

<<<<<<< HEAD
    let mut runtime = Runtime::new({
        let mut reg = Registry::default();
        let comps = reg.get_component_registry_mut();
        for _ in 0..3 {
            comps.insert(Entity::default(), A).unwrap();
        }
        reg
    });


    runtime.insert_system(A {});
    runtime.insert_system(B {});

    runtime.run();
=======
    let mut runtime = Runtime::with_registry({
        let mut r = RegistryBuilder::default();
        r.insert(Entity::default(), A).unwrap();
        r.insert(Entity::default(), A).unwrap();
        r.insert(Entity::default(), A).unwrap();

        r.build()
    });

    runtime.insert_system(SystemCreator::with_function(|| Box::new(A)));
    runtime.insert_system(SystemCreator::with_function(|| Box::new(B)));

    runtime.run();

    println!("Hello, world!, {:?}", Instant::now() - start);
>>>>>>> temp
}
