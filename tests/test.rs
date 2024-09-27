use matrix_engine::engine::{scene::Scene, Engine};
use winit::window::{Window, WindowAttributes};

fn main() {
    let engine = <Engine>::with_scene(Scene::new());

    

    engine.run().unwrap();
}
