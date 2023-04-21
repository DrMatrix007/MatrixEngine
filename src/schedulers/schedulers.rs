use std::sync::Arc;

use crate::dispatchers::{
    dispatchers::DispatcherArgs, system_registry::SystemGroup, systems::SystemArgs,
};

pub trait Scheduler {
    fn run<'a>(
        &mut self,
        dis: &mut SystemGroup,
        args: &mut DispatcherArgs<'a>,
        system_args: Arc<SystemArgs>,
    );
}
