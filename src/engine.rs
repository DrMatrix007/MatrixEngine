use std::{
    any::TypeId,
    cell::RefCell,
    collections::{HashMap, HashSet, VecDeque},
    sync::{
        atomic::{AtomicBool, AtomicU64},
        Arc,
    },
};

use crate::{
    components::{ComponentCollectionState, ComponentRegistry},
    query::{Action, QueryData},
    server_client::ServerBuilder,
    systems::{spawn_system, System, SystemCreator},
};

pub struct Engine {
    components: ComponentRegistry,
    quit: Arc<AtomicBool>,
    target_fps: Arc<AtomicU64>,
    systems: Vec<SystemCreator>,
}

impl Engine {
    pub fn with_registry(components: ComponentRegistry) -> Self {
        Self {
            components,
            quit: Arc::new(AtomicBool::new(false)),
            systems: Vec::new(),
            target_fps: Arc::new(AtomicU64::new(144)),
        }
    }

    pub fn run(mut self) {
        let mut v = Vec::new();
        let server_builder = ServerBuilder::default();
        let mut queries = VecDeque::new();

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

        while let Ok(req) = server.recv() {
            let (req, client) = req.unpack();
            match req {
                crate::query::QueryRequest::Request(req) => match self.query(req) {
                    Ok(data) => client.send(data).unwrap(),
                    Err(req) => {
                        queries.push_back((req, client));
                    }
                },
                crate::query::QueryRequest::Done(data) => {
                    self.handle(data);
                    let size = queries.len();
                    for _ in 0..size {
                        let Some((req,sender)) = queries.pop_front()else {break;};
                        match self.query(req) {
                            Ok(data) => {
                                sender.send(data).unwrap();
                            }
                            Err(req) => queries.push_back((req, sender)),
                        }
                    }
                }
            }
        }

        for i in v.into_iter() {
            if !i.is_finished() {
                i.join().unwrap();
            }
        }
    }

    pub fn insert_system<F: FnOnce() -> Box<dyn System> + Send + 'static>(&mut self, f: F) {
        self.systems.push(SystemCreator::new(Box::new(f)));
    }

    fn query(
        &mut self,
        req: HashSet<Action<TypeId>>,
    ) -> Result<QueryData, HashSet<Action<TypeId>>> {
        for i in req.iter() {
            match i {
                Action::Read(id) => {
                    let Some(vec) = self.components.read_vec(id) else {
                        return Ok(QueryData::default());
                    };
                    if let ComponentCollectionState::Taken = vec {
                        return Err(req);
                    }
                }
                Action::Write(id) => {
                    let Some(vec) = self.components.read_vec(id) else {
                        return Ok(QueryData::default());
                    };
                    let ComponentCollectionState::Available(_) = vec else {
                        return Err(req);
                    };
                }
            }
        }
        let mut ans = HashMap::default();
        for action in req.iter() {
            match action {
                Action::Read(id) => {
                    let Some(vec) = self.components.pop_vec(id) else {
                        return Ok(QueryData::default());
                    };
                    ans.insert(
                        *id,
                        match vec {
                            ComponentCollectionState::Available(b) => {
                                let arc = Arc::new(b);
                                self.components.insert_vec(
                                    *id,
                                    ComponentCollectionState::ReadOnly(arc.clone(), 1),
                                );
                                Action::Read(arc)
                            }
                            ComponentCollectionState::ReadOnly(b, mut count) => {
                                let arc = b.clone();
                                count += 1;
                                self.components.insert_vec(
                                    *action.id(),
                                    ComponentCollectionState::ReadOnly(arc.clone(), count),
                                );
                                Action::Read(arc)
                            }
                            _ => panic!(),
                        },
                    );
                }
                Action::Write(id) => {
                    let Some(vec) = self.components.pop_vec(id) else {
                        return Ok(QueryData::default());
                    };
                    ans.insert(
                        *id,
                        match vec {
                            ComponentCollectionState::Available(data) => {
                                self.components
                                    .insert_vec(*id, ComponentCollectionState::Taken);
                                Action::Write(data)
                            }
                            _ => return Err(req),
                        },
                    );
                }
            }
        }

        Ok(ans)
    }
    fn handle(&mut self, data: QueryData) {
        for (id, query_vec) in data.into_iter() {
            let Some(registry_vec) = self.components.pop_vec(&id) else {continue;};
            match query_vec {
                Action::Read(query_data) => {
                    drop(query_data);

                    match registry_vec {
                        ComponentCollectionState::ReadOnly(query_data, mut count) => {
                            count -= 1;

                            if count <= 0 {
                                self.components.insert_vec(
                                    id,
                                    ComponentCollectionState::Available(
                                        Arc::try_unwrap(query_data)
                                            .expect("this arc should be the last one!"),
                                    ),
                                );
                            } else {
                                self.components.insert_vec(
                                    id,
                                    ComponentCollectionState::ReadOnly(query_data, count),
                                )
                            };
                        }
                        _ => panic!("this should not happen"),
                    }
                }
                Action::Write(data) => {
                    if let ComponentCollectionState::Taken = registry_vec {
                        self.components
                            .insert_vec(id, ComponentCollectionState::Available(data))
                    } else {
                        panic!("this should not happen");
                    }
                }
            }
        }
    }
}
