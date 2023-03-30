use core::panic;
use std::{cell::Ref, marker::PhantomData, sync::Arc};

use crate::{
    components::{Component, ComponentCollection, ComponentRegistry},
    scene::Scene,
    systems::StartupSystem,
};

#[derive(Debug, Clone, Copy, Default)]
pub struct DispatchError;

pub trait Dispatcher {
    type DispatchArgs;
    unsafe fn dispatch<'a>(&mut self, args: &'a mut Self::DispatchArgs);
}
pub struct BoxedDispatcher {}

pub trait DispatchData {
    type DispatchArgs;

    type Target<'a>;
    unsafe fn dispatch(args: &mut Self::DispatchArgs) -> Self::Target<'_>;
}

pub struct ReadColl<T>(PhantomData<T>);
pub struct WriteColl<T>(PhantomData<T>);

struct _Single<T>(PhantomData<T>);

impl<T: Component + 'static> DispatchData for ReadColl<T> {
    type DispatchArgs = Scene;
    type Target<'a> = &'a ComponentCollection<T>;

    unsafe fn dispatch(args: &mut Scene) -> Self::Target<'_> {
        &*args.get_component_registry().get_ptr::<T>() as &'_ ComponentCollection<T>
    }
}

impl<T: Component + 'static> DispatchData for WriteColl<T> {
    type DispatchArgs = Scene;

    type Target<'a> = &'a mut ComponentCollection<T>;

    unsafe fn dispatch(args: &mut Self::DispatchArgs) -> Self::Target<'_> {
        &mut *args.get_component_registry().get_ptr_mut::<T>() as &mut ComponentCollection<T>
    }
}

impl<S: StartupSystem> Dispatcher for S
where
    S::Query: DispatchData<DispatchArgs = Scene>,
{
    type DispatchArgs = Scene;

    unsafe fn dispatch<'a>(&mut self, args: &mut Scene) {
        let a = <<S as StartupSystem>::Query as DispatchData>::dispatch(args);
        self.startup(a);
    }
}

pub trait IntoDispatcher {
    type Target;
    fn into_dispatcher(self) -> Self::Target;
}
