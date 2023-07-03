use crate::par::storage::Storage;

use super::{
    dispatcher::{BoxedFunction, BoxedSendFunction, DispatchError, Dispatcher},
    query::{Query, QueryArgs, QuerySend},
};

pub struct SystemArgs<'a> {
    query: QueryArgs<'a>,
}

impl<'a> SystemArgs<'a> {
    pub fn query_mut(&mut self) -> &mut QueryArgs<'a> {
        &mut self.query
    }

    pub fn query(&self) -> &QueryArgs<'a> {
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

impl<Query> BoxedSystem<Query> {
    pub fn new(data: impl System<Query = Query> + 'static) -> Self {
        Self {
            data: Storage::from_boxed(Box::new(data)),
        }
    }
}

impl<'a, Q: Query + 'static> Dispatcher<SystemArgs<'a>, BoxedFunction> for BoxedSystem<Q> {
    fn dispatch(
        &mut self,
        input: &mut SystemArgs<'a>,
    ) -> Result<BoxedFunction, super::dispatcher::DispatchError> {
        let mut sys = self
            .data
            .try_write()
            .expect("the system should not be taken");
        let data = Q::query(input.query_mut());

        match data {
            Ok(q) => Ok(Box::new(move || sys.run(q))),
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

impl<'a, Q: QuerySend + 'static> Dispatcher<SystemArgs<'a>, BoxedSendFunction>
    for BoxedSendSystem<Q>
where
    Q::Target: Send,
{
    fn dispatch(
        &mut self,
        input: &mut SystemArgs<'a>,
    ) -> Result<BoxedSendFunction, super::dispatcher::DispatchError> {
        let mut sys = self
            .data
            .try_write()
            .expect("the system should not be taken");
        let data = Q::query(input.query_mut());

        match data {
            Ok(q) => Ok(Box::new(move || sys.run(q))),
            Err(err) => Err(DispatchError::QueryError(err)),
        }
    }
}
