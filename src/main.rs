#![allow(dead_code)]


mod matrix_engine;
use matrix_engine::{application::*, layer::*};
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
        println!("started!");
    }

    fn on_update(&mut self, _args: &LayerArgs) {
        if _args.time.elapsed().as_secs_f64() > self.max {

            println!("frame: {}", _args.time.elapsed().as_secs_f64());
            _args.stop_application();
        }
    }

    fn on_clean_up(&mut self) {}
}
fn main() {
    let mut app = Application::new();
    app.set_target_fps(144);

    app.push_layer(TimerLayer::new(0.1));
    app.push_layer(TimerLayer::new(0.1));
    app.push_layer(TimerLayer::new(0.1));
    app.push_layer(TimerLayer::new(0.1));

    app.run();
}
