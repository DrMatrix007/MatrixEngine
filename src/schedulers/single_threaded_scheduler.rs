use std::sync::Arc;

use crate::dispatchers::{
    dispatchers::DispatcherArgs,
    systems::{SystemArgs, UnsafeBoxedSystem},
};

use super::schedulers::Scheduler;

pub struct SingleThreadScheduler;

impl Scheduler for SingleThreadScheduler {
    fn run<'a>(
        &mut self,
        dis: &mut Vec<UnsafeBoxedSystem>,
        args: &mut DispatcherArgs<'a>,
        system_args: Arc<SystemArgs>,
    ) {
        for i in dis {
            let data = unsafe { i.as_mut().dispatch(args) };
            i.as_mut()
                .try_run(system_args.clone(),data)
                .map_err(|_| ())
                .expect("this function should not return Err(())");
        }
    }
}
