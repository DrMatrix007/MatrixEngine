use std::any::Any;

use crate::{
    components::{
        components::{Component, ComponentCollection, ComponentRegistry},
        resources::{Resource, ResourceHolder, ResourceRegistry},
    },
    schedulers::access::{Access, AccessAction, AccessType},
};

pub struct DispatcherArgs<'a> {
    components: &'a mut ComponentRegistry,
    resources: &'a mut ResourceRegistry,
}

impl<'a> DispatcherArgs<'a> {
    pub fn new(components: &'a mut ComponentRegistry, resources: &'a mut ResourceRegistry) -> Self {
        Self {
            components,
            resources,
        }
    }

    pub unsafe fn get_components_ptr<T: Component + 'static>(
        &mut self,
    ) -> *const ComponentCollection<T> {
        self.components.get_ptr::<T>()
    }
    pub unsafe fn get_components_ptr_mut<T: Component + 'static>(
        &mut self,
    ) -> *mut ComponentCollection<T> {
        self.components.get_ptr_mut::<T>()
    }

    pub unsafe fn get_resource_ptr<T: Resource + 'static>(&mut self) -> *const ResourceHolder<T> {
        self.resources.get_ptr::<T>()
    }
    pub unsafe fn get_resource_ptr_mut<T: Resource + 'static>(&mut self) -> *mut ResourceHolder<T> {
        self.resources.get_ptr_mut::<T>()
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct DispatchError;

pub trait Dispatcher<'a> {
    type DispatchArgs: 'a;
    type RunArgs;

    unsafe fn dispatch(&mut self, args: &mut Self::DispatchArgs) -> BoxedData;

    fn try_run(&mut self, args: Self::RunArgs, b: BoxedData) -> Result<(), BoxedData>;

    fn access() -> Access
    where
        Self: Sized;
}

pub trait DispatchData<'a> {
    type DispatcherArgs: 'a;
    type Target: 'static;

    unsafe fn dispatch(args: &mut DispatcherArgs<'a>) -> Self::Target
    where
        Self: Sized;

    fn access() -> Access
    where
        Self: Sized;
    unsafe fn from_target_to_data(data: Self::Target) -> Self
    where
        Self: Sized;
}

pub trait ExclusiveDispatchData<'a> {
    type DispatchArgs: 'a;
    type Target: 'static;

    unsafe fn dispatch(args: &mut DispatcherArgs<'a>) -> Self::Target
    where
        Self: Sized;

    unsafe fn from_target_to_data(data: Self::Target) -> Self
    where
        Self: Sized;
}

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

impl<'a, T: Component + Sync + 'static> DispatchData<'a> for &'a ComponentCollection<T> {
    type DispatcherArgs = DispatcherArgs<'a>;

    type Target = *const ComponentCollection<T>;

    unsafe fn dispatch<'b>(args: &mut Self::DispatcherArgs) -> Self::Target {
        args.get_components_ptr::<T>()
    }

    fn access() -> Access
    where
        Self: Sized,
    {
        Access::from_iter([(AccessType::component::<T>(), AccessAction::Read(1))])
    }

    unsafe fn from_target_to_data(data: Self::Target) -> Self
    where
        Self: Sized,
    {
        &*data as Self
    }
}

impl<'a, T: Component + 'static> DispatchData<'a> for &'a mut ComponentCollection<T> {
    type DispatcherArgs = DispatcherArgs<'a>;
    type Target = *mut ComponentCollection<T>;

    unsafe fn dispatch<'b>(args: &mut Self::DispatcherArgs) -> Self::Target {
        args.get_components_ptr_mut::<T>()
    }

    fn access() -> Access
    where
        Self: Sized,
    {
        Access::from_iter([(AccessType::component::<T>(), AccessAction::Write)])
    }

    unsafe fn from_target_to_data(data: Self::Target) -> Self
    where
        Self: Sized,
    {
        &mut *data as Self
    }
}

impl<'a, T: Resource + Sync + 'static> DispatchData<'a> for &'a ResourceHolder<T> {
    type DispatcherArgs = DispatcherArgs<'a>;

