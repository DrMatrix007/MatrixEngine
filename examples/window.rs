use std::{
    f32::consts::PI,
    process::exit,
    time::{Duration, Instant},
};

use matrix_engine::{
    arl::matrix_renderer::{
        camera::Camera,
        matrix_render_object::MatrixRenderObject,
        matrix_renderer_system::{
            create_matrix_instance, matrix_renderer, prepare_renderer_frame, update_surface_size,
        },
        square::Square,
        transform::Transform,
    },
    engine::{
        Engine, EngineState,
        commands::{
            CommandBuffer, add_entity_command::AddEntityCommand,
            add_window_resource_command::AddWindowResourceCommand,
        },
        query::Res,
        runtime::SingleThreadedRuntime,
        system_registries::{Stage, StageDescriptor},
        systems::QuerySystem,
    },
    math::{
        matrix::{ColVector, Matrix},
        vector::Vector,
    },
};
use winit::{
    event::{DeviceEvent, WindowEvent},
    event_loop::EventLoop,
    window::WindowAttributes,
};

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

    let mut engine = Engine::new(SingleThreadedRuntime);

    engine.add_system_to_scene(StageDescriptor::Startup, start);
    engine.add_system_to_scene(StageDescriptor::Startup, create_matrix_instance);
    engine.add_system_to_scene(StageDescriptor::Render, matrix_renderer);
    engine.add_system_to_scene(StageDescriptor::WindowEvent, exit_on_close);
    engine.add_system_to_scene(StageDescriptor::WindowEvent, update_surface_size);
    engine.add_system_to_scene(StageDescriptor::PreRender, prepare_renderer_frame);
    engine.add_system_to_scene(StageDescriptor::DeviceEvent, MouseMovement::new());
    engine.add_system_to_scene(StageDescriptor::Update, |cam: &mut Res<Camera>| {
        if let Some(cam) = cam.as_mut() {

            // println!("{:?}", cam.raw().proj);
        }
    });

    let log_fps = create_fps_counter();

    engine
        .scene_mut()
        .add_system(StageDescriptor::PostUpdate, log_fps);

    event_loop.run_app(&mut engine).unwrap();
}

fn start(commands: &mut CommandBuffer, camera: &mut Res<Camera>) {
    commands.add_command(AddWindowResourceCommand::new(WindowAttributes::default()));

    let max = 1000;

    for _ in 0..max {
        commands.add_command(
            AddEntityCommand::new()
                .with(MatrixRenderObject::new(Square))
                .unwrap()
                .with(Transform::new(
                    Matrix::new([[0.0, 0.0, 0.0]]),
                    Matrix::identity(),
                    Matrix::ones(),
                ))
                .unwrap(),
        );
    }
    let cam = Camera::new(
        ColVector::new([[0.0], [0.0], [0.0]]),
        ColVector::new([[0.1], [0.5], [-1.]]),
        ColVector::new([[0.0], [1.0], [0.0]]),
        1.0,
        PI / 4.0,
        0.1,
        10000.0,
    );

    camera.replace(cam);
}

fn exit_on_close(stage: &mut Stage) {
    if let Stage::WindowEvent(WindowEvent::CloseRequested) = stage {
        exit(0);
    };
}

fn create_fps_counter() -> impl FnMut() {
    let mut last_time = Instant::now();
    let mut frame_count = 0;

    move || {
        frame_count += 1;
        let now = Instant::now();

        if now.duration_since(last_time) >= Duration::from_secs(2) {
            println!("FPS: {}", frame_count / 2);
            frame_count = 0;
            last_time = now;
        }
    }
}

pub struct MouseMovement {
    yaw: f32,
    pitch: f32,
}

impl MouseMovement {
    pub fn new() -> Self {
        Self {
            yaw: -90.0, // facing -Z initially
            pitch: 0.0,
        }
    }
}

impl Default for MouseMovement {
    fn default() -> Self {
        Self::new()
    }
}
impl QuerySystem<EngineState, (Res<Camera>, Stage)> for MouseMovement {
    fn run(&mut self, (cam, stage): &mut (Res<Camera>, Stage)) {
        // Match on stage to see if it contains a DeviceEvent::MouseMotion event
        if let Stage::DeviceEvent(_, event) = stage
            && let DeviceEvent::MouseMotion { delta: (dx, dy) } = event
        {
            let sensitivity = 0.1;

            // Update yaw and pitch based on mouse delta (invert dy for pitch)
            self.yaw += (*dx as f32) * sensitivity;
            self.pitch -= (*dy as f32) * sensitivity;

            // Clamp the pitch so the camera doesn't flip
            self.pitch = self.pitch.clamp(-89.0, 89.0);

            // Calculate new front vector based on updated yaw and pitch
            let yaw_rad = self.yaw.to_radians();
            let pitch_rad = self.pitch.to_radians();

            let front_x = yaw_rad.cos() * pitch_rad.cos();
            let front_y = pitch_rad.sin();
            let front_z = yaw_rad.sin() * pitch_rad.cos();

            let length = (front_x * front_x + front_y * front_y + front_z * front_z).sqrt();

            let front = Matrix::new([[front_x / length], [front_y / length], [front_z / length]]);

            // Update camera direction
            if let Some(cam) = cam.as_mut() {
                cam.direction = front;
            }
        }
    }
}
