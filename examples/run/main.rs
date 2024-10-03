use std::{f32::consts::PI, time::Instant};

use matrix_engine::{
    engine::{
        entity::Entity,
        events::{MatrixEvent, MatrixEventable},
        plugins::{window_plugin::WindowPlugin, Plugin},
        query::{ReadE, ReadSystemID, WriteC, WriteE, WriteR},
        runtimes::single_threaded::SingleThreaded,
        Engine, EngineArgs,
    },
    math::matrix::{Vector3, Vector4},
    renderer::{
        camera::Camera,
        pipelines::models::{sphere::Sphere, square::Square},
        render_object::RenderObject,
        renderer_plugin::RendererPlugin,
    },
};
use winit::keyboard::KeyCode;

struct Example1;

impl<CustomEvents: MatrixEventable> Plugin<CustomEvents> for Example1 {
    fn build(&self, scene: &mut matrix_engine::engine::scene::Scene<CustomEvents>) {
        let mut latest = Instant::now();
        let mut v = Vec::<f32>::new();
        let mut latest_second = Instant::now();
        scene.add_send_system(
            move |(events, write_events, id): &mut (
                ReadE<CustomEvents>,
                WriteE<CustomEvents>,
                ReadSystemID,
            )| {
                let now = Instant::now();

                v.push(1.0 / (now - latest).as_secs_f32());

                if (now - latest_second).as_secs() > 0 {
                    let fps = v.iter().sum::<f32>() / v.len() as f32;
                    println!("fps: {:10.5}, {:10.5}", fps, 1.0 / fps);
                    latest_second = now;
                }
                latest = now;

                if events.is_just_pressed(KeyCode::KeyW) {
                    write_events.send(MatrixEvent::DestroySystem(**id)).unwrap();
                }
            },
        );
        scene.add_send_startup_system(
            |data: &mut WriteC<RenderObject>, camera: &mut WriteR<Camera, CustomEvents>| {
                data.insert(
                    Entity::new(),
                    RenderObject::new(Square, "./img.jpg".to_string()),
                );

                camera.insert_and_notify(Camera {
                    eye: Vector3::new(0.0, 0.0, -1.),
                    dir: Vector3::new(0., 0., 2.),
                    up: Vector3::new(0., 1., 0.),
                    aspect: 1.,
                    fovy: PI / 4.,
                    znear: 0.1,
                    zfar: 1000.,
                });
            },
        );

        let mut yaw = 0.0; // Horizontal rotation around the y-axis
        let mut pitch = 0.0; // Vertical rotation
        scene.add_send_system(
            move |camera: &mut WriteR<Camera, CustomEvents>, events: &mut ReadE<CustomEvents>| {
                if let Some(camera) = camera.get_mut() {
                    let dt = events.dt();
                    let move_speed = dt * 3.;
                    let rotation_speed = dt;

                    // Get forward (z-axis), right (x-axis), and up (y-axis) direction vectors
                    let forward = camera.dir.normalized();
                    let right = forward.cross(&Vector3::unit_y()).normalized();
                    let up = Vector3::unit_y();

                    if events.is_pressed(KeyCode::KeyW) {
                        camera.eye += &forward * move_speed;
                    }
                    if events.is_pressed(KeyCode::KeyS) {
                        camera.eye -= &forward * move_speed;
                    }
                    if events.is_pressed(KeyCode::KeyA) {
                        camera.eye -= &right * move_speed;
                    }
                    if events.is_pressed(KeyCode::KeyD) {
                        camera.eye += &right * move_speed;
                    }
                    if events.is_pressed(KeyCode::Space) {
                        camera.eye += &up * move_speed;
                    }
                    if events.is_pressed(KeyCode::ControlLeft) {
                        camera.eye -= &up * move_speed;
                    }

                    if events.is_pressed(KeyCode::ArrowLeft) {
                        yaw -= rotation_speed; // Rotate left
                    }
                    if events.is_pressed(KeyCode::ArrowRight) {
                        yaw += rotation_speed; // Rotate right
                    }
                    if events.is_pressed(KeyCode::ArrowUp) {
                        pitch += rotation_speed; // Look up
                    }
                    if events.is_pressed(KeyCode::ArrowDown) {
                        pitch -= rotation_speed; // Look down
                    }

                    if events.is_just_pressed(KeyCode::KeyE) {
                        camera.fovy *= 2.0;
                    }
                    if events.is_just_pressed(KeyCode::KeyQ) {
                        camera.fovy /= 2.0;
                    }

                    // Update the camera's direction (yaw and pitch)
                    let (sin_yaw, cos_yaw) = yaw.sin_cos();
                    let (sin_pitch, cos_pitch) = pitch.sin_cos();

                    // Calculate the new forward direction after applying yaw and pitch
                    let new_forward =
                        Vector3::new(cos_pitch * cos_yaw, sin_pitch, cos_pitch * sin_yaw);

                    camera.dir = new_forward.normalized();
                }
            },
        );
    }
}

fn main() {
    let mut engine = <Engine>::new(EngineArgs::new(SingleThreaded, SingleThreaded));

    engine.add_scene_plugin(WindowPlugin::new("hello example!"));

    engine.add_scene_plugin(RendererPlugin);

    engine.add_scene_plugin(Example1);

    engine.run().unwrap();
}
