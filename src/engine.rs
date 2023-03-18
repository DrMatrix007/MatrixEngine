use std::{
    any::{Any, TypeId},
    collections::{HashMap, HashSet, VecDeque},
    sync::{
        atomic::{AtomicBool, AtomicU64},
        Arc,
    },
};

use crate::{
    components::{ComponentCollectionState, ComponentRegistry, ComponentRegistryBuilder},
    query::{Action, QueryRawData},
    server_client::ServerBuilder,
    systems::{spawn_system, System, SystemCreator},
};

pub struct EngineArgs {
    pub component_registry: ComponentRegistry,
    pub target_fps: u64,
    pub systems: Vec<SystemCreator>,
}

pub struct Engine {
    components: ComponentRegistry,
    quit: Arc<AtomicBool>,
    target_fps: Arc<AtomicU64>,
    systems: Vec<SystemCreator>,
}

impl Engine {
    pub fn new(args: EngineArgs) -> Self {
        Self {
            components: args.component_registry,
            quit: Arc::new(AtomicBool::new(false)),
            systems: args.systems,
            target_fps: Arc::new(AtomicU64::new(args.target_fps)),
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

    fn query(
        &mut self,
        req: HashSet<Action<TypeId>>,
    ) -> Result<QueryRawData, HashSet<Action<TypeId>>> {
        for i in req.iter() {
            match i {
                Action::Read(id) => {
                    let Some(vec) = self.components.read_vec(id) else {
                        return Ok(QueryRawData::default());
                    };
                    if let ComponentCollectionState::Taken = vec {
                        return Err(req);
                    }
                }
                Action::Write(id) => {
                    let Some(vec) = self.components.read_vec(id) else {
                        return Ok(QueryRawData::default());
                    };
                    let ComponentCollectionState::Available(_) = vec else {
                        return Err(req);
                    };
                }
            }
        }
        let mut ans = HashMap::<
            TypeId,
            Action<Arc<Box<dyn Any + Send + Sync>>, Box<dyn Any + Send + Sync>>,
        >::default();
        for action in req.iter() {
            match action {
                Action::Read(id) => {
                    let Some(vec) = self.components.pop_vec(id) else {
                        return Ok(QueryRawData::default());
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
                        return Ok(QueryRawData::default());
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
    fn handle(&mut self, data: QueryRawData) {
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

pub struct EngineBuilder {
    component_registry: ComponentRegistry,
    systems: Vec<SystemCreator>,
    target_fps: u64,
}

impl EngineBuilder {
    pub fn new() -> Self {
        Self {
            component_registry: Default::default(),
            systems: Default::default(),
            target_fps: 60,
        }
    }
    pub fn with_fps(mut self, fps: u64) -> Self {
        self.target_fps = fps;
        self
    }
    pub fn with_system<T: System + Send + 'static>(mut self, t: T) -> Self {
        self.systems
            .push(SystemCreator::new(Box::new(move || Box::new(t))));
        self
    }

    pub fn with_registry(mut self, r: ComponentRegistry) -> Self {
        self.component_registry = r;
        self
    }
    pub fn with_registry_builder<F: FnOnce(&mut ComponentRegistryBuilder)>(mut self, f: F) -> Self {
        let mut b = Default::default();
        f(&mut b);
        self.component_registry = b.build();
        self
    }

    pub fn build(self) -> Engine {
        Engine::new(EngineArgs {
            component_registry: self.component_registry,
            target_fps: self.target_fps,
            systems: self.systems,
        })
    }
}

impl Default for EngineBuilder {
    fn default() -> Self {
        Self::new()
    }
}
