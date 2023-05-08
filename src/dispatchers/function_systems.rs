use std::marker::PhantomData;

use super::{
    context::Context,
    dispatcher::{DispatchedData, DispatchedSendData, Dispatcher},
    systems::{AsyncSystem, ExclusiveSystem},
};

type BoxedFunction<Comps> = Box<dyn FnMut(&Context, &mut Comps)>;

pub struct BoxedExclusiveFunctionSystem<Comps: DispatchedData> {
    f: BoxedFunction<Comps>,
    marker: PhantomData<Comps>,
}
impl<Comps> ExclusiveSystem for BoxedExclusiveFunctionSystem<Comps>
where
    Comps: DispatchedData,
{
    type Query = Comps;

    fn run(&mut self, ctx: &Context, comps: &mut Self::Query) {
        (self.f)(ctx, comps);
    }
}

impl<Comps: DispatchedData> BoxedExclusiveFunctionSystem<Comps> {
    pub fn new(f: impl FnMut(&Context, &mut Comps) + 'static) -> Self {
        Self {
            f: Box::new(f),
            marker: PhantomData,
        }
    }
}

type BoxedAsyncFunction<Comps> = Box<dyn FnMut(&Context, &mut Comps) + Send + Sync>;

pub struct BoxedAsyncFunctionSystem<Comps: DispatchedSendData> {
    f: BoxedAsyncFunction<Comps>,
}

impl<Comps> AsyncSystem for BoxedAsyncFunctionSystem<Comps>
where
    Comps: DispatchedSendData,
{
    type Query = Comps;

    fn run(&mut self, ctx: &Context, comps: &mut Comps) {
        (self.f)(ctx, comps);
    }
}

impl<Comps: DispatchedSendData> BoxedAsyncFunctionSystem<Comps> {
    pub fn new(f: impl FnMut(&Context, &mut Comps) + Send + Sync + 'static) -> Self {
        Self { f: Box::new(f) }
    }
}

impl<Comps: DispatchedSendData, F: FnMut(&Context, &mut Comps) + Send + Sync + 'static> From<F>
    for BoxedAsyncFunctionSystem<Comps>
{
    fn from(value: F) -> Self {
        BoxedAsyncFunctionSystem::new(value)
    }
}

impl<Comps: DispatchedData, F: FnMut(&Context, &mut Comps) + 'static> From<F>
    for BoxedExclusiveFunctionSystem<Comps>
{
    fn from(value: F) -> Self {
        BoxedExclusiveFunctionSystem::new(value)
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
impl<F: FnMut(&Context, &mut Comps) + Send + Sync + 'static, Comps: DispatchedSendData>
    IntoAsyncFunctionSystem<Comps> for F
{
    fn function_into_async_system(self) -> BoxedAsyncFunctionSystem<Comps> {
        BoxedAsyncFunctionSystem::new(self)
    }
}

pub struct Wrapper<F, Comps>(F, PhantomData<Comps>);
pub trait Wrappable<F, Comps> {
    fn wrap(self) -> Wrapper<F, Comps>;
}
impl<F: FnMut(&Context, &mut C), C: DispatchedData> Wrappable<F, C> for F {
    fn wrap(self) -> Wrapper<F, C> {
        Wrapper(self, PhantomData)
    }
}

impl<F: FnMut(&Context,&mut  Comps), Comps: DispatchedData> ExclusiveSystem for Wrapper<F, Comps> {
    type Query = Comps;

    fn run(&mut self, ctx: &Context, comps: &mut Self::Query) {
        self.0(ctx, comps);
    }
}

impl<F: FnMut(&Context, &mut Comps) + Send + Sync, Comps: DispatchedSendData> AsyncSystem
    for Wrapper<F, Comps>
{
    type Query = Comps;

    fn run(&mut self, ctx: &Context, comps: &mut Self::Query) {
        self.0(ctx, comps);
    }
}
