use matrix_engine::core::{engine::Engine, entity::Entity, scene::SceneBuilder, window::Window};


struct A;

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    let window = Window::new(&mut glfw, (1000, 500), "nice").unwrap();


    let scene = SceneBuilder::new(|reg| {
        for _ in 0..100 {
            let e = Entity::new();
            reg.set(e, A);
        }
    })
    .build();

    let mut engine = Engine::new(scene);

    engine.run();
}
