use matrix_engine::{engine::{Engine, EngineArgs}, world::World, schedulers::multi_threaded_scheduler::MultiThreadedScheduler, dispatchers::systems::AsyncSystem};


struct PanicSystem;

impl<'a> AsyncSystem<'a> for PanicSystem {
    type Query = ();

    fn run(&mut self, args: &matrix_engine::dispatchers::systems::SystemArgs, comps: <Self as AsyncSystem<'a>>::Query) {
        panic!()
    }
}


fn main() {
    let mut world = World::default();

    world.add_system(PanicSystem);

    let mut engine = Engine::new(EngineArgs {
        world,
        scheduler: MultiThreadedScheduler::with_amount_of_cpu_cores().unwrap(),
        fps: 144,
    });

    engine.run();
}
