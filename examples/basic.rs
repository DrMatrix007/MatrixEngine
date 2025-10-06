use matrix_engine::engine::{
    Engine,
    commands::{CommandBuffer, add_entity_command::AddEntityCommand},
    component::Component,
    query::Read,
    runtime::SingleThreadedRuntime,
    system_registries::StageDescriptor,
};
use winit::event_loop::EventLoop;

#[derive(Debug)]
struct A;

impl Component for A {}

fn start(commands: &mut CommandBuffer, _: &mut Read<A>) {
    commands.add_command(AddEntityCommand::new().with(A).unwrap());
}

fn modify(_: &mut Read<A>) {}

fn main() {
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

    let mut engine = Engine::new(SingleThreadedRuntime);

    engine
        .scene_mut()
        .add_system(StageDescriptor::Startup, start);

    engine
        .scene_mut()
        .add_system(StageDescriptor::Update, modify);

    event_loop.run_app(&mut engine).unwrap();
}
