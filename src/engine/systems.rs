use std::marker::PhantomData;

use super::{
    components::ComponentAccessError,
    entity::Entity,
    query::{Query, QueryError},
    EngineArgs,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SystemError {
    MissingArgs,
}

pub trait System<Queryable, EngineArgs> {
    fn prepare_args(
        &mut self,
        queryable: &mut Queryable,
        system_id: Entity,
    ) -> Result<(), QueryError>;

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
    fn prepare_args(
        &mut self,
        queryable: &mut Queryable,
        system_id: Entity,
    ) -> Result<(), QueryError> {
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

pub struct QuerySystemFn<Q: Query<Queryable>, Queryable, Fn: FnMut(&mut Q)>(
    Fn,
    PhantomData<(Q, Queryable)>,
);
pub struct QuerySystemFnWithArgs<
    Q: Query<Queryable>,
    Queryable,
    EngineArgs,
    Fn: FnMut(&mut EngineArgs, &mut Q),
>(Fn, PhantomData<(Q, Queryable, EngineArgs)>);

impl<Q: Query<Queryable>, Queryable, EngineArgs, Fn: FnMut(&mut EngineArgs, &mut Q)>
    QuerySystemFnWithArgs<Q, Queryable, EngineArgs, Fn>
{
    pub fn new(f: Fn) -> Self {
        Self(f, PhantomData)
    }
}

impl<Q: Query<Queryable>, Queryable, Fn: FnMut(&mut Q)> QuerySystemFn<Q, Queryable, Fn> {
    pub fn new(f: Fn) -> Self {
        Self(f, PhantomData)
    }
}

impl<Q: Query<Queryable>, Queryable, Fn: FnMut(&mut Q), EngineArgs>
    QuerySystem<Queryable, EngineArgs> for QuerySystemFn<Q, Queryable, Fn>
{
    type Query = Q;

    fn run(&mut self, _engine_args: &mut EngineArgs, args: &mut Self::Query) {
        (self.0)(args);
    }
}
impl<Q: Query<Queryable>, Queryable, Fn: FnMut(&mut EngineArgs, &mut Q), EngineArgs>
    QuerySystem<Queryable, EngineArgs> for QuerySystemFnWithArgs<Q, Queryable, EngineArgs, Fn>
{
    type Query = Q;

    fn run(&mut self, engine_args: &mut EngineArgs, args: &mut Self::Query) {
        (self.0)(engine_args, args);
    }
}

pub trait IntoNonSendSystem<Queryable, EngineArgs, Placeholder> {
    fn into_system(self) -> impl System<Queryable, EngineArgs>;
}

pub struct FnPlaceHolderNonSend<Q: Query<Queryable>, Queryable>(PhantomData<(Q, Queryable)>);

impl<Q: Query<Queryable>, Queryable, EngineArgs, F: FnMut(&mut Q)>
    IntoNonSendSystem<Queryable, EngineArgs, FnPlaceHolderNonSend<Q, Queryable>> for F
{
    fn into_system(self) -> impl System<Queryable, EngineArgs> {
        QuerySystemWrapper::new(QuerySystemFn::new(self))
    }
}
pub struct QsNonSendPlaceHolder<Q: Query<Queryable>, Queryable>(PhantomData<(Q, Queryable)>);
impl<
        Q: Query<Queryable>,
        Queryable,
        EngineArgs,
        QS: QuerySystem<Queryable, EngineArgs, Query = Q>,
    > IntoNonSendSystem<Queryable, EngineArgs, QsNonSendPlaceHolder<Q, Queryable>> for QS
{
    fn into_system(self) -> impl System<Queryable, EngineArgs> {
        QuerySystemWrapper::new(self)
    }
}
pub trait IntoSendSystem<Queryable, EngineArgs:Send, Placeholder> {
    fn into_system(self) -> impl System<Queryable, EngineArgs> + Send;
}

pub struct FnSendPlaceHolder<Q: Query<Queryable> + Send, Queryable: Send>(
    PhantomData<(Q, Queryable)>,
);
impl<Q: Query<Queryable> + Send, Queryable: Send, EngineArgs: Send, F: FnMut(&mut Q) + Send>
    IntoSendSystem<Queryable, EngineArgs, FnPlaceHolderNonSend<Q, Queryable>> for F
{
    fn into_system(self) -> impl System<Queryable, EngineArgs> + Send {
        QuerySystemWrapper::new(QuerySystemFn::new(self))
    }
}
pub struct FnSendPlaceHolderWithArgs<Q: Query<Queryable> + Send, Queryable: Send>(
    PhantomData<(Q, Queryable)>,
);
impl<
        Q: Query<Queryable> + Send,
        Queryable: Send,
        EngineArgs: Send,
        F: FnMut(&mut EngineArgs, &mut Q) + Send,
    > IntoSendSystem<Queryable, EngineArgs, FnSendPlaceHolderWithArgs<Q, Queryable>> for F
{
    fn into_system(self) -> impl System<Queryable, EngineArgs> + Send {
        QuerySystemWrapper::new(QuerySystemFnWithArgs::new(self))
    }
}

pub struct FnNonSendPlaceHolderWithArgs<Q: Query<Queryable>, Queryable>(
    PhantomData<(Q, Queryable)>,
);
impl<Q: Query<Queryable>, Queryable, EngineArgs, F: FnMut(&mut EngineArgs, &mut Q)>
    IntoNonSendSystem<Queryable, EngineArgs, FnNonSendPlaceHolderWithArgs<Q, Queryable>> for F
{
    fn into_system(self) -> impl System<Queryable, EngineArgs> {
        QuerySystemWrapper::new(QuerySystemFnWithArgs::new(self))
    }
}
pub struct QsSendPlaceHolder<Q: Query<Queryable> + Send, Queryable: Send>(
    PhantomData<(Q, Queryable)>,
);
impl<
        Q: Query<Queryable> + Send,
        Queryable: Send,
        EngineArgs: Send,
        QS: QuerySystem<Queryable, EngineArgs, Query = Q> + Send,
    > IntoSendSystem<Queryable, EngineArgs, QsSendPlaceHolder<Q, Queryable>> for QS
{
    fn into_system(self) -> impl System<Queryable, EngineArgs> + Send {
        QuerySystemWrapper::new(self)
    }
}

pub struct SystemRegistry<Queryable, SendEngineArgs: Send, NonSendEngineArgs> {
    send_systems: Vec<BoxedSendSystem<Queryable, SendEngineArgs>>,
    non_send_systems: Vec<BoxedNonSendSystem<Queryable, NonSendEngineArgs>>,
}

impl<Queryable, SendEngineArgs: Send, NonSendEngineArgs>
    SystemRegistry<Queryable, SendEngineArgs, NonSendEngineArgs>
{
    pub fn new() -> Self {
        Self {
            send_systems: Vec::new(),
            non_send_systems: Vec::new(),
        }
    }
    pub fn add_send_system(&mut self, system: Box<dyn System<Queryable, SendEngineArgs> + Send>) {
        self.send_systems.push(BoxedSendSystem::new(system));
    }

    pub fn add_non_send_system(&mut self, system: Box<dyn System<Queryable, NonSendEngineArgs>>) {
        self.non_send_systems.push(BoxedNonSendSystem::new(system));
    }

    pub fn send_systems(&self) -> &Vec<BoxedSendSystem<Queryable, SendEngineArgs>> {
        &self.send_systems
    }

    pub fn send_systems_mut(&mut self) -> &mut Vec<BoxedSendSystem<Queryable, SendEngineArgs>> {
        &mut self.send_systems
    }

    pub fn non_send_systems(&self) -> &Vec<BoxedNonSendSystem<Queryable, NonSendEngineArgs>> {
        &self.non_send_systems
    }

    pub fn non_send_systems_mut(
        &mut self,
    ) -> &mut Vec<BoxedNonSendSystem<Queryable, NonSendEngineArgs>> {
        &mut self.non_send_systems
    }
}

impl<Queryable, SendEngineArgs: Send, NonSendEngineArgs> Default
    for SystemRegistry<Queryable, SendEngineArgs, NonSendEngineArgs>
{
    fn default() -> Self {
        Self::new()
    }
}

pub struct BoxedSendSystem<Queryable, Args: Send> {
    id: Entity,
    system: Box<dyn System<Queryable, Args> + Send>,
}

impl<Queryable, Args: Send> BoxedSendSystem<Queryable, Args> {
    pub fn new(system: Box<dyn System<Queryable, Args> + Send>) -> Self {
        Self {
            id: Entity::new(),
            system,
        }
    }
    pub fn system(&self) -> &dyn System<Queryable, Args> {
        &*self.system
    }
    pub fn system_mut(&mut self) -> &mut dyn System<Queryable, Args> {
        &mut *self.system
    }
    pub fn id(&self) -> &Entity {
        &self.id
    }

    pub fn prepare_args(&mut self, queryable: &mut Queryable) -> Result<(), QueryError> {
        self.system.prepare_args(queryable, self.id)
    }
    pub fn run(&mut self, args: &mut Args) -> Result<(), SystemError> {
        self.system.run(args)
    }
    pub fn consume(&mut self, queryable: &mut Queryable) -> Result<(), ComponentAccessError> {
        self.system.consume(queryable)
    }
}

pub struct BoxedNonSendSystem<Queryable, Args> {
    id: Entity,
    system: Box<dyn System<Queryable, Args>>,
}

impl<Queryable, Args> BoxedNonSendSystem<Queryable, Args> {
    pub fn new(system: Box<dyn System<Queryable, Args>>) -> Self {
        Self {
            id: Entity::new(),
            system,
        }
    }
    pub fn system(&self) -> &dyn System<Queryable, Args> {
        &*self.system
    }
    pub fn system_mut(&mut self) -> &mut dyn System<Queryable, Args> {
        &mut *self.system
    }
    pub fn id(&self) -> &Entity {
        &self.id
    }

    pub fn prepare_args(&mut self, queryable: &mut Queryable) -> Result<(), QueryError> {
        self.system.prepare_args(queryable, self.id)
    }
    pub fn run(&mut self, args: &mut Args) -> Result<(), SystemError> {
        self.system.run(args)
    }
    pub fn consume(&mut self, queryable: &mut Queryable) -> Result<(), ComponentAccessError> {
        self.system.consume(queryable)
    }
}
#[cfg(test)]
mod test {
    use crate::engine::{
        components::ComponentRegistry, entity::Entity, event_registry::EventRegistry, query::{ReadC, WriteC}, scene::SceneRegistry
    };

    use super::{IntoSendSystem, QuerySystem, System};

    fn system_a(_data: &mut ReadC<()>) {}

    struct A;
    impl QuerySystem<SceneRegistry, ()> for A {
        type Query = WriteC<()>;

        fn run(&mut self, _engine_args: &mut (), _args: &mut Self::Query) {}
    }

    fn system_b(_args: &mut (), _data: &mut ReadC<()>) {}

    fn system_boxed<T: 'static>(
        a: impl IntoSendSystem<SceneRegistry, (), T> + 'static,
    ) -> Box<dyn System<SceneRegistry, ()>> {
        Box::new(a.into_system())
    }

    #[test]
    fn test_systems() {
        let reg = ComponentRegistry::new();

        let reg = &mut SceneRegistry { components: reg,events: EventRegistry::new() };

        let mut b: Box<dyn System<SceneRegistry, ()>> = system_boxed(system_a);
        let mut c: Box<dyn System<SceneRegistry, ()>> = system_boxed(system_b);

        let mut d: Box<dyn System<SceneRegistry, ()>> = system_boxed(A);
        let mut e: Box<dyn System<SceneRegistry, ()>> = system_boxed(A);

        b.prepare_args(reg, Entity::new()).unwrap();
        c.prepare_args(reg, Entity::new()).unwrap();
        d.prepare_args(reg, Entity::new()).unwrap_err();

        b.run(&mut ()).unwrap();
        c.run(&mut ()).unwrap();

        b.consume(reg).unwrap();
        c.consume(reg).unwrap();

        d.prepare_args(reg, Entity::new()).unwrap();
        e.prepare_args(reg, Entity::new()).unwrap_err();

        d.run(&mut ()).unwrap();

        d.consume(reg).unwrap();
    }
}
