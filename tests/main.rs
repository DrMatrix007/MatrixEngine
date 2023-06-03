use matrix_engine::{
    components::component::Component,
    dispatchers::dispatcher::components::{ReadComponents, WriteComponents},
    engine::Engine,
    schedulers::multi_threaded_scheduler::MultiThreadedScheduler,
};

#[derive(Debug)]
struct A;
impl Component for A {}
fn main() {
    let engine = Engine::new(matrix_engine::engine::EngineArgs {
        scheduler: MultiThreadedScheduler::with_amount_of_cpu_cores().unwrap(),
        fps: 60,
    });

    let mut scene = engine.create_scene();

    for _ in 0..10 {
        scene.create_entity().add(A).unwrap();
    }

    scene
        .add_async_system(|_: &mut WriteComponents<A>| {
            println!("1");
        })
        .add_async_system(|_: &mut ReadComponents<A>| {
            println!("2");
        });
    engine.run(scene);
}
