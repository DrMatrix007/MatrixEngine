use crate::dispatchers::{dispatchers::DispatcherArgs, systems::UnsafeBoxedDispatcher};

use super::schedulers::Scheduler;

pub struct SingleThreadScheduler;

impl Scheduler for SingleThreadScheduler {
    fn run<'a>(&mut self, dis: &mut Vec<UnsafeBoxedDispatcher>, args: &mut DispatcherArgs<'a>) {
        for i in dis {
            let data = unsafe { i.as_mut().dispatch(args) };
            i.as_mut()
                .try_run(data)
                .map_err(|_| ())
                .expect("this function should not return Err(())");
        }
    }
}
