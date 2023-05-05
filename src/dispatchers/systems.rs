use std::{any::Any};

use super::{
    context::Context,
    dispatcher::{DispatchError, DispatchedData, DispatchedSendData, Dispatcher, DispatcherArgs},
};

pub struct BoxedData {
    data: Box<dyn Any>,
}

impl BoxedData {
    pub fn new(data: impl Any) -> Self {
        Self {
            data: Box::new(data),
        }
    }
    pub fn downcast_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.data.downcast_mut()
    }
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.data.downcast_ref()
    }
    fn downcast<T: 'static>(self) -> Option<T> {
        Some(*(self.data.downcast().ok()?))
    }
}

pub struct BoxedAsyncData {
    data: Box<dyn Any + Send + Sync>,
}

impl BoxedAsyncData {
    pub fn new(data: impl Any + Send + Sync) -> Self {
        Self {
            data: Box::new(data),
        }
    }
    pub fn downcast_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.data.downcast_mut()
    }
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.data.downcast_ref()
    }

    fn downcast<T: 'static>(self) -> Option<T> {
        Some(*(self.data.downcast().ok()?))
    }
}

pub trait ExclusiveSystem: Dispatcher<BoxedData, Context> {
    type Query: DispatchedData;

    fn run(&mut self, ctx: &Context, comps: Self::Query);
}

impl<T: ExclusiveSystem> Dispatcher<BoxedData, Context> for T {
    fn dispatch(&mut self, args: &mut DispatcherArgs<'_>) -> Result<BoxedData, DispatchError> {
        match <T::Query as DispatchedData>::dispatch(args) {
            Ok(data) => Ok(BoxedData::new(data)),
            Err(err) => Err(err),
        }
    }

    fn try_run<'a>(&mut self, args: &Context, b: BoxedData) -> Result<(), DispatchError> {
        let Some(data) = b.downcast::<<T::Query as DispatchedData>::Target>() else {
            return Err(DispatchError);
        };
        self.run(
            args,
            <T::Query as DispatchedData>::from_target_to_data(data),
        );
        Ok(())
    }
}

pub trait AsyncSystem: Dispatcher<BoxedAsyncData, Context> + Send + Sync {
    type Query: DispatchedSendData;

    fn run(&mut self, ctx: &Context, comps: <Self as AsyncSystem>::Query);
}

impl<T: AsyncSystem> Dispatcher<BoxedAsyncData, Context> for T {
    fn dispatch(&mut self, args: &mut DispatcherArgs<'_>) -> Result<BoxedAsyncData, DispatchError> {
        match <T::Query as DispatchedSendData>::dispatch(args) {
            Ok(data) => Ok(BoxedAsyncData::new(data)),
            Err(err) => Err(err),
        }
    }

    fn try_run<'a>(&mut self, args: &Context, b: BoxedAsyncData) -> Result<(), DispatchError> {
        let Some(data) = b.downcast::<<T::Query as DispatchedSendData>::Target>() else {
            return Err(DispatchError);
        };
        self.run(
            args,
            <T::Query as DispatchedSendData>::from_target_to_data(data),
        );
        Ok(())
    }
}

// pub trait IntoAsyncSystem<Q: DispatchedSendData> {
//     fn into_async_system(self) -> Box<dyn AsyncSystem<Query = Q>>;
// }

// impl<Sys: AsyncSystem> IntoAsyncSystem<Sys::Query> for Sys {
//     fn into_async_system(self) -> Box<dyn AsyncSystem<Query = Sys::Query>> {
//         Box::new(self)
//     }
// }

// pub trait IntoExclusiveSystem<Q: DispatchedData> {
//     fn into_exclusive_system(self) -> Box<dyn ExclusiveSystem<Query = Q>>;
// }

// impl<Sys: ExclusiveSystem> IntoExclusiveSystem<Sys::Query> for Sys {
//     fn into_exclusive_system(self) -> Box<dyn ExclusiveSystem<Query = Sys::Query>> {
//         Box::new(self)
//     }
// }

mod tests {

    #[test]
    #[allow(unused)]
    fn test() {
        use crate::components::component::{Component, ComponentCollection};

        use crate::dispatchers::context::Context;

        struct A;
        impl Component for A {}

        // fn a(a: &Context, b: &ComponentCollection<A>) {}
        let a = |a: &Context, b: &ComponentCollection<A>| {};
    }
}
