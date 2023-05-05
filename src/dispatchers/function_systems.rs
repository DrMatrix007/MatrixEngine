use std::marker::PhantomData;

use super::{
    context::Context,
    dispatcher::{DispatchError, DispatchedData, DispatchedSendData, Dispatcher},
    systems::{AsyncSystem, BoxedData, ExclusiveSystem},
};

enum BoxedExclusiveFunction<Comps: DispatchedData> {
    WithContext(Box<dyn FnMut(&Context, Comps)>),
    NoContext(Box<dyn FnMut(Comps)>),
}

impl<Comps: DispatchedData> BoxedExclusiveFunction<Comps> {
    pub fn call(&mut self, c: &Context, comps: Comps) {
        match self {
            BoxedExclusiveFunction::WithContext(f) => {
                f(c, comps);
            }
            BoxedExclusiveFunction::NoContext(f) => {
                f(comps);
            }
        }
    }
}

pub struct BoxedExclusiveFunctionSystem<Comps: DispatchedData> {
    f: BoxedExclusiveFunction<Comps>,
    marker: PhantomData<Comps>,
}
impl<Comps> ExclusiveSystem for BoxedExclusiveFunctionSystem<Comps>
where
    Comps: DispatchedData,
{
    type Query = Comps;

    fn run(&mut self, ctx: &Context, comps: Self::Query) {
        self.f.call(ctx, comps);
    }
}

impl<Comps: DispatchedData> BoxedExclusiveFunctionSystem<Comps> {
    pub fn new_with_context(f: impl FnMut(&Context, Comps) + 'static) -> Self {
        Self {
            f: BoxedExclusiveFunction::WithContext(Box::new(f)),
            marker: PhantomData,
        }
    }
    pub fn new_without_context(f: impl FnMut(Comps) + 'static) -> Self {
        Self {
            f: BoxedExclusiveFunction::NoContext(Box::new(f)),
            marker: PhantomData,
        }
    }
}

enum BoxedAsyncFunction<Comps: DispatchedSendData> {
    WithContext(Box<dyn FnMut(&Context, Comps) + Send + Sync>),
    NoContext(Box<dyn FnMut(Comps) + Send + Sync>),
}

impl<Comps: DispatchedSendData> BoxedAsyncFunction<Comps> {
    pub fn call(&mut self, c: &Context, comps: Comps) {
        match self {
            BoxedAsyncFunction::WithContext(f) => {
                f(c, comps);
            }
            BoxedAsyncFunction::NoContext(f) => {
                f(comps);
            }
        }
    }
}

pub struct BoxedAsyncFunctionSystem<Comps: DispatchedSendData> {
    f: BoxedAsyncFunction<Comps>,
}
impl<Comps> AsyncSystem for BoxedAsyncFunctionSystem<Comps>
where
    Comps: DispatchedSendData,
{
    type Query = Comps;

    fn run(&mut self, ctx: &Context, comps: Comps) {
        self.f.call(ctx, comps);
    }
}

impl<Comps: DispatchedSendData> BoxedAsyncFunctionSystem<Comps> {
    pub fn new_with_context(f: impl FnMut(&Context, Comps) + Send + Sync + 'static) -> Self {
        Self {
            f: BoxedAsyncFunction::WithContext(Box::new(f)),
        }
    }
    pub fn new_without_context(f: impl FnMut(Comps) + Send + Sync + 'static) -> Self {
        Self {
            f: BoxedAsyncFunction::NoContext(Box::new(f)),
        }
    }
}

impl<Comps: DispatchedSendData, F: FnMut(&Context, Comps) + Send + Sync + 'static> From<F>
    for BoxedAsyncFunctionSystem<Comps>
{
    fn from(value: F) -> Self {
        BoxedAsyncFunctionSystem::new_with_context(value)
    }
}

impl<Comps: DispatchedData, F: FnMut(&Context, Comps) + 'static> From<F>
    for BoxedExclusiveFunctionSystem<Comps>
{
    fn from(value: F) -> Self {
        BoxedExclusiveFunctionSystem::new_with_context(value)
    }
}

pub trait IntoBoxedSystem<BoxedData, RunArgs> {
    type Target: Dispatcher<BoxedData, RunArgs> + ?Sized;
    fn into_system(self) -> Box<Self::Target>;
}

impl<T: Dispatcher<BoxedData, RunArgs> + 'static, RunArgs, BoxedData>
    IntoBoxedSystem<BoxedData, RunArgs> for T
{
    type Target = T;
    fn into_system(self) -> Box<T> {
        Box::new(self)
    }
}

pub trait IntoAsyncFunctionSystem<Comps: DispatchedSendData> {
    fn function_into_async_system(self) -> BoxedAsyncFunctionSystem<Comps>;
}
impl<F: FnMut(&Context, Comps) + Send + Sync + 'static, Comps: DispatchedSendData>
    IntoAsyncFunctionSystem<Comps> for F
{
    fn function_into_async_system(self) -> BoxedAsyncFunctionSystem<Comps> {
        BoxedAsyncFunctionSystem::new_with_context(self)
    }
}

pub struct Wrapper<F, Comps>(F, PhantomData<Comps>);
pub trait Wrappable<F, Comps> {
    fn wrap(self) -> Wrapper<F, Comps>;
}
impl<F: FnMut(&Context, C), C: DispatchedData> Wrappable<F, C> for F {
    fn wrap(self) -> Wrapper<F, C> {
        Wrapper(self, PhantomData)
    }
}

impl<F: FnMut(&Context, Comps), Comps: DispatchedData> ExclusiveSystem for Wrapper<F, Comps> {
    type Query = Comps;

    fn run(&mut self, ctx: &Context, comps: Self::Query) {
        self.0(ctx, comps);
    }
}

impl<F: FnMut(&Context, Comps) + Send + Sync, Comps: DispatchedSendData> AsyncSystem
    for Wrapper<F, Comps>
{
    type Query = Comps;

    fn run(&mut self, ctx: &Context, comps: Self::Query) {
        self.0(ctx, comps);
    }
}
