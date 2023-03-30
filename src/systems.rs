use std::marker::PhantomData;

use crate::{
    components::ComponentRegistry,
    dispatchers::{DispatchData, Dispatcher, IntoDispatcher}, scene::Scene,
};

pub trait StartupSystem : Dispatcher<DispatchArgs = Scene> {
    type Query: DispatchData;
    fn startup(&mut self, comps: <Self::Query as DispatchData>::Target<'_>);
}

pub struct BoxedFunction<Q: DispatchData> {
    f: Box<dyn FnMut(Q)>,
}

impl<Q: DispatchData> BoxedFunction<Q> {
    pub fn call(&mut self, q: Q) {
        (self.f)(q);
    }
}

trait StartupSystemFunction : StartupSystem<Query = <Self as StartupSystemFunction>::Q> {
    type Q;
}

// impl<T,Data:DispatchData> StartupSystemFunction for T where T:FnMut(Data) {

// }

impl<Data: DispatchData, T: FnMut(Data) + 'static> From<T> for BoxedFunction<Data> {
    fn from(value: T) -> Self {
        BoxedFunction { f: Box::new(value) }
    }
}