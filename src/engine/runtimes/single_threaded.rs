use crate::timing::Stopwatch;

use super::Runtime;

pub struct SingleThreaded;

impl<Q, S: Send, N> Runtime<Q, S, N> for SingleThreaded {
    fn run(
        &mut self,
        systems: &mut crate::engine::systems::SystemRegistry<Q, S, N>,
        queryable: &mut Q,
        mut send_engine_args: S,
        mut non_send_egnine_args: N,
    ) {
        // let mut stopwatch = Stopwatch::new("systems");
        for i in systems.send_systems_mut() {
            i.prepare_args(queryable).unwrap();
            // stopwatch.debug_elapesd();
            i.run(&mut send_engine_args).unwrap();
            // stopwatch.debug_elapesd();
            i.consume(queryable).unwrap();
            // stopwatch.debug_elapesd();
            // println!();
        }
        for i in systems.non_send_systems_mut() {
            i.prepare_args(queryable).unwrap();
            // stopwatch.debug_elapesd();
            i.run(&mut non_send_egnine_args).unwrap();
            // stopwatch.debug_elapesd();
            i.consume(queryable).unwrap();
            // stopwatch.debug_elapesd();
            // println!();
        }

        // println!("\n\n\n\n\n");
    }
}
