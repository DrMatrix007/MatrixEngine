use std::{any::Any, marker::PhantomData};

use crate::impl_all;

use super::{
    components::{Component, ComponentMap, ComponentRegistry},
    read_write_state::{RwReadState, RwWriteState},
};

pub trait System {
    type Args;
    type DispatcherArgs;
    fn prepare_args(
        &mut self,
        args: &mut Self::DispatcherArgs,
    ) -> Result<Self::Args, DispatchError>;
    fn update(&mut self, args: Self::Args);
    fn consume();
}

#[derive(Clone, Copy)]
pub enum DispatchError {
    NotAvailabele,
}

pub struct SystemArgs {
    pub components: RwWriteState<ComponentRegistry>,
}

pub trait QuerySystem {
    type Query: Query<SystemArgs>;

    fn try_dispatch(&mut self, args: Self::Query) -> Result<Box<dyn Any>, DispatchError>;
    fn update(&mut self, args: <Self::Query as Query<SystemArgs>>::Data);
}

pub trait Query<Args> {
    type Data;
    fn check_fetch_availability(data: &mut Args) -> bool;
    fn try_fetch(data: &mut Args) -> Result<Self::Data, DispatchError>;
}

pub struct ReadC<T>(PhantomData<T>);
pub struct WriteC<T>(PhantomData<T>);

impl<T: Component> Query<SystemArgs> for ReadC<T> {
    type Data = RwReadState<ComponentMap<T>>;

    fn try_fetch(data: &mut SystemArgs) -> Result<Self::Data, DispatchError> {
        match data.components.get::<T>().read() {
            Ok(data) => Ok(data),
            Err(_) => Err(DispatchError::NotAvailabele),
        }
    }

    fn check_fetch_availability(data: &mut SystemArgs) -> bool {
        data.components.get::<T>().can_read()
    }
}

impl<T: Component> Query<SystemArgs> for WriteC<T> {
    type Data = RwWriteState<ComponentMap<T>>;

    fn try_fetch(data: &mut SystemArgs) -> Result<Self::Data, DispatchError> {
        match data.components.get::<T>().write() {
            Ok(data) => Ok(data),
            Err(_) => Err(DispatchError::NotAvailabele),
        }
    }
    fn check_fetch_availability(data: &mut SystemArgs) -> bool {
        data.components.get::<T>().can_write()
    }
}

macro_rules! impl_query {
    ($($t:ident),+) => {
        impl<Args,$($t:Query<Args>,)+> Query<Args> for ($($t,)+) {
            type Data = ($($t::Data,)+);

            fn try_fetch(data:&mut Args) -> Result<Self::Data,DispatchError> {
                Ok(($($t::try_fetch(data)?,)+))
            }
            fn check_fetch_availability(data:&mut Args) -> bool {
                ($($t::check_fetch_availability(data))&&+)
            }

        }
    };
}

// impl_query!(A, B, C);
impl_all!(impl_query);

pub mod tests {
    #[test]
    fn test() {

        // let b:Box<dyn System<Args=()>> = unreachable!();
    }
}
