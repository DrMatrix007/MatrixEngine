use std::{
    sync::{atomic::AtomicUsize, Arc},
    time::{Duration, Instant},
};

use log::info;
use tokio::sync::Mutex;
use winit::{
    event::Event,
    event_loop::{EventLoop, EventLoopBuilder},
};

use self::{
    events::engine_event::EngineEvent,
    runtime::Runtime,
    scenes::{resources::resource_registry::ResourceRegistry, scene_builder::SceneBuilder},
    systems::{query::ComponentQueryArgs, system_registry::SystemRegistry},
};

pub mod events;
pub mod runtime;
pub mod scenes;
pub mod systems;

pub struct Engine {
    runtime: Box<dyn Runtime<ComponentQueryArgs>>,
    target_fps: AtomicUsize,
    engine_systems: SystemRegistry<ComponentQueryArgs>,
    engine_resources: Arc<Mutex<ResourceRegistry>>,
    event_loop: Option<EventLoop<EngineEvent>>,
}

impl Engine {
    pub fn new(runtime: impl Runtime<ComponentQueryArgs> + 'static, fps: usize) -> Self {
        Self {
            runtime: Box::new(runtime),
            target_fps: fps.into(),
            engine_systems: Default::default(),
            engine_resources: Default::default(),
            event_loop: Some(EventLoopBuilder::<EngineEvent>::with_user_event().build()),
        }
    }

    pub fn run(mut self, builder: &SceneBuilder) -> ! {
        let mut current_scene = builder.build();

        let event_loop = self.event_loop.take().unwrap();

        self.runtime.use_event_loop_proxy(event_loop.create_proxy());

        let mut last_frame_time = Instant::now();

        let resources = self.engine_resources.clone().try_lock_owned().unwrap();

        let scene_registry = current_scene.try_lock_registry().unwrap();
        let mut args = ComponentQueryArgs::new(scene_registry, resources);

        self.runtime
            .add_available(&mut self.engine_systems, &mut args);

        self.runtime
            .add_available(current_scene.systems_mut(), &mut args);
        drop(args);
        event_loop.run(move |event, target, control_flow| {
            self.on_event(
                &mut current_scene,
                &event,
                target,
                control_flow,
                &mut last_frame_time,
            );
        });
    }

    fn on_event(
        &mut self,
        current_scene: &mut scenes::Scene,
        event: &Event<'_, EngineEvent>,
        target: &winit::event_loop::EventLoopWindowTarget<EngineEvent>,
        control_flow: &mut winit::event_loop::ControlFlow,
        last_frame_time: &mut Instant,
    ) {
        let resources = self.engine_resources.clone().try_lock_owned().unwrap();

        current_scene.process_event(
            &event,
            target, 
            self.runtime.as_mut(),
            resources,
            control_flow,
        );
        let resources = self.engine_resources.clone().try_lock_owned().unwrap();

        let scene_registry = current_scene.try_lock_registry().unwrap();
        let mut args = ComponentQueryArgs::new(scene_registry, resources);

        let frame_duration = Duration::from_secs(1)
            / self.target_fps.load(std::sync::atomic::Ordering::Relaxed) as u32;
        let elapsed = Instant::now().duration_since(*last_frame_time);
        if frame_duration > elapsed {
            if self.runtime.is_done() {
                spin_sleep::sleep(frame_duration - elapsed);
                self.runtime
                    .add_available(&mut self.engine_systems, &mut args);

                self.runtime
                    .add_available(current_scene.systems_mut(), &mut args);
                *last_frame_time = Instant::now();
            }
        } else {
            self.runtime
                .add_available(&mut self.engine_systems, &mut args);

            self.runtime
                .add_available(current_scene.systems_mut(), &mut args);
        }

        if let Event::UserEvent(event) = &event {
            self.runtime.process_engine_event(
                event,
                &mut args,
                &mut [&mut current_scene.systems_mut(), &mut self.engine_systems],
            );
        }

        // self.runtime.cleanup_systems(
        //     &mut args,
        //     &mut [&mut current_scene.systems_mut(), &mut self.engine_systems],
        // );
    }

    pub fn engine_systems(&self) -> &SystemRegistry<ComponentQueryArgs> {
        &self.engine_systems
    }

    pub fn engine_systems_mut(&mut self) -> &mut SystemRegistry<ComponentQueryArgs> {
        &mut self.engine_systems
    }
}
