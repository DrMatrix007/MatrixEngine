#![allow(dead_code, unused_imports, unused_variables)]

use std::{sync::Mutex, time::Instant};

use matrix_engine::systems::SystemArgs;
use matrix_engine::{components::Component, systems::System};

use matrix_engine::query;
use matrix_engine::{entity::Entity, registry::Registry, runtime::Runtime, systems::SystemCreator};

#[derive(Debug)]
struct A;

impl Component for A {}

impl System for A {
    fn update(&mut self, mut args: SystemArgs) {
        query!(args,|read a: A| {
            println!("a: {:?}",a);
        });

        args.stop();
    }
}

#[derive(Debug)]
struct B;
impl System for B {
    fn update(&mut self, mut args: SystemArgs) {
        query!(args,|read a: A| {
            println!("b: {:?}",a);
        });

        args.stop();
    }
}

impl Component for B {}

fn main() {
    let start = Instant::now();

    let mut runtime = Runtime::new(
        {
            let mut reg = Registry::default();
            let comps = reg.get_component_registry_mut();
            for _ in 0..10 {
                comps.insert(Entity::default(), A).unwrap();
            }
            reg
        },
        1000000,
    );

    runtime.insert_system(A {});
    runtime.insert_system(B {});

    runtime.run();
}
