use std::{thread, time::Duration};

use matrix_engine::{
    engine::{
        entity::Entity,
        events::MatrixEventable,
        plugins::{window_plugin::WindowPlugin, Plugin},
        query::{ReadC, WriteC},
        runtimes::multi_threading::MultiThreaded,
        Engine, EngineArgs,
    },
    renderer::{pipelines::models::square::Square, render_object::RenderObject},
};

struct ExamplePlugin;

impl<CustomEvents: MatrixEventable> Plugin<CustomEvents> for ExamplePlugin {
    fn build(&self, scene: &mut matrix_engine::engine::scene::Scene<CustomEvents>) {
        scene.add_send_system(|data: &mut ReadC<u64>| {
            println!("started1");
            thread::sleep(Duration::from_secs(4));
            println!("ended1")
        });
        scene.add_send_system(|data: &mut ReadC<u64>, data2: &mut WriteC<()>| {
            println!("started2");
            thread::sleep(Duration::from_secs(2));
            println!("ended2")
        });
        scene.add_send_system(|data: &mut ReadC<u64>, data2: &mut WriteC<()>| {
            println!("started3");
            thread::sleep(Duration::from_secs(2));
            println!("ended3")
        });

    }
}

fn main() {
    let mut engine = <Engine>::new(EngineArgs::new(
        MultiThreaded::with_cpu_count(),
        MultiThreaded::with_cpu_count(),
    ));

    engine.add_scene_plugin(WindowPlugin::new("multithreaded"));

    engine.add_scene_plugin(ExamplePlugin);

    engine.run().unwrap();
}
