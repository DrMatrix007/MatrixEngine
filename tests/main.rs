use matrix_engine::{
    components::component::{Component, ComponentCollection},
    dispatchers::{
        component_group::ComponentGroup,
        context::{Context, SceneCreator},
        dispatcher::{DispatchedData, WriteStorage},
        systems::AsyncSystem,
    },
    engine::{Engine, EngineArgs},
    entity::Entity,
    schedulers::multi_threaded_scheduler::MultiThreadedScheduler,
};
use rand::Rng;

#[derive(Clone, Copy, Debug)]
struct A(pub i32);
impl Component for A {}

#[derive(Clone, Copy, Debug)]
struct B(pub i32);
impl Component for B {}

#[derive(Clone, Copy, Debug)]
struct C(pub i32);
impl Component for C {}

struct AddData;
impl AsyncSystem for AddData {
    type Query = (
        WriteStorage<ComponentCollection<A>>,
        WriteStorage<ComponentCollection<B>>,
        WriteStorage<ComponentCollection<C>>,
    );

    fn run(&mut self, _ctx: &Context, (a, b, c): &mut <Self as AsyncSystem>::Query) {
        let mut t = rand::thread_rng();
        for i in 0..1000 {
            let e = Entity::default();
            if t.gen::<f32>() < 0.5 {
                a.get().insert(e, A(i));
            }
            b.get().insert(e, B(i));
            c.get().insert(e, C(i));
        }
    }
}

struct ReadData;
impl AsyncSystem for ReadData {
    type Query = ComponentGroup<(
        WriteStorage<ComponentCollection<A>>,
        WriteStorage<ComponentCollection<B>>,
        WriteStorage<ComponentCollection<C>>,
    )>;

    fn run(&mut self, ctx: &Context, comps: &mut <Self as AsyncSystem>::Query) {
        for d in comps.iter() {
            println!("{:?}", d);
            assert!(d.1 .0 == d.2 .0 && d.1 .0 == d.3 .0);
        }
        println!("{:?}", comps.get().0.iter().count());
        println!("{:?}", comps.get().1.iter().count());
        println!("{:?}", comps.get().2.iter().count());
        ctx.quit();
    }
}

fn main() {
    let engine = Engine::new(EngineArgs {
        fps: 144,
        scheduler: MultiThreadedScheduler::with_amount_of_cpu_cores().unwrap(),
    });

    let ctx = engine.ctx();

    let mut scene = ctx.create_scene();

    scene
        .add_startup_async_system(AddData)
        .add_async_system(ReadData);

    engine.run(scene);
}
