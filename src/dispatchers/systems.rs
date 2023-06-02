use std::any::Any;

use super::{
    context::Context,
    dispatcher::{DispatchError, DispatchedData, DispatchedSendData, Dispatcher, DispatcherArgs},
};

pub struct ExclusiveBoxedData {
    data: Box<dyn Any>,
}

impl ExclusiveBoxedData {
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

pub struct AsyncBoxedData {
    data: Box<dyn Any + Send + Sync>,
}

impl AsyncBoxedData {
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

pub trait ExclusiveSystem: Dispatcher<ExclusiveBoxedData, Context> {
    type Query: DispatchedData;

    fn run(&mut self, ctx: &Context, comps: &mut Self::Query);
}
pub trait IntoExclusiveSystem<Marker = ()> {
    fn into_exclusive_system(self) -> Box<dyn Dispatcher<ExclusiveBoxedData, Context>>;
}
pub struct ExclusiveMarker;
impl<T: ExclusiveSystem+'static> IntoExclusiveSystem<ExclusiveMarker> for T {
    fn into_exclusive_system(self) -> Box<dyn Dispatcher<ExclusiveBoxedData, Context>> {
        Box::new(self)
    }
}

impl<T: ExclusiveSystem> Dispatcher<ExclusiveBoxedData, Context> for T {
    fn dispatch(
        &mut self,
        args: &mut DispatcherArgs<'_>,
    ) -> Result<ExclusiveBoxedData, DispatchError> {
        match <T::Query as DispatchedData>::dispatch(args) {
            Ok(data) => Ok(ExclusiveBoxedData::new(data)),
            Err(err) => Err(err),
        }
    }

    fn try_run<'a>(&mut self, args: &Context, b: ExclusiveBoxedData) -> Result<(), DispatchError> {
        let Some(data) = b.downcast::<<T::Query as DispatchedData>::Target>() else {
            return Err(DispatchError::WrongBoxedData);
        };
        self.run(
            args,
            &mut <T::Query as DispatchedData>::from_target_to_data(data),
        );
        Ok(())
    }
}

pub trait AsyncSystem: Dispatcher<AsyncBoxedData, Context> + Send {
    type Query: DispatchedSendData;

    fn run(&mut self, ctx: &Context, comps: &mut <Self as AsyncSystem>::Query);
}
pub trait IntoAsyncSystem<Marker = ()> {
    fn into_async_system(self) -> Box<dyn Dispatcher<AsyncBoxedData, Context>+Send>;
}
pub struct AsyncMarker;

impl<T: AsyncSystem+'static> IntoAsyncSystem<AsyncMarker> for T {
    fn into_async_system(self) -> Box<dyn Dispatcher<AsyncBoxedData, Context>+Send> {
        Box::new(self)
    }
}

impl<T: AsyncSystem> Dispatcher<AsyncBoxedData, Context> for T {
    fn dispatch(&mut self, args: &mut DispatcherArgs<'_>) -> Result<AsyncBoxedData, DispatchError> {
        match <T::Query as DispatchedSendData>::dispatch(args) {
            Ok(data) => Ok(AsyncBoxedData::new(data)),
            Err(err) => Err(err),
        }
    }

    fn try_run<'a>(&mut self, args: &Context, b: AsyncBoxedData) -> Result<(), DispatchError> {
        let Some(data) = b.downcast::<<T::Query as DispatchedSendData>::Target>() else {
            return Err(DispatchError::WrongBoxedData);
        };
        self.run(
            args,
            &mut <T::Query as DispatchedSendData>::from_target_to_data(data),
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
