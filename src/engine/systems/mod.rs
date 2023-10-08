use std::any::Any;

use self::{
    query::{ComponentQueryArgs, Query, QueryCleanup, QueryError, QuerySend},
    system_registry::{SystemRef, SystemSendRef},
};

pub mod query;
pub mod system_registry;

pub enum SystemControlFlow {
    Continue,
    Quit,
    Remove,
}

pub trait Dispatcher<Args> {
    type Result;
    fn dispatch(
        self,
        args: &mut Args,
    ) -> Result<Box<dyn FnOnce() -> Self::Result>, (Self, QueryError)>
    where
        Self: Sized;
}

pub trait DispatcherSend<Args>: Dispatcher<Args>
where
    Self::Result: Send,
{
    fn dispatch_send(
        self,
        args: &mut Args,
    ) -> Result<Box<dyn FnOnce() -> Self::Result + Send>, (Self, QueryError)>
    where
        Self: Sized;
}
pub trait System<Args = ComponentQueryArgs> {
    fn prepare_args(&self, args: &mut Args) -> Result<Box<dyn Any>, QueryError>;
    fn run_boxed_args(
        &mut self,
        args: Box<dyn Any>,
    ) -> Result<(Box<dyn QueryCleanup<Args>>, SystemControlFlow), ()>;
}

pub trait QuerySystem<Args = ComponentQueryArgs>: System<Args> {
    type Query: Query<Args>;

    fn run(&mut self, args: &mut Self::Query) -> SystemControlFlow;
}

pub trait SystemSend<Args>: Send + System<Args> {
    fn prepare_args_send(&self, args: &mut Args) -> Result<Box<dyn Any + Send + Sync>, QueryError>;

    fn run_boxed_args_send(
        &mut self,
        args: Box<dyn Any + Send + Sync>,
    ) -> Result<(Box<dyn QueryCleanup<Args> + Send + Sync>, SystemControlFlow), ()>;
}

impl<Args, S: QuerySystem<Args> + Send> System<Args> for S {
    fn prepare_args(&self, args: &mut Args) -> Result<Box<dyn Any>, QueryError> {
        Ok(<S::Query as Query<Args>>::get(args).map(|x| Box::new(x))?)
    }

    fn run_boxed_args(
        &mut self,
        args: Box<dyn Any>,
    ) -> Result<(Box<dyn QueryCleanup<Args>>, SystemControlFlow), ()> {
        match args.downcast() {
            Ok(mut args) => {
                let state = self.run(&mut args);
                Ok((args, state))
            }
            Err(_) => Err(()),
        }
    }
}
impl<Args: 'static, S: QuerySystem<Args> + Send> SystemSend<Args> for S
where
    S::Query: QuerySend<Args>,
{
    fn prepare_args_send(&self, args: &mut Args) -> Result<Box<dyn Any + Send + Sync>, QueryError> {
        Ok(<S::Query as QuerySend<Args>>::get(args).map(|x| Box::new(x))?)
    }

    fn run_boxed_args_send(
        &mut self,
        args: Box<dyn Any + Send + Sync>,
    ) -> Result<(Box<dyn QueryCleanup<Args> + Send + Sync>, SystemControlFlow), ()> {
        match args.downcast() {
            Ok(mut args) => {
                let state = self.run(&mut args);
                Ok((args, state))
            }
            Err(_) => Err(()),
        }
    }
}

impl<Args: 'static> Dispatcher<Args> for SystemRef<Args> {
    type Result = (
        Box<dyn QueryCleanup<Args>>,
        (SystemRef<Args>, SystemControlFlow),
    );
    fn dispatch(
        mut self,
        args: &mut Args,
    ) -> Result<Box<dyn FnOnce() -> Self::Result>, (Self, QueryError)> {
        let args = match unsafe { self.system_mut() }.prepare_args(args) {
            Ok(b) => b,
            Err(e) => {
                return Err((self, e));
            }
        };
        Ok(Box::new(move || {
            let (cleanup, flow) = unsafe { self.system_mut() }.run_boxed_args(args).unwrap();
            (cleanup, (self, flow))
        }))
    }
}
impl<Args: 'static> Dispatcher<Args> for SystemSendRef<Args> {
    type Result = (
        Box<dyn QueryCleanup<Args> + Send + Sync>,
        (SystemSendRef<Args>, SystemControlFlow),
    );

    fn dispatch(
        mut self,
        args: &mut Args,
    ) -> Result<Box<dyn FnOnce() -> Self::Result>, (Self, QueryError)> {
        let args = match unsafe { self.system_mut() }.prepare_args_send(args) {
            Ok(b) => b,
            Err(e) => {
                return Err((self, e));
            }
        };
        Ok(Box::new(move || {
            let (cleanup, flow) = unsafe { self.system_mut() }
                .run_boxed_args_send(args)
                .unwrap();
            (cleanup, (self, flow))
        }))
    }
}

impl<Args: 'static> DispatcherSend<Args> for SystemSendRef<Args> {
    fn dispatch_send(
        mut self,
        args: &mut Args,
    ) -> Result<Box<dyn FnOnce() -> Self::Result + Send>, (Self, QueryError)> {
        let args = match unsafe { self.system_mut() }.prepare_args_send(args) {
            Ok(b) => b,
            Err(e) => {
                return Err((self, e));
            }
        };
        Ok(Box::new(move || {
            let (cleanup, flow) = unsafe { self.system_mut() }
                .run_boxed_args_send(args)
                .unwrap();
            (cleanup, (self, flow))
        }))
    }
}

mod tests {

    use std::sync::Arc;

    use tokio::sync::Mutex;

    use crate::engine::scenes::components::Component;

    use super::{query::components::ReadC, QuerySystem, SystemControlFlow, SystemSend};

    struct A;
    impl Component for A {}

    struct SysA;

    impl QuerySystem for SysA {
        type Query = ReadC<A>;

        fn run(&mut self, args: &mut Self::Query) -> SystemControlFlow {
            SystemControlFlow::Continue
        }
    }

    fn t() {
        let s = SysA;

        let b: Arc<Mutex<dyn SystemSend<_>>> = Arc::new(Mutex::new(s));

        b.blocking_lock_owned();
    }
}
