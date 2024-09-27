use matrix_engine::engine::{scene::Scene, Engine};

fn main() {
    let engine = <Engine>::with_scene(Scene::new());

    engine.run().unwrap();
}
