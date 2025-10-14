use std::{
    f32::consts::PI,
    process::exit,
    time::{Duration, Instant},
};

use cgmath::{InnerSpace, Point3, Quaternion, Rad, Vector3};
use matrix_engine::{
    arl::matrix_renderer::{
        camera::Camera, cube::Cube, matrix_render_object::MatrixRenderObject, matrix_renderer_system::{
            create_matrix_instance, matrix_renderer, prepare_renderer_frame, update_surface_size,
        }, square::Square, transform::Transform
    },
    engine::{
        commands::{
            add_entity_command::AddEntityCommand, add_window_resource_command::AddWindowResourceCommand, CommandBuffer
        }, query::Res, runtime::SingleThreadedRuntime, system_registries::{Stage, StageDescriptor}, systems::QuerySystem, Engine, EngineState
    },
};
use winit::{
    event::{DeviceEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
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
    engine.add_system_to_scene(StageDescriptor::PostUpdate, create_fps_counter());

    event_loop.run_app(&mut engine).unwrap();
}

fn start(commands: &mut CommandBuffer, camera: &mut Res<Camera>) {
    commands.add_command(AddWindowResourceCommand::new(WindowAttributes::default()));

    let x_max = 100;
    let y_max = 100;
    let z_max = 100;

    for x in (0..x_max).rev() {
        for y in (0..y_max).rev() {
            for z in (0..z_max).rev() {
                commands.add_command(
                    AddEntityCommand::new()
                        .with(MatrixRenderObject::new(Cube, "rickroll.jpeg"))
                        .unwrap()
                        .with(Transform::new(
                            Vector3::from([(x * 10) as _, (y * 10) as _, (-z * 10) as _]),
                            Quaternion::new(1.0, 0.0, 0.0, 0.0),
                            Vector3::new(1.0, 1.0, 1.0),
                        ))
                        .unwrap(),
                );
            }
        }
    }
    let cam = Camera::new(
        Point3::from([0.0, 0.0, 0.0]),
        Vector3::from([0.0, 0.0, 1.]),
        Vector3::from([0.0, 1.0, 0.0]),
        cgmath::PerspectiveFov {
            fovy: Rad(PI / 2.0),
            aspect: 1.0,
            near: 0.1,
            far: 10000.0,
        },
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
            yaw: -90.0,
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
        let sensitivity = 0.01;
        let movement_speed = 0.5;

        if let Stage::DeviceEvent(_, event) = stage
            && let DeviceEvent::MouseMotion { delta: (dx, dy) } = event
        {
            self.yaw += (*dx as f32) * sensitivity;
            self.pitch -= (*dy as f32) * sensitivity;

            self.pitch = self.pitch.clamp(-89.0, 89.0);

            let yaw_rad = self.yaw.to_radians();
            let pitch_rad = self.pitch.to_radians();

            let front_x = yaw_rad.cos() * pitch_rad.cos();
            let front_y = pitch_rad.sin();
            let front_z = yaw_rad.sin() * pitch_rad.cos();

            let front = Vector3::from([front_x, front_y, front_z]).normalize();

            if let Some(cam) = cam.as_mut() {
                cam.direction = front;
            }
        }

        if let Stage::DeviceEvent(_, event) = stage
            && let DeviceEvent::Key(key_event) = event
            && let Some(cam) = cam.as_mut()
        {
            let world_up = Vector3::unit_y();
            let front = cam.direction;

            let right = world_up.cross(front).normalize();

            match key_event.physical_key {
                PhysicalKey::Code(KeyCode::KeyW) => {
                    cam.pos += front * movement_speed;
                }
                PhysicalKey::Code(KeyCode::KeyS) => {
                    cam.pos -= front * movement_speed;
                }
                PhysicalKey::Code(KeyCode::KeyA) => {
                    cam.pos += right * movement_speed;
                }
                PhysicalKey::Code(KeyCode::KeyD) => {
                    cam.pos -= right * movement_speed;
                }
                _ => {}
            }

            println!("{:?}", cam.pos);
        }
    }
}
