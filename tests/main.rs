use matrix_engine::engine::{runtime::SingleThreaded, scenes::scene_builder::SceneBuilder, Engine};

fn main() {
    let runtime = SingleThreaded;
    let engine = Engine::new(runtime);

    let scene_builder = SceneBuilder::new(|_| {});

    engine.run(&scene_builder);
}
