use std::{
    any::TypeId,
    cell::{RefCell, RefMut, Ref},
    collections::{HashSet, hash_map},
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        mpsc::{SendError, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant}, borrow::{BorrowMut, Borrow},
};

use crate::{
    query::{Action, QueryData, QueryRequest},
    server_client::{Client, Request},
};

#[derive(Debug)]
pub struct QueryResult<'a> {
    data: QueryData,
    sender: &'a mut Client<QueryRequest, QueryData>,
}

impl<'a> QueryResult<'a> {
    pub fn new(data: QueryData, sender: &'a mut Client<QueryRequest, QueryData>) -> Self {
        Self { data, sender }
    }
    pub fn data_mut(&mut self) -> &mut QueryData {
        &mut self.data
    }
    pub fn finish(self) {
        self.sender.send(QueryRequest::Done(self.data)).unwrap();
    }
}

pub struct SystemArgs {
    quit: Arc<AtomicBool>,
    client: Client<QueryRequest, QueryData>,
}

impl SystemArgs {
    pub fn new(quit: Arc<AtomicBool>, server: Sender<Request<QueryRequest, QueryData>>) -> Self {
        Self {
            quit,
            client: Client::new(server),
        }
    }

    pub fn query<T>(&mut self, actions: T) -> QueryResult<'_>
    where
        T: Iterator<Item = Action<TypeId>>,
    {
        let set = actions.collect::<HashSet<Action<TypeId>>>();
        self.client.send(QueryRequest::Request(set)).unwrap();
        QueryResult {
            data: self.client.recv().unwrap().unpack(),
            sender: &mut self.client,
        }
    }
    pub fn stop(&self) {
        self.quit.store(true, Ordering::Relaxed);
    }
}

pub trait System {
    fn update(&mut self, args: &mut SystemArgs);
}

pub struct SystemCreator {
    f: Box<dyn FnOnce() -> Box<dyn System> + Send>,
}

impl SystemCreator {
    pub fn new(f: Box<dyn FnOnce() -> Box<dyn System> + Send>) -> Self {
        Self { f }
    }
    pub fn create(self) -> Box<dyn System> {
        (self.f)()
    }
}

pub(crate) fn spawn_system(
    sys: SystemCreator,
    target_fps: Arc<AtomicU64>,
    quit: Arc<AtomicBool>,
    sender: Sender<Request<QueryRequest, QueryData>>,
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
