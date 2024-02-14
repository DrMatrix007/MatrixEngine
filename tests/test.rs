use std::{
    f32::consts::PI,
    time::{Duration, Instant},
};

#[allow(unused_imports)]
use matrix_engine::engine::{
    runtime::MultiThreaded,
    scenes::{components::Component, entities::Entity, scene_builder::SceneBuilder},
    systems::{
        query::components::{ReadC, WriteC},
        QuerySystem,
    },
    Engine,
};
use matrix_engine::{
    engine::{
        events::event_registry::EventRegistry,
        runtime::SingleThreaded,
        scenes::{components::transform::Transform, entities::entity_builder::EntityBuilder},
        systems::{query::resources::WriteR, query_group::ComponentRefIterable, SystemControlFlow},
    },
    math::{matrices::Vector3, vectors::Vector3D},
    renderer::{
        matrix_renderer::{
            camera::CameraResource,
            render_object::RenderObject,
            renderer_system::{MatrixRendererResource, MatrixRendererSystem, RendererResourceArgs},
        },
        pipelines::structures::{circle::Circle, cube::Cube, icosphere::Icosphere, plain::Plain},
    },
};
use num_traits::clamp_max;
use wgpu::Color;
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
            if event.is_pressed(winit::keyboard::KeyCode::KeyA) {
                // spin_sleep::sleep(Duration::from_secs_f64(3.));
            }
        }

        SystemControlFlow::Continue
    }
}

struct CameraPlayerSystem {
    fps_counter: FpsCounter,
}

impl CameraPlayerSystem {
    fn new() -> Self {
        Self {
            fps_counter: Default::default(),
        }
    }
}

impl QuerySystem for CameraPlayerSystem {
    type Query = WriteR<CameraResource>;

    fn run(
        &mut self,
        events: &EventRegistry,
        cam: &mut <Self as QuerySystem>::Query,
    ) -> SystemControlFlow {
        let cam = match cam.get_mut() {
            Some(cam) => cam,
            None => return SystemControlFlow::Continue,
        };

        let mut delta = Vector3::<f32>::zeros();

        let speed = 1.0;
        let _rotate_speed = PI / 4.0;

        for window_events in events.all_window_events() {
            if window_events.is_pressed(winit::keyboard::KeyCode::KeyA) {
                *delta.x_mut() -= speed;
            }
            if window_events.is_pressed(winit::keyboard::KeyCode::KeyD) {
                *delta.x_mut() += speed;
            }
            if window_events.is_pressed(winit::keyboard::KeyCode::KeyW) {
                *delta.z_mut() -= speed;
            }
            if window_events.is_pressed(winit::keyboard::KeyCode::KeyS) {
                *delta.z_mut() += speed;
            }
            if window_events.is_pressed(winit::keyboard::KeyCode::Space) {
                *delta.y_mut() += speed;
            }
            if window_events.is_pressed(winit::keyboard::KeyCode::KeyC) {
                *delta.y_mut() -= speed;
            }
        }

        match events.mouse_scroll_delta().1.total_cmp(&0.) {
            std::cmp::Ordering::Less => {
                cam.camera_mut().prespective.fovy_rad =
                    clamp_max(cam.camera().prespective.fovy_rad * 2., PI / 1.1)
            }
            std::cmp::Ordering::Equal => {}
            std::cmp::Ordering::Greater => cam.camera_mut().prespective.fovy_rad /= 2.,
        }

        // delta = cam.camera().rotation.euler_into_rotation_matrix3() * delta * dt;
        let (x, y) = events.mouse_delta();
        // self.theta += (x as f32) * dt * rotate_speed;
        // self.phi += (y as f32) * dt * rotate_speed;
        // *cam.camera_mut().rotation.y_mut() = self.theta;
        // *cam.camera_mut().rotation.x_mut() = self.phi;
        // cam.camera_mut().position += delta;

        let dt = self.fps_counter.capture().as_secs_f32();

        let sens = cam.camera().prespective.fovy_rad;
        cam.camera_mut()
            .rotate_camera(x as f32 * dt * sens, y as f32 * dt * sens);
        cam.camera_mut().move_camera(delta * dt);

        // println!(
        //     "{:.4} {:.4}",
        //     self.fps_counter.capture().as_secs_f32() - events.calc_delta_time().as_secs_f32(),events.calc_delta_time().as_secs_f32()
        // );

        SystemControlFlow::Continue
    }
}

#[derive(Debug)]
struct FpsCounter {
    last: Instant,
}

impl Default for FpsCounter {
    fn default() -> Self {
        Self {
            last: Instant::now(),
        }
    }
}

impl FpsCounter {
    pub fn new() -> Self {
        Self {
            last: Instant::now(),
        }
    }
    pub fn capture(&mut self) -> Duration {
        let now = Instant::now();
        let duration = now - self.last;
        self.last = now;

        duration
    }
    pub fn capture_as_fps(&mut self) -> f64 {
        let d = self.capture();
        1. / d.as_secs_f64()
    }
}

#[derive(Default)]
pub struct RotateAll {
    toggle: bool,
}

impl QuerySystem for RotateAll {
    type Query = (WriteC<Transform>, ReadC<RenderObject>);

    fn run(&mut self, events: &EventRegistry, args: &mut Self::Query) -> SystemControlFlow {
        let dt = events.calc_delta_time().as_secs_f32();
        println!("frame");
        if events
            .all_window_events()
            .find(|x| x.is_pressed_down(winit::keyboard::KeyCode::KeyG))
            .is_some()
        {
            println!("toggled");
            self.toggle = !self.toggle;
        }
        if self.toggle {
            for (_e, (a, _b)) in args.component_iter() {
                a.apply_rotation(Vector3::from([[1., 1., 1.]]) * dt);
            }
        }

        SystemControlFlow::Continue
    }
}

fn main() {
    // let runtime = MultiThreaded::new(10);
    let runtime = SingleThreaded::new();

    let mut engine = Engine::new(runtime, 144);

    let window = WindowBuilder::new()
        .build(engine.event_loop().unwrap())
        .unwrap();

    let renderer_resource = MatrixRendererResource::new(RendererResourceArgs {
        background_color: Color {
            r: 0.69,
            g: 0.69,
            b: 0.69,
            a: 0.69,
        },
        window,
    });

    engine.lock_engine_resources().insert(renderer_resource);

    engine
        .engine_systems_mut()
        .push_send(MatrixRendererSystem::default());

    let builder = SceneBuilder::new(|scene_reg, system_reg| {
        for z in 0..100 {
            for y in 0..10 {
                for x in 0..10 {
                    let mut t = Transform::identity();
                    t.apply_position_diff(Vector3::from([[x as f32, y as f32, z as f32]]));
                    EntityBuilder::new(scene_reg.components_mut())
                        .add(A)
                        .unwrap()
                        .add(RenderObject::new(
                            Icosphere::<2>,
                            "tests/dirt.jpg".to_string(),
                        ))
                        .unwrap()
                        .add(t)
                        .unwrap();
                }
            }
        }
        system_reg.push_send(SysC);
        system_reg.push_send(SysD);
        system_reg.push_send(CameraPlayerSystem::new());
        system_reg.push_send(RotateAll::default());
    });

    engine.run(&builder)
}
