use crate::par::storage::Storage;

use super::{
    dispatcher::{DispatchError, DispatchedFunction, Dispatcher},
    query::{Query, QueryArgs, QueryError, QuerySend},
};

pub struct SystemArgs {
    query: QueryArgs,
}

impl<'a> SystemArgs {
    pub fn query_mut(&mut self) -> &mut QueryArgs {
        &mut self.query
    }

    pub fn query(&self) -> &QueryArgs {
        &self.query
    }
}

pub trait System {
    type Query: Query;

    fn run(&mut self, q: <Self::Query as Query>::Target);
}

pub struct BoxedSystem<Query> {
    data: Storage<dyn System<Query = Query>>,
}

pub struct BoxedSystemFunction<Out = ()>(Box<dyn FnOnce() -> Out>);

impl<Out> BoxedSystemFunction<Out> {
    pub fn new(f: impl FnOnce() -> Out + 'static) -> Self {
        Self(Box::new(f))
    }
}
pub struct BoxedSendSystemFunction<Out = ()>(Box<dyn FnOnce() -> Out + Send>);

impl<Out> BoxedSendSystemFunction<Out> {
    pub fn new(f: impl FnOnce() -> Out + Send + 'static) -> Self {
        Self(Box::new(f))
    }
}

impl<T> DispatchedFunction for BoxedSystemFunction<T> {
    type Out = T;

    fn call(self) -> Self::Out {
        (self.0)()
    }
}
impl<T> DispatchedFunction for BoxedSendSystemFunction<T> {
    type Out = T;

    fn call(self) -> Self::Out {
        (self.0)()
    }
}

impl<Query> BoxedSystem<Query> {
    pub fn new(data: impl System<Query = Query> + 'static) -> Self {
        Self {
            data: Storage::from_boxed(Box::new(data)),
        }
    }
}

impl<'a, Q: Query + 'static> Dispatcher<SystemArgs, BoxedSystemFunction> for BoxedSystem<Q> {
    fn dispatch(
        &mut self,
        input: &mut SystemArgs,
    ) -> Result<BoxedSystemFunction, super::dispatcher::DispatchError> {
        let mut sys = self
            .data
            .try_write()
            .expect("the system should not be taken");
        let data = Q::query(input.query_mut());

        match data {
            Ok(q) => Ok(BoxedSystemFunction::new(move || sys.run(q))),
            Err(err) => Err(DispatchError::QueryError(err)),
        }
    }
}

pub struct BoxedSendSystem<Query: QuerySend>
where
    Query::Target: Send,
{
    data: Storage<dyn System<Query = Query> + Send + Sync>,
}

impl<'a, Q: QuerySend + 'static> Dispatcher<SystemArgs, BoxedSendSystemFunction>
    for BoxedSendSystem<Q>
where
    Q::Target: Send,
{
    fn dispatch(
        &mut self,
        input: &mut SystemArgs,
    ) -> Result<BoxedSendSystemFunction, super::dispatcher::DispatchError> {
        let mut sys = match self.data.try_write() {
            Ok(data) => data,
            Err(e) => return Err(DispatchError::QueryError(QueryError::StorageError(e))),
        };

        let data = Q::query(input.query_mut());

        match data {
            Ok(q) => Ok(BoxedSendSystemFunction::new(move || sys.run(q))),
            Err(err) => Err(DispatchError::QueryError(err)),
        }
    }
}
