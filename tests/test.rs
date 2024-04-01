use matrix_engine::core::{components::Component, engine::Engine, entity::Entity, scene::{Scene, SceneBuilder}};
use winit::window::WindowBuilder;


struct A;



fn main() {
    let scene = SceneBuilder::new(|reg|{
        for _ in 0..100 {
            let e = Entity::new();
            reg.try_set(e,A).unwrap();
        }
    }).build();

    let engine = Engine::new(scene);
    let _window = WindowBuilder::new().build(engine.event_loop()).unwrap();    

    engine.run();
}
