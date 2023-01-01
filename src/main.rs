#![allow(dead_code)]

extern crate lazy_static;

use matrix_engine::{components::Component, systems::System};
use std::time:: Instant;

use crate::matrix_engine::{application::Application, components::Entity};

pub mod matrix_engine;

struct A;

impl Component for A {}

struct B;

struct C(i64);

impl System for B {
    fn update(&mut self, _: matrix_engine::systems::SystemArgs) {}
}

impl System for C {
    fn update(&mut self, args: matrix_engine::systems::SystemArgs) {
        self.0 += 1;
        if self.0 == 1000 {
            args.stop();
        }
    }
}


fn main() {
    let start = Instant::now();
    let mut app = Application::default();
    app.mod_registry(|reg| {
        for _ in 0..10 {
            let e = Entity::default();

            reg.insert(e, A {}).unwrap();
        }

        reg.insert_system(B {});
        reg.insert_system(C(0));
    });
    app.run();
    println!("Hello, world!, {:?}",Instant::now()-start);
}
