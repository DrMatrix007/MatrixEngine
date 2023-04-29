use matrix_engine::{
    components::{
        components::{Component, ComponentCollection},
        resources::{Resource, ResourceHolder},
    },
    dispatchers::systems::{AsyncSystem, ExclusiveSystem},
    engine::{Engine, EngineArgs},
    entity::Entity,
    events::Events,
    scene::Scene,
    schedulers::multi_threaded_scheduler::MultiThreadedScheduler,
};
use winit::{event_loop::EventLoopWindowTarget, window::WindowBuilder};

struct PanicSystem;

impl AsyncSystem for PanicSystem {
    type Query<'a> = ();

    fn run<'a>(
        &mut self,
        _: &matrix_engine::dispatchers::systems::SystemArgs,
        _: <Self as AsyncSystem>::Query<'a>,
    ) {
        // panic!()
    }
}

struct PrintSystem;

impl AsyncSystem for PrintSystem {
    type Query<'a> = ();

    fn run<'a>(
        &mut self,
        _: &matrix_engine::dispatchers::systems::SystemArgs,
        _: <Self as AsyncSystem>::Query<'a>,
    ) {
        println!("print");
    }
}

struct A;
impl Component for A {}

struct TakeA;

impl AsyncSystem for TakeA {
    type Query<'a> = &'a ComponentCollection<A>;

    fn run<'a>(
        &mut self,
        _args: &matrix_engine::dispatchers::systems::SystemArgs,
        comps: <Self as AsyncSystem>::Query<'a>,
    ) {
        assert!(comps.iter().count() > 0);
    }
}

struct AddA;

impl AsyncSystem for AddA {
    type Query<'a> = &'a mut ComponentCollection<A>;

    fn run<'a>(
        &mut self,
        _args: &matrix_engine::dispatchers::systems::SystemArgs,
        comps: <Self as AsyncSystem>::Query<'a>,
    ) {
        for _ in 0..10 {
            comps.insert(Entity::new(), A);
        }
    }
}

struct EventReader;

impl AsyncSystem for EventReader {
    type Query<'a> = &'a Events;

    fn run<'a>(
        &mut self,
        _: &matrix_engine::dispatchers::systems::SystemArgs,
        comps: <Self as AsyncSystem>::Query<'a>,
    ) {
        if comps.is_pressed_down(winit::event::VirtualKeyCode::A) {
            println!("A");
        }
    }
}

struct ExclusiveTest;

impl ExclusiveSystem for ExclusiveTest {
    type Query<'a> = &'a EventLoopWindowTarget<()>;

    fn run<'a>(
        &mut self,
        _: &matrix_engine::dispatchers::systems::SystemArgs,
        _: <Self as ExclusiveSystem>::Query<'a>,
    ) {
    }
}

struct Window {
    pub w: winit::window::Window,
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
        _args: &matrix_engine::dispatchers::systems::SystemArgs,
        (target, window): <Self as ExclusiveSystem>::Query<'_>,
    ) {
        window.get_or_insert_with(|| {
            let w = WindowBuilder::new().build(target).unwrap();
            Window { w }
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
