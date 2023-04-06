use std::{
    any::{Any, TypeId},
    marker::PhantomData,
};

use crate::{
    components::components::{Component, ComponentCollection, ComponentRegistry},
    scene::Scene,
    schedulers::access::{Access, AccessAction, AccessType},
};

use super::systems::System;


pub struct DispatcherArgs<'a>{
    components: &'a mut ComponentRegistry,
}

impl<'a> DispatcherArgs<'a> {
    pub unsafe fn get_components_ptr<T:Component+'static>(&mut self) -> *const ComponentCollection<T> {
        self.components.get_ptr::<T>()
    }
    pub unsafe fn get_components_ptr_mut <T:Component+'static>(&mut self) -> *mut ComponentCollection<T> {
        self.components.get_ptr_mut::<T>()
    }
}


#[derive(Debug, Clone, Copy, Default)]
pub struct DispatchError;

pub trait Dispatcher<'a> {
    type DispatchArgs:'a;

    unsafe fn dispatch(&mut self, args: &mut Self::DispatchArgs) -> BoxedData where Self:Sized ;

    fn try_run(&mut self, b: BoxedData) -> Result<(), BoxedData>  where Self:Sized;

    fn access() -> Access where Self:Sized;
}

pub trait DispatchData<'a> {

    type DispatcherArgs:'a;
    type Target: 'static;

    unsafe fn dispatch(args: &mut DispatcherArgs<'a>) -> Self::Target
    where
        Self: Sized;

    fn access() -> Access
    where
        Self: Sized;
}

struct _Single<T>(PhantomData<T>);

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
    pub fn new<T: 'static>(t: T) -> Self {
        Self { data: Box::new(t) }
    }
}

unsafe impl Send for BoxedData {}

impl<'a, S: System<'a> + 'static> Dispatcher<'a> for S
where
S::Query: DispatchData<'a,DispatcherArgs = DispatcherArgs<'a>>,
{
    unsafe fn dispatch(&mut self, args: &mut Self::DispatchArgs) -> BoxedData {
        BoxedData::new(<<S as System<'a>>::Query as DispatchData<'a>>::dispatch(
            args,
        ))
    }

    fn try_run(&mut self, b: BoxedData) -> Result<(), BoxedData> {
        self.run(*(b.downcast()?));
        Ok(())
    }
    fn access() -> Access
    where
        Self: Sized,
    {
        <Self as System>::Query::access()
    }

    type DispatchArgs = DispatcherArgs<'a>;
}

impl<'a, T: Component + 'static> DispatchData<'a> for &'a ComponentCollection<T> {
    type DispatcherArgs = DispatcherArgs<'a>;

    type Target = *const ComponentCollection<T>;

    unsafe fn dispatch<'b>(mut args: &mut Self::DispatcherArgs) -> Self::Target {
        args.get_components_ptr::<T>()
    }

    fn access() -> Access
    where
        Self: Sized,
    {
        Access::from_iter([(AccessType::component::<T>(), AccessAction::Read(1))])
    }
}

impl<'a, T: Component + 'static> DispatchData<'a> for &'a mut ComponentCollection<T> {
    type DispatcherArgs = DispatcherArgs<'a>;
    type Target = *mut ComponentCollection<T>;

    unsafe fn dispatch<'b>(mut args: &mut Self::DispatcherArgs) -> Self::Target {
        args.get_components_ptr_mut::<T>()
    }

    fn access() -> Access
    where
        Self: Sized,
    {
        Access::from_iter([(AccessType::component::<T>(), AccessAction::Write)])
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
        impl<'a,$($t: DispatchData<'a,DispatcherArgs=DispatcherArgs<'a>>,)*> DispatchData<'a> for ($($t,)*) {
            type Target = ($($t::Target,)*);
            type DispatcherArgs = DispatcherArgs<'a>;
            unsafe fn dispatch(scene:&mut Self::DispatcherArgs) -> Self::Target {
                ($($t::dispatch(scene),)*)
            }
            fn access()-> Access where Self:Sized {
                let mut ans = Access::default();
                $(ans.try_combine(&$t::access()).expect("the access should not overlap");)*
                ans
            }
        }
    };
}

// impl_tuple_dispatch_data!(A,);
// impl_all!(
//     impl_tuple_dispatch_data,
//     A,
//     B,
//     C,
//     D,
//     E,
//     F,
//     G,
//     H,
//     I,
//     J,
//     K,
//     L,
//     M,
//     N,
//     O,
//     P,
//     Q,
//     R,
//     S,
//     T,
//     U,
//     V,
//     W,
//     X,
//     Y,
//     Z
// );
// impl_tuple_dispatch_data!(A,B,C);
impl_all!(impl_tuple_dispatch_data,A,B,C);
mod tests {

    #[test]
    fn test_dispatchers() {
        use crate::{
            components::components::{Component, ComponentCollection},
            dispatchers::dispatchers::DispatchData,
        };
        struct A;
        impl Component for A {}

        struct B;
        impl Component for B {}

        type Q1 = (
            &'static mut ComponentCollection<A>,
            &'static ComponentCollection<B>,
        );
        type Q2 = (&'static ComponentCollection<B>,);

        // Q1::access().try_combine(&Q2::access()).unwrap();
    }
}
