use matrix_engine::{
    dispatchers::systems::AsyncSystem,
    engine::{Engine, EngineArgs},
    scene::Scene,
    schedulers::multi_threaded_scheduler::MultiThreadedScheduler,
};

struct PanicSystem;

impl<'a> AsyncSystem<'a> for PanicSystem {
    type Query = ();

    fn run(
        &mut self,
        _: &matrix_engine::dispatchers::systems::SystemArgs,
        _: <Self as AsyncSystem<'a>>::Query,
    ) {
        panic!()
    }
}

struct PrintSystem;

impl<'a> AsyncSystem<'a> for PrintSystem {
    type Query = ();

    fn run(&mut self, _: &matrix_engine::dispatchers::systems::SystemArgs, _: <Self as AsyncSystem<'a>>::Query) {
        println!("print");
    }
}

fn main() {
    let mut scene = Scene::default();

    scene.add_system(PanicSystem).add_startup_system(PrintSystem);

    let engine = Engine::new(EngineArgs {
        scene,
        scheduler: MultiThreadedScheduler::with_amount_of_cpu_cores().unwrap(),
        fps: 144,
        resources: None,
    });

    engine.run();
}
