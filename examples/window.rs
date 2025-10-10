use std::time::{Duration, Instant};

use matrix_engine::{
    arl::matrix_renderer::{
        matrix_render_object::MatrixRenderObject,
        matrix_renderer_system::{
            create_matrix_instance, matrix_renderer, prepare_renderer_frame, update_surface_size,
        },
        pentagon::Pentagon,
    },
    engine::{
        Engine,
        commands::{
            CommandBuffer, add_entity_command::AddEntityCommand,
            add_window_resource_command::AddWindowResourceCommand,
        },
        runtime::SingleThreadedRuntime,
        system_registries::StageDescriptor,
    },
};
use winit::{event_loop::EventLoop, window::WindowAttributes};

fn start(commands: &mut CommandBuffer) {
    commands.add_command(AddWindowResourceCommand::new(WindowAttributes::default()));
    commands.add_command(
        AddEntityCommand::new()
            .with(MatrixRenderObject::new(Pentagon))
            .unwrap(),
    );
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

    let mut engine = Engine::new(SingleThreadedRuntime);

    engine.add_system_to_scene(StageDescriptor::Startup, start);

    engine.add_system_to_scene(StageDescriptor::Startup, create_matrix_instance);

    engine.add_system_to_scene(StageDescriptor::Render, matrix_renderer);

    engine.add_system_to_scene(StageDescriptor::WindowEvent, update_surface_size);
    
    engine.add_system_to_scene(StageDescriptor::PreRender, prepare_renderer_frame);

    let mut last_time = Instant::now();
    let mut frame_count = 0;

    let log_fps = move || {
        frame_count += 1;
        let now = Instant::now();

        if now.duration_since(last_time) >= Duration::from_secs(2) {
            println!("FPS: {}", frame_count / 2);
            frame_count = 0;
            last_time = now;
        }
    };

    engine
        .scene_mut()
        .add_system(StageDescriptor::Update, log_fps);

    event_loop.run_app(&mut engine).unwrap();
}
