use matrix_engine::{
    components::{
        component::{Component, ComponentCollection},
        resources::{Resource, ResourceHolder},
    },
    dispatchers::{
        context::{Context, ResourceHolderManager, SceneCreator},
        dispatcher::{ReadEventLoopWindowTarget, ReadStorage, WriteStorage, DispatchedData},
        function_systems::{Wrappable},
        systems::{AsyncSystem, ExclusiveSystem},
    },
    engine::{Engine, EngineArgs},
    entity::Entity,
    events::event_registry::EventRegistry,
    schedulers::multi_threaded_scheduler::MultiThreadedScheduler,
};
use winit::{window::WindowBuilder};

struct PanicSystem;

impl AsyncSystem for PanicSystem {
    type Query = ();

    fn run(&mut self, _: &matrix_engine::dispatchers::context::Context, _: &mut Self::Query) {
        // panic!()
    }
}

struct PrintSystem;

impl AsyncSystem for PrintSystem {
    type Query = ReadStorage<EventRegistry>;

    fn run(&mut self, _: &Context, e: &mut Self::Query) {
        if e.get().is_resource_created::<Window>() {
            println!("created");
        }
    }
}

struct A;
impl Component for A {}

struct TakeA;

impl AsyncSystem for TakeA {
    type Query = ReadStorage<ComponentCollection<A>>;

    fn run(&mut self, _args: &matrix_engine::dispatchers::context::Context, comps: &mut Self::Query) {
        assert!(comps.get().iter().count() > 0);
    }
}

struct AddA;

impl AsyncSystem for AddA {
    type Query = WriteStorage<ComponentCollection<A>>;

    fn run(
        &mut self,
        _args: &matrix_engine::dispatchers::context::Context,
        comps: &mut Self::Query,
    ) {
        for _ in 0..10 {
            comps.get().insert(Entity::new(), A);
        }
    }
}

struct ExclusiveTest;

impl ExclusiveSystem for ExclusiveTest {
    type Query = ReadEventLoopWindowTarget;

    fn run(&mut self, _: &matrix_engine::dispatchers::context::Context, _: &mut Self::Query) {}
}

struct Window {
    pub _w: winit::window::Window,
}

impl Resource for Window {}

struct CreateWindow;

impl ExclusiveSystem for CreateWindow {
    type Query = (
        ReadEventLoopWindowTarget,
        WriteStorage<ResourceHolder<Window>>,
    );

    fn run(
        &mut self,
        ctx: &matrix_engine::dispatchers::context::Context,
        (target, window): &mut Self::Query,
    ) {
        ctx.get_or_insert_resource_with(window.holder_mut(), || {
            let w = WindowBuilder::new().build(target.get()).unwrap();
            Window { _w: w }
        });
    }
}

fn main() {
    // fn a(a: &Context, b: &ComponentCollection<A>) {}
    let a = |_a: &Context, _b: &mut ReadStorage<ComponentCollection<A>>| {
        println!("bruh");
    };
    let engine = Engine::new(EngineArgs {
        scheduler: MultiThreadedScheduler::with_amount_of_cpu_cores().unwrap(),
        fps: 144,
        resources: None,
    });
    let ctx = engine.ctx();
    let mut scene = ctx.create_scene();
    scene
        .add_startup_exclusive_system(CreateWindow)
        .add_startup_exclusive_system(a.wrap())
        .add_async_system(TakeA)
        .add_async_system(TakeA)
        .add_async_system(PrintSystem)
        .add_startup_async_system(AddA);

    engine.run(scene);
}
