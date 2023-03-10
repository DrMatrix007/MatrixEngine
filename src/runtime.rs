<<<<<<< HEAD
use winit::event_loop::EventLoop;

use crate::systems::SystemCollection;

use super::{
    registry::Registry,
    systems::{System, SystemArgs},
=======
use std::{
    collections::VecDeque,
    sync::{
        atomic::{AtomicBool, AtomicU64},
        Arc,
    },
};

use crate::{
    queries::query::{Query, QueryRequest, QueryResult},
    registry::QueryError,
    server_client::{RequestSender, ServerBuilder},
};

use super::{
    registry::Registry,
    systems::{spawn_system, SystemCreator},
>>>>>>> temp
};

#[derive(Default)]
pub struct Runtime {
<<<<<<< HEAD
    event_loop: EventLoop<()>,
    registry: Registry,
    systems: SystemCollection,
}

impl Runtime {
    pub fn new(r: Registry) -> Self {
        Self {
            event_loop: EventLoop::default(),
            registry: r,
            systems: SystemCollection::default(),
        }
    }
    pub fn window_target(&self) -> &EventLoop<()> {
        &self.event_loop
    }
    pub fn run(mut self) -> ! {
        self.event_loop.run(move |_, target, control_flow| {
            let mut args = SystemArgs::new(control_flow, &mut self.registry, target);
            self.systems.update(&mut args);

            for i in self.systems.iter_mut() {
                i.update(&mut args)
            }
        });
    }
    pub fn registry(&self) -> &Registry {
        &self.registry
    }
    pub fn registry_mut(&mut self) -> &mut Registry {
        &mut self.registry
    }
    pub fn insert_system<T: System + 'static>(&mut self, system: T) {
        self.systems.insert_system(Box::new(system));
=======
    registry: Registry,
    systems: Vec<SystemCreator>,
    quit: Arc<AtomicBool>,
    target_fps: Arc<AtomicU64>,
}
pub enum HandleDone {
    Data(QueryResult),
    HandledQueryData,
}

impl Runtime {
    pub fn with_registry(r: Registry) -> Self {
        Self {
            registry: r,
            quit: Arc::new(AtomicBool::new(false)),
            systems: vec![],
            target_fps: Arc::new(AtomicU64::new(144)),
        }
    }

    pub fn run(mut self) {
        let mut v = Vec::new();
        let server_builder = ServerBuilder::default();
        let mut queries = VecDeque::<(Query, RequestSender<QueryResult>)>::new();

        while let Some(sys) = self.systems.pop() {
            let main = server_builder.sender();

            v.push(spawn_system(
                sys,
                self.target_fps.clone(),
                self.quit.clone(),
                main,
            ));
        }
        let server = server_builder.build();
        while let Ok(request) = server.recv() {
            let (data, response) = request.unpack();
            match self.handle_query(data) {
                Ok(res) => match res {
                    HandleDone::Data(data) => {
                        response.send(data).unwrap();
                    }
                    HandleDone::HandledQueryData => {
                        for _ in 0..queries.len() {
                            let (query, sender) = queries.pop_front().unwrap();
                            match self.handle_query(QueryRequest::Query(query)) {
                                Ok(data) => {
                                    match data {
                                        HandleDone::Data(data) => sender.send(data).unwrap(),
                                        HandleDone::HandledQueryData => {
                                            panic!("should not be here")
                                        }
                                    };
                                }
                                Err((_, q)) => {
                                    queries.push_back((q, sender));
                                }
                            };
                        }
                    }
                },
                Err((err, q)) => {
                    match err {
                        QueryError::CantRead => panic!(),
                        QueryError::Taken => queries.push_back((q, response)),
                        QueryError::Empty => {
                            response.send(QueryResult::Empty).unwrap();
                        }
                    };
                }
            };
        }

        for i in v {
            i.join().unwrap();
        }
    }

    pub fn insert_system(&mut self, a: SystemCreator) {
        self.systems.push(a);
    }

    fn handle_query(&mut self, data: QueryRequest) -> Result<HandleDone, (QueryError, Query)> {
        Ok(match data {
            QueryRequest::Query(q) => match self.registry.components.query(&q) {
                Ok(e) => HandleDone::Data(QueryResult::Ok { data: e }),
                Err(e) => match e {
                    QueryError::CantRead => return Err((e, q)),
                    QueryError::Taken => return Err((e, q)),
                    QueryError::Empty => HandleDone::Data(QueryResult::Empty),
                },
            },
            QueryRequest::QueryDone(data) => {
                self.registry.components.update_query_result(data);
                HandleDone::HandledQueryData
            }
        })
>>>>>>> temp
    }
}
