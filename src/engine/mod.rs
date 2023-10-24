use std::{
    sync::{atomic::AtomicIsize, Arc},
    time::{Duration, Instant},
};

use tokio::sync::{Mutex, MutexGuard};
use winit::{
    event::{Event, StartCause},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder, EventLoopProxy},
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
    target_fps: AtomicIsize,
    engine_systems: SystemRegistry<ComponentQueryArgs>,
    engine_resources: Arc<Mutex<ResourceRegistry>>,
    event_loop: Option<EventLoop<EngineEvent>>,
}

impl Engine {
    pub fn new(runtime: impl Runtime<ComponentQueryArgs> + 'static, fps: isize) -> Self {
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

        let resources = self.engine_resources.clone().try_lock_owned().unwrap();

        let scene_registry = current_scene.try_lock_registry().unwrap();
        let mut args = ComponentQueryArgs::new(scene_registry, resources);

        self.runtime
            .add_available(&mut self.engine_systems, &mut args);

        self.runtime
            .add_available(current_scene.systems_mut(), &mut args);
        drop(args);

        let event_proxy = event_loop.create_proxy();

        let mut last_frame_time = Instant::now();

        event_loop.run(move |event, target, control_flow| {
            self.on_event(
                &mut current_scene,
                event,
                target,
                &event_proxy,
                control_flow,
                &mut last_frame_time,
            );
        });
    }

    fn on_event(
        &mut self,
        current_scene: &mut scenes::Scene,
        event: Event<'_, EngineEvent>,
        target: &winit::event_loop::EventLoopWindowTarget<EngineEvent>,
        proxy: &EventLoopProxy<EngineEvent>,
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

        // if let Event::UserEvent(EngineEvent::SystemDone(_, _)) = &event {
        //     if frame_duration < elapsed {
        //         self.runtime
        //             .add_available(&mut self.engine_systems, &mut args);

        //         self.runtime
        //             .add_available(current_scene.systems_mut(), &mut args);
        //     }
        // }

        let frame_duration = match self.target_fps.load(std::sync::atomic::Ordering::Relaxed) {
            fps if fps > 0 => Duration::from_secs_f64(1. / fps as f64),
            _ => Duration::ZERO,
        };

        if let Event::UserEvent(EngineEvent::SystemDone(_, _)) = &event {
        } else if let Event::NewEvents(reason) = &event {
            match reason {
                start_cause @ (StartCause::Init | StartCause::ResumeTimeReached { .. }) => {
                    println!("frame");
                    *last_frame_time = Instant::now();

                    self.runtime
                        .add_available(&mut self.engine_systems, &mut args);

                    self.runtime
                        .add_available(current_scene.systems_mut(), &mut args);
                    proxy
                        .send_event(EngineEvent::UpdateDeltaTime {
                            frame_start: Instant::now(),
                        })
                        .unwrap();

                    *control_flow = ControlFlow::WaitUntil(*last_frame_time + frame_duration);
                    //     StartCause::ResumeTimeReached { start, requested_resume }=> {
                    //         format!("{:?}",*requested_resume-*start)
                    //     }
                    //     a=>{format!("{a:?}")}
                    // }, frame_duration);
                }
                _ => {}
            }
        } else {
            // *control_flow = ControlFlow::Poll;
        }
        self.runtime.process_event(
            event,
            &mut args,
            frame_duration,
            &mut [&mut current_scene.systems_mut(), &mut self.engine_systems],
        );

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

    pub fn event_loop(&self) -> Option<&EventLoop<EngineEvent>> {
        self.event_loop.as_ref()
    }

    pub fn event_loop_mut(&mut self) -> Option<&mut EventLoop<EngineEvent>> {
        self.event_loop.as_mut()
    }

    pub fn lock_engine_resources(&self) -> MutexGuard<'_, ResourceRegistry> {
        self.engine_resources.blocking_lock()
    }
}
