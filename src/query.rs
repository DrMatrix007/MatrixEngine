use std::{
    any::{Any, TypeId},
    collections::{hash_map, HashMap, HashSet},
    sync::Arc,
    vec,
};

use crate::{
    components::{Component, ComponentCollection},
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

pub type QueryCollectionData = Action<Arc<Box<dyn Any + Send + Sync>>, Box<dyn Any + Send + Sync>>;

// #[derive(Default,Debug)]
pub type QueryRawData = HashMap<TypeId, QueryCollectionData>;
pub type QueryRawDataRefMut<'a> = HashMap<&'a TypeId, &'a mut QueryCollectionData>;
pub trait QueryData {
    type Single<'b>;

    fn from_raw(
        vec: QueryRawDataRefMut<'_>,
    ) -> (
        HashMap<&'_ Entity, Self::Single<'_>>,
        QueryRawDataRefMut<'_>,
    );
    fn ids() -> Box<dyn Iterator<Item = Action<TypeId>>>;
}

impl<'a, T: Component + 'a + 'static> QueryData for &'a T {
    type Single<'b> = &'b T;

    fn from_raw<'b>(
        mut vec: QueryRawDataRefMut<'b>,
    ) -> (
        HashMap<&'b Entity, Self::Single<'b>>,
        QueryRawDataRefMut<'b>,
    ) {
        (
            match vec.remove(&TypeId::of::<T>()).unwrap() {
                Action::Read(data) => data
                    .downcast_ref::<ComponentCollection<T>>()
                    .unwrap()
                    .iter()
                    .collect::<HashMap<&'b Entity, &'b T>>(),
                Action::Write(data) => data
                    .downcast_ref::<ComponentCollection<T>>()
                    .unwrap()
                    .iter()
                    .collect::<HashMap<&'b Entity, &'b T>>(),
            },
            vec,
        )
    }

    fn ids() -> Box<dyn Iterator<Item = Action<TypeId>>> {
        Box::new(std::iter::once(Action::Read(TypeId::of::<T>())))
    }
}
impl<'a, T: Component + 'static> QueryData for &'a mut T {
    type Single<'b> = &'b mut T;

    fn from_raw<'b>(
        mut vec: QueryRawDataRefMut<'b>,
    ) -> (
        HashMap<&'b Entity, Self::Single<'b>>,
        QueryRawDataRefMut<'b>,
    ) {
        (
            match vec.remove(&TypeId::of::<T>()).unwrap() {
                Action::Read(_) => panic!(""),
                Action::Write(data) => data
                    .downcast_mut::<ComponentCollection<T>>()
                    .unwrap()
                    .iter_mut()
                    .collect::<HashMap<&'b Entity, &'b mut T>>(),
            },
            vec,
        )
    }

    fn ids() -> Box<dyn Iterator<Item = Action<TypeId>>> {
        Box::new(std::iter::once(Action::Write(TypeId::of::<T>())))
    }
}

macro_rules! impl_query_data {
    ($n:tt $t:tt $(,$ns:tt $ts:tt)* $(,)?) => {
        impl<'a, $t:QueryData+'a,$($ts:QueryData+'a,)*> QueryData for ($t,$($ts,)*) {
            type Single<'b> = ($t::Single<'b>, $($ts::Single<'b>,)*);

            #[allow(unused_mut,irrefutable_let_patterns)]
            fn from_raw<'b>(vec: QueryRawDataRefMut<'b>) -> (HashMap<&'b Entity,Self::Single<'b>>,QueryRawDataRefMut<'b>) {
                // let mut map = vec.iter_mut().collect::<HashMap::<_,_>>();

                let (mut $n,vec) = $t::from_raw(vec);
                $(let (mut $ns,vec) = $ts::from_raw(vec);)*
                let mut ans = HashMap::default();
                for (e,x) in $n.into_iter() {
                    let ($(Some($ns),)*) = ($($ns.remove(e),)*) else {
                        continue;
                    };
                    ans.insert(e,(x,$($ns),*));
                }

                (ans,vec)
            }
            fn ids() -> Box<dyn Iterator<Item = Action<TypeId>>> {
                Box::new($t::ids() $(.chain($ts::ids()))*)
            }
        }

    };
}
macro_rules! impl_query_data_all {
    ($n:tt $t:tt $(,$ns:tt $ts:tt)+ $(,)?) => {
        impl_query_data!($n $t $(,$ns $ts)+);
        impl_query_data_all!($($ns $ts,)*);
    };
    ($n:tt $t:tt $(,)?) => {
        impl_query_data!($n $t);
    };
}

impl_query_data_all!(a A,b B,c C, d D, e E, f F, g G, h H, i I, j J, k K, l L, m M, n N, o O, p P, q Q, r R, s S, t T, u U, v V, w W, x X, y Y, z Z);

impl QueryData for () {
    type Single<'b> = ();

    fn from_raw(
        vec: QueryRawDataRefMut<'_>,
    ) -> (
        HashMap<&'_ Entity, Self::Single<'_>>,
        QueryRawDataRefMut<'_>,
    ) {
        (Default::default(), vec)
    }

    fn ids() -> Box<dyn Iterator<Item = Action<TypeId>>> {
        Box::new(std::iter::empty())
    }
}
// impl_query_data_all!(a A,b B);

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
    Done(QueryRawData),
}
