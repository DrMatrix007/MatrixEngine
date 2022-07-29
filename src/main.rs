#![allow(dead_code)]

use matrix_engine::matrix_engine::{layer::{LayerArgs, Layer}, application::Application};

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



    app.run();
}
