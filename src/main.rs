#![allow(dead_code)]

use std::time::Duration;

use matrix_engine::{
    matrix_engine::{
        application::Application,
        layer::{Layer, LayerArgs},
    },
    unwrap_or_return,
};

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
        let mut reg = unwrap_or_return!(_args.write_registry());
        let e = reg.create_entity();

        let mut v = unwrap_or_return!(reg.write_vec::<i128>());
        v.push(e, 1);
    }

    fn on_update(&mut self, _args: &LayerArgs) {
    
        {
            let mut reg = unwrap_or_return!(_args.write_registry());
            let e = reg.create_entity();

            let mut v = unwrap_or_return!(reg.write_vec::<i128>());
            let sum: i128 = v.len() as i128;
            v.push(e, sum);
        }
        if _args.time.elapsed().as_secs_f64() > self.max {
            println!("frame: {}", _args.time.elapsed().as_secs_f64());
            _args.stop_application();

            let r = unwrap_or_return!(_args.read_registry());
            let v = unwrap_or_return!(r.read_vec::<i128>());
            println!("len: {}", v.len());
            for (_, _) in v.iter() {
                // println!("{}: {}", e, i);
            }
        }
    }

    fn on_clean_up(&mut self) {
    }
}

fn main() {
    let mut app = Application::new();
    app.set_target_fps(100);

    app.push_layer(TimerLayer::new(1.0));

    app.run();
}
