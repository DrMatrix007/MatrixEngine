use crate::dispatchers::{dispatcher::DispatcherArgs, system_registry::SystemGroup};

pub trait Scheduler {
    fn run(&mut self, dis: &mut SystemGroup, args: &mut DispatcherArgs<'_>);
}
