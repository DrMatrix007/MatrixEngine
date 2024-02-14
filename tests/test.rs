use matrix_engine::core::engine::Engine;
use winit::window::WindowBuilder;

fn main() {
    let engine = Engine::new();

    let window = WindowBuilder::new().build(engine.event_loop()).unwrap();

    engine.run();
}

