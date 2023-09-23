use matrix_engine::engine::{scenes::scene_builder::SceneBuilder, Engine};

fn main() {
    let engine = Engine::new();

    let scene_builder = SceneBuilder::new(|_| {});

    engine.run(&scene_builder);
}
