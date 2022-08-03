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
    counter: u128
}

impl TimerLayer {
    fn new(max: f64) -> Self {
        Self { max,counter:0 }
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
    
    self.counter+=1;
        // {
        //     let mut reg = unwrap_or_return!(_args.write_registry());
        //     let e = reg.create_entity();

        //     let mut v = unwrap_or_return!(reg.write_vec::<i128>());
        //     let sum: i128 = v.len() as i128;
        //     v.push(e, sum);
        // }
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
        println!("counted: {}",self.counter);
    }
}

fn main() {
    let mut app = Application::new();
    app.set_target_fps(Duration::from_secs_f64(1.0 / 10.0));

    app.push_layer(TimerLayer::new(1.0));

    app.run();
}
