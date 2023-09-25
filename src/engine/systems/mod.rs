use std::any::Any;

use tokio::sync::OwnedMutexGuard;

use self::query::{ComponentQueryArgs, Query, QueryError, QuerySend};

pub mod query;
pub mod system_registry;

pub trait Dispatcher<Args> {
    fn dispatch(self, args: &mut Args) -> Result<Box<dyn FnOnce()>, (Self,QueryError)> where Self:Sized;
}

pub trait DispatcherSend<Args>: Dispatcher<Args> {
    fn dispatch_send(self, args: &mut Args) -> Result<Box<dyn FnOnce() + Send>, (Self,QueryError)>where Self:Sized;
}
pub trait System<Args = ComponentQueryArgs> {
    fn query(&self, args: &mut Args) -> Result<Box<dyn Any>, QueryError>;
    fn run_boxed_args(&mut self, args: Box<dyn Any>) -> Result<(), ()>;
}

pub trait SceneSystem<Args = ComponentQueryArgs>: System<Args> {
    type Query: Query<Args>;

    fn run(&mut self, args: &mut <Self::Query as Query<Args>>::Target);
}

pub trait SystemSend<Args>: Send + System<Args> {
    fn query_send(&self, args: &mut Args) -> Result<Box<dyn Any + Send + Sync>, QueryError>;
}

impl<Args, S: SceneSystem<Args> + Send> System<Args> for S {
    fn query(&self, args: &mut Args) -> Result<Box<dyn Any>, QueryError> {
        Ok(<S::Query as Query<Args>>::get(args).map(|x| Box::new(x))?)
    }

    fn run_boxed_args(&mut self, mut args: Box<dyn Any>) -> Result<(), ()> {
        match args.downcast_mut() {
            Some(mut args) => {
                self.run(&mut args);
                Ok(())
            }
            None => Err(()),
        }
    }
}
impl<Args: 'static, S: SceneSystem<Args> + Send> SystemSend<Args> for S
where
    <S::Query as Query<Args>>::Target: Send + Sync,
    S::Query: QuerySend<Args>,
{
    fn query_send(&self, args: &mut Args) -> Result<Box<dyn Any + Send + Sync>, QueryError> {
        Ok(<S::Query as Query<Args>>::get(args).map(|x| Box::new(x))?)
    }
}

impl<Args: 'static> Dispatcher<Args> for OwnedMutexGuard<dyn System<Args>> {
    fn dispatch(mut self, args: &mut Args) -> Result<Box<dyn FnOnce()>, (Self,QueryError)> {
        let args = match self.query(args) {
            Ok(b) => {b},
            Err(e) => {return Err((self,e));},
        };
        Ok(Box::new(move || {
            self.run_boxed_args(args).unwrap();
        }))
    }
}
impl<Args: 'static> Dispatcher<Args> for OwnedMutexGuard<dyn SystemSend<Args>> {
    fn dispatch(mut self, args: &mut Args) -> Result<Box<dyn FnOnce()>, (Self,QueryError)> {
        let args = match self.query(args) {
            Ok(b) => {b},
            Err(e) => {return Err((self,e));},
        };
        Ok(Box::new(move || {
            self.run_boxed_args(args).unwrap();
        }))
    }
}

impl<Args:'static> DispatcherSend<Args> for OwnedMutexGuard<dyn SystemSend<Args>> {
    fn dispatch_send(mut self, args: &mut Args) -> Result<Box<dyn FnOnce() + Send>, (Self,QueryError)> {
        let args = match self.query_send(args) {
            Ok(b) => {b},
            Err(e) => {return Err((self,e));},
        };
        Ok(Box::new(move || {
            self.run_boxed_args(args).unwrap();
        }))
    }
}

mod tests {

    use std::sync::Arc;

    use tokio::sync::Mutex;

    use crate::engine::scenes::components::Component;

    use super::{query::ReadC, SceneSystem, SystemSend};

    struct A;
    impl Component for A {}

    struct SysA;

    impl SceneSystem for SysA {
        type Query = ReadC<A>;

        fn run(
            &mut self,
            args: &mut <Self::Query as super::query::Query<super::query::ComponentQueryArgs>>::Target,
        ) {
        }
    }

    fn t() {
        let s = SysA;

        let b: Arc<Mutex<dyn SystemSend<_>>> = Arc::new(Mutex::new(s));

        b.blocking_lock_owned();
    }
}
