#![allow(dead_code)]

extern crate lazy_static;



use matrix_engine::{components::Component, systems::System};
use std::time::Instant;

use crate::matrix_engine::{application::Application, components::Entity};

pub mod matrix_engine;

struct A(i64);
impl Component for A {}

struct B;
impl Component for B{}

struct SystemB;

struct SystemC(i64);

impl System for SystemB {
    fn update(&mut self, args: matrix_engine::systems::SystemArgs) {
        if let Some(reg) = args.read_component_registry() {
            println!("hmm?");
            query!(reg,|write a:A| {
                println!("bruh {}",a.0);
                a.0+=1;
            });

            query!(reg,|write a:A| {
               
                println!("{}",a.0);
            });
            args.stop();
        }

    }
}


impl System for SystemC {
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
        for i in 0..10 {
            let e = Entity::default();

            reg.insert(e, A(i)).unwrap();
            reg.insert(e, B {}).unwrap();
        }

        reg.insert_system(SystemB {});
        reg.insert_system(SystemC(0));
    });
    app.run();
    println!("Hello, world!, {:?}", Instant::now() - start);
}
