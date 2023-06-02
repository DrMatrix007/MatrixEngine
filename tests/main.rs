use matrix_engine::{
    dispatchers::{context::Context, systems::ExclusiveSystem}, engine::Engine,
    schedulers::multi_threaded_scheduler::MultiThreadedScheduler,
};

struct Create;

impl ExclusiveSystem for Create {
    type Query=();

    fn run(&mut self, _ctx: &Context, _comps: &mut Self::Query) {
        println!("created!");
    }
}

fn main() {
    let engine = Engine::new(matrix_engine::engine::EngineArgs {
        scheduler: MultiThreadedScheduler::with_amount_of_cpu_cores().unwrap(),
        fps: 60,
    });

    let mut scene = engine.create_scene();

    scene.add_exclusive_system(|_: &Context, (): &mut ()| {
        println!("bruh");
    }).add_startup_exclusive_system(Create);
    engine.run(scene);
}
