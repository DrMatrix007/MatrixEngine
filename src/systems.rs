use crate::{
    dispatchers::{DispatchData, Dispatcher},
    scene::Scene,
};

pub trait StartupSystem: Dispatcher<DispatchArgs = Scene> {
    type Query<'a>: DispatchData;
    fn startup<'a>(&mut self, comps: Self::Query<'a>);
}
pub struct BoxedFunction<Q: DispatchData> {
    f: Box<dyn FnMut(Q)>,
}

impl<Q: DispatchData> BoxedFunction<Q> {
    pub fn call(&mut self, q: Q) {
        (self.f)(q);
    }
}
pub trait System: Dispatcher<DispatchArgs = Scene> {
    type Query<'a>: DispatchData;

    fn update<'a>(&mut self, comps: Self::Query<'a>);
}
