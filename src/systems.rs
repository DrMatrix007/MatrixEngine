use std::{
    any::TypeId,
    collections::{hash_map, HashMap, HashSet},
    marker::PhantomData,
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        mpsc::Sender,
        Arc,
    },
    time::{Duration, Instant},
};

use crate::{
    entity::Entity,
    query::{Action, QueryData, QueryRawData, QueryRequest},
    server_client::{Client, Request},
};

#[derive(Debug)]
pub struct QueryResult<T: QueryData> {
    data: QueryRawData,
    marker: PhantomData<T>,
    // sender: &'a mut Client<QueryRequest, QueryRawData>,
}

pub struct QueryResultData<'b, Data: QueryData> {
    maker: PhantomData<&'b Data>,
    data: HashMap<&'b Entity, Data::Single<'b>>,
}

impl<'b, Data: QueryData> IntoIterator for QueryResultData<'b, Data> {
    fn into_iter(self) -> hash_map::IntoIter<&'b Entity, Data::Single<'b>> {
        self.data.into_iter()
    }

    type Item = (&'b Entity, Data::Single<'b>);

    type IntoIter = hash_map::IntoIter<&'b Entity, Data::Single<'b>>;
}

impl<'b, Data: QueryData> QueryResultData<'b, Data> {
    pub fn iter<'a>(&'a self) -> hash_map::Iter<'a, &'b Entity, Data::Single<'b>> {
        self.data.iter()
    }
    pub fn iter_mut<'a>(&'a mut self) -> hash_map::IterMut<'a, &'b Entity, Data::Single<'b>> {
        self.data.iter_mut()
    }
}

impl<'b, Data: QueryData> QueryResultData<'b, Data> {}

impl<'a: 'b, 'b, Data: QueryData> From<&'a mut QueryResult<Data>> for QueryResultData<'b, Data> {
    fn from(value: &'a mut QueryResult<Data>) -> Self {
        Self {
            data: Data::from_raw(value.data.iter_mut().collect::<HashMap<_, _>>()).0,
            maker: PhantomData,
        }
    }
}

impl<T: QueryData> QueryResult<T> {
    pub fn new(data: QueryRawData) -> Self {
        Self {
            data,
            marker: PhantomData,
        }
    }
    pub fn data_mut(&mut self) -> &mut QueryRawData {
        &mut self.data
    }

    pub fn iter_mut(&mut self) -> QueryResultData<'_, T> {
        QueryResultData::from(self)
    }
}

pub struct SystemArgs {
    quit: Arc<AtomicBool>,
    client: Client<QueryRequest, QueryRawData>,
}

impl SystemArgs {
    pub fn new(quit: Arc<AtomicBool>, server: Sender<Request<QueryRequest, QueryRawData>>) -> Self {
        Self {
            quit,
            client: Client::new(server),
        }
    }
    pub fn submit<T: QueryData>(&self, data: QueryResult<T>) {
        self.client.send(QueryRequest::Done(data.data)).unwrap();
    }
    pub fn query<T>(&mut self) -> QueryResult<T>
    where
        T: QueryData, // T: IntoIterator<Item = Action<TypeId>>,
    {
        let set = T::ids().collect::<HashSet<Action<TypeId>>>();
        self.client.send(QueryRequest::Request(set)).unwrap();
        QueryResult::new(self.client.recv().unwrap().unpack())
    }
    pub fn stop(&self) {
        self.quit.store(true, Ordering::Relaxed);
    }
    pub fn clone_quit(&self) -> Arc<AtomicBool> {
        self.quit.clone()
    }
}

pub trait System {
    fn update(&mut self, args: &mut SystemArgs);
}

pub struct SystemBuilder {
    f: Box<dyn FnOnce() -> Box<dyn System> + Send>,
}

impl SystemBuilder {
    pub fn new(f: Box<dyn FnOnce() -> Box<dyn System> + Send>) -> Self {
        Self { f }
    }
    pub fn build(self) -> Box<dyn System> {
        (self.f)()
    }
}

pub trait SystemRunner: Send {
    fn run(self, args: SystemRunnerArgs);
}
pub struct SystemRunnerArgs {
    system_args: SystemArgs,
    target_fps: Arc<AtomicU64>,
}

impl SystemRunnerArgs {
    pub fn new(system_args: SystemArgs, target_fps: Arc<AtomicU64>) -> Self {
        Self {
            system_args,
            target_fps,
        }
    }
    pub fn unpack(self) -> (SystemArgs, Arc<AtomicU64>) {
        (self.system_args, self.target_fps)
    }
}

pub struct SystemGroupRunner {
    systems: Vec<SystemBuilder>,
}

impl SystemRunner for SystemGroupRunner {
    fn run(mut self, args: SystemRunnerArgs) {
        let target_fps = args.target_fps;
        let mut args = args.system_args;
        let mut systems = Vec::new();
        while let Some(sys) = self.systems.pop() {
            systems.push(sys.build());
        }
        let quit = args.quit.clone();

        let mut loop_handler = LoopHandler::new(target_fps);

        while !quit.load(std::sync::atomic::Ordering::Relaxed) {
            for sys in systems.iter_mut() {
                sys.update(&mut args);
            }
            loop_handler.wait();
        }
    }
}

impl SystemGroupRunner {
    pub fn new<T: IntoIterator<Item = SystemBuilder>>(data: T) -> Self {
        Self {
            systems: data.into_iter().collect(),
        }
    }
}

pub trait ToSystemBuilder {
    fn to_builder(self) -> SystemBuilder;
}

impl<T: System + Send + 'static> ToSystemBuilder for T {
    fn to_builder(self) -> SystemBuilder {
        SystemBuilder {
            f: Box::new(move || Box::new(self)),
        }
    }
}
pub struct LoopHandler {
    target: Duration,
    fps: f64,
    start: Instant,
    len: Duration,
    target_fps: Arc<AtomicU64>,
}

impl LoopHandler {
    pub fn new(target_fps: Arc<AtomicU64>) -> Self {
        Self {
            target: Duration::ZERO,
            fps: 0.0,
            start: Instant::now(),
            len: Duration::ZERO,
            target_fps,
        }
    }
    pub fn wait(&mut self) {
        self.fps = 1.0 / self.target_fps.load(Ordering::Relaxed) as f64;
        if self.fps.is_finite() {
            self.target = Duration::from_secs_f64(self.fps);
            self.len = Instant::now() - self.start;
            if self.len < self.target {
                spin_sleep::sleep(self.target - self.len);
            }
            self.start = Instant::now();
        }
    }
}
