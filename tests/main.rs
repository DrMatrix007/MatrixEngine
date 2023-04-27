use matrix_engine::{
    components::components::{Component, ComponentCollection},
    dispatchers::systems::AsyncSystem,
    engine::{Engine, EngineArgs},
    entity::Entity,
    events::Events,
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
        // panic!()
    }
}

struct PrintSystem;

impl<'a> AsyncSystem<'a> for PrintSystem {
    type Query = ();

    fn run(
        &mut self,
        _: &matrix_engine::dispatchers::systems::SystemArgs,
        _: <Self as AsyncSystem<'a>>::Query,
    ) {
        println!("print");
    }
}

struct A;
impl Component for A {}

struct TakeA;

impl<'a> AsyncSystem<'a> for TakeA {
    type Query = &'a ComponentCollection<A>;

    fn run(
        &mut self,
        _args: &matrix_engine::dispatchers::systems::SystemArgs,
        comps: <Self as AsyncSystem<'a>>::Query,
    ) {
        assert!(comps.iter().count() > 0);
    }
}

struct AddA;

impl<'a> AsyncSystem<'a> for AddA {
    type Query = &'a mut ComponentCollection<A>;

    fn run(
        &mut self,
        _args: &matrix_engine::dispatchers::systems::SystemArgs,
        comps: <Self as AsyncSystem<'a>>::Query,
    ) {
        for _ in 0..10 {
            comps.insert(Entity::new(), A);
        }
    }
}

struct EventReader;

impl<'a> AsyncSystem<'a> for EventReader {
    type Query = &'a Events;

    fn run(
        &mut self,
        args: &matrix_engine::dispatchers::systems::SystemArgs,
        comps: <Self as AsyncSystem<'a>>::Query,
    ) {
        if comps.is_pressed_down(winit::event::VirtualKeyCode::A) {
            println!("A");
        }
    }
}

fn main() {
    let mut scene = Scene::default();

    scene
        .add_system(TakeA)
        .add_exclusive_system(TakeA)
        .add_exclusive_startup_system(AddA)
        .add_startup_system(PrintSystem);

    let engine = Engine::new(EngineArgs {
        scene,
        scheduler: MultiThreadedScheduler::with_amount_of_cpu_cores().unwrap(),
        fps: 144,
        resources: None,
    });

    engine.run();
}
