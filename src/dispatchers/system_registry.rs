use super::{
    dispatcher::DispatcherCollection,
    system::{BoxedSystemFunction, SystemArgs},
};

#[derive(Default)]
pub struct SystemRegistry {
    exclusive_system: DispatcherCollection<SystemArgs, BoxedSystemFunction>,
    async_systems: DispatcherCollection<SystemArgs, BoxedSystemFunction>,
}
