use super::Runtime;

pub struct SingleThreaded;

impl<Q,S,N> Runtime<Q,S,N> for SingleThreaded {
    fn run(
        &mut self,
        systems: &mut crate::engine::systems::SystemRegistry<Q, S, N>,
        queryable: &mut Q,
        mut send_engine_args:S,
        mut non_send_egnine_args:N,
        
    ) {
        for i in systems.send_systems_mut() {
            i.prepare_args(queryable).unwrap();
            i.run(&mut send_engine_args).unwrap();
            i.consume(queryable).unwrap();
        }
        for i in systems.non_send_systems_mut() {
            i.prepare_args(queryable).unwrap();
            i.run(&mut non_send_egnine_args).unwrap();
            i.consume(queryable).unwrap();
        }
    }
}