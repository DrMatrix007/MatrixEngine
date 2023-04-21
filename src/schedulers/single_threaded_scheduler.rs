use std::sync::Arc;

use crate::dispatchers::{
    dispatchers::DispatcherArgs, system_registry::SystemGroup, systems::SystemArgs,
};

use super::schedulers::Scheduler;

pub struct SingleThreadScheduler;

impl Scheduler for SingleThreadScheduler {
    fn run<'a>(
        &mut self,
        dis: &mut SystemGroup,
        args: &mut DispatcherArgs<'a>,
        system_args: Arc<SystemArgs>,
    ) {
        for i in dis.iter_normal() {
            let data = unsafe { i.as_mut().dispatch(args) };
            i.as_mut()
                .try_run(system_args.clone(), data)
                .map_err(|_| ())
                .expect("this function should not return Err(())");
        }
        for i in dis.iter_exclusive() {
            let data = unsafe { i.as_mut().dispatch(args) };
            i.as_mut()
                .try_run(system_args.clone(), data)
                .map_err(|_| ())
                .expect("this function should not return Err(())");
        }
    }
}
