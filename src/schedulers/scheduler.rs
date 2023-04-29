use std::sync::Arc;

use crate::dispatchers::{
    dispatcher::DispatcherArgs, system_registry::SystemGroup, systems::SystemArgs,
};

pub trait Scheduler {
    fn run(
        &mut self,
        dis: &mut SystemGroup,
        args: &mut DispatcherArgs<'_>,
        system_args: Arc<SystemArgs>,
    );
}
