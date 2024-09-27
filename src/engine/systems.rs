use std::{any::Any, marker::PhantomData};

use super::{
    components::ComponentAccessError,
    query::{Query, QueryError},
    scene::SceneRegistry,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SystemError {
    MissingArgs,
}

pub trait System<Queryable, EngineArgs> {
    fn prepare_args(&mut self, queryable: &mut Queryable) -> Result<(), QueryError>;

    fn run(&mut self, run_args: &mut EngineArgs) -> Result<(), SystemError>;

    fn consume(&mut self, queryable: &mut Queryable) -> Result<(), ComponentAccessError>;
}

pub trait QuerySystem<Queryable, EngineArgs> {
    type Query: Query<Queryable>;
    fn run(&mut self, engine_args: &mut EngineArgs, args: &mut Self::Query);
}
pub struct QuerySystemWrapper<
    Q: Query<Queryable>,
    Queryable,
    EngineArgs,
    QS: QuerySystem<Queryable, EngineArgs, Query = Q>,
> {
    system: QS,
    args: Option<Q>,
    marker: PhantomData<(Queryable, EngineArgs)>,
}

impl<
        Q: Query<Queryable>,
        Queryable,
        EngineArgs,
        QS: QuerySystem<Queryable, EngineArgs, Query = Q>,
    > QuerySystemWrapper<Q, Queryable, EngineArgs, QS>
{
    pub fn new(system: QS) -> Self {
        Self {
            system,
            args: None,
            marker: PhantomData,
        }
    }
}

impl<
        Q: Query<Queryable>,
        Queryable,
        EngineArgs,
        QS: QuerySystem<Queryable, EngineArgs, Query = Q>,
    > System<Queryable, EngineArgs> for QuerySystemWrapper<Q, Queryable, EngineArgs, QS>
{
    fn prepare_args(&mut self, queryable: &mut Queryable) -> Result<(), QueryError> {
        assert!(self.args.is_none());
        self.args = Some(Q::query(queryable)?);
        Ok(())
    }

    fn run(&mut self, engine_args: &mut EngineArgs) -> Result<(), SystemError> {
        let args = self.args.as_mut();
        if let Some(args) = args {
            self.system.run(engine_args, args);
            Ok(())
        } else {
            Err(SystemError::MissingArgs)
        }
    }

    fn consume(&mut self, queryable: &mut Queryable) -> Result<(), ComponentAccessError> {
        self.args.take().unwrap().consume(queryable)
    }
}

pub struct FnWrapper<Q: Query<SceneRegistry>, Fn: FnMut(&mut Q)>(Fn, PhantomData<Q>);

impl<Q: Query<SceneRegistry>, Fn: FnMut(&mut Q)> FnWrapper<Q, Fn> {
    pub fn new(f: Fn) -> Self {
        Self(f, PhantomData)
    }
}

impl<Q: Query<SceneRegistry>, Fn: FnMut(&mut Q)> QuerySystem<SceneRegistry, ()>
    for FnWrapper<Q, Fn>
{
    type Query = Q;

    fn run(&mut self, _engine_args: &mut (), args: &mut Self::Query) {
        (self.0)(args);
    }
}

#[cfg(test)]
mod test {
    use crate::engine::{query::ReadC, scene::SceneRegistry};

    use super::{FnWrapper, QuerySystem, QuerySystemWrapper, System};

    fn system_a(data: &mut ReadC<()>) {}

    #[test]
    fn test_r() {
        

        let b: Box<dyn System<SceneRegistry, ()>> =
            Box::new(QuerySystemWrapper::new(FnWrapper::new(system_a)));

        
    }
}
