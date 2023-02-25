#![allow(dead_code, unused_imports,unused_variables)]

pub mod matrix_engine;
use std::{time::Instant, sync::Mutex};

use matrix_engine::{components::Component, systems::System};

use crate::matrix_engine::{registry::{Registry, RegistryBuilder}, runtime::Runtime, systems::SystemCreator, entity::Entity};

#[derive(Debug)]
struct A;

impl Component for A {}

impl System for A {
    fn update(&mut self, args: matrix_engine::systems::SystemArgs) {

        query!(&args,|read a: A| {
            println!("{:?}",a);
        });

        args.stop();
    }
}

#[derive(Debug)]
struct B;

impl Component for B{}

fn main() {
    let start = Instant::now();

    let mut runtime = Runtime::with_registry({
        let mut r = RegistryBuilder::default();
        
        r.components.insert(Entity::default(),A).unwrap();
        r.components.insert(Entity::default(),A).unwrap();
        r.components.insert(Entity::default(),A).unwrap();
        

        r.build()
    });

    runtime.insert(SystemCreator::with_function(|| Box::new(A)));

    runtime.run();

    println!("Hello, world!, {:?}", Instant::now() - start);
}
