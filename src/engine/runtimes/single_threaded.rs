use super::Runtime;

pub struct SingleThreaded;

impl<Q, S: Send, N> Runtime<Q, S, N> for SingleThreaded {
    fn run(
        &mut self,
        systems: &mut crate::engine::systems::SystemRegistry<Q, S, N>,
        queryable: &mut Q,
        send_engine_args: S,
        non_send_egnine_args: N,
    ) {
        for i in systems.send_systems_mut() {
            i.prepare_args(queryable).unwrap();
            i.run(&send_engine_args).unwrap();
            i.consume(queryable).unwrap();
        }
        for i in systems.non_send_systems_mut() {
            i.prepare_args(queryable).unwrap();
            i.run(&non_send_egnine_args).unwrap();
            i.consume(queryable).unwrap();
            // stopwatch.debug_elapesd();
            // println!();
        }

        // println!("\n\n\n\n\n");
    }
}
