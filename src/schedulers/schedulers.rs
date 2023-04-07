use std::sync::Arc;

use crate::dispatchers::{dispatchers::DispatcherArgs, systems::{UnsafeBoxedSystem, SystemArgs}};

pub trait Scheduler {
    fn run<'a>(
        &mut self,
        dis: &mut Vec<UnsafeBoxedSystem>,
        args: &mut DispatcherArgs<'a>,
        system_args: Arc<SystemArgs>,
    );
}
