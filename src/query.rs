use std::{
    any::TypeId,
    cell::RefCell,
    collections::{hash_map, HashMap, HashSet, VecDeque},
    io::Read,
    sync::Arc,
    vec,
};

use crate::{
    components::{Component, IComponentCollection},
    entity::Entity,
};
#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub enum Action<Read, Write = Read> {
    Read(Read),
    Write(Write),
}
impl<Read: Iterator<Item = A>, Write: Iterator<Item = B>, A, B> Action<Read, Write> {
    pub fn into_actions(self) -> vec::IntoIter<Action<A, B>> {
        match self {
            Action::Read(iter) => iter
                .map(|x| Action::<A, B>::Read(x))
                .collect::<Vec<_>>()
                .into_iter(),
            Action::Write(iter) => iter
                .map(|x| Action::<A, B>::Write(x))
                .collect::<Vec<_>>()
                .into_iter(),
        }
    }
}

impl<Read, Write> Action<Read, Write> {
    pub fn try_read_only(&self) -> Option<&Read> {
        match self {
            Action::Read(data) => Some(data),
            Action::Write(_) => None,
        }
    }
    pub fn try_write(&mut self) -> Option<&mut Write> {
        match self {
            Action::Write(data) => Some(data),
            Action::Read(_) => None,
        }
    }
}

impl<T> Action<T> {
    pub fn read(&self) -> &T {
        match self {
            Action::Read(t) | Action::Write(t) => t,
        }
    }
}
impl<'a, T> Action<&'a T, &'a mut T> {
    pub fn read(&'a self) -> &'a T {
        match self {
            Action::Read(r) => r,
            Action::Write(r) => r,
        }
    }
    pub fn write(&'a mut self) -> Option<&'a mut T> {
        if let Action::Write(r) = self {
            Some(r)
        } else {
            None
        }
    }
}
impl Action<TypeId> {
    pub(crate) fn id(&self) -> &TypeId {
        match self {
            Action::Read(id) | Action::Write(id) => id,
        }
    }
}

pub type QueryCollectionData =
    Action<Arc<Box<dyn IComponentCollection>>, Box<dyn IComponentCollection>>;

// #[derive(Default,Debug)]
pub type QueryData = HashMap<TypeId, QueryCollectionData>;

pub trait QueryIterable {
    fn create_iter(&mut self) -> QueryIterableCollection;
}
impl QueryIterable for QueryData {
    fn create_iter(&mut self) -> QueryIterableCollection {
        let mut data = HashMap::<Entity, HashMap<TypeId, ComponentRef>>::default();

        for (id, vec) in self.iter_mut() {
            match vec {
                Action::Read(vec) => {
                    for (e, comp) in vec.iter() {
                        data.entry(*e).or_default().insert(*id, Action::Read(comp));
                    }
                }
                Action::Write(vec) => {
                    for (e, comp) in vec.iter_mut() {
                        data.entry(*e).or_default().insert(*id, Action::Write(comp));
                    }
                }
            }
        }

        QueryIterableCollection {
            data: data.into_iter(),
        }
    }
}

trait QueryGroup {
    // fn from_vec(vec:Vec<>)
}

type ComponentRef<'a> = Action<&'a dyn Component, &'a mut dyn Component>;

pub struct QueryIterableCollection<'a> {
    data: hash_map::IntoIter<Entity, HashMap<TypeId, ComponentRef<'a>>>,
}

impl<'a> Iterator for QueryIterableCollection<'a> {
    type Item = (Entity, HashMap<TypeId, ComponentRef<'a>>);

    fn next(&mut self) -> Option<Self::Item> {
        self.data.next()
    }
}

pub type Query = HashSet<Action<TypeId>>;
#[derive(Debug)]
pub enum QueryRequest {
    Request(Query),
    Done(QueryData),
}
