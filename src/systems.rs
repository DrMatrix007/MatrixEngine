<<<<<<< HEAD
use std::{collections::VecDeque, future::Future};

use winit::event_loop::{ControlFlow, EventLoopWindowTarget};

use crate::{registry::Registry, resources::ResourceManager};

use super::registry::ComponentRegistry;

pub struct SystemArgs<'a> {
    control_flow: &'a mut ControlFlow,
    registry: &'a mut Registry,
    event_loop: &'a EventLoopWindowTarget<()>,
}

impl<'a> SystemArgs<'a> {
    pub fn new(
        control_flow: &'a mut ControlFlow,
        registry: &'a mut Registry,
        event_loop: &'a EventLoopWindowTarget<()>,
    ) -> Self {
        Self {
            control_flow,
            registry,
            event_loop,
        }
    }

    pub fn stop(&mut self) {
        *self.control_flow = ControlFlow::Exit;
    }
    pub fn components(&mut self) -> &mut ComponentRegistry {
        self.registry.get_component_registry_mut()
    }
    pub fn window_target(&self) -> &EventLoopWindowTarget<()> {
        self.event_loop
    }
    pub fn resources(&mut self) -> &mut ResourceManager {
        self.registry.get_resource_manager_mut()
=======
use std::{
    any::TypeId,
    collections::HashSet,
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        mpsc::Sender,
        Arc,
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

use crate::{
    queries::query::{Action, Query, QueryRequest, QueryResult},
    server_client::{Client, Request},
};

pub struct SystemArgs {
    quit: Arc<AtomicBool>,
    conn: Client<QueryRequest, QueryResult>,
}

impl SystemArgs {
    pub fn new(quit: Arc<AtomicBool>, sender: Sender<Request<QueryRequest, QueryResult>>) -> Self {
        Self {
            quit,
            conn: Client::new(sender),
        }
    }

    pub fn stop(&self) {
        self.quit.store(true, Ordering::Relaxed);
    }
    pub fn query<T>(&self, m: impl Iterator<Item = T>) -> Option<QueryResult>
    where
        HashSet<Action<TypeId>>: FromIterator<T>,
    {
        let map = m.collect();
        self.conn
            .send(QueryRequest::Query(Query { data: map }))
            .ok()?;

        self.conn.recv().ok().map(|x| x.unpack())
>>>>>>> temp
    }
}

pub trait System {
    fn update(&mut self, args: &mut SystemArgs);
<<<<<<< HEAD
    fn setup(&mut self, _: &mut SystemArgs) {}
}

impl<F: FnMut(&mut SystemArgs)> System for F {
    fn update(&mut self, args: &mut SystemArgs) {
        self(args);
    }
}

#[derive(Default)]
pub struct SystemCollection {
    queue: VecDeque<Box<dyn System>>,
    systems: Vec<Box<dyn System>>,
}

impl SystemCollection {
    pub fn update(&mut self, args: &mut SystemArgs) {
        while let Some(mut s) = self.queue.pop_back() {
            s.setup(args);
            self.systems.push(s);
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Box<dyn System>> {
        self.systems.iter()
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Box<dyn System>> {
        self.systems.iter_mut()
    }

    pub(crate) fn insert_system(&mut self, system: Box<dyn System>) {
        self.queue.push_back(system);
=======
}

pub(crate) fn spawn_system(
    sys: SystemCreator,
    target_fps: Arc<AtomicU64>,
    quit: Arc<AtomicBool>,
    sender: Sender<Request<QueryRequest, QueryResult>>,
) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut sys = sys.create();
        let mut target;
        let mut fps;
        let mut start = Instant::now();
        let mut len;
        let quit = quit.clone();
        let mut args = SystemArgs::new(quit.clone(), sender);
        while !quit.load(std::sync::atomic::Ordering::Relaxed) {
            sys.update(&mut args);
            fps = 1.0 / target_fps.load(Ordering::Relaxed) as f64;
            if fps.is_finite() {
                target = Duration::from_secs_f64(fps);
                len = Instant::now() - start;
                if len < target {
                    spin_sleep::sleep(target - len);
                }
                start = Instant::now();
            }
        }
    })
}

pub struct SystemCreator {
    creator: Box<dyn FnOnce() -> Box<dyn System> + Send + Sync>,
}

impl SystemCreator {
    pub fn default_function<T: System + Default + 'static>() -> Self {
        Self {
            creator: Box::new(|| Box::<T>::default()),
        }
    }
    pub fn with_function(f: impl FnOnce() -> Box<dyn System> + Send + Sync + 'static) -> Self {
        Self {
            creator: Box::new(f),
        }
    }

    pub fn create(self) -> Box<dyn System> {
        (self.creator)()
>>>>>>> temp
    }
}
