use crate::dispatchers::system::BoxedSystemFunction;

use super::query::QueryError;
use std::collections::VecDeque;

pub enum DispatchError {
    QueryError(QueryError),
}

pub trait Dispatcher<In, FOut: DispatchedFunction> {
    fn dispatch(&mut self, input: &mut In) -> Result<FOut, DispatchError>;
}

pub trait DispatchedFunction {
    type Out;
    fn call(self) -> Self::Out;
}

pub struct DispatcherCollection<In, Func: DispatchedFunction> {
    data: VecDeque<Box<dyn Dispatcher<In, Func>>>,
}

impl<In, Func: DispatchedFunction> Default for DispatcherCollection<In, Func> {
    fn default() -> Self {
        Self {
            data: Default::default(),
        }
    }
}

impl<In, Func: DispatchedFunction> DispatcherCollection<In, Func> {
    pub fn push_back(&mut self, dispatcher: impl Dispatcher<In, Func> + 'static) {
        self.data.push_back(Box::new(dispatcher));
    }
    pub fn pop_back(&mut self) -> Option<Box<dyn Dispatcher<In, Func>>> {
        self.data.pop_back()
    }
}
#[test]
fn test() {
    struct A;

    impl Dispatcher<(), BoxedSystemFunction> for A {
        fn dispatch(&mut self, _input: &mut ()) -> Result<BoxedSystemFunction, DispatchError> {
            Ok(BoxedSystemFunction::new(|| {}))
        }
    }
}
