use std::marker::PhantomData;

use crate::{
    engine::query::{Query, QueryError},
    impl_all, utils::lockable::LockableError,
};

pub trait System: Send {
    type Registry;

    fn prepare_args(&mut self, registry: &mut Self::Registry) -> Result<(), QueryError>;

    fn run(&mut self);

    fn consume_args(&mut self, registry: &mut Self::Registry) -> Result<(), QueryError>;
}

pub trait QuerySystem<Registry, Args: Query<Registry>>: Send {
    fn prepare_args(&mut self, registry: &mut Registry) -> Result<Args, QueryError> {
        Args::prepare(registry)
    }

    fn run(&mut self, args: &mut Args);

    fn consume_args(&mut self, registry: &mut Registry, args: Args) -> Result<(), QueryError> {
        args.consume(registry)
    }
}

pub struct QuerySystemHolder<Registry, Q: Query<Registry>, QSystem: QuerySystem<Registry, Q>> {
    system: QSystem,
    args: Option<Q>,
    marker: PhantomData<Registry>,
}
unsafe impl<Registry, Q: Query<Registry> + Send, QSystem: QuerySystem<Registry, Q> + Send> Send
    for QuerySystemHolder<Registry, Q, QSystem>
{
}

impl<Registry, Q: Query<Registry>, QSystem: QuerySystem<Registry, Q>>
    QuerySystemHolder<Registry, Q, QSystem>
{
    pub fn new(system: QSystem) -> Self {
        Self {
            system,
            args: None,
            marker: PhantomData,
        }
    }
}

impl<Registry, Q: Query<Registry>, QSystem: QuerySystem<Registry, Q>> System
    for QuerySystemHolder<Registry, Q, QSystem>
{
    type Registry = Registry;

    fn prepare_args(&mut self, registry: &mut Self::Registry) -> Result<(), QueryError> {
        self.args = Some(self.system.prepare_args(registry)?);
        Ok(())
    }

    fn run(&mut self) {
        self.system
            .run(self.args.as_mut().expect("the args were empty"));
    }

    fn consume_args(&mut self, registry: &mut Self::Registry) -> Result<(), QueryError> {
        if let Some(args) = self.args.take() {
            self.system.consume_args(registry, args)?;
            Ok(())
        } else {
            Err(QueryError::LockableError(LockableError::NotAvailable))
        }
    }
}

macro_rules! impl_fn_query {
    ($($t:ident),+) => {
        #[allow(non_snake_case)]
        impl<Reg,$($t: Query<Reg>),+, Function: FnMut($(&mut $t),+) + Send> QuerySystem<Reg, ($($t,)+)> for Function {
            fn run(&mut self, args: &mut ($($t,)+)) {
                let ($($t,)+) = args;
                self($($t,)+);
            }
        }
    };
}

impl_all!(mini impl_fn_query);

impl<Reg, Function: FnMut() + Send> QuerySystem<Reg, ()> for Function {
    fn run(&mut self, _: &mut ()) {
        self()
    }
}
