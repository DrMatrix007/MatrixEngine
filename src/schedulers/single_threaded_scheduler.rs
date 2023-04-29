use std::sync::Arc;

use crate::dispatchers::{system_registry::SystemGroup, systems::SystemArgs, dispatchers::DispatcherArgs};

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
            let mut data = i
                .as_mut()
                .dispatch(args)
                .expect("this runs only on the main thread and its should not crash");
            i.as_mut()
                .try_run(system_args.clone(), &mut data)
                .map_err(|_| ())
                .expect("this function should not return Err(())");
        }
        for i in dis.iter_exclusive() {
            let mut data = i
                .as_mut()
                .dispatch(args)
                .expect("this runs only on the main thread and its should not crash");
            i.as_mut()
                .try_run(system_args.clone(), &mut data)
                .map_err(|_| ())
                .expect("this function should not return Err(())");
        }
    }
}
