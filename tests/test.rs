use matrix_engine::engine::{
    events::event_registry::EventRegistry, runtime::SingleThreaded, systems::SystemControlFlow,
};
#[allow(unused_imports)]
use matrix_engine::{
    engine::{
        runtime::MultiThreaded,
        scenes::{components::Component, entities::Entity, scene_builder::SceneBuilder},
        systems::{
            query::components::{ReadC, WriteC},
            QuerySystem,
        },
        Engine,
    },
    renderer::renderer_system::RendererSystem,
};
use winit::window::WindowBuilder;

#[derive(Debug)]
struct A;
impl Component for A {}

struct B;
impl Component for B {}

struct SysC;
impl QuerySystem for SysC {
    type Query = (WriteC<A>, ReadC<B>);

    fn run(
        &mut self,
        _event: &EventRegistry,
        _args: &mut Self::Query,
    ) -> matrix_engine::engine::systems::SystemControlFlow {
        SystemControlFlow::Continue
    }
}

struct SysD;
impl QuerySystem for SysD {
    type Query = (WriteC<B>, ReadC<A>);

    fn run(
        &mut self,
        _event: &EventRegistry,
        _args: &mut Self::Query,
    ) -> matrix_engine::engine::systems::SystemControlFlow {
        for event in _event.all_window_events() {
            if event.is_pressed(winit::event::VirtualKeyCode::A) {
                println!("dam");
            }
        }

        SystemControlFlow::Continue
    }
}

fn main() {
    let runtime = MultiThreaded::new(8);

    let mut engine = Engine::new(runtime, 144);

    let window = WindowBuilder::new()
        .build(engine.event_loop().unwrap())
        .unwrap();

    engine
        .engine_systems_mut()
        .push_send(RendererSystem::new(window));

    let builder = SceneBuilder::new(|scene_reg, system_reg| {
        for _i in 1..100 {
            scene_reg
                .components_mut()
                .try_add_component(Entity::new(), A)
                .unwrap();
        }
        system_reg.push_send(SysC);
        system_reg.push_send(SysD);
    });

    engine.run(&builder)
}
