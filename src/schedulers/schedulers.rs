use crate::dispatchers::{dispatchers::DispatcherArgs, systems::UnsafeBoxedDispatcher};

pub trait Scheduler {
    fn run<'a>(&mut self, dis: &mut Vec<UnsafeBoxedDispatcher>, args: &mut DispatcherArgs<'a>);
}
