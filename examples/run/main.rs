use matrix_engine::engine::{
    plugins::window_plugin::WindowPlugin, runtimes::single_threaded::SingleThreaded, Engine,
    EngineArgs,
};



fn main() {
    let mut engine = <Engine>::new(EngineArgs {
        runtime: Box::new(SingleThreaded),
        startup_runtime: Box::new(SingleThreaded),
    });

    engine.add_scene_plugin(WindowPlugin::new("hello example!"));

    engine.run().unwrap();
}
