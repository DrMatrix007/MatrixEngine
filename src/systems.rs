use std::{
    any::TypeId,
    collections::HashSet,
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        mpsc::{SendError, Sender},
        Arc,
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

use crate::{
    queries::query::{Action, Query, QueryData, QueryRequest, QueryResult},
    server_client::{Client, Request, Response},
};

pub struct SystemArgs {
    quit: Arc<AtomicBool>,
    conn: Client<QueryRequest, QueryResult>,
}
pub struct QueryCollection {
    data: QueryResult,
    server: Sender<Request<QueryRequest, QueryResult>>,
    clinet_sender: Sender<Response<QueryResult>>,
}

impl QueryCollection {
    pub fn finish(self) -> Result<(), SendError<Request<QueryRequest, QueryResult>>> {
        if let QueryResult::Ok { data } = self.data {
            self.server.send(Request::new(
                QueryRequest::QueryDone(data),
                self.clinet_sender.clone(),
            ))
        } else {
            Ok(())
        }
    }
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
    pub fn query<T>(&self, m: impl Iterator<Item = T>) -> Option<QueryCollection>
    where
        HashSet<Action<TypeId>>: FromIterator<T>,
    {
        let map = m.collect();
        self.conn
            .send(QueryRequest::Query(Query { data: map }))
            .ok()?;

        self.conn.recv().ok().map(|x| QueryCollection {
            data: x.unpack(),
            server: self.conn.server_sender(),
            clinet_sender: self.conn.sender(),
        })
    }
}

pub trait System {
    fn update(&mut self, args: &mut SystemArgs);
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
    }
}
