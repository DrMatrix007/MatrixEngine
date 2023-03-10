use std::{
    any::TypeId,
    collections::{HashMap, HashSet},
};

use crate::components::IComponentCollection;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum Action<A, B = A> {
    Read(A),
    Write(B),
}

impl<A, B> Action<A, B> {
    pub fn read(&self) -> Option<&A> {
        match self {
            Action::Read(data) => Some(data),
            Action::Write(_) => None,
        }
    }
    pub fn write(&mut self) -> Option<&mut B> {
        if let Self::Write(data) = self {
            Some(data)
        } else {
            None
        }
    }
}

impl<T> Action<T> {
    pub fn unpack(self) -> T {
        match self {
            Action::Read(data) | Action::Write(data) => data,
        }
    }
    // pub fn read(&self) -> &T {
    //     match self {
    //         Action::Read(data) | Action::Write(data) => data,
    //     }
    // }
    // pub fn write(&mut self) -> Option<&mut T> {
    //     if let Self::Write(data) = self {
    //         Some(data)
    //     } else {
    //         None
    //     }
    // }
}

impl Action<TypeId> {
    pub fn id(&self) -> TypeId {
        match self {
            Action::Read(id) | Action::Write(id) => *id,
        }
    }
}

#[derive(Clone)]
pub struct Query {
    pub data: HashSet<Action<TypeId>>,
}

// unsafe impl Send for Query {}
// unsafe impl Sync for Query {}

#[derive(Default)]
pub struct  QueryData {
    pub data: HashMap<TypeId, Action<Box<dyn IComponentCollection>>>,
}
impl QueryData {
    pub fn with(data: HashMap<TypeId, Action<Box<dyn IComponentCollection>>>) -> QueryData {
        Self { data }
    }
}

pub enum QueryResult {
    Ok { data: QueryData },
    Empty,
}

pub enum QueryRequest {
    Query(Query),
    QueryDone(QueryData),
}

// pub enum Query
