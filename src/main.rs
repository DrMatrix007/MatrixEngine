#![allow(dead_code)]

mod matrix_engine;
use matrix_engine::{application::*, event::Event, layer::*};
#[derive(Debug)]
struct Test;

pub mod tests;
struct WorkLayer {}

struct TimerLayer {
    max: f64,
}

impl TimerLayer {
    fn new(max: f64) -> Self {
        Self { max }
    }
}
impl Layer for TimerLayer {
    fn on_start(&mut self, _args: &LayerArgs) {
        if let Some(mut registry) = _args.write_registry() {
            let mut e = registry.create_entity();
            for _ in 1..100 {
                e = registry.create_entity();

                registry.insert_component(e, Test {});
            }
            println!("{:?}", registry.borrow_component_mut::<Test>(e));
        }
        println!("?");
    }

    fn on_update(&mut self, mut _args: &LayerArgs) {
        print!("{}\n", (_args.time.elapsed().as_secs_f64()));

    }

    fn on_clean_up(&mut self) {}
}
fn main() {
    let mut app = Application::new();
    app.set_target_fps(144);



    app.run();
}
