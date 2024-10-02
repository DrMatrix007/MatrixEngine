use std::time::Instant;

use matrix_engine::{
    engine::{
        entity::Entity, events::{MatrixEvent, MatrixEventable}, plugins::{window_plugin::WindowPlugin, Plugin}, query::{ReadE, ReadSystemID, WriteC, WriteE}, runtimes::single_threaded::SingleThreaded, Engine, EngineArgs
    },
    renderer::{pipelines::models::square::Square, render_object::RenderObject, renderer_plugin::RendererPlugin},
};
use winit::keyboard::KeyCode;

struct Example1;

impl<CustomEvents: MatrixEventable> Plugin<CustomEvents> for Example1 {
    fn build(&self, scene: &mut matrix_engine::engine::scene::Scene<CustomEvents>) {
        let mut latest = Instant::now();
        let mut v = Vec::<f32>::new();
        let mut latest_second = Instant::now();
        scene.add_send_system(
            move |(events, write_events, id): &mut (
                ReadE<CustomEvents>,
                WriteE<CustomEvents>,
                ReadSystemID,
            )| {
                let now = Instant::now();

                v.push(1.0 / (now - latest).as_secs_f32());

                if (now - latest_second).as_secs() > 0 {
                    let fps = v.iter().sum::<f32>() / v.len() as f32;
                    println!("fps: {:10.5}, {:10.5}", fps, 1.0 / fps);
                    latest_second = now;
                }
                latest = now;

                if events.is_just_pressed(KeyCode::KeyW) {
                    write_events.send(MatrixEvent::DestroySystem(**id)).unwrap();
                }
            },
        );
        scene.add_send_startup_system(|data: &mut WriteC<RenderObject>| {
            data.insert(
                Entity::new(),
                RenderObject::new(Square, "./img.jpg".to_string()),
            );
        });
    }
}

fn main() {
    let mut engine = <Engine>::new(EngineArgs::new(SingleThreaded, SingleThreaded));

    engine.add_scene_plugin(WindowPlugin::new("hello example!"));

    engine.add_scene_plugin(RendererPlugin);

    engine.add_scene_plugin(Example1);

    engine.run().unwrap();
}
