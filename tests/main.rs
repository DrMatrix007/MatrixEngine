use matrix_engine::{
    components::{
        component::{Component, ComponentCollection},
        resources::{Resource, ResourceHolder},
    },
    dispatchers::systems::{AsyncSystem, ExclusiveSystem},
    engine::{Engine, EngineArgs},
    entity::Entity,
    scene::Scene,
    schedulers::multi_threaded_scheduler::MultiThreadedScheduler,
};
use winit::{event_loop::EventLoopWindowTarget, window::WindowBuilder};

struct PanicSystem;

impl AsyncSystem for PanicSystem {
    type Query<'a> = ();

    fn run(
        &mut self,
        _: &matrix_engine::dispatchers::systems::SystemContext,
        _: <Self as AsyncSystem>::Query<'_>,
    ) {
        // panic!()
    }
}

struct PrintSystem;

impl AsyncSystem for PrintSystem {
    type Query<'a> = ();

    fn run(
        &mut self,
        _: &matrix_engine::dispatchers::systems::SystemContext,
        _: <Self as AsyncSystem>::Query<'_>,
    ) {
        println!("print");
    }
}

struct A;
impl Component for A {}

struct TakeA;

impl AsyncSystem for TakeA {
    type Query<'a> = &'a ComponentCollection<A>;

    fn run(
        &mut self,
        _args: &matrix_engine::dispatchers::systems::SystemContext,
        comps: <Self as AsyncSystem>::Query<'_>,
    ) {
        assert!(comps.iter().count() > 0);
    }
}

struct AddA;

impl AsyncSystem for AddA {
    type Query<'a> = &'a mut ComponentCollection<A>;

    fn run(
        &mut self,
        _args: &matrix_engine::dispatchers::systems::SystemContext,
        comps: <Self as AsyncSystem>::Query<'_>,
    ) {
        for _ in 0..10 {
            comps.insert(Entity::new(), A);
        }
    }
}


struct ExclusiveTest;

impl ExclusiveSystem for ExclusiveTest {
    type Query<'a> = &'a EventLoopWindowTarget<()>;

    fn run(
        &mut self,
        _: &matrix_engine::dispatchers::systems::SystemContext,
        _: <Self as ExclusiveSystem>::Query<'_>,
    ) {
    }
}

struct Window {
    pub _w: winit::window::Window,
}

impl Resource for Window {}

struct CreateWindow;

impl ExclusiveSystem for CreateWindow {
    type Query<'a> = (
        &'a EventLoopWindowTarget<()>,
        &'a mut ResourceHolder<Window>,
    );

    fn run(
        &mut self,
        _args: &matrix_engine::dispatchers::systems::SystemContext,
        (target, window): <Self as ExclusiveSystem>::Query<'_>,
    ) {
        window.get_or_insert_with(|| {
            let w = WindowBuilder::new().build(target).unwrap();
            Window { _w: w }
        });

    }
}

fn main() {
    let mut scene = Scene::default();

    scene
        .add_startup_exclusive_system(CreateWindow)
        .add_async_system(TakeA)
        .add_async_system(TakeA)
        .add_startup_async_system(AddA)
        .add_startup_async_system(PrintSystem);

    let engine = Engine::new(EngineArgs {
        scene,
        scheduler: MultiThreadedScheduler::with_amount_of_cpu_cores().unwrap(),
        fps: 144,
        resources: None,
    });

    engine.run();
}
