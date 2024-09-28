pub mod single_threaded;

use super::systems::SystemRegistry;

pub trait Runtime<Queryable, SendEngineArgs: Send, NonSendEngineArgs> {
    fn run(
        &mut self,
        systems: &mut SystemRegistry<Queryable, SendEngineArgs, NonSendEngineArgs>,
        queryable: &mut Queryable,
        send_args: SendEngineArgs,
        non_send_args: NonSendEngineArgs,
    );
}
