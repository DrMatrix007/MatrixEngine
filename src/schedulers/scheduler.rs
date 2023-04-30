use std::sync::Arc;

use crate::dispatchers::{
    dispatcher::DispatcherArgs, system_registry::SystemGroup, systems::SystemContext,
};

pub trait Scheduler {
    fn run(
        &mut self,
        dis: &mut SystemGroup,
        args: &mut DispatcherArgs<'_>,
    );
}
