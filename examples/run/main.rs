use matrix_engine::engine::{
    entity::Entity,
    query::{ReadC, WriteC},
    runtimes::single_threaded::SingleThreaded,
    scene::{NonSendEngineStartupArgs, Scene},
    Engine, EngineArgs,
};
use winit::window::{Window, WindowAttributes};

fn main() {
    let mut scene = Scene::new();

    scene.add_non_send_startup_system(
        |args: &mut NonSendEngineStartupArgs, data: &mut WriteC<Window>| {
            data.insert(
                Entity::new(),
                args.event_loop
                    .create_window(WindowAttributes::default())
                    .unwrap(),
            );
        },
    );

    scene.add_send_system(|data: &mut ReadC<Window>| {
        for (_, w) in data.iter() {
            w.request_redraw();
        }
        println!("update")
    });

    let engine = <Engine>::new(EngineArgs {
        runtime: Box::new(SingleThreaded),
        startup_runtime: Box::new(SingleThreaded),
        scene,
    });

    engine.run().unwrap();
}
