use std::{
    f32::consts::{E, PI},
    time::Instant,
};

use matrix_engine::{
    engine::{
        component_iters::IntoWrapper,
        entity::Entity,
        events::MatrixEventable,
        plugins::{window_plugin::WindowPlugin, Plugin},
        query::{ReadC, ReadE, WriteC, WriteR},
        runtimes::single_threaded::SingleThreaded,
        transform::Transform,
        Engine, EngineArgs,
    },
    math::matrix::Vector3,
    renderer::{
        camera::Camera, pipelines::models::cube::Cube, render_object::RenderObject,
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
        scene.add_send_system(move |(): &mut ()| {
            let now = Instant::now();

            v.push(1.0 / (now - latest).as_secs_f32());

            if (now - latest_second).as_secs() > 0 {
                let fps = v.iter().sum::<f32>() / v.len() as f32;
                println!("fps: {:10.5}, {:10.5}", fps, 1.0 / fps);
                latest_second = now;
            }
            latest = now;
        });
        scene.add_send_startup_system(
            |render_objs: &mut WriteC<RenderObject>,
             transforms: &mut WriteC<Transform>,
             camera: &mut WriteR<Camera, CustomEvents>| {
                for i in 0..100 {
                    for y in 0..100 {
                        for z in 0..100 {
                            let e = Entity::new();
                            render_objs.insert(e, RenderObject::new(Cube, "./img.jpg".to_string()));
                            transforms.insert(
                                e,
                                Transform::new_position(Vector3::new(
                                    5. * i as f32,
                                    5. * y as f32,
                                    5. * z as f32,
                                )),
                            );
                        }
                    }
                }

                camera.insert_and_notify(Camera {
                    eye: Vector3::new(0.0, 0.0, -1.),
                    dir: Vector3::new(1., 0., 0.),
                    up: Vector3::new(0., 1., 0.),
                    aspect: 1.,
                    fovy: PI / 4.,
                    znear: 0.001,
                    zfar: 1000.,
                });
            },
        );

        let mut yaw: f32 = 0.0; // Horizontal rotation around the y-axis
        let mut pitch: f32 = 0.0; // Vertical rotation
        let mut x = 0.;
        scene.add_send_system(
            move |camera: &mut WriteR<Camera, CustomEvents>, events: &mut ReadE<CustomEvents>| {
                if let Some(camera) = camera.get_mut() {
                    let dt = events.dt();
                    let move_speed = dt * 10.;
                    let rotation_speed = dt * 4. * camera.fovy / PI;

                    // Get forward (z-axis), right (x-axis), and up (y-axis) direction vectors
                    let forward = camera.dir.normalized();
                    let right = forward.cross(&Vector3::unit_y()).normalized();
                    let up = right.cross(&forward);

                    if events.keyboard().is_pressed(KeyCode::KeyW) {
                        camera.eye += &forward * move_speed;
                    }
                    if events.keyboard().is_pressed(KeyCode::KeyS) {
                        camera.eye -= &forward * move_speed;
                    }
                    if events.keyboard().is_pressed(KeyCode::KeyA) {
                        camera.eye -= &right * move_speed;
                    }
                    if events.keyboard().is_pressed(KeyCode::KeyD) {
                        camera.eye += &right * move_speed;
                    }
                    if events.keyboard().is_pressed(KeyCode::Space) {
                        camera.eye += &up * move_speed;
                    }
                    if events.keyboard().is_pressed(KeyCode::ControlLeft) {
                        camera.eye -= &up * move_speed;
                    }

                    match events.mouse_wheel_delta() {
                        dx if dx != 0. => {
                            x += dx;
                            camera.fovy = PI / (1. + E.powf(x))
                        }
                        _ => (),
                    }

                    let (x, y) = events.mouse_dx();
                    yaw += x * rotation_speed;
                    pitch -= y * rotation_speed;

                    pitch = pitch.clamp(-PI / 2. + 0.01, PI / 2. - 0.01);
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
        let mut is_on = false;
        scene.add_send_system(
            move |transforms: &mut WriteC<Transform>,
                  obj: &mut ReadC<RenderObject>,
                  events: &mut ReadE<CustomEvents>| {
                if events.keyboard().is_just_pressed(KeyCode::KeyG) {
                    is_on = !is_on;
                    println!("started {}", is_on);
                }
                if is_on {
                    let dt = events.dt();
                    for (_, (t, _)) in (transforms.iter_mut(), obj.iter()).into_wrapper() {
                        *t.rotation.y_mut() += dt * 1.;
                    }
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
