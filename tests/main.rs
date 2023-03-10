#![allow(dead_code, unused_imports, unused_variables)]

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
struct A;

impl Component for A {}

impl System for A {
    fn update(&mut self, args: &mut SystemArgs) {
        let ans = args
            .query([Action::Write(TypeId::of::<A>())].into_iter())
            .unwrap();


        ans.finish().unwrap();

        args.stop();

        
    }
}

#[derive(Clone, Debug)]
struct B;
impl System for B {
    fn update(&mut self, args: &mut SystemArgs) {
        let ans = args
        .query([Action::Write(TypeId::of::<A>())].into_iter())
        .unwrap();

    args.stop();
    ans.finish().unwrap();

    }
}

impl Component for B {}

fn main() {
    let start = Instant::now();

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
}
