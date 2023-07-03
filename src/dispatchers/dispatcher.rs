use std::marker::PhantomData;

use crate::par::storage::Storage;

use super::query::QueryError;

pub enum DispatchError {
    QueryError(QueryError),
}

pub trait Dispatcher<In, FOut: DispatchedFunction> {
    fn dispatch(
        &mut self,
        input: &mut In,
    ) -> Result<FOut, DispatchError>;
}

pub trait DispatchedFunction {
    type Out;
    fn call(self) -> Self::Out;
}

pub type BoxedFunction<Out = ()> = Box<dyn FnOnce() -> Out>;
pub type BoxedSendFunction<Out = ()> = Box<dyn FnOnce() -> Out + Send>;

impl<T> DispatchedFunction for BoxedFunction<T> {
    type Out = T;

    fn call(self) -> Self::Out {
        (self)()
    }
}
impl<T> DispatchedFunction for BoxedSendFunction<T> {
    type Out = T;

    fn call(self) -> Self::Out {
        (self)()
    }
}

#[test]
fn test() {
    struct A;

    impl Dispatcher<(), Box<dyn FnOnce() + Send>> for A {
        fn dispatch(&mut self, _input: &mut ()) -> Result<Box<dyn FnOnce() + Send>, DispatchError> {
            Ok(Box::new(|| {}))
        }
    }
}
