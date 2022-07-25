#![allow(dead_code)]

mod matrix_engine;
pub mod tests;
use matrix_engine::{application::*, event::Event, layer::*};
#[derive(Debug)]
struct Test;

struct MyLayer {}
impl MyLayer {
    fn new() -> Self {
        MyLayer {}
    }
}
impl Layer for MyLayer {
    fn on_start(&mut self, _args: &LayerArgs) {
        let registry = _args.registry.lock();
        if let Ok(mut registry) = registry {
            let mut e = registry.create_entity();
            for _ in 1..100 {
                e = registry.create_entity();

                registry.insert_component(e, Test {});
            }
            println!("{:?}", registry.borrow_component_mut::<Test>(e));
        }
    }

    fn on_update(&mut self, mut _args: &LayerArgs) {
        // print!("{}\n", (_args.delta_time.as_secs_f64()));
    }

    fn on_clean_up(&mut self) {}
}
struct MyEvent;
impl Event for MyEvent {}

fn main() {
    let mut app = Application::new();

    {
        let layer = MyLayer::new();
        app.push_layer(layer);

        // app.push_layer(FpsLayer::new());
    }

    app.run();
}