    type Target = *const ResourceHolder<T>;

    unsafe fn dispatch<'b>(args: &mut Self::DispatcherArgs) -> Self::Target {
        args.get_resource_ptr::<T>()
    }

    fn access() -> Access
    where
        Self: Sized,
    {
        Access::from_iter([(AccessType::resource::<T>(), AccessAction::Read(1))])
    }

    unsafe fn from_target_to_data(data: Self::Target) -> Self
    where
        Self: Sized,
    {
        &*data as Self
    }
}

impl<'a, T: Resource + 'static> DispatchData<'a> for &'a mut ResourceHolder<T> {
    type DispatcherArgs = DispatcherArgs<'a>;
    type Target = *mut ResourceHolder<T>;

    unsafe fn dispatch<'b>(args: &mut Self::DispatcherArgs) -> Self::Target {
        args.get_resource_ptr_mut::<T>()
    }

    fn access() -> Access
    where
        Self: Sized,
    {
        Access::from_iter([(AccessType::resource::<T>(), AccessAction::Write)])
    }

    unsafe fn from_target_to_data(data: Self::Target) -> Self
    where
        Self: Sized,
    {
        &mut *data as Self
    }
}
trait SingleDispatchData<'a>: DispatchData<'a> {}

impl<'a, T: Component + Sync + 'static> SingleDispatchData<'a> for &'a ComponentCollection<T> {}
impl<'a, T: Component + Sync + 'static> SingleDispatchData<'a> for &'a mut ComponentCollection<T> {}
impl<'a, T: Resource + Sync + 'static> SingleDispatchData<'a> for &'a ResourceHolder<T> {}
impl<'a, T: Resource + Sync + 'static> SingleDispatchData<'a> for &'a mut ResourceHolder<T> {}

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

        #[allow(non_snake_case)]
        impl<'a,$($t: SingleDispatchData<'a,DispatcherArgs=DispatcherArgs<'a>>,)*> DispatchData<'a> for ($($t,)*) {
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
            unsafe fn from_target_to_data(data: Self::Target) -> Self
            where
                Self: Sized,
            {
                let ($($t,)*) = data;
                ($($t::from_target_to_data($t),)*)
            }
        }
    };
}

// impl_all!(impl_tuple_dispatch_data, A, B, C);
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
// impl_all!(impl_tuple_dispatch_data, A, B, C);

impl<'a> DispatchData<'a> for &'a ComponentRegistry {
    type DispatcherArgs = DispatcherArgs<'a>;

    type Target = *const ComponentRegistry;

    unsafe fn dispatch<'b>(args: &mut Self::DispatcherArgs) -> Self::Target {
        args.components
    }

    fn access() -> Access
    where
        Self: Sized,
    {
        Access::all()
    }

    unsafe fn from_target_to_data(data: Self::Target) -> Self
    where
        Self: Sized,
    {
        &*data as Self
    }
}
impl<'a> DispatchData<'a> for &'a mut ComponentRegistry {
    type DispatcherArgs = DispatcherArgs<'a>;

    type Target = *mut ComponentRegistry;

    unsafe fn dispatch<'b>(args: &mut Self::DispatcherArgs) -> Self::Target {
        args.components
    }

    fn access() -> Access
    where
        Self: Sized,
    {
        Access::all()
    }

    unsafe fn from_target_to_data(data: Self::Target) -> Self
    where
        Self: Sized,
    {
        &mut *data as Self
    }
}

impl<'a> DispatchData<'a> for () {
    type DispatcherArgs = DispatcherArgs<'a>;

    type Target = ();

    unsafe fn dispatch(_: &mut DispatcherArgs<'a>) -> Self::Target
    where
        Self: Sized,
    {
    }

    fn access() -> Access
    where
        Self: Sized,
    {
        Access::empty()
    }

    unsafe fn from_target_to_data(_: Self::Target) -> Self
    where
        Self: Sized,
    {
    }
}

mod tests {

    #[test]
    fn test_dispatchers() {
        use crate::components::components::{Component, ComponentCollection};
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
