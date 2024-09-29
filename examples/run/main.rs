use matrix_engine::engine::{
    events::{MatrixEvent, MatrixEventable},
    plugins::{window_plugin::WindowPlugin, Plugin},
    query::{ReadE, ReadSystemID, WriteE},
    runtimes::single_threaded::SingleThreaded,
    Engine, EngineArgs,
};
use winit::keyboard::KeyCode;

struct Example1;
impl<CustomEvents: MatrixEventable> Plugin<CustomEvents> for Example1 {
    fn build(&self, scene: &mut matrix_engine::engine::scene::Scene<CustomEvents>) {
        let mut i = 0;
        scene.add_send_system(
            move |(events, write_events, id): &mut (ReadE, WriteE<CustomEvents>, ReadSystemID)| {
                println!("{}", i);
                i += 1;

                if events.is_just_pressed(KeyCode::KeyW) {
                    write_events.send(MatrixEvent::DestroySystem(**id)).unwrap();
                }
            },
        );
    }
}

fn main() {
    let mut engine = <Engine>::new(EngineArgs {
        runtime: Box::new(SingleThreaded),
        startup_runtime: Box::new(SingleThreaded),
    });

    engine.add_scene_plugin(WindowPlugin::new("hello example!"));
    // engine.add_scene_plugin(Example1);

    engine.run().unwrap();
}
