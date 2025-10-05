use matrix_engine::engine::{
    Engine, SceneRegistry,
    entity::Entity,
    query::{Read, Write},
    runtime::{SingleThreadedRuntime, Stage},
    systems::QuerySystem,
};

fn start(data: &mut Write<usize>) {
    data.insert(&Entity::new(), 5);
}

fn modify(data: &mut Write<usize>) {
    for (_, v) in data.iter_mut() {
        *v += 1;
    }
}

fn prints(data: &mut Read<usize>) {
    for (_, v) in data.iter() {
        println!("{}",v);
    }
    println!("=======");
}

fn main() {
    let mut engine = Engine::new(SingleThreadedRuntime);

    engine.scene_mut().add_system(Stage::Startup, start);
    engine.scene_mut().add_system(Stage::Update, modify);
    engine.scene_mut().add_system(Stage::PostUpdate, prints);

    engine.run();
}
