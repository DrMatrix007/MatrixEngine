use std::{
    any::{Any, TypeId},
    marker::PhantomData,
};

use crate::{
    access::{Access, AccessType},
    components::{Component, ComponentCollection},
    scene::Scene,
    systems::System,
};

#[derive(Debug, Clone, Copy, Default)]
pub struct DispatchError;

pub trait Dispatcher {
    type DispatchArgs;
    unsafe fn dispatch<'a>(&mut self, args: &'a mut Self::DispatchArgs) -> BoxedData;

    fn try_run(&mut self, b: BoxedData) -> Result<(), BoxedData>;

    fn access(&self) -> Access;
}

pub trait DispatchData {
    type DispatchArgs;

    unsafe fn dispatch(args: &mut Self::DispatchArgs) -> Self;

    fn access() -> Access;
}

struct _Single<T>(PhantomData<T>);

#[derive(Debug)]
pub struct BoxedData {
    pub data: Box<dyn Any>,
}

impl BoxedData {
    pub fn downcast<T: 'static>(self) -> Result<Box<T>, BoxedData> {
        match self.data.downcast::<T>() {
            Ok(data) => Ok(data),
            Err(data) => Err(Self { data }),
        }
    }
}

impl<T: 'static> From<Box<T>> for BoxedData {
    fn from(data: Box<T>) -> Self {
        Self { data }
    }
}
unsafe impl Send for BoxedData {}

impl<S: System + 'static> Dispatcher for S
where
    for<'a> S::Query<'a>: DispatchData<DispatchArgs = Scene>,
{
    type DispatchArgs = Scene;

    unsafe fn dispatch<'a>(&mut self, args: &'a mut Scene) -> BoxedData {
        Box::new(<<S as System>::Query<'static> as DispatchData>::dispatch(
            args,
        ))
        .into()
    }

    fn access(&self) -> Access {
        S::Query::<'static>::access()
    }

    fn try_run(&mut self, b: BoxedData) -> Result<(), BoxedData> {
        self.run(*(b.downcast()?));
        Ok(())
    }
}

impl<'a, T: Component + 'static> DispatchData for &'a ComponentCollection<T> {
    type DispatchArgs = Scene;

    unsafe fn dispatch(args: &mut Scene) -> Self {
        &*args.component_registry_mut().get_ptr::<T>() as Self
    }

    fn access() -> Access {
        Access::from_iter([(TypeId::of::<T>(), AccessType::Read(1))])
    }
}

impl<'a, T: Component + 'static> DispatchData for &'a mut ComponentCollection<T> {
    type DispatchArgs = Scene;

    unsafe fn dispatch(args: &mut Self::DispatchArgs) -> Self {
        &mut *args.component_registry_mut().get_ptr_mut::<T>() as Self
    }

    fn access() -> Access {
        Access::from_iter([(TypeId::of::<T>(), AccessType::Write)])
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
            fn access()-> Access {
                let mut ans = Access::default();
                $(ans.try_combine(&$t::access()).expect("the access should not overlap");)*
                ans
            }
        }
    };
}

// impl_tuple_dispatch_data!(A,);
impl_all!(
    impl_tuple_dispatch_data,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z
);
// impl_tuple_dispatch_data!(A,B,C);

mod tests {

    #[test]
    fn test_dispatchers() {
        use crate::components::{Component, ComponentCollection};
        use crate::dispatchers::DispatchData;
        struct A;
        impl Component for A {}

        struct B;
        impl Component for B {}

        type Q1 = (
            &'static mut ComponentCollection<A>,
            &'static ComponentCollection<B>,
        );
        type Q2 = (&'static ComponentCollection<B>,);

        Q1::access().try_combine(&Q2::access()).unwrap();
    }
}
