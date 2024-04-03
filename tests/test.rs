use matrix_engine::core::{
    components::Component,
    engine::Engine,
    entity::Entity,
    scene::{Scene, SceneBuilder},
    window::Window,
};

struct A;

fn main() {
    
    let glfw = glfw::init(glfw::fail_on_errors).unwrap();

    let window = Window::new(glfw,(1000, 500), "nice");

    let scene = SceneBuilder::new(|reg| {
        for _ in 0..100 {
            let e = Entity::new();
            reg.try_set(e, A).unwrap();
        }
    })
    .build();

    let engine = Engine::new(scene);

    engine.run();
}
