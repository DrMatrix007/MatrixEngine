use std::marker::PhantomData;

use crate::{
    access::Access,
    components::{Component, ComponentCollection},
    scene::Scene,
    systems::{StartupSystem, System},
};

#[derive(Debug, Clone, Copy, Default)]
pub struct DispatchError;

pub trait Dispatcher {
    type DispatchArgs;
    unsafe fn dispatch<'a>(&mut self, args: &'a mut Self::DispatchArgs);

    fn access(&mut self) -> &Access;
}

pub trait DispatchData {
    type DispatchArgs;

    unsafe fn dispatch(args: &mut Self::DispatchArgs) -> Self;
}

struct _Single<T>(PhantomData<T>);

impl<S: StartupSystem> Dispatcher for S
where
    for<'a> S::Query<'a>: DispatchData<DispatchArgs = Scene>,
{
    type DispatchArgs = Scene;

    unsafe fn dispatch<'a>(&mut self, args: &'a mut Scene) {
        let a = <<S as StartupSystem>::Query<'a> as DispatchData>::dispatch(args);
        self.startup(a);
    }

    fn access(&mut self) -> &Access {
        todo!()
    }
}

impl<S: System> Dispatcher for S
where
    for<'a> S::Query<'a>: DispatchData<DispatchArgs = Scene>,
{
    type DispatchArgs = Scene;

    unsafe fn dispatch<'a>(&mut self, args: &'a mut Scene) {
        let a = <<S as System>::Query<'a> as DispatchData>::dispatch(args);
        self.update(a);
    }

    fn access(&mut self) -> &Access {
        todo!()
    }
}

impl<'a, T: Component + 'static> DispatchData for &'a ComponentCollection<T> {
    type DispatchArgs = Scene;

    unsafe fn dispatch(args: &mut Scene) -> Self {
        &*args.component_registry_mut().get_ptr::<T>() as Self
    }
}

impl<'a, T: Component + 'static> DispatchData for &'a mut ComponentCollection<T> {
    type DispatchArgs = Scene;

    unsafe fn dispatch(args: &mut Self::DispatchArgs) -> Self {
        &mut *args.component_registry_mut().get_ptr_mut::<T>() as Self
    }
}


macro_rules! impl_all {
    ($mac:ident, $t:ident, $($ts:ident),+) => {
        $mac!($t,$($ts),*);
        impl_all!($mac,$($ts),+);
    };
    ($mac:ident, $t:ident) => {
        $mac!($t);
    }
}

macro_rules! impl_tuple_dispatch_data {
    ($($t:ident),*) => {
        impl<$($t:DispatchData<DispatchArgs=Scene>,)*> DispatchData for ($($t,)*) {
            type DispatchArgs = Scene;
            unsafe fn dispatch(scene: &mut <Self as DispatchData>::DispatchArgs) -> Self {
                ($($t::dispatch(scene),)*)
            }
        }
    };
}

// impl_tuple_dispatch_data!(A,);
impl_all!(impl_tuple_dispatch_data,A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);
// impl_tuple_dispatch_data!(A,B,C);
