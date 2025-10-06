use crate::{engine::query::{Query, QueryError}, lockable::LockableError};

pub trait System: Send {
    type Registry;

    fn prepare_args(&mut self, registry: &mut Self::Registry) -> Result<(), QueryError>;

    fn run(&mut self);

    fn consume_args(&mut self, registry: &mut Self::Registry) -> Result<(), QueryError>;
}

pub trait QuerySystem<Args: Query>: Send {
    fn prepare_args(&mut self, registry: &mut Args::Registry) -> Result<Args, QueryError> {
        Args::prepare(registry)
    }

    fn run(&mut self, args: &mut Args);

    fn consume_args(
        &mut self,
        registry: &mut Args::Registry,
        args: Args,
    ) -> Result<(), QueryError> {
        args.consume(registry)
    }
}

pub struct QuerySystemHolder<Q: Query, QSystem: QuerySystem<Q>> {
    system: QSystem,
    args: Option<Q>,
}

impl<Q: Query, QSystem: QuerySystem<Q>> QuerySystemHolder<Q, QSystem> {
    pub fn new(system: QSystem) -> Self {
        Self { system, args: None }
    }
}

impl<Q: Query, QSystem: QuerySystem<Q>> System for QuerySystemHolder<Q, QSystem> {
    type Registry = Q::Registry;

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

impl<Q: Query, F: FnMut(&mut Q) + Send> QuerySystem<Q> for F {
    fn run(&mut self, args: &mut Q) {
        self(args)
    }
}
