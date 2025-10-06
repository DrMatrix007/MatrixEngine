use matrix_engine::{engine::{
    commands::{add_window_resource_command::AddWindowResourceCommand, CommandBuffer}, runtime::SingleThreadedRuntime, system_registries::StageDescriptor, Engine
}, renderer::renderer_system::matrix_renderer};
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

    engine
        .scene_mut()
        .add_system(StageDescriptor::Render, matrix_renderer);

    event_loop.run_app(&mut engine).unwrap();
}
