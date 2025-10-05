use matrix_engine::engine::{
    component::Component, entity::Entity, query::{Read, Write}, runtime::SingleThreadedRuntime, system_registries::Stage, Engine
};

#[derive(Debug)]
struct A;

impl Component for A {}

fn start(data: &mut Write<A>) {
}

fn modify(data: &mut Write<A>) {

}

fn prints(data: &mut Read<A>) {
    for (_, v) in data.iter() {
        println!("{:?}", v);
    }
    println!("=======");
}

fn main() {
    let mut engine = Engine::new(SingleThreadedRuntime);

    engine.scene_mut().add_system(Stage::Startup, start);
    engine.scene_mut().add_system(Stage::Update, modify);
    engine.scene_mut().add_system(Stage::PostUpdate, prints);

    engine.startup();
}
