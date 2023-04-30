use crate::dispatchers::{dispatcher::DispatcherArgs, system_registry::SystemGroup};

use super::scheduler::Scheduler;

pub struct SingleThreadScheduler;

impl Scheduler for SingleThreadScheduler {
    fn run(&mut self, dis: &mut SystemGroup, args: &mut DispatcherArgs<'_>) {
        for i in dis.iter_normal() {
            let mut data = i
                .as_mut()
                .dispatch(args)
                .expect("this runs only on the main thread and its should not crash");
            i.try_run(&mut data)
                .map_err(|_| ())
                .expect("this function should not return Err(())");
        }
        for i in dis.iter_exclusive() {
            let mut data = i
                .as_mut()
                .dispatch(args)
                .expect("this runs only on the main thread and its should not crash");
            i.try_run(&mut data)
                .map_err(|_| ())
                .expect("this function should not return Err(())");
        }
    }
}
