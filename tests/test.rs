use std::{f32::consts::PI, time::Duration};

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
        scenes::entities::entity_builder::EntityBuilder,
        systems::{query::resources::WriteR, SystemControlFlow}, runtime::SingleThreaded,
    },
    math::{matrices::Vector3, vectors::Vector3D},
    renderer::{
        matrix_renderer::{
            camera::CameraResource,
            render_object::RenderObject,
            renderer_system::{MatrixRendererResource, MatrixRendererSystem, RendererResourceArgs},
        },
        pipelines::{
            instance_manager::VertexStructure, structures::plain::Plain, transform::Transform,
        },
    },
};
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
            if event.is_pressed(winit::event::VirtualKeyCode::A) {
                // spin_sleep::sleep(Duration::from_secs_f64(3.));
                println!("dam");
            }
        }

        SystemControlFlow::Continue
    }
}

struct CameraPlayerSystem {
    theta: f32,
    phi: f32,
}

impl CameraPlayerSystem {
    fn new() -> Self {
        Self { phi: 0., theta: 0. }
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

        let speed = 4.0;
        let rotate_speed = PI / 4.0;

        let dt = events.delta_time().as_secs_f32();

        for window_events in events.all_window_events() {
            if window_events.is_pressed(winit::event::VirtualKeyCode::A) {
                *delta.x_mut() -= speed;
            }
            if window_events.is_pressed(winit::event::VirtualKeyCode::D) {
                *delta.x_mut() += speed;
            }
            if window_events.is_pressed(winit::event::VirtualKeyCode::W) {
                *delta.z_mut() -= speed;
            }
            if window_events.is_pressed(winit::event::VirtualKeyCode::S) {
                *delta.z_mut() += speed;
            }
            if window_events.is_pressed(winit::event::VirtualKeyCode::Space) {
                *delta.y_mut() += speed;
            }
            if window_events.is_pressed(winit::event::VirtualKeyCode::C) {
                *delta.y_mut() -= speed;
            }
        }
        delta = cam.camera().rotation.euler_into_rotation_matrix3() * delta * dt;
        let (a, b) = events.mouse_delta();
        self.theta += (a as f32) * dt * rotate_speed;
        self.phi += (b as f32) * dt * rotate_speed;
        *cam.camera_mut().rotation.y_mut() = self.theta;
        *cam.camera_mut().rotation.x_mut() = self.phi;
        cam.camera_mut().position += delta;

        SystemControlFlow::Continue
    }
}

fn main() {
    // let runtime = MultiThreaded::new(4);
    let runtime = SingleThreaded::new();

    let mut engine = Engine::new(runtime, 144);

    let window = WindowBuilder::new()
        .build(engine.event_loop().unwrap())
        .unwrap();

    let mut renderer_resource = MatrixRendererResource::new(RendererResourceArgs {
        background_color: Color::GREEN,
        window,
    });

    engine
        .lock_engine_resources()
        .insert(CameraResource::new(&mut renderer_resource));

    engine.lock_engine_resources().insert(renderer_resource);

    engine
        .engine_systems_mut()
        .push_send(MatrixRendererSystem);

    let builder = SceneBuilder::new(|scene_reg, system_reg| {
        for i in 1..100 {
            let mut t = Transform::identity();
            t.apply_position_diff(Vector3::from([[i as _, 0., 0.]]));
            EntityBuilder::new(scene_reg.components_mut())
                .add(A)
                .unwrap()
                .add(RenderObject::new(Plain, "tests/dirt.jpg".to_string()))
                .unwrap()
                .add(t)
                .unwrap();
        }

        system_reg.push_send(SysC);
        system_reg.push_send(SysD);
        system_reg.push_send(CameraPlayerSystem::new());
    });

    engine.run(&builder)
}
