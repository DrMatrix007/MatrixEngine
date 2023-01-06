#![allow(dead_code)]

extern crate lazy_static;

use matrix_engine::{components::Component, systems::System};
use std::time::Instant;

use crate::matrix_engine::{
    application::Application, components::Entity, renderer::window_system::WindowSystem,
};

pub mod matrix_engine;

struct A(pub i64);
impl Component for A {}

struct B(i64);
impl Component for B {}

struct SystemB;

struct SystemC(i64);

impl System for SystemB {
    fn update(&mut self, args: matrix_engine::systems::SystemArgs) {
        if let Some(reg) = args.read_component_registry() {
            query!(reg,|write a:A| {
                println!("nive {}",a.0);
            });
            query!(reg,|write a:A| {

                println!("bruh {}",a.0);
            }, |a,b|{
                a.1.0.cmp(&b.1.0)
            });
            args.stop();
        }

    }
}

impl System for SystemC {
    fn update(&mut self, _: matrix_engine::systems::SystemArgs) {}
}

fn main() {
    let start = Instant::now();
    let mut app = Application::default();
    app.mod_registry(|reg| {
        for i in 0..10 {
            let e = Entity::default();

            reg.insert(e, A(i)).unwrap();
            reg.insert(e, B(0)).unwrap();
        }

        reg.insert_system(SystemB {});
        reg.insert_system(SystemC(0));
        reg.insert_system(WindowSystem::default());
    });
    app.run();
    println!("Hello, world!, {:?}", Instant::now() - start);
}
