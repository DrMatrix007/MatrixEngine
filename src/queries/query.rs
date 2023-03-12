use std::{
    any::TypeId,
    collections::{HashMap, HashSet}, cell::{Ref, RefCell},
};

use crate::components::IComponentCollection;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum Action<Read, Write = Read> {
    Read(Read),
    Write(Write),
}

// impl<T> Action<T> {
//     pub fn read(&self) -> Option<&T> {
//         if let Action::Read(data) = self {
//             Some(data)
//         } else {
//             None
//         }
//     }
//     pub fn write(&mut self) -> Option<&mut T> {
//         if let Self::Write(data) = self {
//             Some(data)
//         } else {
//             None
//         }
//     }
// }

impl<Read,Write> Action<Read,Write> {
        // pub fn unpack(self) -> T {
        //     match self {
        //         Action::Read(data) | Action::Write(data) => data,
        //     }
        // }
        pub fn into_read(self) -> Option<Read> {
            if let Action::Read(data) = self {
                Some(data)
            } else {
                None
            }
        }
        pub fn into_write(self) -> Option<Write> {
            if let Self::Write(data) = self {
                Some(data)
            } else {
                None
            }
        }
        pub fn read(&self) -> Option<&Read> {
            if let Action::Read(data) = self {
                Some(data)
            } else {
                None
            }
        }
        pub fn write(&self) -> Option<&Write> {
            if let Self::Write(data) = self {
                Some(data)
            } else {
                None
            }
        }
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
pub struct QueryData {
    pub data: HashMap<TypeId, Action<Box<dyn IComponentCollection>,RefCell<Box<dyn IComponentCollection>>>>,
}
impl QueryData {
    pub fn with(data: HashMap<TypeId, Action<Box<dyn IComponentCollection>,RefCell<Box<dyn IComponentCollection>>>>) -> QueryData {
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
