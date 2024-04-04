use std::{any::Any, collections::HashMap, marker::PhantomData};

use crate::impl_all;

use super::{
    components::{Component, ComponentMap, ComponentRegistry}, entity::Entity, read_write_state::{RwReadState, RwState, RwWriteState}, window::WindowRegistry
};

pub trait System {
    type Args;
    type DispatcherArgs;
    fn prepare_args(
        &mut self,
        args: &mut Self::DispatcherArgs,
    ) -> Result<Self::Args, DispatchError>;
    fn update(&mut self, args: Self::Args);
}

#[derive(Clone, Copy,Debug)]
pub enum DispatchError {
    NotAvailabele,
}

pub struct SystemArgs {
    pub components: RwWriteState<ComponentRegistry>,
    pub windows: RwState<WindowRegistry>,
}

pub trait QuerySystem {
    type Query: Query<SystemArgs>;

    fn try_dispatch(&mut self, args: Self::Query) -> Result<Box<dyn Any>, DispatchError>;
    fn update(&mut self, args: <Self::Query as Query<SystemArgs>>::Data);
}

pub trait Query<Args> {
    type Data:'static;
    fn check_fetch_availability(data: &mut Args) -> bool;
    fn try_fetch(data: &mut Args) -> Result<Self::Data, DispatchError>;
}

impl<S: QuerySystem> System for S {
    type Args = Box<dyn Any>;

    type DispatcherArgs = SystemArgs;

    fn prepare_args(
        &mut self,
        args: &mut Self::DispatcherArgs,
    ) -> Result<Self::Args, DispatchError> {
        if S::Query::check_fetch_availability(args) {
            Ok(Box::new(S::Query::try_fetch(args).unwrap()))
        } else {
            Err(DispatchError::NotAvailabele)
        }
    }

    fn update(&mut self, args: Self::Args) {
        QuerySystem::update(self, *args.downcast().unwrap());
    }

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

pub struct SystemRegistry {
    send_systems: Vec<RwState<dyn System<Args = SystemArgs,DispatcherArgs = Box<dyn Any>>>>,
}
impl SystemRegistry {
    pub fn new() -> Self {
        Self {
            send_systems: Vec::new()
        }
    }

    pub fn add_system<S: System<Args = SystemArgs,DispatcherArgs = Box<dyn Any>>>(&mut self, sys: S) {
        self.send_systems.push(RwState::<_>::from(Box::new(sys)));
    }
}

impl Default for SystemRegistry {
    fn default() -> Self {

        Self::new()
    }
}


pub mod tests {
    #[test]
    fn test() {

        // let b:Box<dyn System<Args=()>> = unreachable!();
    }
}
