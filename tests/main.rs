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
struct A(pub i32);

impl Component for A {}

impl System for A {
    fn update(&mut self, args: &mut SystemArgs) {
        let mut ans = args
            .query([Action::Write(TypeId::of::<A>())].into_iter())
            .unwrap();

        // for (e, i) in ans.iter_mut::<A>().unwrap() {
        //     i.0 = self.0;
        //     println!("changed!");
        // }

        ans.finish().unwrap();

        args.stop();
    }
}

#[derive(Clone, Debug)]
struct B;
impl System for B {
    fn update(&mut self, args: &mut SystemArgs) {
        let ans = args
            .query([Action::Read(TypeId::of::<A>())].into_iter())
            .unwrap();
        // for (e, i) in ans.iter_ref::<A>().unwrap() {
        //     println!("{}", i.0);
        // }

        args.stop();
        ans.finish().unwrap();
    }
}

impl Component for B {}

fn main() {
    let start = Instant::now();

    let mut runtime = Runtime::with_registry({
        let mut r = RegistryBuilder::default();
        r.insert(Entity::default(), A(5)).unwrap();
        r.insert(Entity::default(), A(7)).unwrap();
        r.insert(Entity::default(), A(8)).unwrap();

        r.build()
    });

    runtime.insert_system(SystemCreator::with_function(|| Box::new(A(6))));
    runtime.insert_system(SystemCreator::with_function(|| Box::new(B)));

    runtime.run();

    println!("Hello, world!, {:?}", Instant::now() - start);
}
