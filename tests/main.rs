use matrix_engine::engine::{engine::Engine, scene::SceneBuilder};

fn main() {
    let engine = Engine::new();

    let scene_builder = SceneBuilder::new(|_| {});

    engine.run(&scene_builder);
}
