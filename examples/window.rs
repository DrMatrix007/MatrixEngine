use matrix_engine::engine::{
    Engine,
    commands::{CommandBuffer, add_window_resource_command::AddWindowResourceCommand},
    runtime::SingleThreadedRuntime,
    system_registries::StageDescriptor,
};
use winit::{event_loop::EventLoop, window::WindowAttributes};

fn start(commands: &mut CommandBuffer) {
    commands.add_command(AddWindowResourceCommand::new(WindowAttributes::default()));
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

    let mut engine = Engine::new(SingleThreadedRuntime);

    engine
        .scene_mut()
        .add_system(StageDescriptor::Startup, start);

    event_loop.run_app(&mut engine).unwrap();
}
